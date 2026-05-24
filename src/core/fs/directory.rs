use std::{fs, io, path, time::UNIX_EPOCH};

use crate::core::{DirEntry, EntryLocation};

pub fn read_fs_directory(path: &path::Path) -> anyhow::Result<Vec<DirEntry>> {
    let mut dir_entries = Vec::new();

    for entry in fs::read_dir(path)? {
        dir_entries.push(dir_entry(entry?)?);
    }

    dir_entries.sort_by(|a, b| b.is_dir.cmp(&a.is_dir).then_with(|| a.name.cmp(&b.name)));
    Ok(dir_entries)
}

fn dir_entry(entry: fs::DirEntry) -> io::Result<DirEntry> {
    let file_name = entry.file_name().to_string_lossy().to_string();
    let file_type = entry.file_type()?;
    let is_symlink = file_type.is_symlink();
    let metadata = metadata_for(&entry, is_symlink);
    let is_dir = match is_symlink {
        true => metadata.as_ref().map(|m| m.is_dir()).unwrap_or(false),
        false => file_type.is_dir(),
    };

    Ok(DirEntry {
        name: file_name,
        is_dir,
        is_symlink,
        is_executable: is_executable(metadata.as_ref()),
        link_target: None,
        location: EntryLocation::Fs(entry.path()),
        size: metadata.as_ref().filter(|_| !is_dir).map(|m| m.len()),
        modified: metadata.as_ref().and_then(modified_secs),
    })
}

fn metadata_for(entry: &fs::DirEntry, is_symlink: bool) -> Option<fs::Metadata> {
    match is_symlink {
        true => fs::metadata(entry.path()).ok(),
        false => entry.metadata().ok(),
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
            "fileman-directory-{}-without-parent",
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
