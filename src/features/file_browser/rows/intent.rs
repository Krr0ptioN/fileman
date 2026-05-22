use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RowIntent {
    None,
    Marked,
    Copy,
    Move,
    Delete,
}

pub fn row_intent(
    path: &Path,
    marked: bool,
    copy_targets: &HashSet<PathBuf>,
    move_targets: &HashSet<PathBuf>,
    delete_targets: &HashSet<PathBuf>,
) -> RowIntent {
    match (
        delete_targets.contains(path),
        move_targets.contains(path),
        copy_targets.contains(path),
        marked,
    ) {
        (true, _, _, _) => RowIntent::Delete,
        (_, true, _, _) => RowIntent::Move,
        (_, _, true, _) => RowIntent::Copy,
        (_, _, _, true) => RowIntent::Marked,
        _ => RowIntent::None,
    }
}
