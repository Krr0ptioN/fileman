use gpui::{Context, KeyDownEvent};

use super::StiffShell;
use crate::features::{
    clipboard::{PasteConflictDecision, PasteConflictPolicy, resolve_paste_conflict},
    file_browser::{apply_confirm_action, apply_rename_action, start_quick_jump},
    keybind::{
        ControlAction, confirm_key_action, control_action, navigation_input, rename_key_action,
    },
};

impl StiffShell {
    pub(super) fn handle_navigation_key(
        &mut self,
        event: &KeyDownEvent,
        cx: &mut Context<Self>,
    ) -> bool {
        let Some(input) = navigation_input(event) else {
            return false;
        };
        if self.preview_pane_focused() {
            return false;
        }
        let (key, rows) = self.held_navigation.rows_for(input);

        self.active_panel_mut().select_relative(key.delta(rows));
        self.active_panel().reveal_selected();
        self.set_status_debounced(
            format!(
                "{} -> {}",
                event.keystroke.key,
                self.active_panel().selected_name()
            ),
            cx,
        );
        self.hide_preview_pane();
        self.schedule_preview_preload(cx);
        true
    }

    pub(super) fn handle_input_mode_key(
        &mut self,
        event: &KeyDownEvent,
        cx: &mut Context<Self>,
    ) -> bool {
        match apply_rename_action(&mut self.input_mode, rename_key_action(event)) {
            Some(outcome) => {
                self.apply_browser_outcome(outcome, cx);
                true
            }
            None => false,
        }
    }

    pub(super) fn handle_paste_conflict_key(
        &mut self,
        event: &KeyDownEvent,
        cx: &mut Context<Self>,
    ) -> bool {
        if self.pending_paste.is_none() || event.is_held {
            return false;
        }
        let policy = match event.keystroke.key.as_str() {
            "s" | "S" => PasteConflictPolicy::Skip,
            "o" | "O" => PasteConflictPolicy::Overwrite,
            "r" | "R" => PasteConflictPolicy::Rename,
            "c" | "C" | "escape" => PasteConflictPolicy::Cancel,
            _ => return true,
        };
        let apply_to_all = event.keystroke.modifiers.shift;
        let Some((_, pending)) = self.pending_paste.take() else {
            return true;
        };
        let plan = resolve_paste_conflict(
            pending,
            PasteConflictDecision {
                policy,
                apply_to_all,
            },
        );
        self.handle_paste_plan(plan, cx);
        true
    }

    pub(super) fn handle_confirm_key(
        &mut self,
        event: &KeyDownEvent,
        cx: &mut Context<Self>,
    ) -> bool {
        match apply_confirm_action(&mut self.pending_confirm, confirm_key_action(event)) {
            Some(outcome) => {
                self.apply_browser_outcome(outcome, cx);
                true
            }
            None => false,
        }
    }

    pub(super) fn handle_control_key(
        &mut self,
        event: &KeyDownEvent,
        cx: &mut Context<Self>,
    ) -> bool {
        if self.handle_pane_focus_key(event, cx) {
            return true;
        }

        match control_action(event) {
            Some(ControlAction::SwitchPanel) => {
                self.active = self.active.other();
                self.ensure_panel_loaded(self.active, cx);
                self.active_panel().reveal_selected();
                self.status = format!("active {}", self.active.label());
                self.hide_preview_pane();
                self.schedule_preview_preload(cx);
                true
            }
            Some(ControlAction::QuickJump) => {
                self.vim_command.clear();
                self.leader_map_open = false;
                let base = self.active_panel().path.clone();
                self.status = start_quick_jump(&mut self.input_mode, base);
                true
            }
            Some(ControlAction::PaneFocusPrefix) => {
                self.pane_focus_prefix = true;
                self.status = "pane".to_string();
                true
            }
            Some(ControlAction::PreviewPageDown) => self.scroll_preview_page(1, cx),
            Some(ControlAction::PreviewPageUp) => self.scroll_preview_page(-1, cx),
            None => false,
        }
    }

    fn handle_pane_focus_key(&mut self, event: &KeyDownEvent, cx: &mut Context<Self>) -> bool {
        if !self.pane_focus_prefix {
            return false;
        }
        if event.is_held || event.keystroke.modifiers.modified() {
            self.pane_focus_prefix = false;
            return false;
        }

        match event.keystroke.key.as_str() {
            "h" | "H" | "k" | "K" => {
                self.focus_browser_pane();
                true
            }
            "j" | "J" | "l" | "L" => {
                self.focus_preview_pane();
                true
            }
            "w" | "W" => {
                self.focus_next_pane();
                true
            }
            "d" | "D" => self.scroll_preview_page(1, cx),
            "u" | "U" => self.scroll_preview_page(-1, cx),
            _ => {
                self.pane_focus_prefix = false;
                false
            }
        }
    }
}
