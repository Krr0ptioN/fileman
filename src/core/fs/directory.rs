use std::{ffi::OsStr, fs, io, path, time::UNIX_EPOCH};

use crate::core::{DirEntry, EntryLocation};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FsEntryKind {
    Directory,
    File,
}

struct PreparedDirEntry {
    entry: fs::DirEntry,
    file_name: std::ffi::OsString,
    kind: FsEntryKind,
    is_symlink: bool,
    metadata: Option<fs::Metadata>,
}

pub fn read_fs_directory(path: &path::Path) -> anyhow::Result<Vec<DirEntry>> {
    read_fs_directory_filtered(path, |_| true, |_, _| true)
}

pub fn read_fs_directory_filtered(
    path: &path::Path,
    mut include_name: impl FnMut(&OsStr) -> bool,
    mut include_entry: impl FnMut(&OsStr, FsEntryKind) -> bool,
) -> anyhow::Result<Vec<DirEntry>> {
    let mut dir_entries = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_name = entry.file_name();
        if !include_name(&file_name) {
            continue;
        }
        let file_type = entry.file_type()?;
        let is_symlink = file_type.is_symlink();
        let metadata = match is_symlink {
            true => fs::metadata(entry.path()).ok(),
            false => None,
        };
        let is_dir = match is_symlink {
            true => metadata.as_ref().is_some_and(fs::Metadata::is_dir),
            false => file_type.is_dir(),
        };
        let kind = match is_dir {
            true => FsEntryKind::Directory,
            false => FsEntryKind::File,
        };
        if include_entry(&file_name, kind) {
            dir_entries.push(
                PreparedDirEntry {
                    entry,
                    file_name,
                    kind,
                    is_symlink,
                    metadata,
                }
                .into_dir_entry()?,
            );
        }
    }

    dir_entries.sort_by(|a, b| b.is_dir.cmp(&a.is_dir).then_with(|| a.name.cmp(&b.name)));
    Ok(dir_entries)
}

impl PreparedDirEntry {
    fn into_dir_entry(self) -> io::Result<DirEntry> {
        let metadata = match self.metadata {
            Some(metadata) => Some(metadata),
            None => self.entry.metadata().ok(),
        };
        let is_dir = self.kind == FsEntryKind::Directory;

        Ok(DirEntry {
            name: self.file_name.to_string_lossy().into_owned(),
            is_dir,
            is_symlink: self.is_symlink,
            is_executable: is_executable(metadata.as_ref()),
            link_target: None,
            location: EntryLocation::Fs(self.entry.path()),
            size: metadata.as_ref().filter(|_| !is_dir).map(|m| m.len()),
            modified: metadata.as_ref().and_then(modified_secs),
        })
    }
}

fn is_executable(metadata: Option<&fs::Metadata>) -> bool {
    match metadata {
        Some(metadata) => executable_bit(metadata),
        None => false,
    }
}

#[cfg(unix)]
fn executable_bit(metadata: &fs::Metadata) -> bool {
    use std::os::unix::fs::PermissionsExt;
    metadata.permissions().mode() & 0o111 != 0
}

#[cfg(not(unix))]
fn executable_bit(_: &fs::Metadata) -> bool {
    false
}

fn modified_secs(metadata: &fs::Metadata) -> Option<u64> {
    metadata
        .modified()
        .ok()
        .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
        .map(|duration| duration.as_secs())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::read_fs_directory;

    #[test]
    fn directory_read_does_not_create_parent_row() {
        let directory = std::env::temp_dir().join(format!(
            "stiff-directory-{}-without-parent",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&directory);
        fs::create_dir_all(&directory).unwrap();

        let entries = read_fs_directory(&directory).unwrap();

        assert_eq!(entries.len(), 0);
        assert!(!entries.iter().any(|entry| entry.name == ".."));

        fs::remove_dir_all(directory).unwrap();
    }
}
