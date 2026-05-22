use gpui::{IntoElement, ParentElement, Styled, div, px};
use gpui_component::{Icon, IconName, h_flex};

use crate::features::file_browser::{rows::RowIntent, tokens};

pub(crate) fn executable_badge(is_executable: bool) -> impl IntoElement {
    h_flex()
        .w(px(14.0))
        .items_center()
        .justify_center()
        .child(if is_executable {
            Icon::new(IconName::Settings)
                .size(px(14.0))
                .text_color(tokens::ICON_EXECUTABLE)
                .into_any_element()
        } else {
            div().into_any_element()
        })
}

pub(crate) fn intent_badge(intent: RowIntent) -> impl IntoElement {
    let (icon, label, color) = match intent {
        RowIntent::None => (IconName::StarOff, "", tokens::TEXT_MUTED),
        RowIntent::Marked => (IconName::Star, "mark", tokens::ACCENT),
        RowIntent::Copy => (IconName::Copy, "yank", tokens::ICON_COPY),
        RowIntent::Move => (IconName::Replace, "move", tokens::ICON_MOVE),
        RowIntent::Delete => (IconName::Delete, "delete", tokens::ICON_DELETE),
    };

    h_flex()
        .w(px(68.0))
        .items_center()
        .gap_1()
        .child(Icon::new(icon).size(px(13.0)).text_color(color))
        .child(div().text_size(px(10.0)).text_color(color).child(label))
}
