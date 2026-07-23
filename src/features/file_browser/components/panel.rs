use std::{collections::HashSet, path::PathBuf};

use gpui::{App, IntoElement, ParentElement, RenderOnce, Styled, Window, div, px, uniform_list};
use gpui_component::v_flex;

use super::{header::PanelHeader, row::FileRowItem};
use crate::features::{
    clipboard::{ClipboardKind, target_paths},
    file_browser::{
        rows::row_intent,
        state::{BrowserPanel, PanelSide, PendingConfirm},
        tokens,
    },
};

#[derive(IntoElement)]
pub struct FilePanel {
    panel: BrowserPanel,
    active: bool,
    pending_confirm: Option<PendingConfirm>,
}

impl FilePanel {
    pub fn new(
        panel: &BrowserPanel,
        active: bool,
        pending_confirm: Option<&PendingConfirm>,
    ) -> Self {
        Self {
            panel: panel.clone(),
            active,
            pending_confirm: pending_confirm.cloned(),
        }
    }
}

impl RenderOnce for FilePanel {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let rows = self.panel.rows.clone();
        let marked = self.panel.marked.clone();
        let copy_targets = target_paths(cx, ClipboardKind::Copy);
        let move_targets = target_paths(cx, ClipboardKind::Move);
        let delete_targets = delete_targets(self.pending_confirm.as_ref());
        let selected_index = self.panel.selected_index;
        let row_count = rows.len();
        let scroll_handle = self.panel.scroll_handle.clone();
        let list_id = list_id(self.panel.side);

        v_flex()
            .flex_1()
            .min_w(px(0.0))
            .min_h(px(0.0))
            .h_full()
            .bg(tokens::BG_PANEL)
            .border_1()
            .border_color(if self.active {
                tokens::BORDER_FOCUS
            } else {
                tokens::BORDER_SUBTLE
            })
            .rounded(px(6.0))
            .overflow_hidden()
            .child(PanelHeader::new(&self.panel, self.active))
            .child(
                div().flex_grow().min_h(px(0.0)).p_1().child(
                    uniform_list(list_id, row_count, move |range, _, _| {
                        range
                            .map(|ix| {
                                let row = &rows[ix];
                                let is_marked = marked.contains(&row.path);
                                let intent = row_intent(
                                    &row.path,
                                    is_marked,
                                    &copy_targets,
                                    &move_targets,
                                    &delete_targets,
                                );
                                FileRowItem::new(ix, row, ix == selected_index, self.active, intent)
                            })
                            .collect::<Vec<_>>()
                    })
                    .track_scroll(scroll_handle)
                    .h_full(),
                ),
            )
    }
}

fn list_id(side: PanelSide) -> &'static str {
    match side {
        PanelSide::Left => "left-rows",
        PanelSide::Right => "right-rows",
    }
}

fn delete_targets(pending_confirm: Option<&PendingConfirm>) -> HashSet<PathBuf> {
    match pending_confirm {
        Some(confirm) => match *confirm {
            PendingConfirm::Delete(ref targets) => {
                targets.iter().map(|target| target.path.clone()).collect()
            }
        },
        None => HashSet::new(),
    }
}
