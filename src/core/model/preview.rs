use std::{
    path::{self, Path},
    sync::Arc,
};

use crate::archive::ContainerKind;

use super::EntryLocation;

pub enum PreviewContent {
    Text(String),
    Binary(Vec<u8>),
    TextChunk { text: String, done: bool },
    BinaryChunk { data: Vec<u8>, done: bool },
    Image(ImageLocation),
}

#[derive(Clone)]
pub enum ImageLocation {
    Fs(Arc<Path>),
    Container {
        kind: ContainerKind,
        archive_path: path::PathBuf,
        inner_path: String,
    },
    Remote {
        host: String,
        path: String,
    },
}

pub enum PreviewRequest {
    Read {
        id: u64,
        location: EntryLocation,
        max_bytes: Option<usize>,
    },
    ListContainer {
        id: u64,
        kind: ContainerKind,
        archive_path: path::PathBuf,
        max_entries: usize,
    },
}
