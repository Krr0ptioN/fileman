use gpui::{ClipboardItem, Context};

pub trait ClipboardWriter {
    fn write_text(&mut self, text: String);
}

impl<T> ClipboardWriter for Context<'_, T> {
    fn write_text(&mut self, text: String) {
        self.write_to_clipboard(ClipboardItem::new_string(text));
    }
}
