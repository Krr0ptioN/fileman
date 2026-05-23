use gpui::{App, IntoElement, ParentElement, RenderOnce, Styled, Window, div, px};
use gpui_component::{h_flex, v_flex};

use crate::features::file_browser::{state::BrowserPanel, tokens};

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
        match (&self.panel.loading, &self.panel.error) {
            (true, _) => "loading".to_string(),
            (_, Some(error)) => error.clone(),
            _ => format!("{} rows", self.panel.rows.len()),
        }
    }
}

impl RenderOnce for PanelHeader {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        v_flex()
            .gap_1()
            .p_3()
            .bg(tokens::BG_PANEL_RAISED)
            .border_b_1()
            .border_color(tokens::BORDER_SUBTLE)
            .child(
                h_flex()
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
            )
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(tokens::TEXT_SECONDARY)
                    .child(self.panel.path.display().to_string()),
            )
    }
}
