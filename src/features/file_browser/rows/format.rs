use std::path::Path;

use crate::core;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileFormat {
    Archive,
    Audio,
    Binary,
    Code,
    Image,
    Pdf,
    Text,
    Video,
    Unknown,
}

impl FileFormat {
    pub fn from_path(path: &Path) -> Self {
        match () {
            _ if core::is_archive_path(path) => Self::Archive,
            _ if core::is_audio_path(path) => Self::Audio,
            _ if core::is_binary_path(path) => Self::Binary,
            _ if core::is_code_path(path) => Self::Code,
            _ if core::is_image_path(path) => Self::Image,
            _ if core::is_pdf_path(path) => Self::Pdf,
            _ if core::is_text_path(path) => Self::Text,
            _ if core::is_video_path(path) => Self::Video,
            _ => Self::Unknown,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Archive => "archive",
            Self::Audio => "audio",
            Self::Binary => "binary",
            Self::Code => "code",
            Self::Image => "image",
            Self::Pdf => "pdf",
            Self::Text => "text",
            Self::Video => "video",
            Self::Unknown => "unknown",
        }
    }
}
