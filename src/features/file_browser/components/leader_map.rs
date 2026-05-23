use gpui::{IntoElement, ParentElement, Styled, div, px};
use gpui_component::{h_flex, v_flex};

use crate::features::{file_browser::tokens, keybind};

pub fn render_leader_map(prefix: String) -> impl IntoElement {
    h_flex()
        .absolute()
        .bottom(px(38.0))
        .left(px(0.0))
        .right(px(0.0))
        .justify_center()
        .child(
            v_flex()
                .w(px(430.0))
                .p_2()
                .gap_1()
                .rounded(px(6.0))
                .border_1()
                .border_color(tokens::BORDER_SUBTLE)
                .bg(tokens::BG_PANEL_RAISED)
                .shadow_md()
                .child(header(&prefix))
                .children(
                    keybind::continuations_for(&prefix)
                        .unwrap_or(&[])
                        .iter()
                        .map(row),
                ),
        )
}

fn header(prefix: &str) -> impl IntoElement {
    h_flex()
        .items_center()
        .gap_2()
        .child(
            div()
                .min_w(px(44.0))
                .text_size(px(12.0))
                .text_color(tokens::ACCENT)
                .child("key"),
        )
        .child(
            div()
                .text_size(px(12.0))
                .text_color(tokens::TEXT_MUTED)
                .child(format!("{prefix} command")),
        )
}

fn row(item: &keybind::LeaderContinuation) -> impl IntoElement {
    h_flex()
        .items_center()
        .gap_2()
        .child(
            div()
                .min_w(px(44.0))
                .text_size(px(12.0))
                .text_color(tokens::TEXT_PRIMARY)
                .child(item.key),
        )
        .child(
            div()
                .text_size(px(12.0))
                .text_color(tokens::TEXT_SECONDARY)
                .child(item.command),
        )
}
