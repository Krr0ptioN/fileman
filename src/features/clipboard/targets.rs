use std::{
    collections::HashSet,
    path::PathBuf,
    sync::{Arc, OnceLock},
};

use gpui::App;

use super::{ClipboardKind, ClipboardState};

static EMPTY_TARGET_PATHS: OnceLock<Arc<HashSet<PathBuf>>> = OnceLock::new();

pub fn target_paths(cx: &App, kind: ClipboardKind) -> Arc<HashSet<PathBuf>> {
    let clipboard = cx.global::<ClipboardState>();
    match clipboard.op.as_ref() {
        Some(clipboard) if clipboard.kind == kind => clipboard.paths.clone(),
        _ => Arc::clone(EMPTY_TARGET_PATHS.get_or_init(|| Arc::new(HashSet::new()))),
    }
}
