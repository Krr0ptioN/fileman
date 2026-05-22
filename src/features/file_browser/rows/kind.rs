use std::path::Path;

use crate::core;

use super::FileFormat;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RowKind {
    Directory,
    Symlink,
    Socket,
    Pipe,
    BlockDevice,
    CharDevice,
    File(FileFormat),
    Other,
}

impl RowKind {
    pub fn from_entry(entry: &core::DirEntry, path: &Path) -> Self {
        match (entry.is_dir, entry.is_symlink) {
            (true, _) => Self::Directory,
            (_, true) => Self::Symlink,
            _ => Self::File(FileFormat::from_path(path)),
        }
    }
}

pub fn kind_label(kind: RowKind) -> &'static str {
    match kind {
        RowKind::Directory => "dir",
        RowKind::Symlink => "link",
        RowKind::Socket => "socket",
        RowKind::Pipe => "pipe",
        RowKind::BlockDevice => "block",
        RowKind::CharDevice => "char",
        RowKind::Other => "other",
        RowKind::File(format) => format.label(),
    }
}
