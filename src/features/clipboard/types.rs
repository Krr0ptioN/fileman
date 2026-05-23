use gpui::Global;

use crate::features::file_browser::FileTarget;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ClipboardKind {
    Copy,
    Move,
}

#[derive(Clone)]
pub struct ClipboardOp {
    pub kind: ClipboardKind,
    pub targets: Vec<FileTarget>,
}

#[derive(Default)]
pub struct ClipboardState {
    pub(crate) op: Option<ClipboardOp>,
}

impl ClipboardState {
    pub fn clear(&mut self) {
        self.op = None;
    }
}

impl Global for ClipboardState {}

impl ClipboardKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::Copy => "copy",
            Self::Move => "move",
        }
    }
}
