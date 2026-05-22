/// Stateful parser for ranger-style normal-mode key chains.
///
/// This slice is intentionally UI-neutral. Event adapters feed text characters
/// from egui, GPUI, replay, or tests into this parser and then map executable
/// sequences onto app commands.
#[derive(Default)]
pub struct VimCommandState {
    pub pending: String,
    pub count: Option<usize>,
}

impl VimCommandState {
    pub fn clear(&mut self) {
        self.pending.clear();
        self.count = None;
    }

    pub fn display(&self) -> Option<String> {
        let mut text = String::new();
        if let Some(count) = self.count {
            text.push_str(&count.to_string());
        }
        text.push_str(&self.pending);
        if text.is_empty() { None } else { Some(text) }
    }

    pub fn push(&mut self, ch: char) -> VimCommandStep {
        if ch.is_whitespace() {
            return VimCommandStep::Ignored;
        }

        let explicit_count = self.count.take();
        if self.pending.is_empty() && ch.is_ascii_digit() {
            if ch == '0' && explicit_count.is_none() {
                self.clear();
                return VimCommandStep::Execute {
                    sequence: "0".to_string(),
                    count: 1,
                    explicit_count: false,
                    had_pending: false,
                };
            }

            let digit = ch.to_digit(10).unwrap_or(0) as usize;
            let base = explicit_count.unwrap_or(0);
            self.count = Some(base.saturating_mul(10).saturating_add(digit).min(9999));
            return VimCommandStep::Pending;
        }

        let had_pending = !self.pending.is_empty();
        let mut sequence = std::mem::take(&mut self.pending);
        sequence.push(ch);
        if is_prefix(&sequence) {
            self.pending = sequence;
            self.count = explicit_count;
            return VimCommandStep::Pending;
        }

        let count = explicit_count.unwrap_or(1).max(1);
        let explicit = explicit_count.is_some();
        self.clear();
        VimCommandStep::Execute {
            sequence,
            count,
            explicit_count: explicit,
            had_pending,
        }
    }
}

pub enum VimCommandStep {
    Ignored,
    Pending,
    Execute {
        sequence: String,
        count: usize,
        explicit_count: bool,
        had_pending: bool,
    },
}

pub fn is_prefix(sequence: &str) -> bool {
    matches!(sequence, "c" | "d" | "g" | "n" | "p" | "u" | "y" | "z")
}

#[cfg(test)]
mod tests {
    use super::{VimCommandState, VimCommandStep};

    fn execute(step: VimCommandStep) -> (String, usize, bool, bool) {
        match step {
            VimCommandStep::Execute {
                sequence,
                count,
                explicit_count,
                had_pending,
            } => (sequence, count, explicit_count, had_pending),
            _ => panic!("expected executable step"),
        }
    }

    #[test]
    fn accumulates_counts_before_a_motion() {
        let mut state = VimCommandState::default();

        assert!(matches!(state.push('1'), VimCommandStep::Pending));
        assert!(matches!(state.push('2'), VimCommandStep::Pending));

        let (sequence, count, explicit_count, had_pending) = execute(state.push('j'));
        assert_eq!(sequence, "j");
        assert_eq!(count, 12);
        assert!(explicit_count);
        assert!(!had_pending);
        assert_eq!(state.display(), None);
    }

    #[test]
    fn keeps_count_visible_while_waiting_for_a_chain_suffix() {
        let mut state = VimCommandState::default();

        assert!(matches!(state.push('2'), VimCommandStep::Pending));
        assert!(matches!(state.push('g'), VimCommandStep::Pending));
        assert_eq!(state.display().as_deref(), Some("2g"));

        let (sequence, count, explicit_count, had_pending) = execute(state.push('g'));
        assert_eq!(sequence, "gg");
        assert_eq!(count, 2);
        assert!(explicit_count);
        assert!(had_pending);
    }

    #[test]
    fn treats_lone_zero_as_a_command_not_a_count() {
        let mut state = VimCommandState::default();

        let (sequence, count, explicit_count, had_pending) = execute(state.push('0'));
        assert_eq!(sequence, "0");
        assert_eq!(count, 1);
        assert!(!explicit_count);
        assert!(!had_pending);
    }

    #[test]
    fn emits_unknown_chain_for_caller_retry() {
        let mut state = VimCommandState::default();

        assert!(matches!(state.push('c'), VimCommandStep::Pending));
        let (sequence, count, explicit_count, had_pending) = execute(state.push('j'));
        assert_eq!(sequence, "cj");
        assert_eq!(count, 1);
        assert!(!explicit_count);
        assert!(had_pending);
    }

    #[test]
    fn clamps_large_counts() {
        let mut state = VimCommandState::default();

        for ch in "123456789".chars() {
            assert!(matches!(state.push(ch), VimCommandStep::Pending));
        }

        let (_, count, explicit_count, _) = execute(state.push('G'));
        assert_eq!(count, 9999);
        assert!(explicit_count);
    }
}
