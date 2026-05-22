use gpui::{IntoElement, ParentElement, Styled, div, px};
use gpui_component::{h_flex, v_flex};

use crate::features::file_browser::{state::BrowserPanel, tokens};

pub(crate) fn render_panel_header(panel: &BrowserPanel, active: bool) -> impl IntoElement {
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
                        .text_color(if active {
                            tokens::TEXT_PRIMARY
                        } else {
                            tokens::TEXT_SECONDARY
                        })
                        .child(panel.title),
                )
                .child(
                    div()
                        .text_size(px(11.0))
                        .text_color(tokens::TEXT_MUTED)
                        .child(panel_header_status(panel)),
                ),
        )
        .child(
            div()
                .text_size(px(12.0))
                .text_color(tokens::TEXT_SECONDARY)
                .child(panel.path.display().to_string()),
        )
}

fn panel_header_status(panel: &BrowserPanel) -> String {
    if panel.loading {
        "loading".to_string()
    } else if let Some(error) = &panel.error {
        error.clone()
    } else {
        format!("{} rows", panel.rows.len())
    }
}
