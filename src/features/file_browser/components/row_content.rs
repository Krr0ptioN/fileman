use gpui::{App, IntoElement, ParentElement, RenderOnce, Styled, Window, div, px};
use gpui_component::h_flex;

use super::{
    badges::{executable_badge, intent_badge},
    icons::row_icon,
};
use crate::features::file_browser::{
    rows::{FileRow, RowIntent, kind_label},
    tokens,
};

#[derive(IntoElement)]
pub(crate) struct FileRowContent {
    row: FileRow,
    selected: bool,
    intent: RowIntent,
}

impl FileRowContent {
    pub(crate) fn new(row: FileRow, selected: bool, intent: RowIntent) -> Self {
        Self {
            row,
            selected,
            intent,
        }
    }
}

impl RenderOnce for FileRowContent {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        h_flex()
            .items_center()
            .gap_2()
            .min_w(px(0.0))
            .flex_1()
            .child(intent_badge(self.intent))
            .child(row_icon(self.row.kind))
            .child(executable_badge(self.row.is_executable))
            .child(FileName::new(self.row.clone(), self.selected))
            .child(
                div()
                    .text_size(px(11.0))
                    .text_color(tokens::TEXT_MUTED)
                    .child(kind_label(self.row.kind)),
            )
    }
}

#[derive(IntoElement)]
struct FileName {
    row: FileRow,
    selected: bool,
}

impl FileName {
    fn new(row: FileRow, selected: bool) -> Self {
        Self { row, selected }
    }
}

impl RenderOnce for FileName {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        div()
            .min_w(px(0.0))
            .text_color(if self.selected {
                tokens::TEXT_PRIMARY
            } else {
                tokens::TEXT_SECONDARY
            })
            .child(self.row.name)
    }
}
