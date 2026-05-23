use std::path::{Path, PathBuf};

use super::state::BrowserPanel;

pub enum PanelNavigation {
    Load {
        path: PathBuf,
        prefer_name: Option<String>,
    },
    Status(String),
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
        false => PanelNavigation::Status(format!("selected {}", row.name)),
        true => PanelNavigation::Load {
            path: row.path.clone(),
            prefer_name: (row.name == "..")
                .then(|| panel.path.file_name()?.to_str().map(str::to_string))
                .flatten(),
        },
    }
}
