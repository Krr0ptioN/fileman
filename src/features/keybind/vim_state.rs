/// Stateful parser for ranger-style normal-mode key chains.
///
/// This slice is intentionally UI-neutral. Event adapters feed text characters
/// from GPUI, replay, or tests into this parser and then map executable
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
        match text.is_empty() {
            true => None,
            false => Some(text),
        }
    }

    pub fn push_with_prefixes(
        &mut self,
        ch: char,
        is_prefix: impl Fn(&str) -> bool,
    ) -> VimCommandStep {
        if ch.is_whitespace() {
            return VimCommandStep::Ignored;
        }

        let explicit_count = self.count.take();
        if self.pending.is_empty() && ch.is_ascii_digit() {
            return self.push_count_or_zero(ch, explicit_count);
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
        self.clear();
        VimCommandStep::Execute {
            sequence,
            count,
            explicit_count: explicit_count.is_some(),
            had_pending,
        }
    }

    fn push_count_or_zero(&mut self, ch: char, explicit_count: Option<usize>) -> VimCommandStep {
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
        VimCommandStep::Pending
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

#[cfg(test)]
mod tests {
    use super::{VimCommandState, VimCommandStep};

    fn is_prefix(sequence: &str) -> bool {
        matches!(sequence, "g" | "y" | "d")
    }

    #[test]
    fn builds_count_before_command() {
        let mut state = VimCommandState::default();

        assert!(matches!(
            state.push_with_prefixes('1', is_prefix),
            VimCommandStep::Pending
        ));
        assert!(matches!(
            state.push_with_prefixes('2', is_prefix),
            VimCommandStep::Pending
        ));
        assert_eq!(state.display().as_deref(), Some("12"));

        assert!(matches!(
            state.push_with_prefixes('j', is_prefix),
            VimCommandStep::Execute {
                ref sequence,
                count: 12,
                explicit_count: true,
                had_pending: false,
            } if sequence == "j"
        ));
        assert!(state.display().is_none());
    }

    #[test]
    fn treats_zero_as_command_without_existing_count() {
        let mut state = VimCommandState::default();

        assert!(matches!(
            state.push_with_prefixes('0', is_prefix),
            VimCommandStep::Execute {
                ref sequence,
                count: 1,
                explicit_count: false,
                had_pending: false,
            } if sequence == "0"
        ));
    }

    #[test]
    fn appends_zero_to_existing_count() {
        let mut state = VimCommandState::default();

        assert!(matches!(
            state.push_with_prefixes('2', is_prefix),
            VimCommandStep::Pending
        ));
        assert!(matches!(
            state.push_with_prefixes('0', is_prefix),
            VimCommandStep::Pending
        ));
        assert_eq!(state.display().as_deref(), Some("20"));
    }

    #[test]
    fn tracks_pending_prefix_and_reports_completed_sequence() {
        let mut state = VimCommandState::default();

        assert!(matches!(
            state.push_with_prefixes('g', is_prefix),
            VimCommandStep::Pending
        ));
        assert_eq!(state.display().as_deref(), Some("g"));

        assert!(matches!(
            state.push_with_prefixes('g', is_prefix),
            VimCommandStep::Execute {
                ref sequence,
                count: 1,
                explicit_count: false,
                had_pending: true,
            } if sequence == "gg"
        ));
    }
}
