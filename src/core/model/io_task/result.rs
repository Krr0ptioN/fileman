use super::IOTask;

pub enum IOResult {
    Completed,
    CompletedRemote(String),
    CompletedSilent,
    Error(String),
    ErrorRemote(String, String),
    PermissionError { message: String, task: IOTask },
}
