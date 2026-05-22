use gpui::KeyDownEvent;

pub fn command_char_from_key(event: &KeyDownEvent) -> Option<char> {
    if event.is_held {
        return None;
    }

    let modifiers = event.keystroke.modifiers;
    if modifiers.control || modifiers.alt || modifiers.platform || modifiers.function {
        return None;
    }

    let key = event
        .keystroke
        .key_char
        .as_deref()
        .unwrap_or(event.keystroke.key.as_str());
    if key.chars().count() != 1 {
        return None;
    }

    key.chars().next().filter(|ch| !ch.is_control())
}
