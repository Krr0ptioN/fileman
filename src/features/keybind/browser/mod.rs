mod general;
mod navigation;
mod operations;
mod selection;

use super::{KeybindArgs, KeybindRegistry, KeybindSpec};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BrowserCommand {
    Move(isize),
    MovePage(isize),
    First,
    Last,
    Line(usize),
    OpenParent,
    OpenSelected,
    ToggleMark(usize),
    ToggleAllMarks,
    ClearMarks,
    Copy,
    CopyPath,
    CopyName,
    CopyFileContents,
    MoveSelection,
    Paste,
    Delete,
    Rename,
    TogglePaneMode,
    SwitchPanel,
    Reload,
    OpenHelp,
}

impl BrowserCommand {
    pub fn requires_rows(self) -> bool {
        !matches!(self, Self::OpenParent | Self::SwitchPanel | Self::OpenHelp)
    }
}

type Registry = KeybindRegistry<BrowserCommand>;

pub fn file_manager_keybinds() -> Registry {
    let mut registry = Registry::default();
    navigation::register(&mut registry);
    selection::register(&mut registry);
    operations::register(&mut registry);
    general::register(&mut registry);
    registry
}

fn bind(
    registry: &mut Registry,
    sequence: &'static str,
    name: &'static str,
    group: &'static str,
    general: bool,
    handler: fn(KeybindArgs) -> BrowserCommand,
) {
    registry.register(KeybindSpec {
        sequence,
        name,
        description: None,
        group,
        general,
        handler,
    });
}

#[cfg(test)]
mod tests {
    use super::{BrowserCommand, file_manager_keybinds};
    use crate::features::keybind::KeybindArgs;

    fn command(sequence: &str, count: usize, explicit_count: bool) -> Option<BrowserCommand> {
        file_manager_keybinds().command_for(
            sequence,
            KeybindArgs {
                count,
                explicit_count,
            },
        )
    }

    #[test]
    fn maps_counted_navigation() {
        assert_eq!(command("j", 4, true), Some(BrowserCommand::Move(4)));
        assert_eq!(command("k", 3, true), Some(BrowserCommand::Move(-3)));
        assert_eq!(command("J", 2, true), Some(BrowserCommand::MovePage(16)));
    }

    #[test]
    fn maps_line_navigation() {
        assert_eq!(command("gg", 1, false), Some(BrowserCommand::Line(0)));
        assert_eq!(command("G", 10, true), Some(BrowserCommand::Line(9)));
        assert_eq!(command("G", 1, false), Some(BrowserCommand::Last));
    }

    #[test]
    fn maps_operations() {
        assert_eq!(command("yy", 1, false), Some(BrowserCommand::Copy));
        assert_eq!(command("yp", 1, false), Some(BrowserCommand::CopyPath));
        assert_eq!(command("dD", 1, false), Some(BrowserCommand::Delete));
        assert_eq!(command("zz", 1, false), None);
    }

    #[test]
    fn derives_leader_entries_from_registered_commands() {
        let registry = file_manager_keybinds();
        assert!(registry.is_prefix("y"));
        assert_eq!(registry.leader_continuations("y").len(), 4);
        assert!(
            registry
                .leader_continuations("")
                .iter()
                .any(|item| item.key == "?")
        );
    }
}
