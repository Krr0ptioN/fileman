use std::{collections::HashSet, path::Path};

use super::state::FileTarget;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ToggleResult {
    Empty,
    Added(usize),
    Removed(usize),
    Cleared,
}

pub(crate) fn toggle_targets(
    selected: &mut Vec<FileTarget>,
    targets: &[FileTarget],
) -> ToggleResult {
    if targets.is_empty() {
        return ToggleResult::Empty;
    }

    let target_paths = target_paths(targets);
    if targets
        .iter()
        .all(|target| selected.iter().any(|item| item.path == target.path))
    {
        selected.retain(|item| !target_paths.contains(item.path.as_path()));
        return if selected.is_empty() {
            ToggleResult::Cleared
        } else {
            ToggleResult::Removed(selected.len())
        };
    }

    for target in targets {
        if !selected.iter().any(|item| item.path == target.path) {
            selected.push(target.clone());
        }
    }
    ToggleResult::Added(selected.len())
}

fn target_paths(targets: &[FileTarget]) -> HashSet<&Path> {
    targets.iter().map(|target| target.path.as_path()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn target(path: &str) -> FileTarget {
        FileTarget {
            path: PathBuf::from(path),
            name: path.to_string(),
            is_dir: false,
        }
    }

    #[test]
    fn toggle_targets_adds_missing_targets() {
        let mut selected = vec![target("/tmp/a")];
        let result = toggle_targets(&mut selected, &[target("/tmp/b")]);

        assert_eq!(result, ToggleResult::Added(2));
        assert_eq!(selected.len(), 2);
    }

    #[test]
    fn toggle_targets_removes_targets_that_are_already_selected() {
        let mut selected = vec![target("/tmp/a"), target("/tmp/b")];
        let result = toggle_targets(&mut selected, &[target("/tmp/a")]);

        assert_eq!(result, ToggleResult::Removed(1));
        assert_eq!(selected, vec![target("/tmp/b")]);
    }

    #[test]
    fn toggle_targets_clears_when_last_target_is_removed() {
        let mut selected = vec![target("/tmp/a")];
        let result = toggle_targets(&mut selected, &[target("/tmp/a")]);

        assert_eq!(result, ToggleResult::Cleared);
        assert!(selected.is_empty());
    }
}
