use std::{collections, path, sync};

use gpui::{ScrollStrategy, UniformListScrollHandle};

use super::rows::FileRow;

#[derive(Clone)]
pub struct BrowserPanel {
    pub side: PanelSide,
    pub title: &'static str,
    pub path: path::PathBuf,
    pub selected_index: usize,
    pub rows: sync::Arc<Vec<FileRow>>,
    pub show_hidden: bool,
    pub show_ignored: bool,
    pub marked: sync::Arc<collections::HashSet<path::PathBuf>>,
    pub loading: bool,
    pub error: Option<String>,
    pub load_generation: u64,
    pub search_generation: u64,
    pub search: Option<FilenameSearchSession>,
    pub scroll_handle: UniformListScrollHandle,
}

impl BrowserPanel {
    pub fn listing_snapshot(&self) -> BrowserListingSnapshot {
        BrowserListingSnapshot {
            path: self.path.clone(),
            selected_index: self.selected_index,
            rows: self.rows.clone(),
            marked: self.marked.clone(),
        }
    }

    pub fn restore_listing(&mut self, snapshot: BrowserListingSnapshot) {
        self.path = snapshot.path;
        self.selected_index = snapshot.selected_index;
        self.rows = snapshot.rows;
        self.marked = snapshot.marked;
        self.loading = false;
        self.error = None;
    }

    pub fn replace_rows(&mut self, rows: Vec<FileRow>) {
        self.rows = sync::Arc::new(rows);
    }

    pub fn clear_rows(&mut self) {
        sync::Arc::make_mut(&mut self.rows).clear();
    }

    pub fn marked_mut(&mut self) -> &mut collections::HashSet<path::PathBuf> {
        sync::Arc::make_mut(&mut self.marked)
    }

    pub fn clear_marks(&mut self) {
        self.marked_mut().clear();
    }

    pub fn select_relative(&mut self, delta: isize) {
        if self.rows.is_empty() {
            self.selected_index = 0;
            return;
        }

        self.selected_index = if delta.is_negative() {
            self.selected_index.saturating_sub(delta.unsigned_abs())
        } else {
            self.selected_index
                .saturating_add(delta as usize)
                .min(self.rows.len() - 1)
        };
    }

    pub fn select_line(&mut self, index: usize) {
        if self.rows.is_empty() {
            self.selected_index = 0;
        } else {
            self.selected_index = index.min(self.rows.len() - 1);
        }
    }

    pub fn select_last(&mut self) {
        if !self.rows.is_empty() {
            self.selected_index = self.rows.len() - 1;
        }
    }

    pub fn reveal_selected(&self) {
        if !self.rows.is_empty() {
            self.scroll_handle
                .scroll_to_item(self.selected_index, ScrollStrategy::Center);
        }
    }

    pub fn selected_name(&self) -> &str {
        self.rows
            .get(self.selected_index)
            .map(|row| row.name.as_ref())
            .unwrap_or("<none>")
    }

    pub fn selected_row(&self) -> Option<&FileRow> {
        self.rows.get(self.selected_index)
    }
}

#[derive(Clone)]
pub struct FilenameSearchSession {
    pub root: path::PathBuf,
    pub query: String,
    pub scope: super::FilenameSearchScope,
    pub generation: u64,
    pub cancel: sync::Arc<sync::atomic::AtomicBool>,
    pub previous: BrowserListingSnapshot,
}

#[derive(Clone)]
pub struct BrowserListingSnapshot {
    pub path: path::PathBuf,
    pub selected_index: usize,
    pub rows: sync::Arc<Vec<FileRow>>,
    pub marked: sync::Arc<collections::HashSet<path::PathBuf>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileTarget {
    pub path: path::PathBuf,
    pub name: String,
    pub is_dir: bool,
}

impl FileTarget {
    pub fn from_row(row: &FileRow) -> Self {
        Self {
            path: row.path.clone(),
            name: row.name.to_string(),
            is_dir: row.is_dir,
        }
    }
}

pub enum InputMode {
    Normal,
    Rename {
        target: FileTarget,
        input: String,
    },
    NewDirectory {
        parent: path::PathBuf,
        input: String,
    },
    QuickJump {
        base: path::PathBuf,
        input: String,
    },
    FilenameSearch {
        root: path::PathBuf,
        input: String,
        scope: super::FilenameSearchScope,
    },
}

#[derive(Clone)]
pub enum PendingConfirm {
    Delete(Vec<FileTarget>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PanelSide {
    Left,
    Right,
}

impl PanelSide {
    pub fn other(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Left => "primary",
            Self::Right => "secondary",
        }
    }
}
