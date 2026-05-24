use gpui::KeyDownEvent;

pub fn command_char_from_key(event: &KeyDownEvent) -> Option<char> {
    command_char_from_input(CommandKeyInput {
        key: event.keystroke.key.as_str(),
        key_char: event.keystroke.key_char.as_deref(),
        is_held: event.is_held,
        control: event.keystroke.modifiers.control,
        alt: event.keystroke.modifiers.alt,
        platform: event.keystroke.modifiers.platform,
        function: event.keystroke.modifiers.function,
    })
}

#[derive(Clone, Copy)]
pub struct CommandKeyInput<'a> {
    pub key: &'a str,
    pub key_char: Option<&'a str>,
    pub is_held: bool,
    pub control: bool,
    pub alt: bool,
    pub platform: bool,
    pub function: bool,
}

pub fn command_char_from_input(input: CommandKeyInput<'_>) -> Option<char> {
    if input.is_held || input.control || input.alt || input.platform || input.function {
        return None;
    }

    let key = input.key_char.unwrap_or(input.key);
    if key.chars().count() != 1 {
        return None;
    }

    key.chars().next().filter(|ch| !ch.is_control())
}

#[cfg(test)]
mod tests {
    use super::{CommandKeyInput, command_char_from_input};

    fn input(key: &'static str) -> CommandKeyInput<'static> {
        CommandKeyInput {
            key,
            key_char: None,
            is_held: false,
            control: false,
            alt: false,
            platform: false,
            function: false,
        }
    }

    #[test]
    fn maps_single_key_to_command_char() {
        assert_eq!(command_char_from_input(input("j")), Some('j'));
        assert_eq!(command_char_from_input(input("?")), Some('?'));
    }

    #[test]
    fn prefers_text_character_from_gpui() {
        let mut input = input("slash");
        input.key_char = Some("/");

        assert_eq!(command_char_from_input(input), Some('/'));
    }

    #[test]
    fn ignores_modified_held_and_named_keys() {
        let mut modified = input("j");
        modified.control = true;
        assert_eq!(command_char_from_input(modified), None);

        let mut held = input("j");
        held.is_held = true;
        assert_eq!(command_char_from_input(held), None);

        assert_eq!(command_char_from_input(input("escape")), None);
    }
}
