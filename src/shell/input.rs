use gpui::{Context, KeyDownEvent, Window};

use super::StiffShell;
use crate::features::keybind::{KeyCommandAction, handle_key_command};

impl StiffShell {
    pub(super) fn on_key_down(
        &mut self,
        event: &KeyDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
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
}
