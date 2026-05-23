use gpui::{App, IntoElement, ParentElement, RenderOnce, Styled, Window, div, px};
use gpui_component::{h_flex, v_flex};

use crate::features::{file_browser::tokens, keybind::LeaderContinuation};

#[derive(IntoElement)]
pub struct LeaderMap {
    prefix: String,
    entries: Vec<LeaderContinuation>,
}

impl LeaderMap {
    pub fn new(prefix: String, entries: Vec<LeaderContinuation>) -> Self {
        Self { prefix, entries }
    }
}

impl RenderOnce for LeaderMap {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
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
                    .child(LeaderHeader(self.prefix))
                    .children(self.entries.into_iter().map(LeaderRow)),
            )
    }
}

#[derive(IntoElement)]
struct LeaderHeader(String);

impl RenderOnce for LeaderHeader {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
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
                    .child(format!("{} command", self.0)),
            )
    }
}

#[derive(IntoElement)]
struct LeaderRow(LeaderContinuation);

impl RenderOnce for LeaderRow {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        h_flex()
            .items_center()
            .gap_2()
            .child(
                div()
                    .min_w(px(44.0))
                    .text_size(px(12.0))
                    .text_color(tokens::TEXT_PRIMARY)
                    .child(self.0.key),
            )
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(tokens::TEXT_SECONDARY)
                    .child(self.0.command),
            )
    }
}
