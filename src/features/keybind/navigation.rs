use gpui::KeyDownEvent;

#[derive(Default)]
pub struct HeldNavigation {
    key: Option<NavigationKey>,
    repeats: usize,
}

impl HeldNavigation {
    pub fn reset(&mut self) {
        self.key = None;
        self.repeats = 0;
    }

    pub fn rows_for(&mut self, input: NavigationInput) -> (NavigationKey, usize) {
        match input {
            NavigationInput::Step(key) => {
                self.reset();
                (key, 1)
            }
            NavigationInput::Repeat(key) => (key, self.advance(key)),
        }
    }

    fn advance(&mut self, key: NavigationKey) -> usize {
        match self.key == Some(key) {
            true => self.repeats = self.repeats.saturating_add(1),
            false => {
                self.key = Some(key);
                self.repeats = 1;
            }
        }
        accelerated_rows(self.repeats)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NavigationInput {
    Step(NavigationKey),
    Repeat(NavigationKey),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NavigationKey {
    Down,
    Up,
}

impl NavigationKey {
    pub fn delta(self, rows: usize) -> isize {
        match self {
            Self::Down => rows as isize,
            Self::Up => -(rows as isize),
        }
    }
}

pub fn navigation_input(event: &KeyDownEvent) -> Option<NavigationInput> {
    match event.keystroke.modifiers.modified() {
        true => None,
        false => navigation_input_for_key(event.keystroke.key.as_str(), event.is_held),
    }
}

fn navigation_input_for_key(key: &str, is_held: bool) -> Option<NavigationInput> {
    match (key, is_held) {
        ("down", false) => Some(NavigationInput::Step(NavigationKey::Down)),
        ("up", false) => Some(NavigationInput::Step(NavigationKey::Up)),
        ("down", true) | ("j", true) => Some(NavigationInput::Repeat(NavigationKey::Down)),
        ("up", true) | ("k", true) => Some(NavigationInput::Repeat(NavigationKey::Up)),
        _ => None,
    }
}

fn accelerated_rows(repeats: usize) -> usize {
    match repeats {
        0..=9 => 1,
        10..=29 => 2,
        30..=59 => 4,
        _ => 8,
    }
}

#[cfg(test)]
mod tests {
    use super::{HeldNavigation, NavigationInput, NavigationKey, navigation_input_for_key};

    #[test]
    fn held_vim_navigation_repeats_but_plain_vim_keys_do_not() {
        assert_eq!(navigation_input_for_key("j", false), None);
        assert_eq!(
            navigation_input_for_key("j", true),
            Some(NavigationInput::Repeat(NavigationKey::Down))
        );
        assert_eq!(
            navigation_input_for_key("k", true),
            Some(NavigationInput::Repeat(NavigationKey::Up))
        );
    }

    #[test]
    fn held_navigation_accelerates() {
        let mut held = HeldNavigation::default();
        for _ in 0..9 {
            assert_eq!(
                held.rows_for(NavigationInput::Repeat(NavigationKey::Down)),
                (NavigationKey::Down, 1)
            );
        }
        assert_eq!(
            held.rows_for(NavigationInput::Repeat(NavigationKey::Down)),
            (NavigationKey::Down, 2)
        );
    }
}
