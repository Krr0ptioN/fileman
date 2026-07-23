use std::{
    fs,
    io::{self, Read, Write},
    path::{Path, PathBuf},
};

pub fn copy_recursively(src: &Path, dst_dir: &Path) -> io::Result<()> {
    let destination = destination_path(src, dst_dir)?;
    copy_recursively_to(src, &destination)
}

pub fn copy_recursively_to(src: &Path, destination: &Path) -> io::Result<()> {
    copy_recursively_to_with_progress(src, destination, &mut |_| {}, &|| false)
}

pub fn copy_recursively_to_with_progress(
    src: &Path,
    destination: &Path,
    on_bytes: &mut dyn FnMut(u64),
    cancelled: &dyn Fn() -> bool,
) -> io::Result<()> {
    let src_canonical = fs::canonicalize(src)?;
    let dst_canonical = resolve_destination(destination)?;
    if dst_canonical.starts_with(&src_canonical) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "copy destination {} is inside source {}",
                destination.display(),
                src.display()
            ),
        ));
    }
    copy_entry_to(src, destination, on_bytes, cancelled)
}

pub fn delete_path(path: &Path, is_dir: bool) -> io::Result<()> {
    match is_dir {
        true => fs::remove_dir_all(path),
        false => fs::remove_file(path),
    }
}

fn copy_dir_to(
    src: &Path,
    destination: &Path,
    on_bytes: &mut dyn FnMut(u64),
    cancelled: &dyn Fn() -> bool,
) -> io::Result<()> {
    fs::create_dir_all(destination)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        copy_entry_to(
            &path,
            &destination.join(entry.file_name()),
            on_bytes,
            cancelled,
        )?;
    }
    Ok(())
}

fn copy_file_to(
    src: &Path,
    destination: &Path,
    on_bytes: &mut dyn FnMut(u64),
    cancelled: &dyn Fn() -> bool,
) -> io::Result<()> {
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut source = fs::File::open(src)?;
    let mut target = fs::File::create(destination)?;
    let mut buffer = vec![0u8; 64 * 1024];
    loop {
        if cancelled() {
            return Err(io::Error::new(io::ErrorKind::Interrupted, "copy cancelled"));
        }
        let read = source.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        target.write_all(&buffer[..read])?;
        on_bytes(read as u64);
    }
    Ok(())
}

fn copy_entry_to(
    src: &Path,
    destination: &Path,
    on_bytes: &mut dyn FnMut(u64),
    cancelled: &dyn Fn() -> bool,
) -> io::Result<()> {
    if cancelled() {
        return Err(io::Error::new(io::ErrorKind::Interrupted, "copy cancelled"));
    }
    let file_type = fs::symlink_metadata(src)?.file_type();
    if file_type.is_symlink() {
        copy_symlink_to(src, destination)
    } else if file_type.is_dir() {
        copy_dir_to(src, destination, on_bytes, cancelled)
    } else {
        copy_file_to(src, destination, on_bytes, cancelled)
    }
}

fn destination_path(src: &Path, dst_dir: &Path) -> io::Result<PathBuf> {
    let file_name = src.file_name().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("source path {} has no file name", src.display()),
        )
    })?;
    Ok(dst_dir.join(file_name))
}

fn resolve_destination(path: &Path) -> io::Result<PathBuf> {
    let absolute = match path.is_absolute() {
        true => path.to_path_buf(),
        false => std::env::current_dir()?.join(path),
    };
    let mut normalized = PathBuf::new();
    for component in absolute.components() {
        match component {
            std::path::Component::Prefix(_) | std::path::Component::RootDir => {
                normalized.push(component.as_os_str());
            }
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                normalized.pop();
            }
            std::path::Component::Normal(part) => normalized.push(part),
        }
    }

    let mut cursor = normalized.as_path();
    let mut missing = Vec::new();
    loop {
        match fs::canonicalize(cursor) {
            Ok(existing) => {
                return Ok(missing
                    .into_iter()
                    .rev()
                    .fold(existing, |resolved, part| resolved.join(part)));
            }
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                let part = cursor.file_name().ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("cannot resolve destination {}", path.display()),
                    )
                })?;
                missing.push(part.to_os_string());
                cursor = cursor.parent().ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("cannot resolve destination {}", path.display()),
                    )
                })?;
            }
            Err(error) => return Err(error),
        }
    }
}

#[cfg(unix)]
fn copy_symlink_to(src: &Path, destination: &Path) -> io::Result<()> {
    std::os::unix::fs::symlink(fs::read_link(src)?, destination)
}

#[cfg(windows)]
fn copy_symlink_to(src: &Path, destination: &Path) -> io::Result<()> {
    let target = fs::read_link(src)?;
    if fs::metadata(src)?.is_dir() {
        std::os::windows::fs::symlink_dir(target, destination)
    } else {
        std::os::windows::fs::symlink_file(target, destination)
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        path::PathBuf,
        sync::atomic::{AtomicU64, Ordering},
    };

    use super::{copy_recursively, copy_recursively_to_with_progress};

    static NEXT_TEMP_ID: AtomicU64 = AtomicU64::new(0);

    fn temp_dir(name: &str) -> PathBuf {
        let id = NEXT_TEMP_ID.fetch_add(1, Ordering::Relaxed);
        let path =
            std::env::temp_dir().join(format!("stiff-copy-{name}-{}-{id}", std::process::id()));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).expect("create test directory");
        path
    }

    #[test]
    fn rejects_copying_directory_into_descendant() {
        let root = temp_dir("descendant");
        let src = root.join("source");
        let dst = src.join("child");
        fs::create_dir_all(&dst).expect("create source child");
        fs::write(src.join("file.txt"), "content").expect("write source file");

        let error = copy_recursively(&src, &dst).expect_err("descendant copy should fail");

        assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
        assert!(!dst.join("source").exists());
        fs::remove_dir_all(root).expect("remove test directory");
    }

    #[test]
    fn rejects_source_without_file_name() {
        let root = temp_dir("root");

        let error = copy_recursively(PathBuf::from("/").as_path(), &root)
            .expect_err("filesystem root should not be copyable");

        assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
        fs::remove_dir_all(root).expect("remove test directory");
    }

    #[test]
    fn creates_missing_destination_directory() {
        let root = temp_dir("missing-destination");
        let source = root.join("source.txt");
        let destination = root.join("new").join("nested");
        fs::write(&source, "content").expect("write source file");

        copy_recursively(&source, &destination).expect("copy into missing destination");

        assert_eq!(
            fs::read_to_string(destination.join("source.txt")).expect("read copied file"),
            "content"
        );
        fs::remove_dir_all(root).expect("remove test directory");
    }

    #[test]
    fn progress_copy_stops_when_cancelled() {
        use std::cell::Cell;

        let root = temp_dir("cancel-progress");
        let source = root.join("source.bin");
        let destination = root.join("destination.bin");
        fs::write(&source, vec![7u8; 128 * 1024]).expect("write source file");
        let copied = Cell::new(0u64);

        let error = copy_recursively_to_with_progress(
            &source,
            &destination,
            &mut |bytes| copied.set(copied.get() + bytes),
            &|| copied.get() >= 64 * 1024,
        )
        .expect_err("copy should cancel");

        assert_eq!(error.kind(), std::io::ErrorKind::Interrupted);
        assert_eq!(copied.get(), 64 * 1024);
        fs::remove_dir_all(root).expect("remove test directory");
    }

    #[cfg(unix)]
    #[test]
    fn preserves_directory_symlink_without_following_it() {
        use std::os::unix::fs::symlink;

        let root = temp_dir("symlink");
        let source_root = root.join("source");
        let target = source_root.join("target");
        let dst = root.join("destination");
        fs::create_dir_all(&target).expect("create symlink target");
        fs::create_dir_all(&dst).expect("create destination");
        fs::write(target.join("file.txt"), "content").expect("write target file");
        symlink("target", source_root.join("link")).expect("create directory symlink");

        copy_recursively(&source_root.join("link"), &dst).expect("copy symlink");

        let copied = dst.join("link");
        assert!(
            fs::symlink_metadata(&copied)
                .expect("read copied metadata")
                .file_type()
                .is_symlink()
        );
        assert_eq!(
            fs::read_link(copied).expect("read copied link"),
            PathBuf::from("target")
        );
        fs::remove_dir_all(root).expect("remove test directory");
    }
}
