use crate::features::clipboard::ClipboardEffect;

use super::ops::FileOperation;

pub enum BrowserCommandEffect {
    None,
    LoadActive {
        path: std::path::PathBuf,
        prefer_name: Option<String>,
    },
    OpenWithSystem(std::path::PathBuf),
    Clipboard(ClipboardEffect),
    RunOperation(FileOperation),
    TogglePaneMode,
    OpenHelp,
    ReloadActive,
}

pub struct BrowserCommandOutcome {
    pub status: Option<String>,
    pub effect: BrowserCommandEffect,
    pub reveal_active: bool,
}

impl BrowserCommandOutcome {
    pub fn effect(effect: BrowserCommandEffect) -> Self {
        Self {
            status: None,
            effect,
            reveal_active: false,
        }
    }

    pub fn status(status: impl Into<String>) -> Self {
        Self {
            status: Some(status.into()),
            effect: BrowserCommandEffect::None,
            reveal_active: false,
        }
    }

    pub fn status_effect(status: impl Into<String>, effect: BrowserCommandEffect) -> Self {
        Self {
            status: Some(status.into()),
            effect,
            reveal_active: false,
        }
    }

    pub fn reveal_active(mut self) -> Self {
        self.reveal_active = true;
        self
    }
}
