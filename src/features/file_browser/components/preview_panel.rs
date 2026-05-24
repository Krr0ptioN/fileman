use gpui::{App, FontWeight, IntoElement, ParentElement, RenderOnce, Styled, Window, div, px};
use gpui_component::{h_flex, scroll::ScrollableElement, v_flex};

use crate::features::file_browser::{PreviewBody, PreviewState, tokens};

#[derive(IntoElement)]
pub struct PreviewPanel {
    preview: PreviewState,
}

impl PreviewPanel {
    pub fn new(preview: &PreviewState) -> Self {
        Self {
            preview: preview.clone(),
        }
    }
}

impl RenderOnce for PreviewPanel {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        v_flex()
            .flex_1()
            .min_w(px(0.0))
            .h_full()
            .bg(tokens::BG_PANEL)
            .border_1()
            .border_color(tokens::BORDER_SUBTLE)
            .rounded(px(6.0))
            .overflow_hidden()
            .child(preview_header(&self.preview))
            .child(preview_body(self.preview.body))
    }
}

fn preview_header(preview: &PreviewState) -> impl IntoElement {
    h_flex()
        .h(px(44.0))
        .px_3()
        .items_center()
        .justify_between()
        .bg(tokens::BG_PANEL_RAISED)
        .border_b_1()
        .border_color(tokens::BORDER_SUBTLE)
        .child(
            div()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(tokens::TEXT_PRIMARY)
                .child(preview.target.name.clone()),
        )
        .child(
            div()
                .text_size(px(11.0))
                .text_color(tokens::TEXT_MUTED)
                .child(preview.target.path.display().to_string()),
        )
}

fn preview_body(body: PreviewBody) -> impl IntoElement {
    let (label, text) = match body {
        PreviewBody::Loading => ("loading", "Loading preview...".to_string()),
        PreviewBody::Text(text) => ("text", text),
        PreviewBody::Binary(text) => ("binary", text),
        PreviewBody::Error(error) => ("error", error),
    };

    v_flex()
        .flex_grow()
        .gap_2()
        .p_3()
        .overflow_y_scrollbar()
        .child(
            div()
                .text_size(px(11.0))
                .text_color(tokens::ACCENT)
                .child(label),
        )
        .child(
            div()
                .text_size(px(12.0))
                .text_color(tokens::TEXT_SECONDARY)
                .whitespace_normal()
                .children(
                    text.lines()
                        .take(500)
                        .map(|line| div().min_h(px(16.0)).child(line.to_string())),
                ),
        )
}
