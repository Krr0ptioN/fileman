use super::{
    FileRow,
    state::{BrowserPanel, InputMode, PanelSide, PendingConfirm},
};

pub struct BrowserCommandState<'a> {
    pub primary: &'a mut BrowserPanel,
    pub secondary: &'a mut BrowserPanel,
    pub active: &'a mut PanelSide,
    pub input_mode: &'a mut InputMode,
    pub pending_confirm: &'a mut Option<PendingConfirm>,
}

impl<'a> BrowserCommandState<'a> {
    pub fn active_panel(&self) -> &BrowserPanel {
        match *self.active {
            PanelSide::Left => self.primary,
            PanelSide::Right => self.secondary,
        }
    }

    pub fn active_panel_mut(&mut self) -> &mut BrowserPanel {
        match *self.active {
            PanelSide::Left => self.primary,
            PanelSide::Right => self.secondary,
        }
    }

    pub fn has_active_rows(&self) -> bool {
        !self.active_panel().rows.is_empty()
    }

    pub fn selected_name(&self) -> String {
        self.active_panel().selected_name().to_string()
    }

    pub fn switch_panel(&mut self) -> String {
        *self.active = self.active.other();
        self.active_panel().reveal_selected();
        format!("active {}", self.active.label())
    }

    pub fn clear_marks(&mut self) {
        self.active_panel_mut().marked.clear();
    }

    pub fn apply_loaded(
        panel: &mut BrowserPanel,
        path: std::path::PathBuf,
        prefer_name: Option<String>,
        generation: u64,
        result: anyhow::Result<Vec<crate::core::DirEntry>>,
    ) -> Option<String> {
        if panel.load_generation != generation {
            return None;
        }

        panel.loading = false;
        let status = match result {
            Ok(entries) => {
                panel.path = path;
                panel.rows = entries.into_iter().map(FileRow::from_entry).collect();
                panel
                    .marked
                    .retain(|path| panel.rows.iter().any(|row| &row.path == path));
                panel.selected_index = prefer_name
                    .and_then(|name| panel.rows.iter().position(|row| row.name == name))
                    .unwrap_or_else(|| usize::from(panel.rows.len() > 1).min(panel.rows.len()));
                panel.error = None;
                let selected = panel.selected_name().to_string();
                format!("{} rows, selected {selected}", panel.rows.len())
            }
            Err(error) => {
                panel.rows.clear();
                panel.selected_index = 0;
                panel.error = Some(error.to_string());
                format!("cannot load {}", panel.path.display())
            }
        };
        panel.reveal_selected();
        Some(status)
    }
}
