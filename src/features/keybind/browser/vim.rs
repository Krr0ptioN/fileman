use super::{BrowserCommand, Registry};
use crate::features::keybind::{VimCommandState, VimCommandStep};

pub enum BrowserVimOutcome {
    Ignored,
    Pending(String),
    Command {
        command: BrowserCommand,
        sequence: String,
    },
}

pub fn apply_browser_vim_char(
    state: &mut VimCommandState,
    keybinds: &Registry,
    ch: char,
) -> BrowserVimOutcome {
    match state.push_with_prefixes(ch, |sequence| keybinds.is_prefix(sequence)) {
        VimCommandStep::Ignored => BrowserVimOutcome::Ignored,
        VimCommandStep::Pending => {
            BrowserVimOutcome::Pending(state.display().unwrap_or_else(|| "normal".to_string()))
        }
        VimCommandStep::Execute {
            sequence,
            count,
            explicit_count,
            had_pending,
        } => {
            let command = keybinds.command_for(
                sequence.as_str(),
                crate::features::keybind::KeybindArgs {
                    count,
                    explicit_count,
                },
            );
            match (command, had_pending) {
                (Some(command), _) => BrowserVimOutcome::Command { command, sequence },
                (None, true) => apply_browser_vim_char(state, keybinds, ch),
                (None, false) => BrowserVimOutcome::Ignored,
            }
        }
    }
}
