use gpui::KeyDownEvent;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ControlAction {
    SwitchPanel,
    QuickJump,
}

pub fn control_action(event: &KeyDownEvent) -> Option<ControlAction> {
    let modifiers = event.keystroke.modifiers;
    control_action_for_input(ControlInput {
        key: event.keystroke.key.as_str(),
        is_held: event.is_held,
        control: modifiers.control,
        alt: modifiers.alt,
        shift: modifiers.shift,
        platform: modifiers.platform,
        modified: modifiers.modified(),
    })
}

#[derive(Clone, Copy)]
struct ControlInput<'a> {
    key: &'a str,
    is_held: bool,
    control: bool,
    alt: bool,
    shift: bool,
    platform: bool,
    modified: bool,
}

fn control_action_for_input(input: ControlInput<'_>) -> Option<ControlAction> {
    match (
        input.is_held,
        input.key,
        input.control,
        input.alt,
        input.shift,
        input.platform,
        input.modified,
    ) {
        (true, _, _, _, _, _, _) => None,
        (false, "tab", _, _, _, _, false) => Some(ControlAction::SwitchPanel),
        (false, "i" | "I", true, false, false, false, _) => Some(ControlAction::SwitchPanel),
        (false, "g" | "G", true, false, false, false, _) => Some(ControlAction::QuickJump),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{ControlAction, ControlInput, control_action_for_input};

    fn input(key: &'static str) -> ControlInput<'static> {
        ControlInput {
            key,
            is_held: false,
            control: false,
            alt: false,
            shift: false,
            platform: false,
            modified: false,
        }
    }

    #[test]
    fn maps_tab_and_ctrl_i_to_switch_panel() {
        assert_eq!(
            control_action_for_input(input("tab")),
            Some(ControlAction::SwitchPanel)
        );

        let mut ctrl_i = input("i");
        ctrl_i.control = true;
        ctrl_i.modified = true;
        assert_eq!(
            control_action_for_input(ctrl_i),
            Some(ControlAction::SwitchPanel)
        );
    }

    #[test]
    fn maps_ctrl_g_to_quick_jump() {
        let mut ctrl_g = input("g");
        ctrl_g.control = true;
        ctrl_g.modified = true;

        assert_eq!(
            control_action_for_input(ctrl_g),
            Some(ControlAction::QuickJump)
        );
    }

    #[test]
    fn ignores_held_and_extra_modified_control_keys() {
        let mut held = input("g");
        held.control = true;
        held.modified = true;
        held.is_held = true;
        assert_eq!(control_action_for_input(held), None);

        let mut shifted = input("g");
        shifted.control = true;
        shifted.shift = true;
        shifted.modified = true;
        assert_eq!(control_action_for_input(shifted), None);
    }
}
