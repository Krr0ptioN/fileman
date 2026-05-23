use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use gpui::prelude::FluentBuilder;
use gpui::{
    App, AppContext, Application, Bounds, Context, FocusHandle, InteractiveElement, IntoElement,
    KeyDownEvent, ParentElement, Render, Styled, UpdateGlobal, Window, WindowBounds, WindowOptions,
    px, size,
};
use gpui_component::{Root, v_flex};

use crate::core;
use crate::features::clipboard::{
    ClipboardKind, ClipboardState, PastePlan, copy_file_contents, copy_target_name,
    copy_target_path, plan_paste, prepare_clipboard,
};
use crate::features::file_browser::tokens;
use crate::features::file_browser::{
    BrowserPanel, CommandBar, FileOperation, FileRow, FileTarget, FilemanAssets, HelpPopup,
    InputMode, LeaderMap, PanelLayout, PanelSide, PendingConfirm, TitleBar, delete_status,
    toggle_targets,
};
use crate::features::keybind::{
    AppKeyHandler, BrowserCommand, BrowserCommandExecutor, BrowserVimInput, ConfirmKeyAction,
    ControlAction, HeldNavigation, HelpAction, KeyCommandAction, KeybindArgs, KeybindRegistry,
    RenameKeyAction, VimCommandState, VimCommandStep, apply_browser_vim_char, confirm_key_action,
    control_action, file_manager_keybinds, handle_key_command, navigation_input, rename_key_action,
};
use crate::features::layout::LayoutState;

pub fn run(start_path: Option<PathBuf>) {
    Application::new()
        .with_assets(FilemanAssets)
        .run(move |app: &mut App| {
            gpui_component::init(app);
            app.set_global(ClipboardState::default());
            app.set_global(LayoutState::default());
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
    primary: BrowserPanel,
    secondary: BrowserPanel,
    active: PanelSide,
    pub(crate) focus_handle: FocusHandle,
    vim_command: VimCommandState,
    input_mode: InputMode,
    pending_confirm: Option<PendingConfirm>,
    held_navigation: HeldNavigation,
    keybinds: KeybindRegistry<BrowserCommand>,
    help_popup_open: bool,
    leader_map_open: bool,
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
            primary: FilemanShell::panel_factory(&start_path, "Primary", PanelSide::Left),
            secondary: FilemanShell::panel_factory(&start_path, "Secondary", PanelSide::Right),
            active: PanelSide::Left,
            focus_handle,
            vim_command: VimCommandState::default(),
            input_mode: InputMode::Normal,
            pending_confirm: None,
            held_navigation: HeldNavigation::default(),
            keybinds: file_manager_keybinds(),
            help_popup_open: false,
            leader_map_open: false,
            operation_in_flight: false,
            status: "normal".to_string(),
        };
        shell.load_panel(PanelSide::Left, start_path.clone(), None, cx);
        shell.load_panel(PanelSide::Right, start_path, None, cx);
        shell
    }

    #[inline]
    fn panel_factory(start_path: &PathBuf, title: &'static str, side: PanelSide) -> BrowserPanel {
        BrowserPanel {
            side,
            title,
            path: start_path.clone(),
            selected_index: 0,
            rows: Vec::new(),
            marked: HashSet::new(),
            loading: false,
            error: None,
            load_generation: 0,
            scroll_handle: Default::default(),
        }
    }

    fn on_key_down(&mut self, event: &KeyDownEvent, window: &mut Window, cx: &mut Context<Self>) {
        if self.dispatch_key_command(event, cx) {
            Self::consume_key_event(window, cx);
        }
    }

    fn dispatch_key_command(&mut self, event: &KeyDownEvent, cx: &mut Context<Self>) -> bool {
        match handle_key_command(self, event, cx) {
            KeyCommandAction::HandledResetNavigation => {
                self.held_navigation.reset();
                true
            }
            KeyCommandAction::HandledKeepNavigation => true,
            KeyCommandAction::Ignored => false,
        }
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

        match rename_key_action(event) {
            RenameKeyAction::Cancel => {
                self.input_mode = InputMode::Normal;
                self.status = "rename cancelled".to_string();
                true
            }
            RenameKeyAction::Backspace => {
                input.pop();
                self.status = format!("rename: {input}");
                true
            }
            RenameKeyAction::Submit => {
                let target = target.clone();
                let new_name = input.trim().to_string();
                self.input_mode = InputMode::Normal;
                if new_name.is_empty() || new_name == target.name {
                    self.status = "rename unchanged".to_string();
                } else {
                    self.run_operation(FileOperation::Rename { target, new_name }, cx);
                }
                true
            }
            RenameKeyAction::Insert(ch) => {
                input.push(ch);
                self.status = format!("rename: {input}");
                true
            }
            RenameKeyAction::Consume => true,
        }
    }

    fn handle_confirm_key(&mut self, event: &KeyDownEvent, cx: &mut Context<Self>) -> bool {
        let Some(confirm) = self.pending_confirm.clone() else {
            return false;
        };

        match confirm_key_action(event) {
            ConfirmKeyAction::Cancel => {
                self.pending_confirm = None;
                self.status = "cancelled".to_string();
                true
            }
            ConfirmKeyAction::Confirm => {
                self.pending_confirm = None;
                match confirm {
                    PendingConfirm::Delete(targets) => {
                        self.run_operation(FileOperation::Delete { targets }, cx);
                    }
                }
                true
            }
            ConfirmKeyAction::Consume => true,
            ConfirmKeyAction::Ignore => false,
        }
    }

    fn handle_control_key(&mut self, event: &KeyDownEvent) -> bool {
        match control_action(event) {
            Some(ControlAction::SwitchPanel) => {
                self.switch_active_panel();
                true
            }
            None => false,
        }
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

    fn paste_clipboard(&mut self, cx: &mut Context<Self>) -> bool {
        let dst_dir = self.active_panel().path.clone();
        let plan = ClipboardState::update_global(cx, |clipboard, _| plan_paste(clipboard, dst_dir));
        match plan {
            PastePlan::Empty => self.status = "clipboard empty".to_string(),
            PastePlan::Ready {
                kind,
                targets,
                dst_dir,
                clear_after_paste,
            } => {
                self.run_operation(
                    FileOperation::Paste {
                        kind,
                        targets,
                        dst_dir,
                    },
                    cx,
                );
                if clear_after_paste {
                    ClipboardState::update_global(cx, |clipboard, _| clipboard.clear());
                }
            }
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
        let left = self.primary.path.clone();
        let right = self.secondary.path.clone();
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
            PanelSide::Left => &self.primary,
            PanelSide::Right => &self.secondary,
        }
    }

    fn active_panel_mut(&mut self) -> &mut BrowserPanel {
        match self.active {
            PanelSide::Left => &mut self.primary,
            PanelSide::Right => &mut self.secondary,
        }
    }

    fn panel_mut(&mut self, side: PanelSide) -> &mut BrowserPanel {
        match side {
            PanelSide::Left => &mut self.primary,
            PanelSide::Right => &mut self.secondary,
        }
    }

    fn command_mode_label(&self, cx: &Context<Self>) -> String {
        match (&self.input_mode, &self.pending_confirm) {
            (InputMode::Rename { .. }, _) => "rename".to_string(),
            (_, Some(_)) => "confirm".to_string(),
            _ if self.help_popup_open => "keys".to_string(),
            _ if self.leader_map_open => "leader".to_string(),
            _ => self
                .vim_command
                .display()
                .unwrap_or_else(|| cx.global::<LayoutState>().pane_mode().label().to_string()),
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

impl BrowserCommandExecutor<Context<'_, FilemanShell>> for FilemanShell {
    fn command_for(&self, sequence: &str, args: KeybindArgs) -> Option<BrowserCommand> {
        self.keybinds.command_for(sequence, args)
    }

    fn has_active_rows(&self) -> bool {
        !self.active_panel().rows.is_empty()
    }

    fn selected_name(&self) -> String {
        self.active_panel().selected_name().to_string()
    }

    fn set_status(&mut self, status: String) {
        self.status = status;
    }

    fn select_relative(&mut self, delta: isize) {
        self.active_panel_mut().select_relative(delta);
    }

    fn select_first(&mut self) {
        self.active_panel_mut().select_line(0);
    }

    fn select_last(&mut self) {
        self.active_panel_mut().select_last();
    }

    fn select_line(&mut self, line: usize) {
        self.active_panel_mut().select_line(line);
    }

    fn open_parent(&mut self, cx: &mut Context<Self>) -> bool {
        FilemanShell::open_parent(self, cx)
    }

    fn open_selected(&mut self, cx: &mut Context<Self>) -> bool {
        FilemanShell::open_selected(self, cx)
    }

    fn toggle_marked(&mut self, count: usize) -> usize {
        FilemanShell::toggle_marked(self, count)
    }

    fn toggle_all_marks(&mut self) -> String {
        FilemanShell::toggle_all_marks(self)
    }

    fn clear_marks(&mut self) {
        self.active_panel_mut().marked.clear();
    }

    fn prepare_copy(&mut self, cx: &mut Context<Self>) -> bool {
        let targets = self.effective_targets();
        self.status = ClipboardState::update_global(cx, |clipboard, _| {
            prepare_clipboard(clipboard, ClipboardKind::Copy, targets)
        });
        true
    }

    fn copy_path(&mut self, cx: &mut Context<Self>) -> bool {
        self.status = copy_target_path(self.selected_target(), cx);
        true
    }

    fn copy_name(&mut self, cx: &mut Context<Self>) -> bool {
        self.status = copy_target_name(self.selected_target(), cx);
        true
    }

    fn copy_file_contents(&mut self, cx: &mut Context<Self>) -> bool {
        self.status = copy_file_contents(self.selected_target(), cx);
        true
    }

    fn prepare_move(&mut self, cx: &mut Context<Self>) -> bool {
        let targets = self.effective_targets();
        self.status = ClipboardState::update_global(cx, |clipboard, _| {
            prepare_clipboard(clipboard, ClipboardKind::Move, targets)
        });
        true
    }

    fn paste(&mut self, cx: &mut Context<Self>) -> bool {
        self.paste_clipboard(cx)
    }

    fn prepare_delete(&mut self) -> bool {
        FilemanShell::prepare_delete(self)
    }

    fn start_rename(&mut self) -> bool {
        FilemanShell::start_rename(self)
    }

    fn toggle_pane_mode(&mut self, cx: &mut Context<Self>) {
        let pane_mode = LayoutState::update_global(cx, |layout, _| layout.toggle_pane_mode());
        self.status = format!("{} pane mode", pane_mode.label());
    }

    fn switch_panel(&mut self) -> bool {
        self.switch_active_panel();
        true
    }

    fn open_help(&mut self) -> bool {
        self.help_popup_open = true;
        self.leader_map_open = false;
        self.status = "help".to_string();
        true
    }

    fn reload(&mut self, cx: &mut Context<Self>) {
        let path = self.active_panel().path.clone();
        self.load_panel(self.active, path, None, cx);
    }
}

impl BrowserVimInput<Context<'_, FilemanShell>> for FilemanShell {
    fn push_command_char(&mut self, ch: char) -> VimCommandStep {
        let keybinds = &self.keybinds;
        self.vim_command
            .push_with_prefixes(ch, |sequence| keybinds.is_prefix(sequence))
    }

    fn show_pending_command(&mut self) {
        self.status = self
            .vim_command
            .display()
            .unwrap_or_else(|| "normal".to_string());
    }
}

impl AppKeyHandler<Context<'_, FilemanShell>> for FilemanShell {
    fn modal_key(&mut self, event: &KeyDownEvent, cx: &mut Context<Self>) -> bool {
        self.handle_input_mode_key(event, cx) || self.handle_confirm_key(event, cx)
    }

    fn control_key(&mut self, event: &KeyDownEvent) -> bool {
        self.handle_control_key(event)
    }

    fn cancel_key(&mut self, event: &KeyDownEvent) -> bool {
        if event.is_held || event.keystroke.modifiers.modified() {
            return false;
        }
        if event.keystroke.key.as_str() != "escape" {
            return false;
        }

        self.vim_command.clear();
        self.help_popup_open = false;
        self.leader_map_open = false;
        self.status = "normal".to_string();
        true
    }

    fn help_key(&mut self, action: HelpAction) -> bool {
        match action {
            HelpAction::Open => {
                self.vim_command.clear();
                self.leader_map_open = false;
                self.help_popup_open = true;
                self.status = "help".to_string();
            }
            HelpAction::Close => {
                self.help_popup_open = false;
                self.status = "normal".to_string();
            }
        }
        true
    }

    fn help_open(&self) -> bool {
        self.help_popup_open
    }

    fn leader_open(&self) -> bool {
        self.leader_map_open
    }

    fn open_leader(&mut self) {
        self.leader_map_open = true;
        self.status = "leader".to_string();
    }

    fn close_leader(&mut self) {
        self.leader_map_open = false;
    }

    fn has_pending_vim(&self) -> bool {
        !self.vim_command.pending.is_empty()
    }

    fn navigation_key(&mut self, event: &KeyDownEvent) -> bool {
        self.handle_navigation_key(event)
    }

    fn vim_char(&mut self, ch: char, cx: &mut Context<Self>) -> bool {
        apply_browser_vim_char(self, ch, cx)
    }
}

impl Render for FilemanShell {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let panel_region = PanelLayout::new(
            &self.primary,
            &self.secondary,
            self.active,
            self.pending_confirm.as_ref(),
        );
        let leader_prefix = match self.leader_map_open {
            true => String::new(),
            false => self.vim_command.pending.clone(),
        };
        let leader_entries = match (
            self.help_popup_open,
            self.leader_map_open,
            leader_prefix.is_empty(),
        ) {
            (true, _, _) | (false, false, true) => Vec::new(),
            _ => self.keybinds.leader_continuations(leader_prefix.as_str()),
        };
        let show_leader_map = !leader_entries.is_empty();

        v_flex()
            .id("fileman-shell")
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(Self::on_key_down))
            .relative()
            .size_full()
            .bg(tokens::BG_CANVAS)
            .text_color(tokens::TEXT_PRIMARY)
            .font_family("Berkeley Mono")
            .child(TitleBar)
            .child(panel_region)
            .child(CommandBar::new(
                self.command_mode_label(cx),
                self.status.as_str(),
            ))
            .when(show_leader_map, |this| {
                this.child(LeaderMap::new(leader_prefix, leader_entries))
            })
            .when(self.help_popup_open, |this| {
                this.child(HelpPopup::new(self.keybinds.help_groups()))
            })
    }
}
