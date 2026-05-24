use std::{fs, path::Path};

use crate::{core, features::clipboard::ClipboardKind};

use super::state::FileTarget;

pub enum FileOperation {
    Paste {
        kind: ClipboardKind,
        targets: Vec<FileTarget>,
        dst_dir: std::path::PathBuf,
    },
    Delete {
        targets: Vec<FileTarget>,
    },
    Rename {
        target: FileTarget,
        new_name: String,
    },
    NewDirectory {
        path: std::path::PathBuf,
    },
}

impl FileOperation {
    pub fn pending_status(&self) -> String {
        match *self {
            Self::Paste {
                ref kind,
                ref targets,
                ..
            } => {
                let op = match *kind {
                    ClipboardKind::Copy => "copying",
                    ClipboardKind::Move => "moving",
                };
                format!("{op} {} item(s)", targets.len())
            }
            Self::Delete { ref targets } => format!("deleting {} item(s)", targets.len()),
            Self::Rename {
                ref target,
                ref new_name,
            } => {
                format!("renaming {} to {new_name}", target.name)
            }
            Self::NewDirectory { ref path } => {
                format!("creating directory {}", path.display())
            }
        }
    }

    pub fn run(self) -> anyhow::Result<String> {
        match self {
            Self::Paste {
                kind,
                targets,
                dst_dir,
            } => {
                for target in &targets {
                    match kind {
                        ClipboardKind::Copy => copy_target(target, &dst_dir)?,
                        ClipboardKind::Move => move_target(target, &dst_dir)?,
                    }
                }
                let op = match kind {
                    ClipboardKind::Copy => "copied",
                    ClipboardKind::Move => "moved",
                };
                Ok(format!("{op} {} item(s)", targets.len()))
            }
            Self::Delete { targets } => {
                for target in &targets {
                    delete_target(target)?;
                }
                Ok(format!("deleted {} item(s)", targets.len()))
            }
            Self::Rename { target, new_name } => {
                let dst = target.path.with_file_name(&new_name);
                fs::rename(&target.path, &dst)?;
                Ok(format!("renamed {} to {new_name}", target.name))
            }
            Self::NewDirectory { path } => {
                fs::create_dir(&path)?;
                Ok(format!("created {}", path.display()))
            }
        }
    }
}

fn copy_target(target: &FileTarget, dst_dir: &Path) -> anyhow::Result<()> {
    if target.path.parent() == Some(dst_dir) {
        anyhow::bail!(
            "copy destination is the source directory for {}",
            target.name
        );
    }
    core::copy_recursively(&target.path, dst_dir)
        .map_err(|error| anyhow::anyhow!("copy {}: {error}", target.path.display()))
}

fn move_target(target: &FileTarget, dst_dir: &Path) -> anyhow::Result<()> {
    if target.path.parent() == Some(dst_dir) {
        anyhow::bail!(
            "move destination is the source directory for {}",
            target.name
        );
    }
    let dst = dst_dir.join(&target.name);
    fs::rename(&target.path, &dst).or_else(|rename_error| {
        core::copy_recursively(&target.path, dst_dir).map_err(|copy_error| {
            anyhow::anyhow!(
                "move {}: rename failed ({rename_error}); copy failed ({copy_error})",
                target.path.display()
            )
        })?;
        delete_target(target)
    })
}

fn delete_target(target: &FileTarget) -> anyhow::Result<()> {
    core::delete_path(&target.path, target.is_dir)
        .map_err(|error| anyhow::anyhow!("delete {}: {error}", target.path.display()))
}
