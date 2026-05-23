mod copy;
mod effect;
mod paste;
mod selection;
mod system;
mod targets;
mod types;

pub use effect::{ClipboardEffect, ClipboardEffectOutcome, ClipboardPaste, apply_clipboard_effect};
pub use targets::target_paths;
pub use types::{ClipboardKind, ClipboardState};
