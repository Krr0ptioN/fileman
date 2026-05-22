use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use gpui::{
    App, AppContext, Application, Bounds, Context, FocusHandle, InteractiveElement, IntoElement,
    KeyDownEvent, ParentElement, Render, Styled, Window, WindowBounds, WindowOptions, px, size,
};
use gpui_component::{Root, h_flex, v_flex};

use crate::core;
use crate::features::file_browser::tokens;
use crate::features::file_browser::{
    BrowserPanel, ClipboardKind, ClipboardOp, FileOperation, FileRow, FileTarget, FilemanAssets,
    InputMode, PaneMode, PanelSide, PendingConfirm, delete_status, render_command_bar,
    render_panel, render_title_bar, selection_status, toggle_targets,
};
use crate::features::keybind::{
    BrowserCommand, HeldNavigation, command_char_from_key, navigation_input,
};
use crate::features::vim_keys::{VimCommandState, VimCommandStep};

pub fn run(start_path: Option<PathBuf>) {
    Application::new()
        .with_assets(FilemanAssets)
        .run(move |app: &mut App| {
            gpui_component::init(app);
            let start_path = start_path.clone();

            let bounds = Bounds::centered(None, size(px(1180.0), px(720.0)), app);
            app.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    app_id: Some("com.fileman.gpui".to_string()),
                    window_min_size: Some(size(px(820.0), px(520.0))),
                    ..Default::default()
                },
                |window, cx| {
                    window.set_window_title("FileMan GPUI");
                    let shell = cx.new(|cx| FilemanShell::new(cx.focus_handle(), start_path, cx));
                    shell.read(cx).focus_handle.focus(window);
                    cx.new(|cx| Root::new(shell, window, cx))
                },
            )
            .expect("failed to open GPUI window");

            app.activate(true);
        });
}

pub struct FilemanShell {
    left: BrowserPanel,
    right: BrowserPanel,
    active: PanelSide,
    pub(crate) focus_handle: FocusHandle,
    vim_command: VimCommandState,
    clipboard: Option<ClipboardOp>,
    input_mode: InputMode,
    pending_confirm: Option<PendingConfirm>,
    pane_mode: PaneMode,
    held_navigation: HeldNavigation,
    operation_in_flight: bool,
    status: String,
}

impl FilemanShell {
    pub(crate) fn new(
        focus_handle: FocusHandle,
        start_path: Option<PathBuf>,
        cx: &mut Context<Self>,
    ) -> Self {
        let start_path = start_path
            .or_else(|| std::env::current_dir().ok())
            .unwrap_or_else(|| PathBuf::from("."));
        let mut shell = Self {
            left: BrowserPanel {
                side: PanelSide::Left,
                title: "Primary",
                path: start_path.clone(),
                selected_index: 0,
                rows: Vec::new(),
                marked: HashSet::new(),
                loading: false,
                error: None,
                load_generation: 0,
                scroll_handle: Default::default(),
            },
            right: BrowserPanel {
                side: PanelSide::Right,
                title: "Secondary",
                path: start_path.clone(),
                selected_index: 0,
                rows: Vec::new(),
                marked: HashSet::new(),
                loading: false,
                error: None,
                load_generation: 0,
                scroll_handle: Default::default(),
            },
            active: PanelSide::Left,
            focus_handle,
            vim_command: VimCommandState::default(),
            clipboard: None,
            input_mode: InputMode::Normal,
            pending_confirm: None,
            pane_mode: PaneMode::Dual,
            held_navigation: HeldNavigation::default(),
            operation_in_flight: false,
            status: "normal".to_string(),
        };
        shell.load_panel(PanelSide::Left, start_path.clone(), None, cx);
        shell.load_panel(PanelSide::Right, start_path, None, cx);
        shell
    }

    fn on_key_down(&mut self, event: &KeyDownEvent, window: &mut Window, cx: &mut Context<Self>) {
        if self.handle_key_command(event, cx) {
            Self::consume_key_event(window, cx);
        }
    }

    fn handle_key_command(&mut self, event: &KeyDownEvent, cx: &mut Context<Self>) -> bool {
        if self.handle_modal_key(event, cx) || self.handle_control_key(event) {
            self.held_navigation.reset();
            return true;
        }

        if self.handle_navigation_key(event) {
            return true;
        }

        self.handle_vim_key(event, cx)
    }

    fn handle_modal_key(&mut self, event: &KeyDownEvent, cx: &mut Context<Self>) -> bool {
        self.handle_input_mode_key(event, cx) || self.handle_confirm_key(event, cx)
    }

    fn handle_vim_key(&mut self, event: &KeyDownEvent, cx: &mut Context<Self>) -> bool {
        self.held_navigation.reset();
        let Some(ch) = command_char_from_key(event) else {
            return false;
        };

        self.apply_vim_char(ch, cx)
    }

    fn consume_key_event(window: &mut Window, cx: &mut Context<Self>) {
        window.prevent_default();
        cx.stop_propagation();
        cx.notify();
    }

    fn handle_navigation_key(&mut self, event: &KeyDownEvent) -> bool {
        let Some(input) = navigation_input(event) else {
            return false;
        };
        let (key, rows) = self.held_navigation.rows_for(input);

        self.active_panel_mut().select_relative(key.delta(rows));
        self.status = format!(
            "{} -> {}",
            event.keystroke.key,
            self.active_panel().selected_name()
        );
        true
    }

    fn handle_input_mode_key(&mut self, event: &KeyDownEvent, cx: &mut Context<Self>) -> bool {
        let InputMode::Rename { target, input } = &mut self.input_mode else {
            return false;
        };

        if event.is_held {
            return true;
        }

        match event.keystroke.key.as_str() {
            "escape" => {
                self.input_mode = InputMode::Normal;
                self.status = "rename cancelled".to_string();
                return true;
            }
            "backspace" => {
                input.pop();
                self.status = format!("rename: {input}");
                return true;
            }
            "enter" => {
                let target = target.clone();
                let new_name = input.trim().to_string();
                self.input_mode = InputMode::Normal;
                if new_name.is_empty() || new_name == target.name {
                    self.status = "rename unchanged".to_string();
                } else {
                    self.run_operation(FileOperation::Rename { target, new_name }, cx);
                }
                return true;
            }
            _ => {}
        }

        if let Some(ch) = command_char_from_key(event) {
            input.push(ch);
            self.status = format!("rename: {input}");
        }
        true
    }

    fn handle_confirm_key(&mut self, event: &KeyDownEvent, cx: &mut Context<Self>) -> bool {
        let Some(confirm) = self.pending_confirm.clone() else {
            return false;
        };
        if event.is_held {
            return true;
        }

        match event.keystroke.key.as_str() {
            "escape" | "n" => {
                self.pending_confirm = None;
                self.status = "cancelled".to_string();
                true
            }
            "enter" | "y" => {
                self.pending_confirm = None;
                match confirm {
                    PendingConfirm::Delete(targets) => {
                        self.run_operation(FileOperation::Delete { targets }, cx);
                    }
                }
                true
            }
            _ => false,
        }
    }

    fn handle_control_key(&mut self, event: &KeyDownEvent) -> bool {
        if event.is_held {
            return false;
        }

        let key = event.keystroke.key.as_str();
        if key == "tab" {
            self.switch_active_panel();
            return true;
        }

        let modifiers = event.keystroke.modifiers;
        if modifiers.control && !modifiers.alt && !modifiers.shift && !modifiers.platform {
            match key {
                "i" | "I" => {
                    self.switch_active_panel();
                    return true;
                }
                _ => {}
            }
        }

        if modifiers.modified() {
            return false;
        }

        match key {
            "tab" => {
                self.switch_active_panel();
                true
            }
            _ => false,
        }
    }

    fn apply_vim_char(&mut self, ch: char, cx: &mut Context<Self>) -> bool {
        match self.vim_command.push(ch) {
            VimCommandStep::Ignored => false,
            VimCommandStep::Pending => {
                self.status = self
                    .vim_command
                    .display()
                    .unwrap_or_else(|| "normal".to_string());
                true
            }
            VimCommandStep::Execute {
                sequence,
                count,
                explicit_count,
                had_pending,
            } => {
                let handled =
                    self.execute_vim_sequence(sequence.as_str(), count, explicit_count, cx);
                if !handled && had_pending {
                    return self.apply_vim_char(ch, cx);
                }
                handled
            }
        }
    }

    fn execute_vim_sequence(
        &mut self,
        sequence: &str,
        count: usize,
        explicit_count: bool,
        cx: &mut Context<Self>,
    ) -> bool {
        let Some(command) = BrowserCommand::from_vim_sequence(sequence, count, explicit_count)
        else {
            return false;
        };

        if command.requires_rows() && self.active_panel().rows.is_empty() {
            self.status = "empty".to_string();
            return true;
        }

        match command {
            BrowserCommand::Move(delta) | BrowserCommand::MovePage(delta) => {
                self.active_panel_mut().select_relative(delta)
            }
            BrowserCommand::First => self.active_panel_mut().select_line(0),
            BrowserCommand::Last => self.active_panel_mut().select_last(),
            BrowserCommand::Line(line) => self.active_panel_mut().select_line(line),
            BrowserCommand::OpenParent => return self.open_parent(cx),
            BrowserCommand::OpenSelected => return self.open_selected(cx),
            BrowserCommand::ToggleMark(count) => {
                let marked = self.toggle_marked(count);
                self.status = format!("{marked} marked");
            }
            BrowserCommand::ToggleAllMarks => {
                self.status = self.toggle_all_marks();
            }
            BrowserCommand::ClearMarks => {
                self.active_panel_mut().marked.clear();
                self.status = "marks cleared".to_string();
            }
            BrowserCommand::Copy => return self.prepare_clipboard(ClipboardKind::Copy),
            BrowserCommand::MoveSelection => return self.prepare_clipboard(ClipboardKind::Move),
            BrowserCommand::Paste => return self.paste_clipboard(cx),
            BrowserCommand::Delete => return self.prepare_delete(),
            BrowserCommand::Rename => return self.start_rename(),
            BrowserCommand::TogglePaneMode => {
                self.pane_mode = self.pane_mode.toggle();
                self.status = format!("{} pane mode", self.pane_mode.label());
            }
            BrowserCommand::SwitchPanel => {
                self.switch_active_panel();
                return true;
            }
            BrowserCommand::Reload => {
                let path = self.active_panel().path.clone();
                self.load_panel(self.active, path, None, cx);
            }
        }

        if !matches!(sequence, "h" | "l") {
            let selected = self.active_panel().selected_name();
            self.status = format!("{sequence} -> {selected}");
        }
        true
    }

    fn switch_active_panel(&mut self) {
        self.active = self.active.other();
        self.active_panel().reveal_selected();
        self.status = format!("active {}", self.active.label());
    }

    fn toggle_marked(&mut self, count: usize) -> usize {
        let panel = self.active_panel_mut();
        if panel.rows.is_empty() {
            return 0;
        }

        for _ in 0..count.max(1) {
            let Some(row) = panel.rows.get(panel.selected_index) else {
                break;
            };
            if row.name != ".." {
                if !panel.marked.remove(&row.path) {
                    panel.marked.insert(row.path.clone());
                }
            }
            if panel.selected_index + 1 < panel.rows.len() {
                panel.selected_index += 1;
            }
        }

        panel.reveal_selected();
        panel.marked.len()
    }

    fn toggle_all_marks(&mut self) -> String {
        let panel = self.active_panel_mut();
        let selectable = panel.rows.iter().filter(|row| row.name != "..").count();
        if selectable == 0 {
            return "nothing selectable".to_string();
        }

        let all_marked = panel
            .rows
            .iter()
            .filter(|row| row.name != "..")
            .all(|row| panel.marked.contains(&row.path));

        if all_marked {
            for row in &panel.rows {
                if row.name != ".." {
                    panel.marked.remove(&row.path);
                }
            }
            return "marks cleared".to_string();
        }

        for row in &panel.rows {
            if row.name != ".." {
                panel.marked.insert(row.path.clone());
            }
        }
        format!("{} marked", panel.marked.len())
    }

    fn prepare_clipboard(&mut self, kind: ClipboardKind) -> bool {
        let targets = self.effective_targets();
        if targets.is_empty() {
            self.status = "nothing selected".to_string();
            return true;
        }

        let label = match kind {
            ClipboardKind::Copy => "copy",
            ClipboardKind::Move => "move",
        };

        let mut clear_clipboard = false;
        match &mut self.clipboard {
            Some(clipboard) if clipboard.kind == kind => {
                self.status =
                    selection_status(label, toggle_targets(&mut clipboard.targets, &targets));
                clear_clipboard = clipboard.targets.is_empty();
            }
            _ => {
                self.status = format!("{label} {} item(s)", targets.len());
                self.clipboard = Some(ClipboardOp { kind, targets });
            }
        }
        if clear_clipboard {
            self.clipboard = None;
        }
        true
    }

    fn paste_clipboard(&mut self, cx: &mut Context<Self>) -> bool {
        let Some(clipboard) = self.clipboard.clone() else {
            self.status = "clipboard empty".to_string();
            return true;
        };

        let dst_dir = self.active_panel().path.clone();
        self.run_operation(
            FileOperation::Paste {
                kind: clipboard.kind,
                targets: clipboard.targets,
                dst_dir,
            },
            cx,
        );
        if matches!(clipboard.kind, ClipboardKind::Move) {
            self.clipboard = None;
        }
        true
    }

    fn prepare_delete(&mut self) -> bool {
        let targets = self.effective_targets();
        if targets.is_empty() {
            self.status = "nothing selected".to_string();
            return true;
        }

        let mut clear_delete = false;
        match &mut self.pending_confirm {
            Some(PendingConfirm::Delete(selected)) => {
                self.status = delete_status(toggle_targets(selected, &targets));
                clear_delete = selected.is_empty();
            }
            None => {
                self.status = format!("delete {} item(s)? y/enter to confirm", targets.len());
                self.pending_confirm = Some(PendingConfirm::Delete(targets));
            }
        }
        if clear_delete {
            self.pending_confirm = None;
        }
        true
    }

    fn start_rename(&mut self) -> bool {
        let Some(target) = self.selected_target() else {
            self.status = "nothing selected".to_string();
            return true;
        };

        self.status = format!("rename: {}", target.name);
        self.input_mode = InputMode::Rename {
            input: target.name.clone(),
            target,
        };
        true
    }

    fn open_parent(&mut self, cx: &mut Context<Self>) -> bool {
        let parent = self.active_panel().path.parent().map(Path::to_path_buf);
        let Some(parent) = parent else {
            self.status = "already at filesystem root".to_string();
            return true;
        };

        let prefer_name = self
            .active_panel()
            .path
            .file_name()
            .and_then(|name| name.to_str())
            .map(str::to_string);
        self.load_panel(self.active, parent, prefer_name, cx);
        true
    }

    fn open_selected(&mut self, cx: &mut Context<Self>) -> bool {
        let Some(row) = self.active_panel().selected_row().cloned() else {
            self.status = "nothing selected".to_string();
            return true;
        };

        if !row.is_dir {
            self.status = format!("selected {}", row.name);
            return true;
        }

        let prefer_name = if row.name == ".." {
            self.active_panel()
                .path
                .file_name()
                .and_then(|name| name.to_str())
                .map(str::to_string)
        } else {
            None
        };
        self.load_panel(self.active, row.path, prefer_name, cx);
        true
    }

    fn load_panel(
        &mut self,
        side: PanelSide,
        path: PathBuf,
        prefer_name: Option<String>,
        cx: &mut Context<Self>,
    ) {
        let generation = {
            let panel = self.panel_mut(side);
            panel.load_generation = panel.load_generation.wrapping_add(1);
            panel.loading = true;
            panel.error = None;
            panel.path = path.clone();
            panel.rows.clear();
            panel.selected_index = 0;
            panel.load_generation
        };
        self.status = format!("loading {}", path.display());

        cx.spawn(async move |shell, cx| {
            let load_path = path.clone();
            let result = cx
                .background_executor()
                .spawn(async move { core::read_fs_directory(&load_path) })
                .await;

            cx.update(|cx| {
                let _ = shell.update(cx, |shell, cx| {
                    shell.apply_loaded_panel(side, path, prefer_name, generation, result);
                    cx.notify();
                });
            })
        })
        .detach();
    }

    fn reload_panels_after_operation(&mut self, cx: &mut Context<Self>) {
        let left = self.left.path.clone();
        let right = self.right.path.clone();
        self.load_panel(PanelSide::Left, left, None, cx);
        self.load_panel(PanelSide::Right, right, None, cx);
    }

    fn run_operation(&mut self, operation: FileOperation, cx: &mut Context<Self>) {
        if self.operation_in_flight {
            self.status = "operation already running".to_string();
            return;
        }

        self.operation_in_flight = true;
        self.status = operation.pending_status();

        cx.spawn(async move |shell, cx| {
            let result = cx
                .background_executor()
                .spawn(async move { operation.run() })
                .await;

            cx.update(|cx| {
                let _ = shell.update(cx, |shell, cx| {
                    shell.operation_in_flight = false;
                    match result {
                        Ok(status) => {
                            shell.status = status;
                            shell.active_panel_mut().marked.clear();
                            shell.reload_panels_after_operation(cx);
                        }
                        Err(error) => {
                            shell.status = error.to_string();
                        }
                    }
                    cx.notify();
                });
            })
        })
        .detach();
    }

    fn apply_loaded_panel(
        &mut self,
        side: PanelSide,
        path: PathBuf,
        prefer_name: Option<String>,
        generation: u64,
        result: anyhow::Result<Vec<core::DirEntry>>,
    ) {
        let panel = self.panel_mut(side);
        if panel.load_generation != generation {
            return;
        }

        let status = {
            panel.loading = false;
            match result {
                Ok(entries) => {
                    panel.path = path;
                    panel.rows = entries.into_iter().map(FileRow::from_entry).collect();
                    panel
                        .marked
                        .retain(|path| panel.rows.iter().any(|row| &row.path == path));
                    panel.selected_index = prefer_name
                        .and_then(|name| panel.rows.iter().position(|row| row.name == name))
                        .unwrap_or_else(|| usize::from(panel.rows.len() > 1).min(panel.rows.len()));
                    panel.error = None;
                    let selected = panel.selected_name().to_string();
                    format!("{} rows, selected {selected}", panel.rows.len())
                }
                Err(error) => {
                    panel.rows.clear();
                    panel.selected_index = 0;
                    panel.error = Some(error.to_string());
                    format!("cannot load {}", panel.path.display())
                }
            }
        };
        self.panel_mut(side).reveal_selected();
        self.status = status;
    }

    fn active_panel(&self) -> &BrowserPanel {
        match self.active {
            PanelSide::Left => &self.left,
            PanelSide::Right => &self.right,
        }
    }

    fn active_panel_mut(&mut self) -> &mut BrowserPanel {
        match self.active {
            PanelSide::Left => &mut self.left,
            PanelSide::Right => &mut self.right,
        }
    }

    fn panel_mut(&mut self, side: PanelSide) -> &mut BrowserPanel {
        match side {
            PanelSide::Left => &mut self.left,
            PanelSide::Right => &mut self.right,
        }
    }

    fn command_mode_label(&self) -> String {
        match (&self.input_mode, &self.pending_confirm) {
            (InputMode::Rename { .. }, _) => "rename".to_string(),
            (_, Some(_)) => "confirm".to_string(),
            _ => self
                .vim_command
                .display()
                .unwrap_or_else(|| self.pane_mode.label().to_string()),
        }
    }

    fn effective_targets(&self) -> Vec<FileTarget> {
        let panel = self.active_panel();
        if !panel.marked.is_empty() {
            return panel
                .rows
                .iter()
                .filter(|row| row.name != ".." && panel.marked.contains(&row.path))
                .map(FileTarget::from_row)
                .collect();
        }
        self.selected_target().into_iter().collect()
    }

    fn selected_target(&self) -> Option<FileTarget> {
        self.active_panel()
            .selected_row()
            .filter(|row| row.name != "..")
            .map(FileTarget::from_row)
    }
}

impl Render for FilemanShell {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let viewport = window.viewport_size();
        let width = f32::from(viewport.width);
        let height = f32::from(viewport.height).max(1.0);
        let aspect = width / height;
        let narrow = width < 920.0 || aspect < 1.15;
        let single = self.pane_mode == PaneMode::Single;

        let panel_region = if single {
            match self.active {
                PanelSide::Left => v_flex()
                    .flex_grow()
                    .gap_2()
                    .p_2()
                    .child(render_panel(
                        &self.left,
                        true,
                        self.clipboard.as_ref(),
                        self.pending_confirm.as_ref(),
                    ))
                    .into_any_element(),
                PanelSide::Right => v_flex()
                    .flex_grow()
                    .gap_2()
                    .p_2()
                    .child(render_panel(
                        &self.right,
                        true,
                        self.clipboard.as_ref(),
                        self.pending_confirm.as_ref(),
                    ))
                    .into_any_element(),
            }
        } else if narrow {
            v_flex()
                .flex_grow()
                .gap_2()
                .p_2()
                .child(render_panel(
                    &self.left,
                    self.active == PanelSide::Left,
                    self.clipboard.as_ref(),
                    self.pending_confirm.as_ref(),
                ))
                .child(render_panel(
                    &self.right,
                    self.active == PanelSide::Right,
                    self.clipboard.as_ref(),
                    self.pending_confirm.as_ref(),
                ))
                .into_any_element()
        } else {
            h_flex()
                .flex_grow()
                .gap_2()
                .p_2()
                .child(render_panel(
                    &self.left,
                    self.active == PanelSide::Left,
                    self.clipboard.as_ref(),
                    self.pending_confirm.as_ref(),
                ))
                .child(render_panel(
                    &self.right,
                    self.active == PanelSide::Right,
                    self.clipboard.as_ref(),
                    self.pending_confirm.as_ref(),
                ))
                .into_any_element()
        };

        v_flex()
            .id("fileman-shell")
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(Self::on_key_down))
            .size_full()
            .bg(tokens::BG_CANVAS)
            .text_color(tokens::TEXT_PRIMARY)
            .font_family("Berkeley Mono")
            .child(render_title_bar())
            .child(panel_region)
            .child(render_command_bar(
                self.command_mode_label(),
                self.status.as_str(),
            ))
    }
}
