#[cfg(unix)]
use std::os::unix::fs::{FileTypeExt, PermissionsExt};
use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

use crate::core;

#[derive(Clone)]
pub(crate) struct FileRow {
    pub(crate) kind: RowKind,
    pub(crate) name: String,
    pub(crate) detail: String,
    pub(crate) path: PathBuf,
    pub(crate) is_dir: bool,
    pub(crate) is_executable: bool,
}

impl FileRow {
    pub(crate) fn from_entry(entry: core::DirEntry) -> Self {
        let path = match &entry.location {
            core::EntryLocation::Fs(path) => path.clone(),
            _ => PathBuf::new(),
        };
        let kind = classify_entry(&entry, &path);
        let is_executable = is_executable_path(&path, kind);
        let link_target = if entry.is_symlink {
            entry.link_target.clone().or_else(|| {
                fs::read_link(&path)
                    .ok()
                    .map(|target| target.display().to_string())
            })
        } else {
            None
        };
        let detail = format_entry_detail(&entry, kind, link_target.as_deref());
        Self {
            kind,
            name: entry.name,
            detail,
            path,
            is_dir: entry.is_dir,
            is_executable,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum RowKind {
    Directory,
    Symlink,
    Socket,
    Pipe,
    BlockDevice,
    CharDevice,
    File(FileFormat),
    Other,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum FileFormat {
    Archive,
    Audio,
    Binary,
    Code,
    Image,
    Pdf,
    Text,
    Video,
    Unknown,
}

impl FileFormat {
    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Archive => "archive",
            Self::Audio => "audio",
            Self::Binary => "binary",
            Self::Code => "code",
            Self::Image => "image",
            Self::Pdf => "pdf",
            Self::Text => "text",
            Self::Video => "video",
            Self::Unknown => "file",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum RowIntent {
    None,
    Marked,
    Copy,
    Move,
    Delete,
}

pub(crate) fn kind_label(kind: RowKind) -> &'static str {
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

pub(crate) fn row_intent(
    path: &Path,
    marked: bool,
    copy_targets: &HashSet<PathBuf>,
    move_targets: &HashSet<PathBuf>,
    delete_targets: &HashSet<PathBuf>,
) -> RowIntent {
    if delete_targets.contains(path) {
        RowIntent::Delete
    } else if move_targets.contains(path) {
        RowIntent::Move
    } else if copy_targets.contains(path) {
        RowIntent::Copy
    } else if marked {
        RowIntent::Marked
    } else {
        RowIntent::None
    }
}

fn classify_entry(entry: &core::DirEntry, path: &Path) -> RowKind {
    if entry.name == ".." {
        return RowKind::Directory;
    }
    if entry.is_symlink {
        return RowKind::Symlink;
    }

    if let Ok(metadata) = fs::symlink_metadata(path) {
        let file_type = metadata.file_type();
        if file_type.is_dir() {
            return RowKind::Directory;
        }
        if file_type.is_file() {
            return RowKind::File(file_format_from_path(path));
        }

        #[cfg(unix)]
        {
            if file_type.is_socket() {
                return RowKind::Socket;
            }
            if file_type.is_fifo() {
                return RowKind::Pipe;
            }
            if file_type.is_block_device() {
                return RowKind::BlockDevice;
            }
            if file_type.is_char_device() {
                return RowKind::CharDevice;
            }
        }

        return RowKind::Other;
    }

    if entry.is_dir {
        RowKind::Directory
    } else {
        RowKind::File(file_format_from_path(path))
    }
}

fn is_executable_path(path: &Path, kind: RowKind) -> bool {
    if !matches!(kind, RowKind::File(_)) {
        return false;
    }

    #[cfg(unix)]
    {
        fs::metadata(path)
            .map(|metadata| metadata.permissions().mode() & 0o111 != 0)
            .unwrap_or(false)
    }

    #[cfg(not(unix))]
    {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| {
                matches!(
                    ext.to_ascii_lowercase().as_str(),
                    "bat" | "cmd" | "com" | "exe" | "ps1"
                )
            })
            .unwrap_or(false)
    }
}

fn file_format_from_path(path: &Path) -> FileFormat {
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();

    match name.as_str() {
        "dockerfile" | "makefile" | "justfile" | "rakefile" | "gemfile" => return FileFormat::Code,
        "license" | "notice" | "readme" => return FileFormat::Text,
        _ => {}
    }

    let Some(ext) = path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(str::to_ascii_lowercase)
    else {
        return FileFormat::Unknown;
    };

    match ext.as_str() {
        "7z" | "bz2" | "gz" | "rar" | "tar" | "tgz" | "xz" | "zip" | "zst" => FileFormat::Archive,
        "aac" | "aiff" | "flac" | "m4a" | "mp3" | "ogg" | "opus" | "wav" => FileFormat::Audio,
        "bin" | "dll" | "dmg" | "exe" | "iso" | "o" | "so" => FileFormat::Binary,
        "c" | "cc" | "cpp" | "css" | "go" | "h" | "hpp" | "html" | "java" | "js" | "jsx" | "kt"
        | "lua" | "php" | "py" | "rb" | "rs" | "scss" | "sh" | "sql" | "svelte" | "swift"
        | "toml" | "ts" | "tsx" | "vue" | "xml" | "yaml" | "yml" => FileFormat::Code,
        "avif" | "bmp" | "gif" | "heic" | "ico" | "jpeg" | "jpg" | "png" | "svg" | "tif"
        | "tiff" | "webp" => FileFormat::Image,
        "pdf" => FileFormat::Pdf,
        "csv" | "log" | "md" | "rst" | "txt" => FileFormat::Text,
        "avi" | "m4v" | "mkv" | "mov" | "mp4" | "mpeg" | "mpg" | "webm" => FileFormat::Video,
        _ => FileFormat::Unknown,
    }
}

fn format_entry_detail(entry: &core::DirEntry, kind: RowKind, link_target: Option<&str>) -> String {
    if entry.name == ".." {
        return "parent".to_string();
    }

    match kind {
        RowKind::Directory => "dir".to_string(),
        RowKind::Symlink => link_target
            .map(|target| format!("link -> {target}"))
            .unwrap_or_else(|| "link".to_string()),
        RowKind::Socket => "socket".to_string(),
        RowKind::Pipe => "pipe".to_string(),
        RowKind::BlockDevice => "block device".to_string(),
        RowKind::CharDevice => "char device".to_string(),
        RowKind::Other => "other".to_string(),
        RowKind::File(format) => entry
            .size
            .map(|size| format!("{} {}", format.label(), format_size(size)))
            .unwrap_or_else(|| format.label().to_string()),
    }
}

fn format_size(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut value = size as f64;
    let mut unit = 0usize;
    while value >= 1024.0 && unit + 1 < UNITS.len() {
        value /= 1024.0;
        unit += 1;
    }
    if unit == 0 {
        format!("{size} {}", UNITS[unit])
    } else {
        format!("{value:.1} {}", UNITS[unit])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn fs_entry(path: PathBuf, name: &str, is_dir: bool, is_symlink: bool) -> core::DirEntry {
        core::DirEntry {
            name: name.to_string(),
            is_dir,
            is_symlink,
            link_target: None,
            location: core::EntryLocation::Fs(path),
            size: Some(42),
            modified: None,
        }
    }

    fn unique_test_dir(test_name: &str) -> PathBuf {
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        std::env::temp_dir().join(format!("fileman-{test_name}-{suffix}"))
    }

    #[test]
    fn detects_file_formats_from_extension_and_special_names() {
        assert_eq!(
            file_format_from_path(Path::new("Cargo.toml")),
            FileFormat::Code
        );
        assert_eq!(file_format_from_path(Path::new("README")), FileFormat::Text);
        assert_eq!(
            file_format_from_path(Path::new("image.png")),
            FileFormat::Image
        );
        assert_eq!(
            file_format_from_path(Path::new("bundle.tar.gz")),
            FileFormat::Archive
        );
        assert_eq!(
            file_format_from_path(Path::new("unknown")),
            FileFormat::Unknown
        );
    }

    #[test]
    fn row_intent_prefers_pending_operation_over_plain_mark() {
        let path = PathBuf::from("/tmp/a");
        let mut copy = HashSet::new();
        let mut mv = HashSet::new();
        let mut delete = HashSet::new();

        assert_eq!(
            row_intent(&path, true, &copy, &mv, &delete),
            RowIntent::Marked
        );

        copy.insert(path.clone());
        assert_eq!(
            row_intent(&path, true, &copy, &mv, &delete),
            RowIntent::Copy
        );

        mv.insert(path.clone());
        assert_eq!(
            row_intent(&path, true, &copy, &mv, &delete),
            RowIntent::Move
        );

        delete.insert(path.clone());
        assert_eq!(
            row_intent(&path, true, &copy, &mv, &delete),
            RowIntent::Delete
        );
    }

    #[test]
    fn builds_file_row_from_regular_file() {
        let root = unique_test_dir("regular");
        fs::create_dir_all(&root).expect("create temp dir");
        let file = root.join("main.rs");
        fs::write(&file, "fn main() {}\n").expect("write file");

        let row = FileRow::from_entry(fs_entry(file, "main.rs", false, false));

        assert_eq!(row.kind, RowKind::File(FileFormat::Code));
        assert_eq!(row.detail, "code 42 B");
        assert!(!row.is_dir);

        fs::remove_dir_all(root).expect("remove temp dir");
    }

    #[cfg(unix)]
    #[test]
    fn marks_executable_regular_files_only() {
        let root = unique_test_dir("executable");
        fs::create_dir_all(&root).expect("create temp dir");
        let file = root.join("run.sh");
        fs::write(&file, "#!/bin/sh\n").expect("write file");
        let mut permissions = fs::metadata(&file).expect("metadata").permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(&file, permissions).expect("chmod");

        let row = FileRow::from_entry(fs_entry(file, "run.sh", false, false));

        assert_eq!(row.kind, RowKind::File(FileFormat::Code));
        assert!(row.is_executable);

        fs::remove_dir_all(root).expect("remove temp dir");
    }

    #[cfg(unix)]
    #[test]
    fn detects_symlink_rows_without_following_as_regular_file() {
        use std::os::unix::fs::symlink;

        let root = unique_test_dir("symlink");
        fs::create_dir_all(&root).expect("create temp dir");
        let target = root.join("target.txt");
        let link = root.join("link.txt");
        fs::write(&target, "target").expect("write target");
        symlink(&target, &link).expect("create symlink");

        let row = FileRow::from_entry(fs_entry(link, "link.txt", false, true));

        assert_eq!(row.kind, RowKind::Symlink);
        assert!(row.detail.starts_with("link -> "));

        fs::remove_dir_all(root).expect("remove temp dir");
    }
}
