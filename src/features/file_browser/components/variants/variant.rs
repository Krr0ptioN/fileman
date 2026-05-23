use gpui::{Pixels, Size};

use crate::features::layout::PaneMode;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LayoutVariant {
    SingleActive,
    DualStacked,
    DualSplit,
}

impl LayoutVariant {
    pub fn resolve(viewport: Size<Pixels>, pane_mode: PaneMode) -> Self {
        let width = f32::from(viewport.width);
        let height = f32::from(viewport.height).max(1.0);
        let aspect = width / height;

        match (pane_mode, width < 920.0 || aspect < 1.15) {
            (PaneMode::Single, _) => Self::SingleActive,
            (PaneMode::Dual, true) => Self::DualStacked,
            (PaneMode::Dual, false) => Self::DualSplit,
        }
    }
}
