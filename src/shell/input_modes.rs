use gpui::{Context, KeyDownEvent};

use super::FilemanShell;
use crate::features::{
    file_browser::{FileOperation, InputMode, PendingConfirm},
    keybind::{
        ConfirmKeyAction, ControlAction, RenameKeyAction, confirm_key_action, control_action,
        navigation_input, rename_key_action,
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
                match new_name.is_empty() || new_name == target.name {
                    true => self.status = "rename unchanged".to_string(),
                    false => self.run_operation(FileOperation::Rename { target, new_name }, cx),
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

    pub(super) fn handle_confirm_key(
        &mut self,
        event: &KeyDownEvent,
        cx: &mut Context<Self>,
    ) -> bool {
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
