use std::path::{Path, PathBuf};

use super::{BrowserCommandEffect, BrowserCommandOutcome, state::BrowserPanel};

pub enum PanelNavigation {
    Load {
        path: PathBuf,
        prefer_name: Option<String>,
    },
    OpenWithSystem {
        path: PathBuf,
        name: String,
    },
    Status(String),
}

impl PanelNavigation {
    pub fn into_outcome(self) -> BrowserCommandOutcome {
        match self {
            Self::Load { path, prefer_name } => {
                BrowserCommandOutcome::effect(BrowserCommandEffect::LoadActive {
                    path,
                    prefer_name,
                })
            }
            Self::OpenWithSystem { path, name } => BrowserCommandOutcome::status_effect(
                format!("opening {name}"),
                BrowserCommandEffect::OpenWithSystem(path),
            ),
            Self::Status(status) => BrowserCommandOutcome::status(status),
        }
    }
}

pub fn parent_navigation(panel: &BrowserPanel) -> PanelNavigation {
    match panel.path.parent().map(Path::to_path_buf) {
        Some(path) => PanelNavigation::Load {
            path,
            prefer_name: panel
                .path
                .file_name()
                .and_then(|name| name.to_str())
                .map(str::to_string),
        },
        None => PanelNavigation::Status("already at filesystem root".to_string()),
    }
}

pub fn selected_navigation(panel: &BrowserPanel) -> PanelNavigation {
    let Some(row) = panel.selected_row() else {
        return PanelNavigation::Status("nothing selected".to_string());
    };

    match row.is_dir {
        true => PanelNavigation::Load {
            path: row.path.clone(),
            prefer_name: (row.name == "..")
                .then(|| panel.path.file_name()?.to_str().map(str::to_string))
                .flatten(),
        },
        false => PanelNavigation::OpenWithSystem {
            path: row.path.clone(),
            name: row.name.clone(),
        },
    }
}
