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
    CopyFiles,
    MoveSelection,
    Paste,
    Delete,
    Rename,
    TogglePaneMode,
    SwitchPanel,
    Reload,
    OpenHelp,
}

impl BrowserCommand {
    pub fn requires_rows(self) -> bool {
        !matches!(self, Self::OpenParent | Self::SwitchPanel | Self::OpenHelp)
    }

    pub fn reports_selection(self) -> bool {
        matches!(
            self,
            Self::Move(_) | Self::MovePage(_) | Self::First | Self::Last | Self::Line(_)
        )
    }
}
