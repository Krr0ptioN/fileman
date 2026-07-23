use std::fs;

use crate::{
    core,
    features::{
        clipboard::{ClipboardKind, PlannedPaste},
        task_queue::{TaskKind, TaskRuntime},
    },
};

use super::state::FileTarget;

pub enum FileOperation {
    Paste {
        kind: ClipboardKind,
        items: Vec<PlannedPaste>,
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
                ref items,
                ..
            } => {
                let op = match *kind {
                    ClipboardKind::Copy => "copying",
                    ClipboardKind::Move => "moving",
                };
                format!("{op} {} item(s)", items.len())
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

    pub fn task_kind(&self) -> TaskKind {
        match *self {
            Self::Paste {
                kind: ClipboardKind::Copy,
                ..
            } => TaskKind::Copy,
            Self::Paste {
                kind: ClipboardKind::Move,
                ..
            } => TaskKind::Move,
            Self::Delete { .. } => TaskKind::Delete,
            Self::Rename { .. } => TaskKind::Rename,
            Self::NewDirectory { .. } => TaskKind::CreateDirectory,
        }
    }

    pub fn item_total(&self) -> u64 {
        match *self {
            Self::Paste { ref items, .. } => items.len() as u64,
            Self::Delete { ref targets } => targets.len() as u64,
            Self::Rename { .. } | Self::NewDirectory { .. } => 1,
        }
    }

    pub fn byte_total(&self) -> u64 {
        match *self {
            Self::Paste { ref items, .. } => items
                .iter()
                .filter_map(|item| fs::metadata(&item.target.path).ok())
                .filter(|metadata| metadata.is_file())
                .map(|metadata| metadata.len())
                .sum(),
            _ => 0,
        }
    }

    pub fn run(self, runtime: &TaskRuntime) -> OperationReport {
        match self {
            Self::Paste { kind, items } => {
                let total = items.len();
                let mut errors = Vec::new();
                let mut completed = 0;
                for item in &items {
                    if runtime.is_cancelled() {
                        break;
                    }
                    let result = paste_target(kind, item, runtime);
                    match result {
                        Ok(()) => completed += 1,
                        Err(error) => errors.push(error.to_string()),
                    }
                    runtime.add_item();
                }
                let op = match kind {
                    ClipboardKind::Copy => "copied",
                    ClipboardKind::Move => "moved",
                };
                OperationReport::new(op, completed, total, runtime.is_cancelled(), errors)
            }
            Self::Delete { targets } => {
                let total = targets.len();
                let mut completed = 0;
                let mut errors = Vec::new();
                for target in &targets {
                    if runtime.is_cancelled() {
                        break;
                    }
                    match delete_target(target) {
                        Ok(()) => completed += 1,
                        Err(error) => errors.push(error.to_string()),
                    }
                    runtime.add_item();
                }
                OperationReport::new("deleted", completed, total, runtime.is_cancelled(), errors)
            }
            Self::Rename { target, new_name } => {
                let dst = target.path.with_file_name(&new_name);
                let result = fs::rename(&target.path, &dst)
                    .map_err(|error| format!("rename {}: {error}", target.path.display()));
                runtime.add_item();
                match result {
                    Ok(()) => OperationReport::new("renamed", 1, 1, false, Vec::new()),
                    Err(error) => OperationReport::new("renamed", 0, 1, false, vec![error]),
                }
            }
            Self::NewDirectory { path } => {
                let result = fs::create_dir(&path)
                    .map_err(|error| format!("mkdir {}: {error}", path.display()));
                runtime.add_item();
                match result {
                    Ok(()) => OperationReport::new("created", 1, 1, false, Vec::new()),
                    Err(error) => OperationReport::new("created", 0, 1, false, vec![error]),
                }
            }
        }
    }
}

pub struct OperationReport {
    pub status: String,
    pub completed: usize,
    pub total: usize,
    pub cancelled: bool,
    pub errors: Vec<String>,
}

impl OperationReport {
    fn new(
        verb: &str,
        completed: usize,
        total: usize,
        cancelled: bool,
        errors: Vec<String>,
    ) -> Self {
        let status = if cancelled {
            format!("{verb} {completed}/{total} item(s), cancelled")
        } else if errors.is_empty() {
            format!("{verb} {completed} item(s)")
        } else {
            format!(
                "{verb} {completed}/{total} item(s), {} failed",
                errors.len()
            )
        };
        Self {
            status,
            completed,
            total,
            cancelled,
            errors,
        }
    }
}

fn paste_target(
    kind: ClipboardKind,
    item: &PlannedPaste,
    runtime: &TaskRuntime,
) -> anyhow::Result<()> {
    if item.target.path == item.destination {
        anyhow::bail!(
            "source and destination are the same for {}",
            item.target.name
        );
    }
    if !item.overwrite && item.destination.exists() {
        anyhow::bail!(
            "destination appeared after planning: {}",
            item.destination.display()
        );
    }
    if item.overwrite && item.destination.exists() {
        let is_dir = item.destination.is_dir();
        core::delete_path(&item.destination, is_dir)?;
    }
    match kind {
        ClipboardKind::Copy => core::copy_recursively_to_with_progress(
            &item.target.path,
            &item.destination,
            &mut |bytes| runtime.add_bytes(bytes),
            &|| runtime.is_cancelled(),
        )
        .map_err(|error| anyhow::anyhow!("copy {}: {error}", item.target.path.display())),
        ClipboardKind::Move => {
            let bytes = fs::metadata(&item.target.path)
                .ok()
                .filter(|metadata| metadata.is_file())
                .map(|metadata| metadata.len())
                .unwrap_or(0);
            fs::rename(&item.target.path, &item.destination)
                .map(|()| runtime.add_bytes(bytes))
                .or_else(|rename_error| {
                    core::copy_recursively_to_with_progress(
                        &item.target.path,
                        &item.destination,
                        &mut |bytes| runtime.add_bytes(bytes),
                        &|| runtime.is_cancelled(),
                    )
                    .map_err(|copy_error| {
                        anyhow::anyhow!(
                            "move {}: rename failed ({rename_error}); copy failed ({copy_error})",
                            item.target.path.display()
                        )
                    })?;
                    delete_target(&item.target)
                })
        }
    }
}

fn delete_target(target: &FileTarget) -> anyhow::Result<()> {
    core::delete_path(&target.path, target.is_dir)
        .map_err(|error| anyhow::anyhow!("delete {}: {error}", target.path.display()))
}
