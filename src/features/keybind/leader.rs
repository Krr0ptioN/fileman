use gpui::KeyDownEvent;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LeaderAction {
    Open,
    Close,
}

pub fn leader_action(event: &KeyDownEvent, open: bool) -> Option<LeaderAction> {
    match (event.is_held, event.keystroke.modifiers.modified()) {
        (true, _) | (_, true) => None,
        _ => leader_action_for_key(event.keystroke.key.as_str(), open),
    }
}

fn leader_action_for_key(key: &str, open: bool) -> Option<LeaderAction> {
    match (key, open) {
        (";" | "space", false) => Some(LeaderAction::Open),
        (";" | "space" | "escape" | "q", true) => Some(LeaderAction::Close),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{LeaderAction, leader_action_for_key};

    #[test]
    fn triggers_leader_map_from_normal_mode() {
        assert_eq!(leader_action_for_key(";", false), Some(LeaderAction::Open));
        assert_eq!(
            leader_action_for_key("space", false),
            Some(LeaderAction::Open)
        );
    }

    #[test]
    fn closes_open_leader_map() {
        assert_eq!(
            leader_action_for_key("escape", true),
            Some(LeaderAction::Close)
        );
        assert_eq!(leader_action_for_key("q", true), Some(LeaderAction::Close));
    }
}
