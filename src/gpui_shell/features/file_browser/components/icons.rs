use gpui::{IntoElement, Styled, px};
use gpui_component::{Icon, IconName};

use crate::gpui_shell::features::file_browser::{
    rows::{FileFormat, RowKind},
    tokens,
};

pub(crate) fn row_icon(kind: RowKind) -> impl IntoElement {
    Icon::new(row_icon_name(kind))
        .size(px(16.0))
        .text_color(row_icon_color(kind))
}

fn row_icon_name(kind: RowKind) -> IconName {
    match kind {
        RowKind::Directory => IconName::FolderClosed,
        RowKind::Symlink => IconName::ExternalLink,
        RowKind::Socket => IconName::Globe,
        RowKind::Pipe => IconName::Minus,
        RowKind::BlockDevice | RowKind::CharDevice => IconName::SquareTerminal,
        RowKind::File(_) => IconName::File,
        RowKind::Other => IconName::Info,
    }
}

fn row_icon_color(kind: RowKind) -> gpui::Rgba {
    match kind {
        RowKind::Directory => tokens::ICON_DIRECTORY,
        RowKind::Symlink => tokens::ICON_SYMLINK,
        RowKind::Socket => tokens::ICON_SOCKET,
        RowKind::Pipe => tokens::ICON_PIPE,
        RowKind::BlockDevice | RowKind::CharDevice => tokens::ICON_DEVICE,
        RowKind::Other => tokens::ICON_OTHER,
        RowKind::File(format) => file_format_color(format),
    }
}

fn file_format_color(format: FileFormat) -> gpui::Rgba {
    match format {
        FileFormat::Archive => tokens::ICON_ARCHIVE,
        FileFormat::Audio => tokens::ICON_AUDIO,
        FileFormat::Binary => tokens::ICON_BINARY,
        FileFormat::Code => tokens::ICON_CODE,
        FileFormat::Image => tokens::ICON_IMAGE,
        FileFormat::Pdf => tokens::ICON_PDF,
        FileFormat::Text => tokens::ICON_TEXT,
        FileFormat::Video => tokens::ICON_VIDEO,
        FileFormat::Unknown => tokens::ICON_FILE,
    }
}
