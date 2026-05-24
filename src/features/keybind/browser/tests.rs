use super::{BrowserCommand, BrowserVimOutcome, apply_browser_vim_char, file_manager_keybinds};
use crate::features::keybind::KeybindArgs;
use crate::features::keybind::VimCommandState;

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
    assert_eq!(command("yf", 1, false), Some(BrowserCommand::CopyFiles));
    assert_eq!(command("dD", 1, false), Some(BrowserCommand::Delete));
    assert_eq!(command("nd", 1, false), Some(BrowserCommand::NewDirectory));
    assert_eq!(command("gp", 1, false), Some(BrowserCommand::Preview));
    assert_eq!(command("zz", 1, false), None);
}

#[test]
fn maps_selection_and_general_commands() {
    assert_eq!(command("v", 3, true), Some(BrowserCommand::ToggleMark(3)));
    assert_eq!(command("V", 1, false), Some(BrowserCommand::ToggleAllMarks));
    assert_eq!(command("uv", 1, false), Some(BrowserCommand::ClearMarks));
    assert_eq!(command("s", 1, false), Some(BrowserCommand::TogglePaneMode));
    assert_eq!(command("gh", 1, false), Some(BrowserCommand::ToggleHidden));
    assert_eq!(command("w", 1, false), Some(BrowserCommand::SwitchPanel));
    assert_eq!(command("r", 1, false), Some(BrowserCommand::Reload));
}

#[test]
fn vim_adapter_emits_pending_status_for_prefixes() {
    let registry = file_manager_keybinds();
    let mut state = VimCommandState::default();

    assert!(matches!(
        apply_browser_vim_char(&mut state, &registry, 'g'),
        BrowserVimOutcome::Pending(ref status) if status == "g"
    ));
    assert!(matches!(
        apply_browser_vim_char(&mut state, &registry, 'g'),
        BrowserVimOutcome::Command {
            command: BrowserCommand::Line(0),
            ref sequence,
        } if sequence == "gg"
    ));
}

#[test]
fn vim_adapter_replays_invalid_prefixed_key_as_new_command() {
    let registry = file_manager_keybinds();
    let mut state = VimCommandState::default();

    assert!(matches!(
        apply_browser_vim_char(&mut state, &registry, 'g'),
        BrowserVimOutcome::Pending(_)
    ));
    assert!(matches!(
        apply_browser_vim_char(&mut state, &registry, 'j'),
        BrowserVimOutcome::Command {
            command: BrowserCommand::Move(1),
            ref sequence,
        } if sequence == "j"
    ));
}

#[test]
fn derives_leader_entries_from_registered_commands() {
    let registry = file_manager_keybinds();
    assert!(registry.is_prefix("y"));
    assert_eq!(registry.leader_continuations("y").len(), 5);
    assert!(
        registry
            .leader_continuations("")
            .iter()
            .any(|item| item.key == "?")
    );
}
