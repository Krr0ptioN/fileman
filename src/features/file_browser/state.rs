use std::{collections::HashSet, path::PathBuf};

use gpui::{ScrollStrategy, UniformListScrollHandle};

use super::rows::FileRow;

#[derive(Clone)]
pub struct BrowserPanel {
    pub side: PanelSide,
    pub title: &'static str,
    pub path: PathBuf,
    pub selected_index: usize,
    pub rows: Vec<FileRow>,
    pub marked: HashSet<PathBuf>,
    pub loading: bool,
    pub error: Option<String>,
    pub load_generation: u64,
    pub scroll_handle: UniformListScrollHandle,
}

impl BrowserPanel {
    pub fn select_relative(&mut self, delta: isize) {
        if self.rows.is_empty() {
            self.selected_index = 0;
            self.reveal_selected();
            return;
        }

        self.selected_index = if delta.is_negative() {
            self.selected_index.saturating_sub(delta.unsigned_abs())
        } else {
            self.selected_index
                .saturating_add(delta as usize)
                .min(self.rows.len() - 1)
        };
        self.reveal_selected();
    }

    pub fn select_line(&mut self, index: usize) {
        if self.rows.is_empty() {
            self.selected_index = 0;
        } else {
            self.selected_index = index.min(self.rows.len() - 1);
        }
        self.reveal_selected();
    }

    pub fn select_last(&mut self) {
        if !self.rows.is_empty() {
            self.selected_index = self.rows.len() - 1;
        }
        self.reveal_selected();
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
            .map(|row| row.name.as_str())
            .unwrap_or("<none>")
    }

    pub fn selected_row(&self) -> Option<&FileRow> {
        self.rows.get(self.selected_index)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileTarget {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
}

impl FileTarget {
    pub fn from_row(row: &FileRow) -> Self {
        Self {
            path: row.path.clone(),
            name: row.name.clone(),
            is_dir: row.is_dir,
        }
    }
}

pub enum InputMode {
    Normal,
    Rename { target: FileTarget, input: String },
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
