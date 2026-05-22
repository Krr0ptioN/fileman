use std::path;

pub struct EditLoadRequest {
    pub id: u64,
    pub path: path::PathBuf,
    pub remote: Option<(String, String)>,
}

pub struct EditLoadResult {
    pub id: u64,
    pub path: path::PathBuf,
    pub text: String,
    pub crlf: bool,
}
