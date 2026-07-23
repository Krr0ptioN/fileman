use gpui::{App, IntoElement, ParentElement, RenderOnce, Styled, Window, px};
use gpui_component::{h_flex, v_flex};

use super::LayoutVariant;
use crate::features::{
    file_browser::{
        components::FilePanel,
        state::{BrowserPanel, PanelSide, PendingConfirm},
    },
    layout::LayoutState,
};

#[derive(IntoElement)]
pub struct PanelLayout {
    primary: BrowserPanel,
    secondary: BrowserPanel,
    primary_tabs: (usize, usize),
    secondary_tabs: (usize, usize),
    active: PanelSide,
    pending_confirm: Option<PendingConfirm>,
}

impl PanelLayout {
    pub fn new(
        primary: &BrowserPanel,
        secondary: &BrowserPanel,
        primary_tabs: (usize, usize),
        secondary_tabs: (usize, usize),
        active: PanelSide,
        pending_confirm: Option<&PendingConfirm>,
    ) -> Self {
        Self {
            primary: primary.clone(),
            secondary: secondary.clone(),
            primary_tabs,
            secondary_tabs,
            active,
            pending_confirm: pending_confirm.cloned(),
        }
    }

    fn single_panel(&self) -> FilePanel {
        match self.active {
            PanelSide::Left => FilePanel::new(
                &self.primary,
                true,
                self.primary_tabs,
                self.pending_confirm.as_ref(),
            ),
            PanelSide::Right => FilePanel::new(
                &self.secondary,
                true,
                self.secondary_tabs,
                self.pending_confirm.as_ref(),
            ),
        }
    }
}

impl RenderOnce for PanelLayout {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let pane_mode = cx.global::<LayoutState>().pane_mode();
        match LayoutVariant::resolve(window.viewport_size(), pane_mode) {
            LayoutVariant::SingleActive => v_flex()
                .flex_grow()
                .min_h(px(0.0))
                .gap_2()
                .p_2()
                .child(self.single_panel())
                .into_any_element(),
            LayoutVariant::DualStacked => v_flex()
                .flex_grow()
                .min_h(px(0.0))
                .gap_2()
                .p_2()
                .child(FilePanel::new(
                    &self.primary,
                    self.active == PanelSide::Left,
                    self.primary_tabs,
                    self.pending_confirm.as_ref(),
                ))
                .child(FilePanel::new(
                    &self.secondary,
                    self.active == PanelSide::Right,
                    self.secondary_tabs,
                    self.pending_confirm.as_ref(),
                ))
                .into_any_element(),
            LayoutVariant::DualSplit => h_flex()
                .flex_grow()
                .min_h(px(0.0))
                .gap_2()
                .p_2()
                .child(FilePanel::new(
                    &self.primary,
                    self.active == PanelSide::Left,
                    self.primary_tabs,
                    self.pending_confirm.as_ref(),
                ))
                .child(FilePanel::new(
                    &self.secondary,
                    self.active == PanelSide::Right,
                    self.secondary_tabs,
                    self.pending_confirm.as_ref(),
                ))
                .into_any_element(),
        }
    }
}
