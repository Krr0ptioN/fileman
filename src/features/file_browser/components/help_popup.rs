use gpui::{App, IntoElement, ParentElement, RenderOnce, Styled, Window, div, px};
use gpui_component::{h_flex, v_flex};

use crate::features::{
    file_browser::tokens,
    keybind::{KeybindGroup, KeybindHelp},
};

#[derive(IntoElement)]
pub struct HelpPopup {
    groups: Vec<KeybindGroup>,
}

impl HelpPopup {
    pub fn new(groups: Vec<KeybindGroup>) -> Self {
        Self { groups }
    }
}

impl RenderOnce for HelpPopup {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        h_flex()
            .absolute()
            .top(px(68.0))
            .left(px(0.0))
            .right(px(0.0))
            .justify_center()
            .child(
                v_flex()
                    .w(px(520.0))
                    .max_w(px(720.0))
                    .p_3()
                    .gap_3()
                    .rounded(px(8.0))
                    .border_1()
                    .border_color(tokens::ROW_SELECTED_ACTIVE_BORDER)
                    .bg(tokens::BG_PANEL_RAISED)
                    .shadow_lg()
                    .child(HelpHeader)
                    .children(self.groups.into_iter().map(HelpGroup)),
            )
    }
}

#[derive(IntoElement)]
struct HelpHeader;

impl RenderOnce for HelpHeader {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
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
}

#[derive(IntoElement)]
struct HelpGroup(KeybindGroup);

impl RenderOnce for HelpGroup {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        v_flex()
            .gap_1()
            .child(
                div()
                    .text_size(px(11.0))
                    .text_color(tokens::ACCENT)
                    .child(self.0.title),
            )
            .children(self.0.bindings.into_iter().map(HelpBinding))
    }
}

#[derive(IntoElement)]
struct HelpBinding(KeybindHelp);

impl RenderOnce for HelpBinding {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        h_flex()
            .items_center()
            .justify_between()
            .gap_4()
            .child(
                div()
                    .min_w(px(126.0))
                    .text_size(px(12.0))
                    .text_color(tokens::TEXT_PRIMARY)
                    .child(self.0.keys),
            )
            .child(
                div()
                    .flex_1()
                    .text_size(px(12.0))
                    .text_color(tokens::TEXT_SECONDARY)
                    .child(self.0.action),
            )
    }
}
