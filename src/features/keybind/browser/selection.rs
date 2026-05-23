use super::{BrowserCommand, Registry};

pub(super) fn register(registry: &mut Registry) {
    super::bind(registry, "v", "toggle mark", "Selection", false, |args| {
        BrowserCommand::ToggleMark(args.count)
    });
    super::bind(
        registry,
        "V",
        "toggle all marks",
        "Selection",
        false,
        |_| BrowserCommand::ToggleAllMarks,
    );
    super::bind(registry, "uv", "clear marks", "Selection", false, |_| {
        BrowserCommand::ClearMarks
    });
    super::bind(registry, "uV", "clear marks", "Selection", false, |_| {
        BrowserCommand::ClearMarks
    });
}
