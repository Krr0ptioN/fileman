use gpui::{IntoElement, ParentElement, Styled, div, px};
use gpui_component::{h_flex, v_flex};

use crate::features::{file_browser::tokens, keybind};

pub fn render_help_popup() -> impl IntoElement {
    v_flex()
        .absolute()
        .top(px(54.0))
        .right(px(18.0))
        .w(px(500.0))
        .max_w(px(720.0))
        .p_3()
        .gap_3()
        .rounded(px(8.0))
        .border_1()
        .border_color(tokens::ROW_SELECTED_ACTIVE_BORDER)
        .bg(tokens::BG_PANEL_RAISED)
        .shadow_lg()
        .child(header())
        .children(keybind::KEYBIND_GROUPS.iter().map(group))
}

fn header() -> impl IntoElement {
    h_flex()
        .items_center()
        .justify_between()
        .child(
            div()
                .text_size(px(14.0))
                .text_color(tokens::TEXT_PRIMARY)
                .font_weight(gpui::FontWeight::SEMIBOLD)
                .child("Key Map"),
        )
        .child(
            div()
                .text_size(px(12.0))
                .text_color(tokens::TEXT_MUTED)
                .child("Esc or q to close"),
        )
}

fn group(group: &keybind::KeybindGroup) -> impl IntoElement {
    v_flex()
        .gap_1()
        .child(
            div()
                .text_size(px(11.0))
                .text_color(tokens::ACCENT)
                .child(group.title),
        )
        .children(group.bindings.iter().map(binding))
}

fn binding(binding: &keybind::KeybindHelp) -> impl IntoElement {
    h_flex()
        .items_center()
        .justify_between()
        .gap_4()
        .child(
            div()
                .min_w(px(126.0))
                .text_size(px(12.0))
                .text_color(tokens::TEXT_PRIMARY)
                .child(binding.keys),
        )
        .child(
            div()
                .flex_1()
                .text_size(px(12.0))
                .text_color(tokens::TEXT_SECONDARY)
                .child(binding.action),
        )
}
