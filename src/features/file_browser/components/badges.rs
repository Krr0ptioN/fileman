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
        RowIntent::None => (None, "", tokens::TEXT_MUTED),
        RowIntent::Marked => (Some(IconName::Star), "mark", tokens::ACCENT),
        RowIntent::Copy => (Some(IconName::Copy), "yank", tokens::ICON_COPY),
        RowIntent::Move => (Some(IconName::Replace), "move", tokens::ICON_MOVE),
        RowIntent::Delete => (Some(IconName::Delete), "delete", tokens::ICON_DELETE),
    };

    h_flex()
        .w(px(68.0))
        .items_center()
        .gap_1()
        .child(match icon {
            Some(name) => Icon::new(name)
                .size(px(13.0))
                .text_color(color)
                .into_any_element(),
            None => div().w(px(13.0)).h(px(13.0)).into_any_element(),
        })
        .child(div().text_size(px(10.0)).text_color(color).child(label))
}
