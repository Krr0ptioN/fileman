use gpui::{Context, KeyDownEvent};

use super::FilemanShell;
use crate::features::{
    file_browser::{apply_confirm_action, apply_rename_action},
    keybind::{
        ControlAction, confirm_key_action, control_action, navigation_input, rename_key_action,
    },
};

impl FilemanShell {
    pub(super) fn handle_navigation_key(&mut self, event: &KeyDownEvent) -> bool {
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

    pub(super) fn handle_control_key(&mut self, event: &KeyDownEvent) -> bool {
        match control_action(event) {
            Some(ControlAction::SwitchPanel) => {
                self.active = self.active.other();
                self.active_panel().reveal_selected();
                self.status = format!("active {}", self.active.label());
                true
            }
            None => false,
        }
    }
}
