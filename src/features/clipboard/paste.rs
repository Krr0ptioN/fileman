use std::{collections, path};

use super::{ClipboardKind, ClipboardState};
use crate::features::file_browser::FileTarget;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PasteConflictPolicy {
    Skip,
    Overwrite,
    Rename,
    Cancel,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PasteConflictDecision {
    pub policy: PasteConflictPolicy,
    pub apply_to_all: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlannedPaste {
    pub target: FileTarget,
    pub destination: path::PathBuf,
    pub disposition: PasteDisposition,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PasteDisposition {
    Create,
    Overwrite,
}

pub struct PasteBatch {
    pub kind: ClipboardKind,
    pub items: Vec<PlannedPaste>,
    pub clear_after_paste: bool,
}

pub struct PasteConflict {
    pub source_name: String,
    pub destination: path::PathBuf,
}

pub struct PendingPaste {
    kind: ClipboardKind,
    dst_dir: path::PathBuf,
    remaining: collections::VecDeque<FileTarget>,
    planned: Vec<PlannedPaste>,
    destinations: collections::HashSet<path::PathBuf>,
    apply_policy: Option<PasteConflictPolicy>,
}

pub enum PastePlan {
    Empty,
    Ready(PasteBatch),
    Conflict {
        conflict: PasteConflict,
        pending: PendingPaste,
    },
    Cancelled,
}

pub fn plan_paste(clipboard: &ClipboardState, dst_dir: path::PathBuf) -> PastePlan {
    let default_conflict_policy = clipboard.default_conflict_policy;
    match clipboard.op.clone() {
        Some(clipboard) => advance(PendingPaste {
            kind: clipboard.kind,
            dst_dir,
            remaining: clipboard.targets.into(),
            planned: Vec::new(),
            destinations: collections::HashSet::new(),
            apply_policy: default_conflict_policy,
        }),
        None => PastePlan::Empty,
    }
}

pub fn resolve_paste_conflict(
    mut pending: PendingPaste,
    decision: PasteConflictDecision,
) -> PastePlan {
    if decision.policy == PasteConflictPolicy::Cancel {
        return PastePlan::Cancelled;
    }
    if decision.apply_to_all {
        pending.apply_policy = Some(decision.policy);
    }
    apply_policy(&mut pending, decision.policy);
    advance(pending)
}

fn advance(mut pending: PendingPaste) -> PastePlan {
    while let Some(target) = pending.remaining.front() {
        let destination = pending.dst_dir.join(&target.name);
        let collides = path_occupied(&destination) || pending.destinations.contains(&destination);
        if collides {
            if let Some(policy) = pending.apply_policy {
                if policy == PasteConflictPolicy::Cancel {
                    return PastePlan::Cancelled;
                }
                apply_policy(&mut pending, policy);
                continue;
            }
            return PastePlan::Conflict {
                conflict: PasteConflict {
                    source_name: target.name.clone(),
                    destination,
                },
                pending,
            };
        }
        push_planned(&mut pending, destination, PasteDisposition::Create);
    }

    PastePlan::Ready(PasteBatch {
        kind: pending.kind,
        items: pending.planned,
        clear_after_paste: pending.kind == ClipboardKind::Move,
    })
}

fn apply_policy(pending: &mut PendingPaste, policy: PasteConflictPolicy) {
    let Some(target) = pending.remaining.front() else {
        return;
    };
    let destination = pending.dst_dir.join(&target.name);
    match policy {
        PasteConflictPolicy::Skip => {
            pending.remaining.pop_front();
        }
        PasteConflictPolicy::Overwrite => {
            push_planned(pending, destination, PasteDisposition::Overwrite)
        }
        PasteConflictPolicy::Rename => {
            let renamed = available_destination(&destination, &pending.destinations);
            push_planned(pending, renamed, PasteDisposition::Create);
        }
        PasteConflictPolicy::Cancel => {}
    }
}

fn push_planned(
    pending: &mut PendingPaste,
    destination: path::PathBuf,
    disposition: PasteDisposition,
) {
    if let Some(target) = pending.remaining.pop_front() {
        pending.destinations.insert(destination.clone());
        pending.planned.push(PlannedPaste {
            target,
            destination,
            disposition,
        });
    }
}

fn available_destination(
    destination: &path::Path,
    reserved: &collections::HashSet<path::PathBuf>,
) -> path::PathBuf {
    let parent = destination.parent().unwrap_or_else(|| path::Path::new(""));
    let stem = destination
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("copy");
    let extension = destination
        .extension()
        .and_then(|extension| extension.to_str());
    for suffix in 1.. {
        let name = match extension {
            Some(extension) => format!("{stem} ({suffix}).{extension}"),
            None => format!("{stem} ({suffix})"),
        };
        let candidate = parent.join(name);
        if !path_occupied(&candidate) && !reserved.contains(&candidate) {
            return candidate;
        }
    }
    unreachable!()
}

fn path_occupied(path: &path::Path) -> bool {
    std::fs::symlink_metadata(path).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::clipboard::types::ClipboardOp;
    use std::{fs, sync::Arc};

    fn clipboard(targets: Vec<FileTarget>) -> ClipboardState {
        ClipboardState {
            op: Some(ClipboardOp {
                kind: ClipboardKind::Copy,
                paths: Arc::new(targets.iter().map(|target| target.path.clone()).collect()),
                targets,
            }),
            default_conflict_policy: None,
        }
    }

    fn target(root: &path::Path, name: &str) -> FileTarget {
        FileTarget {
            path: root.join("source").join(name),
            name: name.to_string(),
            is_dir: false,
        }
    }

    #[test]
    fn skip_leaves_collision_out_and_continues_batch() {
        let root = std::env::temp_dir().join(format!("stiff-paste-skip-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("destination")).unwrap();
        fs::write(root.join("destination/a.txt"), "existing").unwrap();
        let clipboard = clipboard(vec![target(&root, "a.txt"), target(&root, "b.txt")]);

        let PastePlan::Conflict { pending, .. } = plan_paste(&clipboard, root.join("destination"))
        else {
            panic!("expected conflict");
        };
        let resolved = resolve_paste_conflict(
            pending,
            PasteConflictDecision {
                policy: PasteConflictPolicy::Skip,
                apply_to_all: false,
            },
        );

        let PastePlan::Ready(batch) = resolved else {
            panic!("expected ready batch");
        };
        assert_eq!(batch.items.len(), 1);
        assert_eq!(batch.items[0].target.name, "b.txt");
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn cancel_aborts_remaining_plan() {
        let root = std::env::temp_dir().join(format!("stiff-paste-cancel-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("destination")).unwrap();
        fs::write(root.join("destination/a.txt"), "existing").unwrap();
        let clipboard = clipboard(vec![target(&root, "a.txt"), target(&root, "b.txt")]);
        let PastePlan::Conflict { pending, .. } = plan_paste(&clipboard, root.join("destination"))
        else {
            panic!("expected conflict");
        };

        assert!(matches!(
            resolve_paste_conflict(
                pending,
                PasteConflictDecision {
                    policy: PasteConflictPolicy::Cancel,
                    apply_to_all: false,
                }
            ),
            PastePlan::Cancelled
        ));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn rename_chooses_available_suffix() {
        let root = std::env::temp_dir().join(format!("stiff-paste-rename-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("destination")).unwrap();
        fs::write(root.join("destination/a.txt"), "existing").unwrap();
        fs::write(root.join("destination/a (1).txt"), "existing").unwrap();
        let clipboard = clipboard(vec![target(&root, "a.txt")]);
        let PastePlan::Conflict { pending, .. } = plan_paste(&clipboard, root.join("destination"))
        else {
            panic!("expected conflict");
        };

        let PastePlan::Ready(batch) = resolve_paste_conflict(
            pending,
            PasteConflictDecision {
                policy: PasteConflictPolicy::Rename,
                apply_to_all: false,
            },
        ) else {
            panic!("expected ready batch");
        };
        assert_eq!(
            batch.items[0].destination,
            root.join("destination/a (2).txt")
        );
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn apply_to_all_resolves_later_collisions_without_prompting() {
        let root =
            std::env::temp_dir().join(format!("stiff-paste-apply-all-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("destination")).unwrap();
        fs::write(root.join("destination/a.txt"), "existing").unwrap();
        fs::write(root.join("destination/b.txt"), "existing").unwrap();
        let clipboard = clipboard(vec![target(&root, "a.txt"), target(&root, "b.txt")]);
        let PastePlan::Conflict { pending, .. } = plan_paste(&clipboard, root.join("destination"))
        else {
            panic!("expected conflict");
        };

        let PastePlan::Ready(batch) = resolve_paste_conflict(
            pending,
            PasteConflictDecision {
                policy: PasteConflictPolicy::Skip,
                apply_to_all: true,
            },
        ) else {
            panic!("expected ready batch");
        };
        assert!(batch.items.is_empty());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn configured_default_policy_resolves_without_prompting() {
        let root = std::env::temp_dir().join(format!("stiff-paste-default-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("destination")).unwrap();
        fs::write(root.join("destination/a.txt"), "existing").unwrap();
        let mut clipboard = clipboard(vec![target(&root, "a.txt")]);
        clipboard.default_conflict_policy = Some(PasteConflictPolicy::Skip);

        let PastePlan::Ready(batch) = plan_paste(&clipboard, root.join("destination")) else {
            panic!("configured policy should avoid prompt");
        };

        assert!(batch.items.is_empty());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn configured_cancel_policy_aborts_on_first_conflict() {
        let root =
            std::env::temp_dir().join(format!("stiff-paste-default-cancel-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("destination")).unwrap();
        fs::write(root.join("destination/a.txt"), "existing").unwrap();
        let mut clipboard = clipboard(vec![target(&root, "a.txt")]);
        clipboard.default_conflict_policy = Some(PasteConflictPolicy::Cancel);

        assert!(matches!(
            plan_paste(&clipboard, root.join("destination")),
            PastePlan::Cancelled
        ));
        fs::remove_dir_all(root).unwrap();
    }
}
