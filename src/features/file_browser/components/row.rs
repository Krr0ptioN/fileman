use gpui::{
    App, InteractiveElement, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window,
    div, px,
};
use gpui_component::h_flex;

use super::row_content::FileRowContent;
use crate::features::file_browser::{
    rows::{FileRow, RowIntent, RowKind},
    tokens,
};

#[derive(IntoElement)]
pub(crate) struct FileRowItem {
    ix: usize,
    kind: RowKind,
    name: SharedString,
    detail: SharedString,
    is_executable: bool,
    selected: bool,
    active: bool,
    intent: RowIntent,
}

impl FileRowItem {
    pub(crate) fn new(
        ix: usize,
        row: &FileRow,
        selected: bool,
        active: bool,
        intent: RowIntent,
    ) -> Self {
        Self {
            ix,
            kind: row.kind,
            name: row.name.clone(),
            detail: row.detail.clone(),
            is_executable: row.is_executable,
            selected,
            active,
            intent,
        }
    }

    fn bg(&self) -> gpui::Rgba {
        match (self.selected, self.active, self.intent) {
            (true, true, _) => tokens::ROW_SELECTED_ACTIVE,
            (true, false, _) => tokens::ROW_SELECTED_INACTIVE,
            (false, _, RowIntent::None) => tokens::BG_PANEL,
            (false, _, intent) => intent_bg(intent),
        }
    }

    fn border(&self) -> gpui::Rgba {
        match (self.selected, self.active) {
            (true, true) => tokens::ROW_SELECTED_ACTIVE_BORDER,
            (true, false) => tokens::ROW_SELECTED_INACTIVE_BORDER,
            (false, _) => tokens::ROW_BORDER_CLEAR,
        }
    }
}

impl RenderOnce for FileRowItem {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        h_flex()
            .id(("file-row", self.ix))
            .w_full()
            .h(px(32.0))
            .px_3()
            .items_center()
            .justify_between()
            .bg(self.bg())
            .border_1()
            .border_color(self.border())
            .rounded(px(if self.selected { 7.0 } else { 0.0 }))
            .hover(|style| style.bg(tokens::ROW_HOVER))
            .child(FileRowContent::new(
                self.kind,
                self.name,
                self.is_executable,
                self.selected,
                self.intent,
            ))
            .child(
                div()
                    .flex_shrink_0()
                    .text_size(px(12.0))
                    .text_color(tokens::TEXT_SECONDARY)
                    .child(self.detail),
            )
    }
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
