use std::{fs, path::PathBuf};

use super::system::ClipboardWriter;
use crate::features::file_browser::FileTarget;

pub fn copy_target_path(target: Option<FileTarget>, writer: &mut impl ClipboardWriter) -> String {
    match target {
        Some(target) => copy_text(
            writer,
            target.path.to_string_lossy().to_string(),
            format!("copied path {}", target.name),
        ),
        None => "nothing selected".to_string(),
    }
}

pub fn copy_target_name(target: Option<FileTarget>, writer: &mut impl ClipboardWriter) -> String {
    match target {
        Some(target) => copy_text(
            writer,
            target.name.clone(),
            format!("copied name {}", target.name),
        ),
        None => "nothing selected".to_string(),
    }
}

pub fn copy_file_contents(target: Option<FileTarget>, writer: &mut impl ClipboardWriter) -> String {
    match target {
        Some(target) if target.is_dir => "cannot copy directory contents".to_string(),
        Some(target) => copy_file_text(target, writer),
        None => "nothing selected".to_string(),
    }
}

pub fn copy_files(targets: Vec<FileTarget>, writer: &mut impl ClipboardWriter) -> String {
    match targets.as_slice() {
        [] => "nothing selected".to_string(),
        [target] => copy_file_targets(
            writer,
            target_paths(&targets),
            format!("copied file {} to clipboard", target.name),
        ),
        _ => {
            let count = targets.len();
            copy_file_targets(
                writer,
                target_paths(&targets),
                format!("copied {count} files to clipboard"),
            )
        }
    }
}

fn copy_file_text(target: FileTarget, writer: &mut impl ClipboardWriter) -> String {
    match fs::read_to_string(&target.path) {
        Ok(text) => copy_text(writer, text, format!("copied contents {}", target.name)),
        Err(error) => format!("copy contents failed: {error}"),
    }
}

fn copy_text(writer: &mut impl ClipboardWriter, text: String, success: String) -> String {
    writer.write_text(text);
    success
}

fn copy_file_targets(
    writer: &mut impl ClipboardWriter,
    paths: Vec<PathBuf>,
    success: String,
) -> String {
    match writer.write_files(&paths) {
        Ok(()) => success,
        Err(error) => format!("copy file failed: {error}"),
    }
}

fn target_paths(targets: &[FileTarget]) -> Vec<PathBuf> {
    targets.iter().map(|target| target.path.clone()).collect()
}
