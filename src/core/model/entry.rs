use std::path;

use crate::archive::ContainerKind;

#[derive(Clone)]
pub enum EntryLocation {
    Fs(path::PathBuf),
    Container {
        kind: ContainerKind,
        archive_path: path::PathBuf,
        inner_path: String,
    },
    Remote {
        host: String,
        path: String,
    },
}

impl EntryLocation {
    pub fn display_name(&self) -> String {
        match *self {
            EntryLocation::Fs(ref path) => path
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("<unknown>")
                .to_string(),
            EntryLocation::Container { ref inner_path, .. } => inner_path
                .rsplit('/')
                .next()
                .unwrap_or("<unknown>")
                .to_string(),
            EntryLocation::Remote { ref path, .. } => {
                path.rsplit('/').next().unwrap_or("<unknown>").to_string()
            }
        }
    }
}

#[derive(Clone)]
pub struct DirEntry {
    pub name: String,
    pub is_dir: bool,
    pub is_symlink: bool,
    pub is_executable: bool,
    pub link_target: Option<String>,
    pub location: EntryLocation,
    pub size: Option<u64>,
    pub modified: Option<u64>,
}

pub enum DirBatch {
    Append(Vec<DirEntry>),
    Replace(Vec<DirEntry>),
    ContainerRoot(Option<String>),
    Loading,
    Progress { loaded: usize, total: Option<usize> },
    Error(String),
    ConnectionError(String),
}
