use gpui::{App, FontWeight, IntoElement, ParentElement, RenderOnce, Styled, Window, div, px};
use gpui_component::h_flex;

use crate::features::file_browser::tokens;

#[derive(IntoElement)]
pub struct TitleBar;

impl RenderOnce for TitleBar {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        h_flex()
            .h(px(42.0))
            .px_3()
            .items_center()
            .justify_between()
            .bg(tokens::BG_PANEL_RAISED)
            .border_b_1()
            .border_color(tokens::BORDER_SUBTLE)
            .child(
                h_flex()
                    .items_center()
                    .gap_2()
                    .child(div().size(px(10.0)).rounded_full().bg(tokens::ACCENT))
                    .child(
                        div()
                            .text_color(tokens::TEXT_PRIMARY)
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("stiff"),
                    ),
            )
    }
}

#[derive(IntoElement)]
pub struct CommandBar {
    mode: String,
    status: String,
}

impl CommandBar {
    pub fn new(mode: String, status: impl Into<String>) -> Self {
        Self {
            mode,
            status: status.into(),
        }
    }
}

impl RenderOnce for CommandBar {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        h_flex()
            .h(px(34.0))
            .px_3()
            .items_center()
            .justify_end()
            .bg(tokens::BG_PANEL_RAISED)
            .border_t_1()
            .border_color(tokens::BORDER_SUBTLE)
            .child(
                h_flex()
                    .items_center()
                    .gap_2()
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(tokens::ACCENT)
                            .child(self.mode),
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(tokens::TEXT_MUTED)
                            .child(self.status),
                    ),
            )
    }
}
