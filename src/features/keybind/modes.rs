use gpui::KeyDownEvent;

use super::command_char_from_key;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenameKeyAction {
    Cancel,
    Backspace,
    Submit,
    Insert(char),
    Consume,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConfirmKeyAction {
    Cancel,
    Confirm,
    Consume,
    Ignore,
}

pub fn rename_key_action(event: &KeyDownEvent) -> RenameKeyAction {
    if event.is_held {
        return RenameKeyAction::Consume;
    }

    match event.keystroke.key.as_str() {
        "escape" => RenameKeyAction::Cancel,
        "backspace" => RenameKeyAction::Backspace,
        "enter" => RenameKeyAction::Submit,
        _ => command_char_from_key(event)
            .map(RenameKeyAction::Insert)
            .unwrap_or(RenameKeyAction::Consume),
    }
}

pub fn confirm_key_action(event: &KeyDownEvent) -> ConfirmKeyAction {
    if event.is_held {
        return ConfirmKeyAction::Consume;
    }

    match event.keystroke.key.as_str() {
        "escape" | "n" => ConfirmKeyAction::Cancel,
        "enter" | "y" => ConfirmKeyAction::Confirm,
        _ => ConfirmKeyAction::Ignore,
    }
}
