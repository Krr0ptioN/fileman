use gpui::{App, IntoElement, ParentElement, RenderOnce, Styled, Window, div, px};
use gpui_component::{h_flex, v_flex};

use crate::features::{
    file_browser::{state::BrowserPanel, tokens},
    layout::{LayoutState, PaneMode},
};

#[derive(IntoElement)]
pub(crate) struct PanelHeader {
    panel: BrowserPanel,
    active: bool,
}

impl PanelHeader {
    pub(crate) fn new(panel: &BrowserPanel, active: bool) -> Self {
        Self {
            panel: panel.clone(),
            active,
        }
    }

    fn status(&self) -> String {
        match (self.panel.loading, self.panel.error.as_ref()) {
            (true, _) => "loading".to_string(),
            (_, Some(error)) => error.clone(),
            _ if self.panel.show_hidden && self.panel.show_ignored => {
                format!("{} rows | hidden + ignored", self.panel.rows.len())
            }
            _ if self.panel.show_hidden => format!("{} rows | hidden", self.panel.rows.len()),
            _ if self.panel.show_ignored => format!("{} rows | ignored", self.panel.rows.len()),
            _ => format!("{} rows", self.panel.rows.len()),
        }
    }
}

impl RenderOnce for PanelHeader {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let pane_title = match cx.global::<LayoutState>().pane_mode() {
            PaneMode::Single => div(),
            PaneMode::Dual => h_flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .font_weight(gpui::FontWeight::SEMIBOLD)
                        .text_color(if self.active {
                            tokens::TEXT_PRIMARY
                        } else {
                            tokens::TEXT_SECONDARY
                        })
                        .child(self.panel.title),
                )
                .child(
                    div()
                        .text_size(px(11.0))
                        .text_color(tokens::TEXT_MUTED)
                        .child(self.status()),
                ),
        };

        v_flex()
            .gap_1()
            .p_3()
            .bg(tokens::BG_PANEL_RAISED)
            .border_b_1()
            .border_color(tokens::BORDER_SUBTLE)
            .child(pane_title)
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(tokens::TEXT_SECONDARY)
                    .child(self.panel.path.display().to_string()),
            )
    }
}
