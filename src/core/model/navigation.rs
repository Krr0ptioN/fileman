use std::path;

use crate::archive::ContainerKind;

use super::{SearchCase, SearchMode};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ActivePanel {
    Left,
    Right,
}

#[derive(Clone)]
pub enum BrowserMode {
    Fs,
    Container {
        kind: ContainerKind,
        archive_path: path::PathBuf,
        cwd: String,
        root: Option<String>,
    },
    Search {
        root: path::PathBuf,
        query: String,
        mode: SearchMode,
        case: SearchCase,
    },
    Remote {
        host: String,
        path: String,
    },
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SortMode {
    Name,
    Date,
    Size,
    Raw,
}
