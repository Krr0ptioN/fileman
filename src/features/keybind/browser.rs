#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BrowserCommand {
    Move(isize),
    MovePage(isize),
    First,
    Last,
    Line(usize),
    OpenParent,
    OpenSelected,
    ToggleMark(usize),
    ToggleAllMarks,
    ClearMarks,
    Copy,
    CopyPath,
    CopyName,
    CopyFileContents,
    MoveSelection,
    Paste,
    Delete,
    Rename,
    TogglePaneMode,
    SwitchPanel,
    Reload,
}

impl BrowserCommand {
    pub fn from_vim_sequence(sequence: &str, count: usize, explicit_count: bool) -> Option<Self> {
        let count = count.max(1);
        match sequence {
            "j" => Some(Self::Move(count as isize)),
            "k" => Some(Self::Move(-(count as isize))),
            "J" => Some(Self::MovePage((count * 8) as isize)),
            "K" => Some(Self::MovePage(-((count * 8) as isize))),
            "gg" => Some(Self::Line(if explicit_count {
                count.saturating_sub(1)
            } else {
                0
            })),
            "G" => {
                if explicit_count {
                    Some(Self::Line(count.saturating_sub(1)))
                } else {
                    Some(Self::Last)
                }
            }
            "0" => Some(Self::First),
            "h" => Some(Self::OpenParent),
            "l" => Some(Self::OpenSelected),
            "v" => Some(Self::ToggleMark(count)),
            "V" => Some(Self::ToggleAllMarks),
            "uv" | "uV" => Some(Self::ClearMarks),
            "yy" => Some(Self::Copy),
            "yp" => Some(Self::CopyPath),
            "yn" => Some(Self::CopyName),
            "yc" => Some(Self::CopyFileContents),
            "dd" => Some(Self::MoveSelection),
            "pp" => Some(Self::Paste),
            "dD" | "x" => Some(Self::Delete),
            "cw" | "C" => Some(Self::Rename),
            "s" => Some(Self::TogglePaneMode),
            "w" => Some(Self::SwitchPanel),
            "r" | "R" => Some(Self::Reload),
            _ => None,
        }
    }

    pub fn requires_rows(self) -> bool {
        !matches!(self, Self::OpenParent | Self::SwitchPanel)
    }
}

#[cfg(test)]
mod tests {
    use super::BrowserCommand;

    fn command(sequence: &str, count: usize, explicit_count: bool) -> Option<BrowserCommand> {
        BrowserCommand::from_vim_sequence(sequence, count, explicit_count)
    }

    #[test]
    fn maps_counted_navigation() {
        assert_eq!(command("j", 4, true), Some(BrowserCommand::Move(4)));
        assert_eq!(command("k", 3, true), Some(BrowserCommand::Move(-3)));
        assert_eq!(command("J", 2, true), Some(BrowserCommand::MovePage(16)));
    }

    #[test]
    fn maps_line_navigation() {
        assert_eq!(command("gg", 1, false), Some(BrowserCommand::Line(0)));
        assert_eq!(command("G", 10, true), Some(BrowserCommand::Line(9)));
        assert_eq!(command("G", 1, false), Some(BrowserCommand::Last));
    }

    #[test]
    fn maps_operations() {
        assert_eq!(command("yy", 1, false), Some(BrowserCommand::Copy));
        assert_eq!(command("yp", 1, false), Some(BrowserCommand::CopyPath));
        assert_eq!(command("dD", 1, false), Some(BrowserCommand::Delete));
        assert_eq!(command("zz", 1, false), None);
    }
}
