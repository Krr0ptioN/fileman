use gpui::prelude::FluentBuilder;
use gpui::{App, FontWeight, IntoElement, ParentElement, RenderOnce, Styled, Window, div, px};
use gpui_component::{h_flex, scroll::ScrollableElement, v_flex};

use crate::features::file_browser::{
    BinaryPreview, PreviewBody, PreviewKind, PreviewListing, PreviewState, TextPreview, tokens,
};

#[derive(IntoElement)]
pub struct PreviewPanel {
    preview: PreviewState,
}

impl PreviewPanel {
    pub fn new(preview: &PreviewState) -> Self {
        Self {
            preview: preview.clone(),
        }
    }
}

impl RenderOnce for PreviewPanel {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        v_flex()
            .flex_1()
            .min_w(px(0.0))
            .h_full()
            .bg(tokens::BG_PANEL)
            .border_1()
            .border_color(tokens::BORDER_SUBTLE)
            .rounded(px(6.0))
            .overflow_hidden()
            .child(preview_header(&self.preview))
            .child(preview_body(self.preview.body))
    }
}

fn preview_header(preview: &PreviewState) -> impl IntoElement {
    let target = preview.target();

    h_flex()
        .h(px(44.0))
        .px_3()
        .items_center()
        .justify_between()
        .bg(tokens::BG_PANEL_RAISED)
        .border_b_1()
        .border_color(tokens::BORDER_SUBTLE)
        .child(
            div()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(tokens::TEXT_PRIMARY)
                .child(target.name.clone()),
        )
        .child(
            div()
                .text_size(px(11.0))
                .text_color(tokens::TEXT_MUTED)
                .child(target.path.display().to_string()),
        )
}

fn preview_body(body: PreviewBody) -> impl IntoElement {
    let (label, lines, muted) = match body {
        PreviewBody::Loading { kind } => (
            preview_kind_label(kind),
            vec![format!("Loading {} preview...", preview_kind_label(kind))],
            None,
        ),
        PreviewBody::Text(preview) => text_lines(preview),
        PreviewBody::Listing(preview) => listing_lines(preview),
        PreviewBody::Binary(preview) => binary_lines(preview),
        PreviewBody::Error(error) => ("error", vec![error], None),
    };

    v_flex()
        .flex_grow()
        .gap_2()
        .p_3()
        .overflow_y_scrollbar()
        .child(
            div()
                .text_size(px(11.0))
                .text_color(tokens::ACCENT)
                .child(label),
        )
        .when_some(muted, |this, muted| {
            this.child(
                div()
                    .text_size(px(11.0))
                    .text_color(tokens::TEXT_MUTED)
                    .child(muted),
            )
        })
        .child(
            div()
                .text_size(px(12.0))
                .text_color(tokens::TEXT_SECONDARY)
                .whitespace_normal()
                .children(
                    lines
                        .into_iter()
                        .map(|line| div().min_h(px(16.0)).child(empty_line_placeholder(line))),
                ),
        )
}

fn preview_kind_label(kind: PreviewKind) -> &'static str {
    match kind {
        PreviewKind::Archive => "archive",
        PreviewKind::Binary => "binary",
        PreviewKind::Text => "text",
    }
}

fn text_lines(preview: TextPreview) -> (&'static str, Vec<String>, Option<String>) {
    let muted = match preview.truncated {
        true => Some(format!(
            "showing {} loaded lines from line {}",
            preview.loaded_lines,
            preview.first_line + 1
        )),
        false => Some(format!("{} loaded lines", preview.loaded_lines)),
    };

    (
        "text",
        preview.text.lines().map(String::from).collect(),
        muted,
    )
}

fn listing_lines(preview: PreviewListing) -> (&'static str, Vec<String>, Option<String>) {
    let muted = match preview.truncated {
        true => Some("listing truncated".to_string()),
        false => Some(format!("{} entries", preview.entries.len())),
    };

    ("archive", preview.entries, muted)
}

fn binary_lines(preview: BinaryPreview) -> (&'static str, Vec<String>, Option<String>) {
    let muted = match preview.truncated {
        true => Some(format!("showing first {} bytes", preview.bytes_read)),
        false => Some(format!("{} bytes", preview.bytes_read)),
    };

    (
        "binary",
        preview.text.lines().map(String::from).collect(),
        muted,
    )
}

fn empty_line_placeholder(line: String) -> String {
    match line.is_empty() {
        true => " ".to_string(),
        false => line,
    }
}
