use super::{BrowserCommand, Registry};

pub(super) fn register(registry: &mut Registry) {
    super::bind(registry, "?", "open key map", "General", true, |_| {
        BrowserCommand::OpenHelp
    });
    super::bind(registry, "s", "toggle pane mode", "General", true, |_| {
        BrowserCommand::TogglePaneMode
    });
    super::bind(
        registry,
        "gh",
        "toggle hidden entries",
        "General",
        false,
        |_| BrowserCommand::ToggleHidden,
    );
    super::bind(registry, "w", "switch pane", "General", true, |_| {
        BrowserCommand::SwitchPanel
    });
    super::bind(registry, "r", "reload", "General", true, |_| {
        BrowserCommand::Reload
    });
    super::bind(registry, "R", "reload", "General", false, |_| {
        BrowserCommand::Reload
    });
}
