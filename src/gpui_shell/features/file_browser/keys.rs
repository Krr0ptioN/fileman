use gpui::KeyDownEvent;

#[derive(Default)]
pub(crate) struct HeldNavigation {
    key: Option<NavigationKey>,
    repeats: usize,
}

impl HeldNavigation {
    pub(crate) fn reset(&mut self) {
        self.key = None;
        self.repeats = 0;
    }

    pub(crate) fn advance(&mut self, key: NavigationKey) -> usize {
        if self.key == Some(key) {
            self.repeats = self.repeats.saturating_add(1);
        } else {
            self.key = Some(key);
            self.repeats = 1;
        }
        accelerated_rows(self.repeats)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum NavigationKey {
    Down,
    Up,
}

impl NavigationKey {
    pub(crate) fn delta(self, rows: usize) -> isize {
        match self {
            Self::Down => rows as isize,
            Self::Up => -(rows as isize),
        }
    }
}

pub(crate) fn navigation_key(event: &KeyDownEvent) -> Option<NavigationKey> {
    let modifiers = event.keystroke.modifiers;
    if modifiers.control
        || modifiers.alt
        || modifiers.shift
        || modifiers.platform
        || modifiers.function
    {
        return None;
    }

    match event.keystroke.key.as_str() {
        "down" => Some(NavigationKey::Down),
        "up" => Some(NavigationKey::Up),
        _ => match event.keystroke.key_char.as_deref() {
            Some("j") => Some(NavigationKey::Down),
            Some("k") => Some(NavigationKey::Up),
            _ => None,
        },
    }
}

fn accelerated_rows(repeats: usize) -> usize {
    match repeats {
        0..=4 => 1,
        5..=12 => 2,
        13..=24 => 4,
        _ => 8,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn held_navigation_accelerates_by_repeat_count() {
        let mut held = HeldNavigation::default();
        let steps: Vec<_> = (0..26).map(|_| held.advance(NavigationKey::Down)).collect();

        assert_eq!(&steps[0..4], &[1, 1, 1, 1]);
        assert_eq!(&steps[4..12], &[2, 2, 2, 2, 2, 2, 2, 2]);
        assert_eq!(&steps[12..24], &[4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4]);
        assert_eq!(&steps[24..26], &[8, 8]);
    }

    #[test]
    fn held_navigation_resets_when_direction_changes() {
        let mut held = HeldNavigation::default();
        for _ in 0..12 {
            held.advance(NavigationKey::Down);
        }

        assert_eq!(held.advance(NavigationKey::Up), 1);
    }
}
