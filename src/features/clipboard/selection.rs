use super::{ClipboardKind, ClipboardState, types::ClipboardOp};
use crate::features::file_browser::{
    FileTarget,
    selection::{selection_status, toggle_targets},
};

pub fn prepare_clipboard(
    clipboard: &mut ClipboardState,
    kind: ClipboardKind,
    targets: Vec<FileTarget>,
) -> String {
    if targets.is_empty() {
        return "nothing selected".to_string();
    }

    let label = kind.label();
    let mut clear_clipboard = false;
    let status = match &mut clipboard.op {
        Some(clipboard) if clipboard.kind == kind => {
            let changed = toggle_targets(&mut clipboard.targets, &targets);
            clear_clipboard = clipboard.targets.is_empty();
            selection_status(label, changed)
        }
        _ => {
            let len = targets.len();
            clipboard.op = Some(ClipboardOp { kind, targets });
            format!("{label} {len} item(s)")
        }
    };

    if clear_clipboard {
        clipboard.clear();
    }
    status
}
