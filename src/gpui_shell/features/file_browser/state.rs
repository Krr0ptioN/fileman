use std::{collections::HashSet, path::PathBuf};

use gpui::{ScrollStrategy, UniformListScrollHandle};

use super::rows::FileRow;

#[derive(Clone)]
pub(crate) struct BrowserPanel {
    pub(crate) side: PanelSide,
    pub(crate) title: &'static str,
    pub(crate) path: PathBuf,
    pub(crate) selected_index: usize,
    pub(crate) rows: Vec<FileRow>,
    pub(crate) marked: HashSet<PathBuf>,
    pub(crate) loading: bool,
    pub(crate) error: Option<String>,
    pub(crate) load_generation: u64,
    pub(crate) scroll_handle: UniformListScrollHandle,
}

impl BrowserPanel {
    pub(crate) fn select_relative(&mut self, delta: isize) {
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

    pub(crate) fn select_line(&mut self, index: usize) {
        if self.rows.is_empty() {
            self.selected_index = 0;
        } else {
            self.selected_index = index.min(self.rows.len() - 1);
        }
        self.reveal_selected();
    }

    pub(crate) fn select_last(&mut self) {
        if !self.rows.is_empty() {
            self.selected_index = self.rows.len() - 1;
        }
        self.reveal_selected();
    }

    pub(crate) fn reveal_selected(&self) {
        if !self.rows.is_empty() {
            self.scroll_handle
                .scroll_to_item(self.selected_index, ScrollStrategy::Center);
        }
    }

    pub(crate) fn selected_name(&self) -> &str {
        self.rows
            .get(self.selected_index)
            .map(|row| row.name.as_str())
            .unwrap_or("<none>")
    }

    pub(crate) fn selected_row(&self) -> Option<&FileRow> {
        self.rows.get(self.selected_index)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct FileTarget {
    pub(crate) path: PathBuf,
    pub(crate) name: String,
    pub(crate) is_dir: bool,
}

impl FileTarget {
    pub(crate) fn from_row(row: &FileRow) -> Self {
        Self {
            path: row.path.clone(),
            name: row.name.clone(),
            is_dir: row.is_dir,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum ClipboardKind {
    Copy,
    Move,
}

#[derive(Clone)]
pub(crate) struct ClipboardOp {
    pub(crate) kind: ClipboardKind,
    pub(crate) targets: Vec<FileTarget>,
}

pub(crate) enum InputMode {
    Normal,
    Rename { target: FileTarget, input: String },
}

#[derive(Clone)]
pub(crate) enum PendingConfirm {
    Delete(Vec<FileTarget>),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum PaneMode {
    Dual,
    Single,
}

impl PaneMode {
    pub(crate) fn toggle(self) -> Self {
        match self {
            Self::Dual => Self::Single,
            Self::Single => Self::Dual,
        }
    }

    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Dual => "dual",
            Self::Single => "single",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PanelSide {
    Left,
    Right,
}

impl PanelSide {
    pub(crate) fn other(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Left => "primary",
            Self::Right => "secondary",
        }
    }
}
