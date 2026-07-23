use super::BrowserPanel;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BrowserTabId(u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BrowserTabAction {
    Open,
    Next,
    Previous,
    Close,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TabPosition {
    pub number: usize,
    pub total: usize,
}

#[derive(Clone)]
struct BrowserTab {
    id: BrowserTabId,
    panel: BrowserPanel,
}

pub struct BrowserPane {
    tabs: Vec<BrowserTab>,
    active: usize,
    next_id: u64,
}

impl BrowserPane {
    pub fn new(panel: BrowserPanel) -> Self {
        Self {
            tabs: vec![BrowserTab {
                id: BrowserTabId(1),
                panel,
            }],
            active: 0,
            next_id: 2,
        }
    }

    pub fn active(&self) -> &BrowserPanel {
        &self.tabs[self.active].panel
    }

    pub fn active_mut(&mut self) -> &mut BrowserPanel {
        &mut self.tabs[self.active].panel
    }

    pub fn active_id(&self) -> BrowserTabId {
        self.tabs[self.active].id
    }

    pub fn panel_mut(&mut self, id: BrowserTabId) -> Option<&mut BrowserPanel> {
        self.tabs
            .iter_mut()
            .find(|tab| tab.id == id)
            .map(|tab| &mut tab.panel)
    }

    pub fn tab_count(&self) -> usize {
        self.tabs.len()
    }

    pub fn active_number(&self) -> usize {
        self.active + 1
    }

    pub fn position(&self) -> TabPosition {
        TabPosition {
            number: self.active_number(),
            total: self.tab_count(),
        }
    }

    pub fn open_tab(&mut self) {
        let mut panel = self.active().clone();
        if let Some(search) = panel.search.take() {
            panel.restore_listing(search.previous);
        }
        if panel.loading {
            panel.loading = false;
            panel.load_generation = 0;
        }
        panel.scroll_handle = Default::default();
        let tab = BrowserTab {
            id: BrowserTabId(self.next_id),
            panel,
        };
        self.next_id = self.next_id.wrapping_add(1).max(1);
        self.active += 1;
        self.tabs.insert(self.active, tab);
    }

    pub fn select_next(&mut self) {
        self.active = (self.active + 1) % self.tabs.len();
    }

    pub fn select_previous(&mut self) {
        self.active = self.active.checked_sub(1).unwrap_or(self.tabs.len() - 1);
    }

    pub fn close_active(&mut self) -> bool {
        if self.tabs.len() == 1 {
            return false;
        }
        let closed = self.tabs.remove(self.active);
        if let Some(search) = closed.panel.search {
            search
                .cancel
                .store(true, std::sync::atomic::Ordering::Relaxed);
        }
        if self.active == self.tabs.len() {
            self.active -= 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use std::{collections, path, sync};

    use super::*;
    use crate::features::file_browser::PanelSide;

    fn panel(path: &str) -> BrowserPanel {
        BrowserPanel {
            side: PanelSide::Left,
            title: "Primary",
            path: path::PathBuf::from(path),
            selected_index: 3,
            rows: sync::Arc::new(Vec::new()),
            show_hidden: true,
            show_ignored: false,
            marked: sync::Arc::new(collections::HashSet::new()),
            loading: false,
            error: None,
            load_generation: 1,
            search_generation: 0,
            search: None,
            scroll_handle: Default::default(),
        }
    }

    #[test]
    fn new_tab_clones_current_directory_state() {
        let mut pane = BrowserPane::new(panel("/one"));

        pane.open_tab();

        assert_eq!(pane.tab_count(), 2);
        assert_eq!(pane.active_number(), 2);
        assert_eq!(pane.active().path, path::PathBuf::from("/one"));
        assert_eq!(pane.active().selected_index, 3);
        assert!(pane.active().show_hidden);
    }

    #[test]
    fn switching_tabs_restores_each_directory_and_selection() {
        let mut pane = BrowserPane::new(panel("/one"));
        pane.open_tab();
        pane.active_mut().path = path::PathBuf::from("/two");
        pane.active_mut().selected_index = 7;

        pane.select_previous();
        assert_eq!(pane.active().path, path::PathBuf::from("/one"));
        assert_eq!(pane.active().selected_index, 3);

        pane.select_next();
        assert_eq!(pane.active().path, path::PathBuf::from("/two"));
        assert_eq!(pane.active().selected_index, 7);
    }

    #[test]
    fn closing_tab_activates_neighbor_and_last_tab_is_preserved() {
        let mut pane = BrowserPane::new(panel("/one"));
        pane.open_tab();
        pane.active_mut().path = path::PathBuf::from("/two");

        assert!(pane.close_active());
        assert_eq!(pane.tab_count(), 1);
        assert_eq!(pane.active().path, path::PathBuf::from("/one"));
        assert!(!pane.close_active());
        assert_eq!(pane.tab_count(), 1);
    }

    #[test]
    fn tab_cloned_during_load_is_ready_for_its_own_load() {
        let mut source = panel("/one");
        source.loading = true;
        source.load_generation = 9;
        let mut pane = BrowserPane::new(source);

        pane.open_tab();

        assert!(!pane.active().loading);
        assert_eq!(pane.active().load_generation, 0);
    }

    #[test]
    fn background_completion_targets_origin_tab_after_switch() {
        let mut pane = BrowserPane::new(panel("/one"));
        pane.active_mut()
            .marked_mut()
            .insert(path::PathBuf::from("/one/marked.txt"));
        let origin = pane.active_id();
        pane.open_tab();
        pane.active_mut().path = path::PathBuf::from("/two");

        pane.panel_mut(origin)
            .expect("origin tab should remain addressable")
            .clear_marks();

        assert_eq!(pane.active().path, path::PathBuf::from("/two"));
        assert_eq!(pane.active().marked.len(), 1);
        assert!(
            pane.panel_mut(origin)
                .expect("origin tab should remain addressable")
                .marked
                .is_empty()
        );
    }
}
