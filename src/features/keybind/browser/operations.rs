use super::{BrowserCommand, Registry};

pub(super) fn register(registry: &mut Registry) {
    super::bind(
        registry,
        "yy",
        "copy selection",
        "Operations",
        false,
        |_| BrowserCommand::Copy,
    );
    super::bind(registry, "yp", "copy path", "Operations", false, |_| {
        BrowserCommand::CopyPath
    });
    super::bind(registry, "yn", "copy name", "Operations", false, |_| {
        BrowserCommand::CopyName
    });
    super::bind(registry, "yf", "copy file", "Operations", false, |_| {
        BrowserCommand::CopyFiles
    });
    super::bind(
        registry,
        "yc",
        "copy file contents",
        "Operations",
        false,
        |_| BrowserCommand::CopyFileContents,
    );
    super::bind(registry, "dd", "mark for move", "Operations", false, |_| {
        BrowserCommand::MoveSelection
    });
    super::bind(registry, "pp", "paste", "Operations", false, |_| {
        BrowserCommand::Paste
    });
    super::bind(registry, "dD", "delete", "Operations", false, |_| {
        BrowserCommand::Delete
    });
    super::bind(registry, "x", "delete", "Operations", false, |_| {
        BrowserCommand::Delete
    });
    super::bind(registry, "cw", "rename", "Operations", false, |_| {
        BrowserCommand::Rename
    });
    super::bind(registry, "C", "rename", "Operations", false, |_| {
        BrowserCommand::Rename
    });
    super::bind(registry, "nd", "new directory", "Operations", false, |_| {
        BrowserCommand::NewDirectory
    });
    super::bind(
        registry,
        "gp",
        "toggle preview",
        "Operations",
        false,
        |_| BrowserCommand::Preview,
    );
}
