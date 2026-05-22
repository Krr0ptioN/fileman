use std::{collections::HashSet, path::PathBuf};

use gpui::{IntoElement, ParentElement, Styled, div, px, uniform_list};
use gpui_component::v_flex;

use super::{header::render_panel_header, row::render_row};
use crate::features::file_browser::{
    rows::row_intent,
    state::{BrowserPanel, ClipboardKind, ClipboardOp, PanelSide, PendingConfirm},
    tokens,
};

pub fn render_panel(
    panel: &BrowserPanel,
    active: bool,
    clipboard: Option<&ClipboardOp>,
    pending_confirm: Option<&PendingConfirm>,
) -> impl IntoElement + use<> {
    let rows = panel.rows.clone();
    let marked = panel.marked.clone();
    let copy_targets = clipboard_targets(clipboard, ClipboardKind::Copy);
    let move_targets = clipboard_targets(clipboard, ClipboardKind::Move);
    let delete_targets = delete_targets(pending_confirm);
    let selected_index = panel.selected_index;
    let row_count = rows.len();
    let scroll_handle = panel.scroll_handle.clone();
    let list_id = match panel.side {
        PanelSide::Left => "left-rows",
        PanelSide::Right => "right-rows",
    };

    v_flex()
        .flex_1()
        .min_w(px(0.0))
        .h_full()
        .bg(tokens::BG_PANEL)
        .border_1()
        .border_color(if active {
            tokens::BORDER_FOCUS
        } else {
            tokens::BORDER_SUBTLE
        })
        .rounded(px(6.0))
        .overflow_hidden()
        .child(render_panel_header(panel, active))
        .child(
            div().flex_grow().child(
                uniform_list(list_id, row_count, move |range, _, _| {
                    range
                        .map(|ix| {
                            let row = rows[ix].clone();
                            let is_marked = marked.contains(&row.path);
                            let intent = row_intent(
                                &row.path,
                                is_marked,
                                &copy_targets,
                                &move_targets,
                                &delete_targets,
                            );
                            render_row(ix, row, ix == selected_index, active, intent)
                        })
                        .collect::<Vec<_>>()
                })
                .track_scroll(scroll_handle)
                .h_full(),
            ),
        )
}

fn clipboard_targets(clipboard: Option<&ClipboardOp>, kind: ClipboardKind) -> HashSet<PathBuf> {
    let Some(clipboard) = clipboard else {
        return HashSet::new();
    };
    if clipboard.kind != kind {
        return HashSet::new();
    }
    clipboard
        .targets
        .iter()
        .map(|target| target.path.clone())
        .collect()
}

fn delete_targets(pending_confirm: Option<&PendingConfirm>) -> HashSet<PathBuf> {
    match pending_confirm {
        Some(PendingConfirm::Delete(targets)) => {
            targets.iter().map(|target| target.path.clone()).collect()
        }
        None => HashSet::new(),
    }
}
