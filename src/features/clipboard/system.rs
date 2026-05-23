use std::{fmt, path::PathBuf};

use gpui::{ClipboardItem, Context};

pub trait ClipboardWriter {
    fn write_text(&mut self, text: String);
    fn write_files(&mut self, paths: &[PathBuf]) -> Result<(), ClipboardFileError>;
}

#[derive(Debug)]
pub struct ClipboardFileError(String);

impl fmt::Display for ClipboardFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl<T> ClipboardWriter for Context<'_, T> {
    fn write_text(&mut self, text: String) {
        self.write_to_clipboard(ClipboardItem::new_string(text));
    }

    fn write_files(&mut self, paths: &[PathBuf]) -> Result<(), ClipboardFileError> {
        let mut clipboard =
            arboard::Clipboard::new().map_err(|error| ClipboardFileError(error.to_string()))?;
        clipboard
            .set()
            .file_list(paths)
            .map_err(|error| ClipboardFileError(error.to_string()))
    }
}
