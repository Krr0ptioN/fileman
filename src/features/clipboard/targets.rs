use std::{collections::HashSet, path::PathBuf, sync::Arc};

use gpui::App;

use super::{ClipboardKind, ClipboardState};

pub fn target_paths(cx: &App, kind: ClipboardKind) -> Arc<HashSet<PathBuf>> {
    let clipboard = cx.global::<ClipboardState>();
    match clipboard.op.as_ref() {
        Some(clipboard) if clipboard.kind == kind => clipboard.paths.clone(),
        _ => Arc::new(HashSet::new()),
    }
}
