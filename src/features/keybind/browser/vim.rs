use super::executor::execute_browser_sequence;
use super::target::BrowserCommandExecutor;
use crate::features::keybind::VimCommandStep;

pub trait BrowserVimInput<Cx>: BrowserCommandExecutor<Cx> {
    fn push_command_char(&mut self, ch: char) -> VimCommandStep;
    fn show_pending_command(&mut self);
}

pub fn apply_browser_vim_char<T, Cx>(target: &mut T, ch: char, cx: &mut Cx) -> bool
where
    T: BrowserVimInput<Cx>,
{
    match target.push_command_char(ch) {
        VimCommandStep::Ignored => false,
        VimCommandStep::Pending => {
            target.show_pending_command();
            true
        }
        VimCommandStep::Execute {
            sequence,
            count,
            explicit_count,
            had_pending,
        } => {
            let handled =
                execute_browser_sequence(target, sequence.as_str(), count, explicit_count, cx);
            match (handled, had_pending) {
                (false, true) => apply_browser_vim_char(target, ch, cx),
                _ => handled,
            }
        }
    }
}
