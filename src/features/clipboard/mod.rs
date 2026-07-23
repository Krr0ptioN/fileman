mod copy;
mod effect;
mod paste;
mod selection;
mod system;
mod targets;
mod types;

pub use effect::{ClipboardEffect, ClipboardEffectOutcome, apply_clipboard_effect};
pub use paste::{
    PasteBatch, PasteConflict, PasteConflictDecision, PasteConflictPolicy, PastePlan, PendingPaste,
    PlannedPaste, resolve_paste_conflict,
};
pub use targets::target_paths;
pub use types::{ClipboardKind, ClipboardState};
