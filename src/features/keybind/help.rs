use gpui::KeyDownEvent;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HelpAction {
    Open,
    Close,
}

pub fn help_action(event: &KeyDownEvent, open: bool) -> Option<HelpAction> {
    match (event.is_held, event.keystroke.modifiers.modified()) {
        (true, _) | (_, true) => None,
        _ => help_action_for_key(event.keystroke.key.as_str(), open),
    }
}

fn help_action_for_key(key: &str, open: bool) -> Option<HelpAction> {
    match (key, open) {
        (";" | "space", false) => Some(HelpAction::Open),
        (";" | "space" | "escape" | "q", true) => Some(HelpAction::Close),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{HelpAction, help_action_for_key};

    #[test]
    fn opens_help_from_normal_mode() {
        assert_eq!(help_action_for_key(";", false), Some(HelpAction::Open));
        assert_eq!(help_action_for_key("space", false), Some(HelpAction::Open));
    }

    #[test]
    fn closes_open_help() {
        assert_eq!(help_action_for_key("escape", true), Some(HelpAction::Close));
        assert_eq!(help_action_for_key("q", true), Some(HelpAction::Close));
    }
}
