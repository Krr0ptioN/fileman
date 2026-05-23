use gpui::{Context, KeyDownEvent};

use super::FilemanShell;
use crate::features::keybind::{
    AppKeyHandler, BrowserVimOutcome, HelpAction, apply_browser_vim_char,
};

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
        match apply_browser_vim_char(&mut self.vim_command, &self.keybinds, ch) {
            BrowserVimOutcome::Ignored => false,
            BrowserVimOutcome::Pending(status) => {
                self.status = status;
                true
            }
            BrowserVimOutcome::Command { command, sequence } => {
                self.execute_browser_command(command, sequence.as_str(), cx)
            }
        }
    }
}
