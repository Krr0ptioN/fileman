use std::{fs, path::Path};

use crate::core;

use super::state::{ClipboardKind, FileTarget};

pub(crate) enum FileOperation {
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
}

impl FileOperation {
    pub(crate) fn pending_status(&self) -> String {
        match self {
            Self::Paste { kind, targets, .. } => {
                let op = match kind {
                    ClipboardKind::Copy => "copying",
                    ClipboardKind::Move => "moving",
                };
                format!("{op} {} item(s)", targets.len())
            }
            Self::Delete { targets } => format!("deleting {} item(s)", targets.len()),
            Self::Rename { target, new_name } => {
                format!("renaming {} to {new_name}", target.name)
            }
        }
    }

    pub(crate) fn run(self) -> anyhow::Result<String> {
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
    let result = if target.is_dir {
        fs::remove_dir_all(&target.path)
    } else {
        fs::remove_file(&target.path)
    };
    result.map_err(|error| anyhow::anyhow!("delete {}: {error}", target.path.display()))
}
