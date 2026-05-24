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
        format!("active {}", self.active.label())
    }

    pub fn clear_marks(&mut self) {
        self.active_panel_mut().marked.clear();
    }

    pub fn start_loading(panel: &mut BrowserPanel, path: std::path::PathBuf) -> u64 {
        panel.load_generation = panel.load_generation.wrapping_add(1);
        panel.loading = true;
        panel.error = None;
        panel.path = path;
        panel.rows.clear();
        panel.selected_index = 0;
        panel.load_generation
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
                panel.rows = entries
                    .into_iter()
                    .filter(|entry| is_visible_entry(&entry.name, panel.show_hidden))
                    .map(FileRow::from_entry)
                    .collect();
                panel
                    .marked
                    .retain(|path| panel.rows.iter().any(|row| &row.path == path));
                panel.selected_index = prefer_name
                    .and_then(|name| panel.rows.iter().position(|row| row.name == name))
                    .unwrap_or(0);
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
        Some(status)
    }
}

fn is_visible_entry(name: &str, show_hidden: bool) -> bool {
    !matches!(name, "." | "..") && (show_hidden || !name.starts_with('.'))
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, path::PathBuf};

    use super::*;
    use crate::{
        core::{DirEntry, EntryLocation},
        features::file_browser::rows::{FileFormat, FileRow, RowKind},
    };

    fn row(name: &str) -> FileRow {
        FileRow {
            kind: RowKind::File(FileFormat::Text),
            name: name.to_string(),
            detail: String::new(),
            path: PathBuf::from(format!("/tmp/{name}")),
            is_dir: false,
            is_executable: false,
        }
    }

    fn entry(name: &str, is_dir: bool) -> DirEntry {
        DirEntry {
            name: name.to_string(),
            is_dir,
            is_symlink: false,
            is_executable: false,
            link_target: None,
            location: EntryLocation::Fs(PathBuf::from(format!("/next/{name}"))),
            size: None,
            modified: None,
        }
    }

    fn panel() -> BrowserPanel {
        BrowserPanel {
            side: PanelSide::Left,
            title: "Primary",
            path: PathBuf::from("/tmp"),
            selected_index: 1,
            rows: vec![row("old-a"), row("old-b")],
            show_hidden: false,
            marked: HashSet::from([PathBuf::from("/tmp/old-a")]),
            loading: false,
            error: Some("previous error".to_string()),
            load_generation: 7,
            scroll_handle: Default::default(),
        }
    }

    #[test]
    fn start_loading_resets_browser_panel_load_state() {
        let mut panel = panel();

        let generation = BrowserCommandState::start_loading(&mut panel, PathBuf::from("/next"));

        assert_eq!(generation, 8);
        assert_eq!(panel.load_generation, 8);
        assert!(panel.loading);
        assert_eq!(panel.path, PathBuf::from("/next"));
        assert_eq!(panel.selected_index, 0);
        assert!(panel.rows.is_empty());
        assert!(panel.error.is_none());
    }

    #[test]
    fn apply_loaded_ignores_stale_generation() {
        let mut panel = panel();
        let before_rows = panel.rows.len();
        let stale_generation = panel.load_generation + 1;

        let status = BrowserCommandState::apply_loaded(
            &mut panel,
            PathBuf::from("/next"),
            None,
            stale_generation,
            Ok(vec![entry("fresh", false)]),
        );

        assert!(status.is_none());
        assert_eq!(panel.path, PathBuf::from("/tmp"));
        assert_eq!(panel.rows.len(), before_rows);
        assert_eq!(panel.selected_index, 1);
    }

    #[test]
    fn apply_loaded_selects_preferred_name_and_retains_valid_marks() {
        let mut panel = panel();
        panel.marked = HashSet::from([
            PathBuf::from("/next/keep.txt"),
            PathBuf::from("/tmp/stale.txt"),
        ]);
        let generation = panel.load_generation;

        let status = BrowserCommandState::apply_loaded(
            &mut panel,
            PathBuf::from("/next"),
            Some("target.txt".to_string()),
            generation,
            Ok(vec![
                entry("..", true),
                entry(".cache", true),
                entry("keep.txt", false),
                entry("target.txt", false),
            ]),
        );

        assert_eq!(status.as_deref(), Some("2 rows, selected target.txt"));
        assert!(!panel.loading);
        assert_eq!(panel.path, PathBuf::from("/next"));
        assert_eq!(panel.selected_index, 1);
        assert_eq!(panel.selected_name(), "target.txt");
        assert_eq!(
            panel
                .rows
                .iter()
                .map(|row| row.name.as_str())
                .collect::<Vec<_>>(),
            vec!["keep.txt", "target.txt"]
        );
        assert_eq!(
            panel.marked,
            HashSet::from([PathBuf::from("/next/keep.txt")])
        );
        assert!(panel.error.is_none());
    }

    #[test]
    fn apply_loaded_shows_hidden_entries_when_enabled_but_never_parent_rows() {
        let mut panel = panel();
        panel.show_hidden = true;
        let generation = panel.load_generation;

        BrowserCommandState::apply_loaded(
            &mut panel,
            PathBuf::from("/next"),
            None,
            generation,
            Ok(vec![
                entry("..", true),
                entry(".cache", true),
                entry("visible", false),
            ]),
        );

        assert_eq!(
            panel
                .rows
                .iter()
                .map(|row| row.name.as_str())
                .collect::<Vec<_>>(),
            vec![".cache", "visible"]
        );
        assert_eq!(panel.selected_name(), ".cache");
    }

    #[test]
    fn apply_loaded_reports_errors_without_changing_generation() {
        let mut panel = panel();
        let generation = BrowserCommandState::start_loading(&mut panel, PathBuf::from("/next"));

        let status = BrowserCommandState::apply_loaded(
            &mut panel,
            PathBuf::from("/next"),
            None,
            generation,
            Err(anyhow::anyhow!("permission denied")),
        );

        assert_eq!(status.as_deref(), Some("cannot load /next"));
        assert!(!panel.loading);
        assert_eq!(panel.load_generation, 8);
        assert!(panel.rows.is_empty());
        assert_eq!(panel.selected_index, 0);
        assert_eq!(panel.error.as_deref(), Some("permission denied"));
    }
}
