use gpui::KeyDownEvent;

use super::command_char_from_key;
use crate::features::file_browser::{ConfirmModeAction, RenameModeAction};

pub fn rename_key_action(event: &KeyDownEvent) -> RenameModeAction {
    if event.is_held {
        return RenameModeAction::Consume;
    }

    match event.keystroke.key.as_str() {
        "escape" => RenameModeAction::Cancel,
        "backspace" => RenameModeAction::Backspace,
        "enter" => RenameModeAction::Submit,
        _ => command_char_from_key(event)
            .map(RenameModeAction::Insert)
            .unwrap_or(RenameModeAction::Consume),
    }
}

pub fn confirm_key_action(event: &KeyDownEvent) -> ConfirmModeAction {
    if event.is_held {
        return ConfirmModeAction::Consume;
    }

    match event.keystroke.key.as_str() {
        "escape" | "n" => ConfirmModeAction::Cancel,
        "enter" | "y" => ConfirmModeAction::Confirm,
        _ => ConfirmModeAction::Ignore,
    }
}
