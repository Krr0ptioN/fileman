#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BrowserCommand {
    Move(isize),
    MovePage(isize),
    First,
    Last,
    Line(usize),
    OpenParent,
    OpenSelected,
    FilenameSearch,
    CancelSearch,
    NewTab,
    NextTab,
    PreviousTab,
    CloseTab,
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
    CancelTask,
    Delete,
    Rename,
    NewDirectory,
    Preview,
    TogglePaneMode,
    ToggleHidden,
    ToggleIgnored,
    SwitchPanel,
    Reload,
    OpenHelp,
}

impl BrowserCommand {
    pub fn requires_rows(self) -> bool {
        !matches!(
            self,
            Self::OpenParent
                | Self::ToggleHidden
                | Self::ToggleIgnored
                | Self::SwitchPanel
                | Self::OpenHelp
                | Self::NewDirectory
                | Self::CancelTask
                | Self::FilenameSearch
                | Self::CancelSearch
                | Self::NewTab
                | Self::NextTab
                | Self::PreviousTab
                | Self::CloseTab
        )
    }

    pub fn reports_selection(self) -> bool {
        matches!(
            self,
            Self::Move(_) | Self::MovePage(_) | Self::First | Self::Last | Self::Line(_)
        )
    }
}
