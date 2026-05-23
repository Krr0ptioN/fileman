use gpui::KeyDownEvent;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ControlAction {
    SwitchPanel,
}

pub fn control_action(event: &KeyDownEvent) -> Option<ControlAction> {
    let key = event.keystroke.key.as_str();
    let modifiers = event.keystroke.modifiers;

    match (
        event.is_held,
        key,
        modifiers.control,
        modifiers.alt,
        modifiers.shift,
        modifiers.platform,
        modifiers.modified(),
    ) {
        (true, _, _, _, _, _, _) => None,
        (false, "tab", _, _, _, _, false) => Some(ControlAction::SwitchPanel),
        (false, "i" | "I", true, false, false, false, _) => Some(ControlAction::SwitchPanel),
        _ => None,
    }
}
