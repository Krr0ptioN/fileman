use gpui::{InteractiveElement, IntoElement, ParentElement, Styled, div, px};
use gpui_component::h_flex;

use super::{
    badges::{executable_badge, intent_badge},
    icons::row_icon,
};
use crate::features::file_browser::{
    rows::{FileRow, RowIntent, kind_label},
    tokens,
};

pub(crate) fn render_row(
    ix: usize,
    row: FileRow,
    selected: bool,
    active: bool,
    intent: RowIntent,
) -> impl IntoElement {
    let row_bg = if selected && active {
        tokens::ROW_SELECTED_ACTIVE
    } else if selected {
        tokens::ROW_SELECTED_INACTIVE
    } else if intent != RowIntent::None {
        intent_bg(intent)
    } else {
        tokens::BG_PANEL
    };
    let border = if selected && active {
        tokens::ROW_SELECTED_ACTIVE_BORDER
    } else if selected {
        tokens::ROW_SELECTED_INACTIVE_BORDER
    } else {
        tokens::ROW_BORDER_CLEAR
    };

    h_flex()
        .id(("file-row", ix))
        .w_full()
        .h(px(32.0))
        .px_3()
        .items_center()
        .justify_between()
        .bg(row_bg)
        .border_1()
        .border_color(border)
        .rounded(px(if selected { 7.0 } else { 0.0 }))
        .hover(|style| style.bg(tokens::ROW_HOVER))
        .child(
            h_flex()
                .items_center()
                .gap_2()
                .min_w(px(0.0))
                .flex_1()
                .child(intent_badge(intent))
                .child(row_icon(row.kind))
                .child(executable_badge(row.is_executable))
                .child(
                    div()
                        .min_w(px(0.0))
                        .text_color(if selected {
                            tokens::TEXT_PRIMARY
                        } else {
                            tokens::TEXT_SECONDARY
                        })
                        .child(row.name),
                )
                .child(
                    div()
                        .text_size(px(11.0))
                        .text_color(tokens::TEXT_MUTED)
                        .child(kind_label(row.kind)),
                ),
        )
        .child(
            div()
                .flex_shrink_0()
                .text_size(px(12.0))
                .text_color(tokens::TEXT_SECONDARY)
                .child(row.detail),
        )
}

fn intent_bg(intent: RowIntent) -> gpui::Rgba {
    match intent {
        RowIntent::None => tokens::BG_PANEL,
        RowIntent::Marked => tokens::ROW_MARKED,
        RowIntent::Copy => tokens::ROW_COPY,
        RowIntent::Move => tokens::ROW_MOVE,
        RowIntent::Delete => tokens::ROW_DELETE,
    }
}
