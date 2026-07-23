mod format;
mod intent;
mod kind;

use std::path::PathBuf;

use gpui::SharedString;

use crate::core;

pub use format::FileFormat;
pub use intent::{RowIntent, row_intent};
pub use kind::{RowKind, kind_label};

#[derive(Clone)]
pub struct FileRow {
    pub kind: RowKind,
    pub name: SharedString,
    pub detail: SharedString,
    pub path: PathBuf,
    pub is_dir: bool,
    pub is_executable: bool,
}

impl FileRow {
    pub fn from_entry(entry: core::DirEntry) -> Self {
        let path = match entry.location {
            core::EntryLocation::Fs(ref path) => path.clone(),
            _ => PathBuf::new(),
        };
        let kind = RowKind::from_entry(&entry, &path);
        let detail = match kind {
            RowKind::Directory => SharedString::default(),
            _ => entry.size.map(core::format_size).unwrap_or_default().into(),
        };

        Self {
            kind,
            name: entry.name.into(),
            detail,
            path,
            is_dir: entry.is_dir,
            is_executable: entry.is_executable,
        }
    }
}
