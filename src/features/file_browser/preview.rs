use std::path::PathBuf;

use crate::core;

use super::FileTarget;

pub const PREVIEW_MAX_BYTES: usize = 64 * 1024;

#[derive(Clone)]
pub struct PreviewState {
    pub generation: u64,
    pub target: FileTarget,
    pub body: PreviewBody,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PreviewBody {
    Loading,
    Text(String),
    Binary(String),
    Error(String),
}

impl PreviewState {
    pub fn loading(generation: u64, target: FileTarget) -> Self {
        Self {
            generation,
            target,
            body: PreviewBody::Loading,
        }
    }

    pub fn apply_result(&mut self, generation: u64, body: PreviewBody) -> bool {
        if self.generation != generation {
            return false;
        }

        self.body = body;
        true
    }
}

pub fn read_local_preview(path: PathBuf) -> PreviewBody {
    match core::read_bytes_prefix(&path, PREVIEW_MAX_BYTES) {
        Ok(bytes) if core::is_probably_text(&bytes) => {
            PreviewBody::Text(String::from_utf8_lossy(&bytes).into_owned())
        }
        Ok(bytes) => PreviewBody::Binary(core::hexdump(&bytes)),
        Err(error) => PreviewBody::Error(error.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    fn target() -> FileTarget {
        FileTarget {
            path: PathBuf::from("/tmp/source.txt"),
            name: "source.txt".to_string(),
            is_dir: false,
        }
    }

    #[test]
    fn applies_matching_generation_result() {
        let mut preview = PreviewState::loading(3, target());

        assert!(preview.apply_result(3, PreviewBody::Text("hello".to_string())));
        assert_eq!(preview.body, PreviewBody::Text("hello".to_string()));
    }

    #[test]
    fn ignores_stale_generation_result() {
        let mut preview = PreviewState::loading(3, target());

        assert!(!preview.apply_result(2, PreviewBody::Text("stale".to_string())));
        assert_eq!(preview.body, PreviewBody::Loading);
    }
}
