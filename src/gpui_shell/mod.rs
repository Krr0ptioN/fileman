#[cfg(unix)]
use std::os::unix::fs::{FileTypeExt, PermissionsExt};
use std::{
    borrow::Cow,
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

use gpui::{
    App, AppContext, Application, AssetSource, Bounds, Context, FocusHandle, InteractiveElement,
    IntoElement, KeyDownEvent, ParentElement, Render, SharedString, Styled, Window, WindowBounds,
    WindowOptions, div, px, size, uniform_list,
};
use gpui_component::{Icon, IconName, Root, h_flex, v_flex};

use crate::core;
use crate::features::vim_keys::{VimCommandState, VimCommandStep};

pub fn run(start_path: Option<PathBuf>) {
    Application::new()
        .with_assets(FilemanAssets)
        .run(move |cx: &mut App| {
            gpui_component::init(cx);
            let start_path = start_path.clone();

            let bounds = Bounds::centered(None, size(px(1180.0), px(720.0)), cx);
            cx.open_window(
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

            cx.activate(true);
        });
}

struct FilemanAssets;

impl AssetSource for FilemanAssets {
    fn load(&self, path: &str) -> gpui::Result<Option<Cow<'static, [u8]>>> {
        Ok(lucide_svg(path).map(|svg| Cow::Borrowed(svg.as_bytes())))
    }

    fn list(&self, path: &str) -> gpui::Result<Vec<SharedString>> {
        if path == "icons" {
            Ok(LUCIDE_ASSETS
                .iter()
                .map(|(name, _)| SharedString::from(*name))
                .collect())
        } else {
            Ok(Vec::new())
        }
    }
}

const LUCIDE_ASSETS: &[(&str, &str)] = &[
    (
        "copy.svg",
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="14" height="14" x="8" y="8" rx="2" ry="2"/><path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"/></svg>"#,
    ),
    (
        "delete.svg",
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M10 11v6"/><path d="M14 11v6"/><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6"/><path d="M3 6h18"/><path d="M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/></svg>"#,
    ),
    (
        "external-link.svg",
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M15 3h6v6"/><path d="M10 14 21 3"/><path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/></svg>"#,
    ),
    (
        "file.svg",
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z"/><path d="M14 2v4a2 2 0 0 0 2 2h4"/></svg>"#,
    ),
    (
        "folder-closed.svg",
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"/><path d="M2 10h20"/></svg>"#,
    ),
    (
        "globe.svg",
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20"/><path d="M2 12h20"/></svg>"#,
    ),
    (
        "info.svg",
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="M12 16v-4"/><path d="M12 8h.01"/></svg>"#,
    ),
    (
        "minus.svg",
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14"/></svg>"#,
    ),
    (
        "replace.svg",
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M14 4a2 2 0 0 1 2-2"/><path d="M16 2a2 2 0 0 1 2 2"/><path d="M20 6v2a2 2 0 0 1-2 2h-2"/><path d="M4 14v-2a2 2 0 0 1 2-2h2"/><path d="M8 20a2 2 0 0 1-2 2"/><path d="M6 22a2 2 0 0 1-2-2"/><path d="m18 14 4 4-4 4"/><path d="m6 10-4-4 4-4"/></svg>"#,
    ),
    (
        "settings.svg",
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.38a2 2 0 0 0-.73-2.73l-.15-.09a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2Z"/><circle cx="12" cy="12" r="3"/></svg>"#,
    ),
    (
        "square-terminal.svg",
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m7 11 2-2-2-2"/><path d="M11 13h4"/><rect width="18" height="18" x="3" y="3" rx="2" ry="2"/></svg>"#,
    ),
    (
        "star.svg",
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M11.5 2.3a.6.6 0 0 1 1 0l2.9 5.9 6.5.9a.6.6 0 0 1 .3 1l-4.7 4.6 1.1 6.5a.6.6 0 0 1-.9.6L12 18.8l-5.8 3a.6.6 0 0 1-.9-.6l1.1-6.5-4.7-4.6a.6.6 0 0 1 .3-1l6.5-.9Z"/></svg>"#,
    ),
    (
        "star-off.svg",
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m2 2 20 20"/><path d="M8.5 8.2 11.5 2.3a.6.6 0 0 1 1 0l2.9 5.9 6.5.9a.6.6 0 0 1 .3 1l-3.5 3.4"/><path d="m14.7 14.7 3.9 6.5a.6.6 0 0 1-.9.6L12 18.8l-5.8 3a.6.6 0 0 1-.9-.6l1.1-6.5-4.7-4.6a.6.6 0 0 1 .3-1l2.7-.4"/></svg>"#,
    ),
];

fn lucide_svg(path: &str) -> Option<&'static str> {
    let name = path.strip_prefix("icons/").unwrap_or(path);
    LUCIDE_ASSETS
        .iter()
        .find_map(|(asset_name, svg)| (*asset_name == name).then_some(*svg))
}

struct FilemanShell {
    left: BrowserPanel,
    right: BrowserPanel,
    active: PanelSide,
    focus_handle: FocusHandle,
    vim_command: VimCommandState,
    clipboard: Option<ClipboardOp>,
    input_mode: InputMode,
    pending_confirm: Option<PendingConfirm>,
    pane_mode: PaneMode,
    operation_in_flight: bool,
    status: String,
}

impl FilemanShell {
    fn new(focus_handle: FocusHandle, start_path: Option<PathBuf>, cx: &mut Context<Self>) -> Self {
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
            },
            active: PanelSide::Left,
            focus_handle,
            vim_command: VimCommandState::default(),
            clipboard: None,
            input_mode: InputMode::Normal,
            pending_confirm: None,
            pane_mode: PaneMode::Dual,
            operation_in_flight: false,
            status: "normal".to_string(),
        };
        shell.load_panel(PanelSide::Left, start_path.clone(), None, cx);
        shell.load_panel(PanelSide::Right, start_path, None, cx);
        shell
    }

    fn on_key_down(&mut self, event: &KeyDownEvent, window: &mut Window, cx: &mut Context<Self>) {
        if self.handle_input_mode_key(event, cx) || self.handle_confirm_key(event, cx) {
            window.prevent_default();
            cx.stop_propagation();
            cx.notify();
            return;
        }

        if self.handle_control_key(event) {
            window.prevent_default();
            cx.stop_propagation();
            cx.notify();
            return;
        }

        let Some(ch) = vim_char_from_key(event) else {
            return;
        };

        if self.apply_vim_char(ch, cx) {
            window.prevent_default();
            cx.stop_propagation();
            cx.notify();
        }
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

        if let Some(ch) = vim_char_from_key(event) {
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
        if sequence == "h" {
            return self.open_parent(cx);
        }
        if sequence == "w" {
            self.switch_active_panel();
            return true;
        }

        let active = self.active_panel_mut();
        let row_count = active.rows.len();
        if row_count == 0 {
            self.status = "empty".to_string();
            return true;
        }

        match sequence {
            "j" => active.select_relative(count as isize),
            "k" => active.select_relative(-(count as isize)),
            "J" => active.select_relative((count * 8) as isize),
            "K" => active.select_relative(-((count * 8) as isize)),
            "gg" => active.select_line(if explicit_count {
                count.saturating_sub(1)
            } else {
                0
            }),
            "G" => {
                if explicit_count {
                    active.select_line(count.saturating_sub(1));
                } else {
                    active.select_last();
                }
            }
            "0" => active.select_line(0),
            "l" => return self.open_selected(cx),
            "v" => {
                let marked = self.toggle_marked(count);
                self.status = format!("{marked} marked");
            }
            "V" => {
                let marked = self.mark_all();
                self.status = format!("{marked} marked");
            }
            "uv" | "uV" => {
                self.active_panel_mut().marked.clear();
                self.status = "marks cleared".to_string();
            }
            "yy" => return self.prepare_clipboard(ClipboardKind::Copy),
            "dd" => return self.prepare_clipboard(ClipboardKind::Move),
            "pp" => return self.paste_clipboard(cx),
            "dD" | "x" => return self.prepare_delete(),
            "cw" | "C" => return self.start_rename(),
            "s" => {
                self.pane_mode = self.pane_mode.toggle();
                self.status = format!("{} pane mode", self.pane_mode.label());
            }
            "r" | "R" => {
                let path = self.active_panel().path.clone();
                self.load_panel(self.active, path, None, cx);
            }
            _ => return false,
        }

        if !matches!(sequence, "h" | "l") {
            let selected = self.active_panel().selected_name();
            self.status = format!("{sequence} -> {selected}");
        }
        true
    }

    fn switch_active_panel(&mut self) {
        self.active = self.active.other();
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

        panel.marked.len()
    }

    fn mark_all(&mut self) -> usize {
        let panel = self.active_panel_mut();
        panel.marked.clear();
        for row in &panel.rows {
            if row.name != ".." {
                panel.marked.insert(row.path.clone());
            }
        }
        panel.marked.len()
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
        self.status = format!("{label} {} item(s)", targets.len());
        self.clipboard = Some(ClipboardOp { kind, targets });
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

        self.status = format!("delete {} item(s)? y/enter to confirm", targets.len());
        self.pending_confirm = Some(PendingConfirm::Delete(targets));
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
                    .child(self.left.render(
                        true,
                        self.clipboard.as_ref(),
                        self.pending_confirm.as_ref(),
                    ))
                    .into_any_element(),
                PanelSide::Right => v_flex()
                    .flex_grow()
                    .gap_2()
                    .p_2()
                    .child(self.right.render(
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
                .child(self.left.render(
                    self.active == PanelSide::Left,
                    self.clipboard.as_ref(),
                    self.pending_confirm.as_ref(),
                ))
                .child(self.right.render(
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
                .child(self.left.render(
                    self.active == PanelSide::Left,
                    self.clipboard.as_ref(),
                    self.pending_confirm.as_ref(),
                ))
                .child(self.right.render(
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

#[derive(Clone)]
struct BrowserPanel {
    side: PanelSide,
    title: &'static str,
    path: PathBuf,
    selected_index: usize,
    rows: Vec<FileRow>,
    marked: HashSet<PathBuf>,
    loading: bool,
    error: Option<String>,
    load_generation: u64,
}

impl BrowserPanel {
    fn render(
        &self,
        active: bool,
        clipboard: Option<&ClipboardOp>,
        pending_confirm: Option<&PendingConfirm>,
    ) -> impl IntoElement + use<> {
        let rows = self.rows.clone();
        let marked = self.marked.clone();
        let copy_targets = clipboard_targets(clipboard, ClipboardKind::Copy);
        let move_targets = clipboard_targets(clipboard, ClipboardKind::Move);
        let delete_targets = delete_targets(pending_confirm);
        let selected_index = self.selected_index;
        let row_count = rows.len();
        let list_id = match self.side {
            PanelSide::Left => "left-rows",
            PanelSide::Right => "right-rows",
        };

        v_flex()
            .flex_1()
            .min_w(px(0.0))
            .h_full()
            .bg(tokens::BG_PANEL)
            .border_1()
            .border_color(if active {
                tokens::BORDER_FOCUS
            } else {
                tokens::BORDER_SUBTLE
            })
            .rounded(px(6.0))
            .overflow_hidden()
            .child(render_panel_header(self, active))
            .child(
                div().flex_grow().child(
                    uniform_list(list_id, row_count, move |range, _, _| {
                        range
                            .map(|ix| {
                                let row = rows[ix].clone();
                                let is_marked = marked.contains(&row.path);
                                let intent = row_intent(
                                    &row.path,
                                    is_marked,
                                    &copy_targets,
                                    &move_targets,
                                    &delete_targets,
                                );
                                render_row(ix, row, ix == selected_index, active, intent)
                            })
                            .collect::<Vec<_>>()
                    })
                    .h_full(),
                ),
            )
    }

    fn select_relative(&mut self, delta: isize) {
        if self.rows.is_empty() {
            self.selected_index = 0;
            return;
        }

        self.selected_index = if delta.is_negative() {
            self.selected_index.saturating_sub(delta.unsigned_abs())
        } else {
            self.selected_index
                .saturating_add(delta as usize)
                .min(self.rows.len() - 1)
        };
    }

    fn select_line(&mut self, index: usize) {
        if self.rows.is_empty() {
            self.selected_index = 0;
        } else {
            self.selected_index = index.min(self.rows.len() - 1);
        }
    }

    fn select_last(&mut self) {
        if !self.rows.is_empty() {
            self.selected_index = self.rows.len() - 1;
        }
    }

    fn selected_name(&self) -> &str {
        self.rows
            .get(self.selected_index)
            .map(|row| row.name.as_str())
            .unwrap_or("<none>")
    }

    fn selected_row(&self) -> Option<&FileRow> {
        self.rows.get(self.selected_index)
    }
}

#[derive(Clone)]
struct FileRow {
    kind: RowKind,
    name: String,
    detail: String,
    path: PathBuf,
    is_dir: bool,
    is_executable: bool,
}

#[derive(Clone)]
struct FileTarget {
    path: PathBuf,
    name: String,
    is_dir: bool,
}

impl FileTarget {
    fn from_row(row: &FileRow) -> Self {
        Self {
            path: row.path.clone(),
            name: row.name.clone(),
            is_dir: row.is_dir,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ClipboardKind {
    Copy,
    Move,
}

#[derive(Clone)]
struct ClipboardOp {
    kind: ClipboardKind,
    targets: Vec<FileTarget>,
}

enum InputMode {
    Normal,
    Rename { target: FileTarget, input: String },
}

#[derive(Clone)]
enum PendingConfirm {
    Delete(Vec<FileTarget>),
}

enum FileOperation {
    Paste {
        kind: ClipboardKind,
        targets: Vec<FileTarget>,
        dst_dir: PathBuf,
    },
    Delete {
        targets: Vec<FileTarget>,
    },
    Rename {
        target: FileTarget,
        new_name: String,
    },
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum PaneMode {
    Dual,
    Single,
}

impl PaneMode {
    fn toggle(self) -> Self {
        match self {
            Self::Dual => Self::Single,
            Self::Single => Self::Dual,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Dual => "dual",
            Self::Single => "single",
        }
    }
}

impl FileRow {
    fn from_entry(entry: core::DirEntry) -> Self {
        let path = match &entry.location {
            core::EntryLocation::Fs(path) => path.clone(),
            _ => PathBuf::new(),
        };
        let kind = classify_entry(&entry, &path);
        let is_executable = is_executable_path(&path, kind);
        let link_target = if entry.is_symlink {
            entry.link_target.clone().or_else(|| {
                fs::read_link(&path)
                    .ok()
                    .map(|target| target.display().to_string())
            })
        } else {
            None
        };
        let detail = format_entry_detail(&entry, kind, link_target.as_deref());
        Self {
            kind,
            name: entry.name,
            detail,
            path,
            is_dir: entry.is_dir,
            is_executable,
        }
    }
}

#[derive(Clone, Copy)]
enum RowKind {
    Directory,
    Symlink,
    Socket,
    Pipe,
    BlockDevice,
    CharDevice,
    File(FileFormat),
    Other,
}

#[derive(Clone, Copy)]
enum FileFormat {
    Archive,
    Audio,
    Binary,
    Code,
    Image,
    Pdf,
    Text,
    Video,
    Unknown,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum RowIntent {
    None,
    Marked,
    Copy,
    Move,
    Delete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PanelSide {
    Left,
    Right,
}

impl PanelSide {
    fn other(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Left => "primary",
            Self::Right => "secondary",
        }
    }
}

fn render_title_bar() -> impl IntoElement {
    h_flex()
        .h(px(42.0))
        .px_3()
        .items_center()
        .justify_between()
        .bg(tokens::BG_PANEL_RAISED)
        .border_b_1()
        .border_color(tokens::BORDER_SUBTLE)
        .child(
            h_flex()
                .items_center()
                .gap_2()
                .child(div().size(px(10.0)).rounded_full().bg(tokens::ACCENT))
                .child(
                    div()
                        .text_color(tokens::TEXT_PRIMARY)
                        .font_weight(gpui::FontWeight::SEMIBOLD)
                        .child("FileMan"),
                ),
        )
        .child(
            div()
                .text_color(tokens::TEXT_SECONDARY)
                .text_size(px(12.0))
                .child("GPUI shell"),
        )
}

fn render_panel_header(panel: &BrowserPanel, active: bool) -> impl IntoElement {
    v_flex()
        .gap_1()
        .p_3()
        .bg(tokens::BG_PANEL_RAISED)
        .border_b_1()
        .border_color(tokens::BORDER_SUBTLE)
        .child(
            h_flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .font_weight(gpui::FontWeight::SEMIBOLD)
                        .text_color(if active {
                            tokens::TEXT_PRIMARY
                        } else {
                            tokens::TEXT_SECONDARY
                        })
                        .child(panel.title),
                )
                .child(
                    div()
                        .text_size(px(11.0))
                        .text_color(tokens::TEXT_MUTED)
                        .child(panel_header_status(panel)),
                ),
        )
        .child(
            div()
                .text_size(px(12.0))
                .text_color(tokens::TEXT_SECONDARY)
                .child(panel.path.display().to_string()),
        )
}

fn render_row(
    ix: usize,
    row: FileRow,
    selected: bool,
    active: bool,
    intent: RowIntent,
) -> impl IntoElement {
    let row_bg = if selected && active {
        tokens::ROW_SELECTED_ACTIVE
    } else if selected {
        tokens::ROW_SELECTED_INACTIVE
    } else if intent != RowIntent::None {
        intent_bg(intent)
    } else {
        tokens::BG_PANEL
    };
    let border = if selected && active {
        tokens::ROW_SELECTED_ACTIVE_BORDER
    } else if selected {
        tokens::ROW_SELECTED_INACTIVE_BORDER
    } else {
        tokens::ROW_BORDER_CLEAR
    };

    h_flex()
        .id(("file-row", ix))
        .w_full()
        .h(px(32.0))
        .px_3()
        .items_center()
        .justify_between()
        .bg(row_bg)
        .border_1()
        .border_color(border)
        .rounded(px(if selected { 7.0 } else { 0.0 }))
        .hover(|style| style.bg(tokens::ROW_HOVER))
        .child(
            h_flex()
                .items_center()
                .gap_2()
                .min_w(px(0.0))
                .flex_1()
                .child(intent_badge(intent))
                .child(row_icon(row.kind))
                .child(executable_badge(row.is_executable))
                .child(
                    div()
                        .min_w(px(0.0))
                        .text_color(if selected {
                            tokens::TEXT_PRIMARY
                        } else {
                            tokens::TEXT_SECONDARY
                        })
                        .child(row.name),
                )
                .child(
                    div()
                        .text_size(px(11.0))
                        .text_color(tokens::TEXT_MUTED)
                        .child(kind_label(row.kind)),
                ),
        )
        .child(
            div()
                .flex_shrink_0()
                .text_size(px(12.0))
                .text_color(tokens::TEXT_SECONDARY)
                .child(row.detail),
        )
}

fn executable_badge(is_executable: bool) -> impl IntoElement {
    h_flex()
        .w(px(14.0))
        .items_center()
        .justify_center()
        .child(if is_executable {
            Icon::new(IconName::Settings)
                .size(px(14.0))
                .text_color(tokens::ICON_EXECUTABLE)
                .into_any_element()
        } else {
            div().into_any_element()
        })
}

fn intent_badge(intent: RowIntent) -> impl IntoElement {
    let (icon, label, color) = match intent {
        RowIntent::None => (IconName::StarOff, "", tokens::TEXT_MUTED),
        RowIntent::Marked => (IconName::Star, "mark", tokens::ACCENT),
        RowIntent::Copy => (IconName::Copy, "yank", tokens::ICON_COPY),
        RowIntent::Move => (IconName::Replace, "move", tokens::ICON_MOVE),
        RowIntent::Delete => (IconName::Delete, "delete", tokens::ICON_DELETE),
    };

    h_flex()
        .w(px(68.0))
        .items_center()
        .gap_1()
        .child(Icon::new(icon).size(px(13.0)).text_color(color))
        .child(div().text_size(px(10.0)).text_color(color).child(label))
}

fn intent_bg(intent: RowIntent) -> gpui::Rgba {
    match intent {
        RowIntent::None => tokens::BG_PANEL,
        RowIntent::Marked => tokens::ROW_MARKED,
        RowIntent::Copy => tokens::ROW_COPY,
        RowIntent::Move => tokens::ROW_MOVE,
        RowIntent::Delete => tokens::ROW_DELETE,
    }
}

fn row_icon(kind: RowKind) -> impl IntoElement {
    Icon::new(row_icon_name(kind))
        .size(px(16.0))
        .text_color(row_icon_color(kind))
}

fn row_icon_name(kind: RowKind) -> IconName {
    match kind {
        RowKind::Directory => IconName::FolderClosed,
        RowKind::Symlink => IconName::ExternalLink,
        RowKind::Socket => IconName::Globe,
        RowKind::Pipe => IconName::Minus,
        RowKind::BlockDevice | RowKind::CharDevice => IconName::SquareTerminal,
        RowKind::File(_) => IconName::File,
        RowKind::Other => IconName::Info,
    }
}

fn row_icon_color(kind: RowKind) -> gpui::Rgba {
    match kind {
        RowKind::Directory => tokens::ICON_DIRECTORY,
        RowKind::Symlink => tokens::ICON_SYMLINK,
        RowKind::Socket => tokens::ICON_SOCKET,
        RowKind::Pipe => tokens::ICON_PIPE,
        RowKind::BlockDevice | RowKind::CharDevice => tokens::ICON_DEVICE,
        RowKind::Other => tokens::ICON_OTHER,
        RowKind::File(format) => match format {
            FileFormat::Archive => tokens::ICON_ARCHIVE,
            FileFormat::Audio => tokens::ICON_AUDIO,
            FileFormat::Binary => tokens::ICON_BINARY,
            FileFormat::Code => tokens::ICON_CODE,
            FileFormat::Image => tokens::ICON_IMAGE,
            FileFormat::Pdf => tokens::ICON_PDF,
            FileFormat::Text => tokens::ICON_TEXT,
            FileFormat::Video => tokens::ICON_VIDEO,
            FileFormat::Unknown => tokens::ICON_FILE,
        },
    }
}

fn kind_label(kind: RowKind) -> &'static str {
    match kind {
        RowKind::Directory => "dir",
        RowKind::Symlink => "link",
        RowKind::Socket => "socket",
        RowKind::Pipe => "pipe",
        RowKind::BlockDevice => "block",
        RowKind::CharDevice => "char",
        RowKind::Other => "other",
        RowKind::File(format) => format.label(),
    }
}

fn render_command_bar(mode: String, status: &str) -> impl IntoElement {
    h_flex()
        .h(px(34.0))
        .px_3()
        .items_center()
        .justify_between()
        .bg(tokens::BG_PANEL_RAISED)
        .border_t_1()
        .border_color(tokens::BORDER_SUBTLE)
        .child(
            h_flex()
                .items_center()
                .gap_2()
                .child(command_hint("j/k", "move"))
                .child(command_hint("v/V", "mark"))
                .child(command_hint("h/l", "parent/open"))
                .child(command_hint("yy/dd/pp", "copy/move/paste"))
                .child(command_hint("cw/x", "rename/delete"))
                .child(command_hint("s/w", "layout/pane")),
        )
        .child(
            h_flex()
                .items_center()
                .gap_2()
                .child(
                    div()
                        .text_size(px(12.0))
                        .text_color(tokens::ACCENT)
                        .child(mode),
                )
                .child(
                    div()
                        .text_size(px(12.0))
                        .text_color(tokens::TEXT_MUTED)
                        .child(status.to_string()),
                ),
        )
}

fn command_hint(key: &'static str, label: &'static str) -> impl IntoElement {
    h_flex()
        .items_center()
        .gap_1()
        .child(
            div()
                .px_1()
                .rounded(px(3.0))
                .border_1()
                .border_color(tokens::BORDER_SUBTLE)
                .text_color(tokens::ACCENT)
                .text_size(px(11.0))
                .child(key),
        )
        .child(
            div()
                .text_size(px(12.0))
                .text_color(tokens::TEXT_SECONDARY)
                .child(label),
        )
}

fn vim_char_from_key(event: &KeyDownEvent) -> Option<char> {
    if event.is_held {
        return None;
    }

    let modifiers = event.keystroke.modifiers;
    if modifiers.control || modifiers.alt || modifiers.platform || modifiers.function {
        return None;
    }

    event
        .keystroke
        .key_char
        .as_deref()
        .unwrap_or(event.keystroke.key.as_str())
        .chars()
        .next()
        .filter(|ch| !ch.is_control())
}

fn panel_header_status(panel: &BrowserPanel) -> String {
    if panel.loading {
        "loading".to_string()
    } else if let Some(error) = &panel.error {
        error.clone()
    } else {
        format!("{} rows", panel.rows.len())
    }
}

fn clipboard_targets(clipboard: Option<&ClipboardOp>, kind: ClipboardKind) -> HashSet<PathBuf> {
    let Some(clipboard) = clipboard else {
        return HashSet::new();
    };
    if clipboard.kind != kind {
        return HashSet::new();
    }
    clipboard
        .targets
        .iter()
        .map(|target| target.path.clone())
        .collect()
}

fn delete_targets(pending_confirm: Option<&PendingConfirm>) -> HashSet<PathBuf> {
    match pending_confirm {
        Some(PendingConfirm::Delete(targets)) => {
            targets.iter().map(|target| target.path.clone()).collect()
        }
        None => HashSet::new(),
    }
}

fn row_intent(
    path: &Path,
    marked: bool,
    copy_targets: &HashSet<PathBuf>,
    move_targets: &HashSet<PathBuf>,
    delete_targets: &HashSet<PathBuf>,
) -> RowIntent {
    if delete_targets.contains(path) {
        RowIntent::Delete
    } else if move_targets.contains(path) {
        RowIntent::Move
    } else if copy_targets.contains(path) {
        RowIntent::Copy
    } else if marked {
        RowIntent::Marked
    } else {
        RowIntent::None
    }
}

fn classify_entry(entry: &core::DirEntry, path: &Path) -> RowKind {
    if entry.name == ".." {
        return RowKind::Directory;
    }
    if entry.is_symlink {
        return RowKind::Symlink;
    }

    if let Ok(metadata) = fs::symlink_metadata(path) {
        let file_type = metadata.file_type();
        if file_type.is_dir() {
            return RowKind::Directory;
        }
        if file_type.is_file() {
            return RowKind::File(file_format_from_path(path));
        }

        #[cfg(unix)]
        {
            if file_type.is_socket() {
                return RowKind::Socket;
            }
            if file_type.is_fifo() {
                return RowKind::Pipe;
            }
            if file_type.is_block_device() {
                return RowKind::BlockDevice;
            }
            if file_type.is_char_device() {
                return RowKind::CharDevice;
            }
        }

        return RowKind::Other;
    }

    if entry.is_dir {
        RowKind::Directory
    } else {
        RowKind::File(file_format_from_path(path))
    }
}

fn is_executable_path(path: &Path, kind: RowKind) -> bool {
    if !matches!(kind, RowKind::File(_)) {
        return false;
    }

    #[cfg(unix)]
    {
        fs::metadata(path)
            .map(|metadata| metadata.permissions().mode() & 0o111 != 0)
            .unwrap_or(false)
    }

    #[cfg(not(unix))]
    {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| {
                matches!(
                    ext.to_ascii_lowercase().as_str(),
                    "bat" | "cmd" | "com" | "exe" | "ps1"
                )
            })
            .unwrap_or(false)
    }
}

fn file_format_from_path(path: &Path) -> FileFormat {
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();

    match name.as_str() {
        "dockerfile" | "makefile" | "justfile" | "rakefile" | "gemfile" => return FileFormat::Code,
        "license" | "notice" | "readme" => return FileFormat::Text,
        _ => {}
    }

    let Some(ext) = path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(str::to_ascii_lowercase)
    else {
        return FileFormat::Unknown;
    };

    match ext.as_str() {
        "7z" | "bz2" | "gz" | "rar" | "tar" | "tgz" | "xz" | "zip" | "zst" => FileFormat::Archive,
        "aac" | "aiff" | "flac" | "m4a" | "mp3" | "ogg" | "opus" | "wav" => FileFormat::Audio,
        "bin" | "dll" | "dmg" | "exe" | "iso" | "o" | "so" => FileFormat::Binary,
        "c" | "cc" | "cpp" | "css" | "go" | "h" | "hpp" | "html" | "java" | "js" | "jsx" | "kt"
        | "lua" | "php" | "py" | "rb" | "rs" | "scss" | "sh" | "sql" | "svelte" | "swift"
        | "toml" | "ts" | "tsx" | "vue" | "xml" | "yaml" | "yml" => FileFormat::Code,
        "avif" | "bmp" | "gif" | "heic" | "ico" | "jpeg" | "jpg" | "png" | "svg" | "tif"
        | "tiff" | "webp" => FileFormat::Image,
        "pdf" => FileFormat::Pdf,
        "csv" | "log" | "md" | "rst" | "txt" => FileFormat::Text,
        "avi" | "m4v" | "mkv" | "mov" | "mp4" | "mpeg" | "mpg" | "webm" => FileFormat::Video,
        _ => FileFormat::Unknown,
    }
}

fn format_entry_detail(entry: &core::DirEntry, kind: RowKind, link_target: Option<&str>) -> String {
    if entry.name == ".." {
        return "parent".to_string();
    }

    match kind {
        RowKind::Directory => "dir".to_string(),
        RowKind::Symlink => link_target
            .map(|target| format!("link -> {target}"))
            .unwrap_or_else(|| "link".to_string()),
        RowKind::Socket => "socket".to_string(),
        RowKind::Pipe => "pipe".to_string(),
        RowKind::BlockDevice => "block device".to_string(),
        RowKind::CharDevice => "char device".to_string(),
        RowKind::Other => "other".to_string(),
        RowKind::File(format) => entry
            .size
            .map(|size| format!("{} {}", format.label(), format_size(size)))
            .unwrap_or_else(|| format.label().to_string()),
    }
}

impl FileFormat {
    fn label(self) -> &'static str {
        match self {
            Self::Archive => "archive",
            Self::Audio => "audio",
            Self::Binary => "binary",
            Self::Code => "code",
            Self::Image => "image",
            Self::Pdf => "pdf",
            Self::Text => "text",
            Self::Video => "video",
            Self::Unknown => "file",
        }
    }
}

fn format_size(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut value = size as f64;
    let mut unit = 0usize;
    while value >= 1024.0 && unit + 1 < UNITS.len() {
        value /= 1024.0;
        unit += 1;
    }
    if unit == 0 {
        format!("{size} {}", UNITS[unit])
    } else {
        format!("{value:.1} {}", UNITS[unit])
    }
}

impl FileOperation {
    fn pending_status(&self) -> String {
        match self {
            Self::Paste { kind, targets, .. } => {
                let op = match kind {
                    ClipboardKind::Copy => "copying",
                    ClipboardKind::Move => "moving",
                };
                format!("{op} {} item(s)", targets.len())
            }
            Self::Delete { targets } => format!("deleting {} item(s)", targets.len()),
            Self::Rename { target, new_name } => {
                format!("renaming {} to {new_name}", target.name)
            }
        }
    }

    fn run(self) -> anyhow::Result<String> {
        match self {
            Self::Paste {
                kind,
                targets,
                dst_dir,
            } => {
                for target in &targets {
                    match kind {
                        ClipboardKind::Copy => copy_target(target, &dst_dir)?,
                        ClipboardKind::Move => move_target(target, &dst_dir)?,
                    }
                }
                let op = match kind {
                    ClipboardKind::Copy => "copied",
                    ClipboardKind::Move => "moved",
                };
                Ok(format!("{op} {} item(s)", targets.len()))
            }
            Self::Delete { targets } => {
                for target in &targets {
                    delete_target(target)?;
                }
                Ok(format!("deleted {} item(s)", targets.len()))
            }
            Self::Rename { target, new_name } => {
                let dst = target.path.with_file_name(&new_name);
                fs::rename(&target.path, &dst)?;
                Ok(format!("renamed {} to {new_name}", target.name))
            }
        }
    }
}

fn copy_target(target: &FileTarget, dst_dir: &Path) -> anyhow::Result<()> {
    if target.path.parent() == Some(dst_dir) {
        anyhow::bail!(
            "copy destination is the source directory for {}",
            target.name
        );
    }
    core::copy_recursively(&target.path, dst_dir)
        .map_err(|error| anyhow::anyhow!("copy {}: {error}", target.path.display()))
}

fn move_target(target: &FileTarget, dst_dir: &Path) -> anyhow::Result<()> {
    if target.path.parent() == Some(dst_dir) {
        anyhow::bail!(
            "move destination is the source directory for {}",
            target.name
        );
    }

    let dst = dst_dir.join(&target.name);
    match fs::rename(&target.path, &dst) {
        Ok(()) => Ok(()),
        Err(rename_error) => {
            core::copy_recursively(&target.path, dst_dir).map_err(|copy_error| {
                anyhow::anyhow!(
                    "move {}: rename failed ({rename_error}); copy failed ({copy_error})",
                    target.path.display()
                )
            })?;
            delete_target(target)
        }
    }
}

fn delete_target(target: &FileTarget) -> anyhow::Result<()> {
    let result = if target.is_dir {
        fs::remove_dir_all(&target.path)
    } else {
        fs::remove_file(&target.path)
    };
    result.map_err(|error| anyhow::anyhow!("delete {}: {error}", target.path.display()))
}

mod tokens {
    use gpui::Rgba;

    const fn rgb(hex: u32) -> Rgba {
        Rgba {
            r: ((hex >> 16) & 0xff) as f32 / 255.0,
            g: ((hex >> 8) & 0xff) as f32 / 255.0,
            b: (hex & 0xff) as f32 / 255.0,
            a: 1.0,
        }
    }

    const fn rgba(hex: u32, a: f32) -> Rgba {
        Rgba {
            r: ((hex >> 16) & 0xff) as f32 / 255.0,
            g: ((hex >> 8) & 0xff) as f32 / 255.0,
            b: (hex & 0xff) as f32 / 255.0,
            a,
        }
    }

    pub const BG_CANVAS: Rgba = rgb(0x0a0a0a);
    pub const BG_PANEL: Rgba = rgb(0x111111);
    pub const BG_PANEL_RAISED: Rgba = rgb(0x171717);
    pub const BORDER_SUBTLE: Rgba = rgb(0x262626);
    pub const BORDER_FOCUS: Rgba = rgb(0x3b82f6);
    pub const TEXT_PRIMARY: Rgba = rgb(0xfafafa);
    pub const TEXT_SECONDARY: Rgba = rgb(0xa1a1aa);
    pub const TEXT_MUTED: Rgba = rgb(0x71717a);
    pub const ROW_HOVER: Rgba = rgb(0x1f1f1f);
    pub const ROW_BORDER_CLEAR: Rgba = rgba(0xffffff, 0.0);
    pub const ROW_SELECTED_ACTIVE: Rgba = rgba(0x4f9cf9, 0.22);
    pub const ROW_SELECTED_ACTIVE_BORDER: Rgba = rgba(0xdbeafe, 0.34);
    pub const ROW_SELECTED_INACTIVE: Rgba = rgba(0xffffff, 0.08);
    pub const ROW_SELECTED_INACTIVE_BORDER: Rgba = rgba(0xffffff, 0.16);
    pub const ROW_MARKED: Rgba = rgba(0x2563eb, 0.16);
    pub const ROW_COPY: Rgba = rgba(0x0891b2, 0.16);
    pub const ROW_MOVE: Rgba = rgba(0xa855f7, 0.16);
    pub const ROW_DELETE: Rgba = rgba(0xef4444, 0.16);
    pub const ACCENT: Rgba = rgb(0x3b82f6);
    pub const ICON_COPY: Rgba = rgb(0x22d3ee);
    pub const ICON_MOVE: Rgba = rgb(0xc084fc);
    pub const ICON_DELETE: Rgba = rgb(0xfb7185);
    pub const ICON_EXECUTABLE: Rgba = rgb(0x22c55e);
    pub const ICON_DIRECTORY: Rgba = rgb(0xfbbf24);
    pub const ICON_SYMLINK: Rgba = rgb(0x38bdf8);
    pub const ICON_SOCKET: Rgba = rgb(0xa78bfa);
    pub const ICON_PIPE: Rgba = rgb(0x22c55e);
    pub const ICON_DEVICE: Rgba = rgb(0xf97316);
    pub const ICON_ARCHIVE: Rgba = rgb(0xeab308);
    pub const ICON_AUDIO: Rgba = rgb(0xec4899);
    pub const ICON_BINARY: Rgba = rgb(0x94a3b8);
    pub const ICON_CODE: Rgba = rgb(0x34d399);
    pub const ICON_IMAGE: Rgba = rgb(0x60a5fa);
    pub const ICON_PDF: Rgba = rgb(0xf87171);
    pub const ICON_TEXT: Rgba = rgb(0xcbd5e1);
    pub const ICON_VIDEO: Rgba = rgb(0xc084fc);
    pub const ICON_FILE: Rgba = rgb(0xa1a1aa);
    pub const ICON_OTHER: Rgba = rgb(0xfacc15);
}
