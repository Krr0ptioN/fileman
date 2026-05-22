use std::{fs, io, path::Path};

pub fn copy_recursively(src: &Path, dst_dir: &Path) -> io::Result<()> {
    match src.is_dir() {
        true => copy_dir(src, dst_dir),
        false => copy_file(src, dst_dir),
    }
}

pub fn delete_path(path: &Path, is_dir: bool) -> io::Result<()> {
    match is_dir {
        true => fs::remove_dir_all(path),
        false => fs::remove_file(path),
    }
}

fn copy_dir(src: &Path, dst_dir: &Path) -> io::Result<()> {
    let dest = dst_dir.join(src.file_name().unwrap());
    fs::create_dir_all(&dest)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        match path.is_dir() {
            true => copy_recursively(&path, &dest)?,
            false => {
                fs::copy(&path, dest.join(entry.file_name()))?;
            }
        }
    }
    Ok(())
}

fn copy_file(src: &Path, dst_dir: &Path) -> io::Result<()> {
    let dest = dst_dir.join(src.file_name().unwrap());
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::copy(src, dest)?;
    Ok(())
}
