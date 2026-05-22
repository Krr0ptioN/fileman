use gpui::{FontWeight, IntoElement, ParentElement, Styled, div, px};
use gpui_component::h_flex;

use crate::features::file_browser::tokens;

pub fn render_title_bar() -> impl IntoElement {
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
                        .child("FileMan"),
                ),
        )
        .child(
            div()
                .text_color(tokens::TEXT_SECONDARY)
                .text_size(px(12.0))
                .child("GPUI shell"),
        )
}

pub fn render_command_bar(mode: String, status: &str) -> impl IntoElement {
    h_flex()
        .h(px(34.0))
        .px_3()
        .items_center()
        .justify_between()
        .bg(tokens::BG_PANEL_RAISED)
        .border_t_1()
        .border_color(tokens::BORDER_SUBTLE)
        .child(
            h_flex()
                .items_center()
                .gap_2()
                .child(command_hint("j/k", "move"))
                .child(command_hint("v/V", "mark"))
                .child(command_hint("h/l", "parent/open"))
                .child(command_hint("yy/dd/pp", "copy/move/paste"))
                .child(command_hint("cw/x", "rename/delete"))
                .child(command_hint("s/w", "layout/pane")),
        )
        .child(
            h_flex()
                .items_center()
                .gap_2()
                .child(
                    div()
                        .text_size(px(12.0))
                        .text_color(tokens::ACCENT)
                        .child(mode),
                )
                .child(
                    div()
                        .text_size(px(12.0))
                        .text_color(tokens::TEXT_MUTED)
                        .child(status.to_string()),
                ),
        )
}

fn command_hint(key: &'static str, label: &'static str) -> impl IntoElement {
    h_flex()
        .items_center()
        .gap_1()
        .child(
            div()
                .px_1()
                .rounded(px(3.0))
                .border_1()
                .border_color(tokens::BORDER_SUBTLE)
                .text_color(tokens::ACCENT)
                .text_size(px(11.0))
                .child(key),
        )
        .child(
            div()
                .text_size(px(12.0))
                .text_color(tokens::TEXT_SECONDARY)
                .child(label),
        )
}
