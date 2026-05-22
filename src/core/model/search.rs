use std::path;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SearchMode {
    Name,
    Content,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SearchCase {
    Sensitive,
    Insensitive,
}

pub struct SearchRequest {
    pub id: u64,
    pub root: path::PathBuf,
    pub needle: String,
    pub case: SearchCase,
    pub mode: SearchMode,
    pub remote: Option<(String, String)>,
}

#[derive(Clone)]
pub struct SearchResult {
    pub path: path::PathBuf,
    pub is_dir: bool,
    pub size: Option<u64>,
    pub modified: Option<u64>,
    pub remote_path: Option<String>,
}

#[derive(Clone, Copy)]
pub struct SearchProgress {
    pub scanned: usize,
    pub matched: usize,
}

pub enum SearchEvent {
    Match { id: u64, result: SearchResult },
    Progress { id: u64, progress: SearchProgress },
    Done { id: u64, progress: SearchProgress },
    Error { id: u64, message: String },
}
