use gpui::KeyDownEvent;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ControlAction {
    SwitchPanel,
    FilenameSearch,
    QuickJump,
    PaneFocusPrefix,
    PreviewPageDown,
    PreviewPageUp,
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
        (false, "f7", false, true, false, false, _) => Some(ControlAction::FilenameSearch),
        (false, "i" | "I", true, false, false, false, _) => Some(ControlAction::SwitchPanel),
        (false, "g" | "G", true, false, false, false, _) => Some(ControlAction::QuickJump),
        (false, "w" | "W", true, false, false, false, _) => Some(ControlAction::PaneFocusPrefix),
        (false, "d" | "D", true, false, false, false, _) => Some(ControlAction::PreviewPageDown),
        (false, "u" | "U", true, false, false, false, _) => Some(ControlAction::PreviewPageUp),
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
    fn maps_alt_f7_to_filename_search() {
        let mut alt_f7 = input("f7");
        alt_f7.alt = true;
        alt_f7.modified = true;

        assert_eq!(
            control_action_for_input(alt_f7),
            Some(ControlAction::FilenameSearch)
        );
    }

    #[test]
    fn maps_ctrl_w_to_pane_focus_prefix() {
        let mut ctrl_w = input("w");
        ctrl_w.control = true;
        ctrl_w.modified = true;

        assert_eq!(
            control_action_for_input(ctrl_w),
            Some(ControlAction::PaneFocusPrefix)
        );
    }

    #[test]
    fn maps_ctrl_d_and_ctrl_u_to_preview_page_navigation() {
        let mut ctrl_d = input("d");
        ctrl_d.control = true;
        ctrl_d.modified = true;
        assert_eq!(
            control_action_for_input(ctrl_d),
            Some(ControlAction::PreviewPageDown)
        );

        let mut ctrl_u = input("u");
        ctrl_u.control = true;
        ctrl_u.modified = true;
        assert_eq!(
            control_action_for_input(ctrl_u),
            Some(ControlAction::PreviewPageUp)
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
