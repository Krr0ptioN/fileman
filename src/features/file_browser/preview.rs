use crate::core;

use super::{FileTarget, path_is_gitignored, rows::FileFormat};

pub const DEFAULT_PREVIEW_VISIBLE_LINES: usize = 48;
pub const DEFAULT_PREVIEW_PRELOAD_LINES: usize = 24;
pub const TEXT_PREVIEW_MAX_BYTES: usize = 128 * 1024;
pub const BINARY_PREVIEW_BYTES_PER_LINE: usize = 16;

#[derive(Clone)]
pub struct PreviewState {
    pub generation: u64,
    pub request: PreviewRequest,
    pub body: PreviewBody,
}

#[derive(Clone)]
pub struct PreviewCacheEntry {
    pub request: PreviewRequest,
    pub body: PreviewBody,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PreviewRequest {
    pub target: FileTarget,
    pub viewport: PreviewViewport,
    pub scroll_line: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PreviewViewport {
    pub visible_lines: usize,
    pub preload_lines: usize,
    pub max_bytes: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PreviewBody {
    Loading { kind: PreviewKind },
    Text(TextPreview),
    Listing(PreviewListing),
    Binary(BinaryPreview),
    Error(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PreviewKind {
    Archive,
    Binary,
    Text,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PreviewPreloadDecision {
    Preload,
    SkipGitIgnored,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TextPreview {
    pub text: String,
    pub first_line: usize,
    pub loaded_lines: usize,
    pub truncated: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PreviewListing {
    pub entries: Vec<String>,
    pub truncated: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BinaryPreview {
    pub text: String,
    pub bytes_read: usize,
    pub truncated: bool,
}

impl TextPreview {
    pub fn loaded_end_line(&self) -> usize {
        self.first_line.saturating_add(self.loaded_lines)
    }

    pub fn contains_line(&self, line: usize) -> bool {
        line >= self.first_line && line < self.loaded_end_line()
    }

    pub fn append(&mut self, next: TextPreview) -> bool {
        if next.first_line != self.loaded_end_line() {
            return false;
        }

        self.text.push_str(&next.text);
        self.loaded_lines = self.loaded_lines.saturating_add(next.loaded_lines);
        self.truncated = next.truncated;
        true
    }
}

impl PreviewBody {
    pub fn merge_extension(&mut self, extension: PreviewBody) -> bool {
        match (self, extension) {
            (&mut Self::Text(ref mut current), Self::Text(next)) => current.append(next),
            _ => false,
        }
    }
}

impl PreviewState {
    pub fn loading(generation: u64, request: PreviewRequest) -> Self {
        let kind = classify_preview(&request.target);
        Self {
            generation,
            request,
            body: PreviewBody::Loading { kind },
        }
    }

    pub fn loaded(generation: u64, request: PreviewRequest, body: PreviewBody) -> Self {
        Self {
            generation,
            request,
            body,
        }
    }

    pub fn target(&self) -> &FileTarget {
        &self.request.target
    }

    pub fn apply_result(&mut self, generation: u64, body: PreviewBody) -> bool {
        if self.generation != generation {
            return false;
        }

        self.body = body;
        true
    }
}

impl PreviewCacheEntry {
    pub fn new(request: PreviewRequest, body: PreviewBody) -> Self {
        Self { request, body }
    }

    pub fn matches_target(&self, target: &FileTarget) -> bool {
        self.request.target.path == target.path
            && self.request.target.name == target.name
            && self.request.target.is_dir == target.is_dir
    }

    pub fn matches_request(&self, request: &PreviewRequest) -> bool {
        self.matches_target(&request.target)
            && self.request.scroll_line == request.scroll_line
            && self.request.viewport == request.viewport
    }
}

impl PreviewRequest {
    pub fn initial(target: FileTarget) -> Self {
        Self {
            target,
            viewport: PreviewViewport::default(),
            scroll_line: 0,
        }
    }

    pub fn line_budget(&self) -> usize {
        self.viewport.visible_lines + self.viewport.preload_lines
    }
}

impl Default for PreviewViewport {
    fn default() -> Self {
        Self {
            visible_lines: DEFAULT_PREVIEW_VISIBLE_LINES,
            preload_lines: DEFAULT_PREVIEW_PRELOAD_LINES,
            max_bytes: TEXT_PREVIEW_MAX_BYTES,
        }
    }
}

pub fn load_local_preview(request: PreviewRequest) -> PreviewBody {
    let handlers: &[&dyn PreviewHandler] = &[
        &ArchivePreviewHandler,
        &TextPreviewHandler,
        &BinaryPreviewHandler,
    ];

    match handlers.iter().find(|handler| handler.matches(&request)) {
        Some(handler) => handler.load(request),
        None => PreviewBody::Error("no preview handler".to_string()),
    }
}

pub fn classify_preview(target: &FileTarget) -> PreviewKind {
    if core::container_kind_from_path(&target.path).is_some() {
        return PreviewKind::Archive;
    }

    match FileFormat::from_path(&target.path) {
        FileFormat::Code | FileFormat::Text => PreviewKind::Text,
        _ => PreviewKind::Binary,
    }
}

pub fn preview_preload_decision(target: &FileTarget) -> PreviewPreloadDecision {
    match path_is_gitignored(&target.path, target.is_dir) {
        true => PreviewPreloadDecision::SkipGitIgnored,
        false => PreviewPreloadDecision::Preload,
    }
}

trait PreviewHandler {
    fn matches(&self, request: &PreviewRequest) -> bool;
    fn load(&self, request: PreviewRequest) -> PreviewBody;
}

struct ArchivePreviewHandler;

impl PreviewHandler for ArchivePreviewHandler {
    fn matches(&self, request: &PreviewRequest) -> bool {
        core::container_kind_from_path(&request.target.path).is_some()
    }

    fn load(&self, request: PreviewRequest) -> PreviewBody {
        let Some(kind) = core::container_kind_from_path(&request.target.path) else {
            return PreviewBody::Error("unsupported archive".to_string());
        };

        match core::read_container_directory(kind, &request.target.path, "") {
            Ok(entries) => {
                let line_budget = request.line_budget();
                let entries = entries
                    .into_iter()
                    .filter(|entry| entry.name != "..")
                    .skip(request.scroll_line)
                    .map(|entry| match entry.is_dir {
                        true => format!("{}/", entry.name),
                        false => entry.name,
                    })
                    .collect::<Vec<_>>();
                let truncated = entries.len() > line_budget;
                PreviewBody::Listing(PreviewListing {
                    entries: entries.into_iter().take(line_budget).collect(),
                    truncated,
                })
            }
            Err(error) => PreviewBody::Error(error.to_string()),
        }
    }
}

struct TextPreviewHandler;

impl PreviewHandler for TextPreviewHandler {
    fn matches(&self, request: &PreviewRequest) -> bool {
        matches!(
            FileFormat::from_path(&request.target.path),
            FileFormat::Code | FileFormat::Text
        )
    }

    fn load(&self, request: PreviewRequest) -> PreviewBody {
        match core::read_text_lines_prefix(
            &request.target.path,
            request.scroll_line,
            request.line_budget(),
            request.viewport.max_bytes,
        ) {
            Ok(preview) => PreviewBody::Text(TextPreview {
                text: preview.text,
                first_line: request.scroll_line,
                loaded_lines: preview.lines_read,
                truncated: preview.truncated,
            }),
            Err(error) => PreviewBody::Error(error.to_string()),
        }
    }
}

struct BinaryPreviewHandler;

impl PreviewHandler for BinaryPreviewHandler {
    fn matches(&self, _request: &PreviewRequest) -> bool {
        true
    }

    fn load(&self, request: PreviewRequest) -> PreviewBody {
        let max_bytes = request.line_budget() * BINARY_PREVIEW_BYTES_PER_LINE;
        match core::read_bytes_prefix(&request.target.path, max_bytes) {
            Ok(bytes) if core::is_probably_text(&bytes) => PreviewBody::Text(TextPreview {
                text: String::from_utf8_lossy(&bytes).into_owned(),
                first_line: request.scroll_line,
                loaded_lines: bytes.iter().filter(|byte| **byte == b'\n').count() + 1,
                truncated: bytes.len() >= max_bytes,
            }),
            Ok(bytes) => {
                let truncated = bytes.len() >= max_bytes;
                PreviewBody::Binary(BinaryPreview {
                    text: core::hexdump(&bytes),
                    bytes_read: bytes.len(),
                    truncated,
                })
            }
            Err(error) => PreviewBody::Error(error.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    use super::*;

    fn target(path: PathBuf) -> FileTarget {
        FileTarget {
            name: path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("source.txt")
                .to_string(),
            path,
            is_dir: false,
        }
    }

    #[test]
    fn applies_matching_generation_result() {
        let request = PreviewRequest::initial(target(PathBuf::from("/tmp/source.txt")));
        let mut preview = PreviewState::loading(3, request);

        assert!(preview.apply_result(
            3,
            PreviewBody::Text(TextPreview {
                text: "hello".to_string(),
                first_line: 0,
                loaded_lines: 1,
                truncated: false,
            })
        ));
        assert!(matches!(preview.body, PreviewBody::Text(_)));
    }

    #[test]
    fn ignores_stale_generation_result() {
        let request = PreviewRequest::initial(target(PathBuf::from("/tmp/source.txt")));
        let mut preview = PreviewState::loading(3, request);

        assert!(!preview.apply_result(
            2,
            PreviewBody::Text(TextPreview {
                text: "stale".to_string(),
                first_line: 0,
                loaded_lines: 1,
                truncated: false,
            })
        ));
        assert_eq!(
            preview.body,
            PreviewBody::Loading {
                kind: PreviewKind::Text
            }
        );
    }

    #[test]
    fn text_handler_reads_only_line_budget() {
        let path =
            std::env::temp_dir().join(format!("stiff-preview-{}-lines.txt", std::process::id()));
        fs::write(&path, "one\ntwo\nthree\nfour\n").unwrap();

        let mut request = PreviewRequest::initial(target(path.clone()));
        request.viewport.visible_lines = 2;
        request.viewport.preload_lines = 1;
        let body = load_local_preview(request);

        fs::remove_file(path).unwrap();

        assert!(matches!(
            body,
            PreviewBody::Text(TextPreview {
                loaded_lines: 3,
                truncated: true,
                ..
            })
        ));
    }

    #[test]
    fn archive_paths_are_routed_to_archive_handler() {
        let target = target(PathBuf::from("/tmp/archive.zip"));

        assert_eq!(classify_preview(&target), PreviewKind::Archive);
    }

    #[test]
    fn text_handler_honors_scroll_line() {
        let path =
            std::env::temp_dir().join(format!("stiff-preview-{}-window.txt", std::process::id()));
        fs::write(&path, "one\ntwo\nthree\nfour\n").unwrap();

        let mut request = PreviewRequest::initial(target(path.clone()));
        request.scroll_line = 2;
        request.viewport.visible_lines = 1;
        request.viewport.preload_lines = 1;
        let body = load_local_preview(request);

        fs::remove_file(path).unwrap();

        assert!(matches!(
            body,
            PreviewBody::Text(TextPreview {
                ref text,
                first_line: 2,
                loaded_lines: 2,
                ..
            }) if text == "three\nfour\n"
        ));
    }

    #[test]
    fn text_preview_appends_adjacent_extension() {
        let mut preview = TextPreview {
            text: "one\ntwo\n".to_string(),
            first_line: 0,
            loaded_lines: 2,
            truncated: true,
        };

        assert!(preview.append(TextPreview {
            text: "three\nfour\n".to_string(),
            first_line: 2,
            loaded_lines: 2,
            truncated: false,
        }));

        assert_eq!(preview.text, "one\ntwo\nthree\nfour\n");
        assert_eq!(preview.loaded_lines, 4);
        assert!(!preview.truncated);
    }

    #[test]
    fn text_preview_rejects_non_adjacent_extension() {
        let mut preview = TextPreview {
            text: "one\ntwo\n".to_string(),
            first_line: 0,
            loaded_lines: 2,
            truncated: true,
        };

        assert!(!preview.append(TextPreview {
            text: "four\n".to_string(),
            first_line: 3,
            loaded_lines: 1,
            truncated: false,
        }));
        assert_eq!(preview.text, "one\ntwo\n");
        assert_eq!(preview.loaded_lines, 2);
        assert!(preview.truncated);
    }

    #[test]
    fn cache_entry_matches_same_target_identity() {
        let target = target(PathBuf::from("/tmp/source.txt"));
        let request = PreviewRequest::initial(target.clone());
        let entry = PreviewCacheEntry::new(
            request,
            PreviewBody::Loading {
                kind: PreviewKind::Text,
            },
        );

        assert!(entry.matches_target(&target));
        assert!(entry.matches_request(&PreviewRequest::initial(target.clone())));
        assert!(!entry.matches_target(&FileTarget {
            path: PathBuf::from("/tmp/other.txt"),
            name: "other.txt".to_string(),
            is_dir: false,
        }));
    }

    #[test]
    fn skips_optimistic_preload_for_gitignored_target() {
        let root = test_repository("ignored");
        fs::write(root.join(".gitignore"), "generated/*\n").unwrap();
        let target = target(root.join("generated/output.log"));

        assert_eq!(
            preview_preload_decision(&target),
            PreviewPreloadDecision::SkipGitIgnored
        );

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn nested_gitignore_whitelist_restores_optimistic_preload() {
        let root = test_repository("whitelist");
        fs::create_dir_all(root.join("generated")).unwrap();
        fs::write(root.join(".gitignore"), "generated/*\n").unwrap();
        fs::write(root.join("generated/.gitignore"), "!output.log\n").unwrap();
        let target = target(root.join("generated/output.log"));

        assert_eq!(
            preview_preload_decision(&target),
            PreviewPreloadDecision::Preload
        );

        fs::remove_dir_all(root).unwrap();
    }

    fn test_repository(suffix: &str) -> PathBuf {
        let root =
            std::env::temp_dir().join(format!("stiff-preview-{}-{}", std::process::id(), suffix));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join(".git")).unwrap();
        fs::create_dir_all(root.join("generated")).unwrap();
        root
    }
}
