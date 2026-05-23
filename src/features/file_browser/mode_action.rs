#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenameModeAction {
    Cancel,
    Backspace,
    Submit,
    Insert(char),
    Consume,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConfirmModeAction {
    Cancel,
    Confirm,
    Consume,
    Ignore,
}
