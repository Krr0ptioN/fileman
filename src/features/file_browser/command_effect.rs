use std::path::PathBuf;

use crate::features::clipboard::ClipboardKind;

use super::state::FileTarget;

pub enum BrowserCommandEffect {
    None,
    LoadActive {
        path: PathBuf,
        prefer_name: Option<String>,
    },
    PrepareClipboard {
        kind: ClipboardKind,
        targets: Vec<FileTarget>,
    },
    CopyPath(Option<FileTarget>),
    CopyName(Option<FileTarget>),
    CopyFileContents(Option<FileTarget>),
    PasteInto(PathBuf),
    TogglePaneMode,
    OpenHelp,
    ReloadActive,
}

pub struct BrowserCommandOutcome {
    pub status: Option<String>,
    pub effect: BrowserCommandEffect,
}

impl BrowserCommandOutcome {
    pub fn effect(effect: BrowserCommandEffect) -> Self {
        Self {
            status: None,
            effect,
        }
    }

    pub fn status(status: impl Into<String>) -> Self {
        Self {
            status: Some(status.into()),
            effect: BrowserCommandEffect::None,
        }
    }

    pub fn status_effect(status: impl Into<String>, effect: BrowserCommandEffect) -> Self {
        Self {
            status: Some(status.into()),
            effect,
        }
    }
}
