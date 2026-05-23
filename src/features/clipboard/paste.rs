use std::path::PathBuf;

use super::{ClipboardKind, ClipboardState};
use crate::features::file_browser::FileTarget;

// TODO: support pasting into empty directories, and pasting multiple items into the same directory
// TODO: support pasting with name conflicts (e.g. by auto-renaming)
// TODO: show progress for long-running paste operations
// TODO: handle errors more gracefully (e.g. if some items fail to copy/move, still reload the panels and show which ones failed)
// TODO: support undo for paste operations (e.g. by keeping track of what was copied/moved and where, and providing an "undo paste" command that deletes/moves back those items)
// TODO: support copying/moving between different
//  - panels (e.g. by allowing the user to select which panel to paste into, or by automatically pasting into the opposite panel)
//  - directories in the same panel (e.g. by allowing the user to select which directory to paste into, or by automatically pasting into the current directory)
//
// TODO: support copying/moving with different
//  - options (e.g. by allowing the user to choose whether to overwrite existing files, whether to copy file permissions, etc.)
//  - modes (e.g. by allowing the user to choose whether to copy/move recursively, whether to follow symlinks, etc.)
//  - targets (e.g. by allowing the user to choose whether to copy/move the selected items, the marked items, or all items in the current directory)
//
// TODO: Windows support: handle differences in file operations (e.g. move semantics, permissions, etc.) and clipboard handling (e.g. file paths vs. file contents)
//  - UX improvements: show a preview of what will be copied/moved and where, allow the user to cancel the operation while it's in progress, etc.
//  - Performance improvements: optimize file operations for large files/directories, use async I/O where possible, etc.
//  - Code improvements: refactor the file operation logic to be more modular and testable, add error handling and logging, etc.
//  - Future features: support for additional file operations (e.g. compressing/extracting files, creating symbolic links, etc.), support for plugins/extensions that can add new commands and features, etc.
//
// TODO: Wayland clipboard support
// - wl-clipboard or similar tools to interact with the system clipboard)
//
// NOTE: some of these TODOs may require significant changes to the code structure and architecture, and
// may be better suited for a future version of the application rather than being implemented all at once.
// For example, supporting copying/moving between different panels or directories may require a more complex
// clipboard structure that can keep track of the source and destination of the items being copied/moved,
// and may also require changes to the UI to allow the user to select the destination for the paste operation.

pub enum PastePlan {
    Empty,
    Ready {
        kind: ClipboardKind,
        targets: Vec<FileTarget>,
        dst_dir: PathBuf,
        clear_after_paste: bool,
    },
}

pub fn plan_paste(clipboard: &ClipboardState, dst_dir: PathBuf) -> PastePlan {
    match clipboard.op.clone() {
        Some(clipboard) => PastePlan::Ready {
            kind: clipboard.kind,
            targets: clipboard.targets,
            dst_dir,
            clear_after_paste: matches!(clipboard.kind, ClipboardKind::Move),
        },
        None => PastePlan::Empty,
    }
}
