use std::{collections::HashSet, path::PathBuf};

use gpui::App;

use super::{ClipboardKind, ClipboardState};

pub fn target_paths(cx: &App, kind: ClipboardKind) -> HashSet<PathBuf> {
    let clipboard = cx.global::<ClipboardState>();
    match clipboard.op.as_ref() {
        Some(clipboard) if clipboard.kind == kind => clipboard
            .targets
            .iter()
            .map(|target| target.path.clone())
            .collect(),
        _ => HashSet::new(),
    }
}
