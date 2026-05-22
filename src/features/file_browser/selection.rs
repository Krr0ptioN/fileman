use std::collections::HashSet;

use super::state::FileTarget;

#[derive(Debug, PartialEq, Eq)]
pub enum ToggleResult {
    Empty,
    Added(usize),
    Removed(usize),
    Cleared,
}

pub fn toggle_targets(selected: &mut Vec<FileTarget>, targets: &[FileTarget]) -> ToggleResult {
    if targets.is_empty() {
        return ToggleResult::Empty;
    }

    let existing: HashSet<_> = selected.iter().map(|t| t.path.clone()).collect();
    let to_add: Vec<_> = targets
        .iter()
        .filter(|t| !existing.contains(&t.path))
        .cloned()
        .collect();

    if to_add.is_empty() {
        let to_remove: HashSet<_> = targets.iter().map(|t| &t.path).collect();
        let initial_len = selected.len();
        selected.retain(|t| !to_remove.contains(&t.path));
        let removed = initial_len - selected.len();
        if selected.is_empty() {
            ToggleResult::Cleared
        } else {
            ToggleResult::Removed(removed)
        }
    } else {
        let added = to_add.len();
        selected.extend(to_add);
        ToggleResult::Added(added)
    }
}

pub fn selection_status(label: &str, result: ToggleResult) -> String {
    match result {
        ToggleResult::Empty => "nothing selected".to_string(),
        ToggleResult::Added(count) => format!("{label} {count} item(s)"),
        ToggleResult::Removed(count) => format!("{label} {count} item(s)"),
        ToggleResult::Cleared => format!("{label} cleared"),
    }
}

pub fn delete_status(result: ToggleResult) -> String {
    match result {
        ToggleResult::Empty => "nothing selected".to_string(),
        ToggleResult::Added(count) | ToggleResult::Removed(count) => {
            format!("delete {count} item(s)? y/enter to confirm")
        }
        ToggleResult::Cleared => "delete cleared".to_string(),
    }
}
