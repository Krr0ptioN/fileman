use gpui::Global;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum PaneMode {
    Dual,
    #[default]
    Single,
}

impl PaneMode {
    pub fn toggle(self) -> Self {
        match self {
            Self::Dual => Self::Single,
            Self::Single => Self::Dual,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Dual => "dual",
            Self::Single => "single",
        }
    }
}

#[derive(Default)]
pub struct LayoutState {
    pane_mode: PaneMode,
}

impl LayoutState {
    pub fn pane_mode(&self) -> PaneMode {
        self.pane_mode
    }

    pub fn toggle_pane_mode(&mut self) -> PaneMode {
        self.pane_mode = self.pane_mode.toggle();
        self.pane_mode
    }
}

impl Global for LayoutState {}
