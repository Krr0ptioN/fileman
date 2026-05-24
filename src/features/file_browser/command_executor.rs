use crate::features::clipboard::{ClipboardEffect, ClipboardKind};

use super::{
    BrowserCommand, BrowserCommandEffect, BrowserCommandOutcome, BrowserCommandState,
    effective_targets, parent_navigation, selected_navigation, selected_target,
    start_new_directory, start_rename, toggle_all_marks, toggle_marked,
};

pub fn execute_browser_command(
    state: &mut BrowserCommandState<'_>,
    command: BrowserCommand,
    sequence: &str,
) -> BrowserCommandOutcome {
    if command.requires_rows() && !state.has_active_rows() {
        return BrowserCommandOutcome::status("empty");
    }

    let mut outcome = execute_ready_command(state, command);
    if command.reports_selection() {
        outcome.status = Some(format!("{sequence} -> {}", state.selected_name()));
        outcome = outcome.reveal_active();
    }
    outcome
}

fn execute_ready_command(
    state: &mut BrowserCommandState<'_>,
    command: BrowserCommand,
) -> BrowserCommandOutcome {
    match command {
        BrowserCommand::Move(delta) | BrowserCommand::MovePage(delta) => {
            state.active_panel_mut().select_relative(delta);
            BrowserCommandOutcome::effect(BrowserCommandEffect::None)
        }
        BrowserCommand::First => {
            state.active_panel_mut().select_line(0);
            BrowserCommandOutcome::effect(BrowserCommandEffect::None)
        }
        BrowserCommand::Last => {
            state.active_panel_mut().select_last();
            BrowserCommandOutcome::effect(BrowserCommandEffect::None)
        }
        BrowserCommand::Line(line) => {
            state.active_panel_mut().select_line(line);
            BrowserCommandOutcome::effect(BrowserCommandEffect::None)
        }
        BrowserCommand::OpenParent => parent_navigation(state.active_panel()).into_outcome(),
        BrowserCommand::OpenSelected => selected_navigation(state.active_panel()).into_outcome(),
        BrowserCommand::ToggleMark(count) => {
            let marked = toggle_marked(state.active_panel_mut(), count);
            BrowserCommandOutcome::status(format!("{marked} marked")).reveal_active()
        }
        BrowserCommand::ToggleAllMarks => {
            BrowserCommandOutcome::status(toggle_all_marks(state.active_panel_mut()))
        }
        BrowserCommand::ClearMarks => {
            state.clear_marks();
            BrowserCommandOutcome::status("marks cleared")
        }
        BrowserCommand::Copy => clipboard_outcome(state, ClipboardKind::Copy),
        BrowserCommand::MoveSelection => clipboard_outcome(state, ClipboardKind::Move),
        BrowserCommand::CopyPath => BrowserCommandOutcome::effect(BrowserCommandEffect::Clipboard(
            ClipboardEffect::CopyPath(selected_target(state.active_panel())),
        )),
        BrowserCommand::CopyName => BrowserCommandOutcome::effect(BrowserCommandEffect::Clipboard(
            ClipboardEffect::CopyName(selected_target(state.active_panel())),
        )),
        BrowserCommand::CopyFileContents => {
            BrowserCommandOutcome::effect(BrowserCommandEffect::Clipboard(
                ClipboardEffect::CopyFileContents(selected_target(state.active_panel())),
            ))
        }
        BrowserCommand::CopyFiles => {
            BrowserCommandOutcome::effect(BrowserCommandEffect::Clipboard(
                ClipboardEffect::CopyFiles(effective_targets(state.active_panel())),
            ))
        }
        BrowserCommand::Paste => BrowserCommandOutcome::effect(BrowserCommandEffect::Clipboard(
            ClipboardEffect::PasteInto(state.active_panel().path.clone()),
        )),
        BrowserCommand::Delete => {
            let targets = effective_targets(state.active_panel());
            let status = super::prepare_delete(state.pending_confirm, targets);
            BrowserCommandOutcome::status(status)
        }
        BrowserCommand::Rename => {
            let target = selected_target(state.active_panel());
            BrowserCommandOutcome::status(start_rename(state.input_mode, target))
        }
        BrowserCommand::NewDirectory => BrowserCommandOutcome::status(start_new_directory(
            state.input_mode,
            state.active_panel().path.clone(),
        )),
        BrowserCommand::Preview => preview_outcome(state),
        BrowserCommand::TogglePaneMode => {
            BrowserCommandOutcome::effect(BrowserCommandEffect::TogglePaneMode)
        }
        BrowserCommand::ToggleHidden => {
            let panel = state.active_panel_mut();
            panel.show_hidden = !panel.show_hidden;
            let status = match panel.show_hidden {
                true => "showing hidden entries",
                false => "hiding hidden entries",
            };
            BrowserCommandOutcome::status_effect(
                status,
                BrowserCommandEffect::LoadActive {
                    path: panel.path.clone(),
                    prefer_name: panel.selected_row().map(|row| row.name.clone()),
                },
            )
        }
        BrowserCommand::ToggleIgnored => {
            let panel = state.active_panel_mut();
            panel.show_ignored = !panel.show_ignored;
            let status = match panel.show_ignored {
                true => "showing gitignored entries",
                false => "hiding gitignored entries",
            };
            BrowserCommandOutcome::status_effect(
                status,
                BrowserCommandEffect::LoadActive {
                    path: panel.path.clone(),
                    prefer_name: panel.selected_row().map(|row| row.name.clone()),
                },
            )
        }
        BrowserCommand::SwitchPanel => {
            BrowserCommandOutcome::status(state.switch_panel()).reveal_active()
        }
        BrowserCommand::OpenHelp => BrowserCommandOutcome::effect(BrowserCommandEffect::OpenHelp),
        BrowserCommand::Reload => BrowserCommandOutcome::effect(BrowserCommandEffect::ReloadActive),
    }
}

fn clipboard_outcome(
    state: &BrowserCommandState<'_>,
    kind: ClipboardKind,
) -> BrowserCommandOutcome {
    BrowserCommandOutcome::effect(BrowserCommandEffect::Clipboard(ClipboardEffect::Prepare {
        kind,
        targets: effective_targets(state.active_panel()),
    }))
}

fn preview_outcome(state: &BrowserCommandState<'_>) -> BrowserCommandOutcome {
    match selected_target(state.active_panel()) {
        Some(target) => BrowserCommandOutcome::effect(BrowserCommandEffect::Preview(target)),
        None => BrowserCommandOutcome::status("nothing selected"),
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, path::PathBuf};

    use super::*;
    use crate::features::file_browser::{
        rows::{FileFormat, FileRow, RowKind},
        state::{BrowserPanel, InputMode, PanelSide, PendingConfirm},
    };

    fn row(name: &str, is_dir: bool) -> FileRow {
        FileRow {
            kind: if is_dir {
                RowKind::Directory
            } else {
                RowKind::File(FileFormat::Text)
            },
            name: name.to_string(),
            detail: String::new(),
            path: PathBuf::from(format!("/tmp/{name}")),
            is_dir,
            is_executable: false,
        }
    }

    fn panel(side: PanelSide) -> BrowserPanel {
        BrowserPanel {
            side,
            title: side.label(),
            path: PathBuf::from("/tmp"),
            selected_index: 0,
            rows: vec![row("alpha", true), row("beta.txt", false)],
            show_hidden: false,
            show_ignored: false,
            marked: HashSet::new(),
            loading: false,
            error: None,
            load_generation: 0,
            scroll_handle: Default::default(),
        }
    }

    fn with_state(
        primary: &mut BrowserPanel,
        secondary: &mut BrowserPanel,
        active: &mut PanelSide,
        input_mode: &mut InputMode,
        pending_confirm: &mut Option<PendingConfirm>,
        command: BrowserCommand,
        sequence: &str,
    ) -> BrowserCommandOutcome {
        let mut state = BrowserCommandState {
            primary,
            secondary,
            active,
            input_mode,
            pending_confirm,
        };
        execute_browser_command(&mut state, command, sequence)
    }

    #[test]
    fn navigation_commands_mutate_selection_and_request_reveal() {
        let mut primary = panel(PanelSide::Left);
        let mut secondary = panel(PanelSide::Right);
        let mut active = PanelSide::Left;
        let mut input_mode = InputMode::Normal;
        let mut pending_confirm = None;

        let outcome = with_state(
            &mut primary,
            &mut secondary,
            &mut active,
            &mut input_mode,
            &mut pending_confirm,
            BrowserCommand::Move(1),
            "j",
        );

        assert_eq!(primary.selected_index, 1);
        assert_eq!(outcome.status.as_deref(), Some("j -> beta.txt"));
        assert!(outcome.reveal_active);
        assert!(matches!(outcome.effect, BrowserCommandEffect::None));
    }

    #[test]
    fn switch_panel_changes_active_side_and_requests_reveal() {
        let mut primary = panel(PanelSide::Left);
        let mut secondary = panel(PanelSide::Right);
        let mut active = PanelSide::Left;
        let mut input_mode = InputMode::Normal;
        let mut pending_confirm = None;

        let outcome = with_state(
            &mut primary,
            &mut secondary,
            &mut active,
            &mut input_mode,
            &mut pending_confirm,
            BrowserCommand::SwitchPanel,
            "tab",
        );

        assert_eq!(active, PanelSide::Right);
        assert_eq!(outcome.status.as_deref(), Some("active secondary"));
        assert!(outcome.reveal_active);
        assert!(matches!(outcome.effect, BrowserCommandEffect::None));
    }

    #[test]
    fn rename_enters_input_mode_without_runtime_effects() {
        let mut primary = panel(PanelSide::Left);
        let mut secondary = panel(PanelSide::Right);
        let mut active = PanelSide::Left;
        let mut input_mode = InputMode::Normal;
        let mut pending_confirm = None;

        let outcome = with_state(
            &mut primary,
            &mut secondary,
            &mut active,
            &mut input_mode,
            &mut pending_confirm,
            BrowserCommand::Rename,
            "cw",
        );

        assert_eq!(outcome.status.as_deref(), Some("rename: alpha"));
        assert!(!outcome.reveal_active);
        assert!(matches!(outcome.effect, BrowserCommandEffect::None));
        assert!(matches!(
            input_mode,
            InputMode::Rename { ref input, .. } if input == "alpha"
        ));
    }

    #[test]
    fn new_directory_enters_input_mode_without_rows() {
        let mut primary = panel(PanelSide::Left);
        primary.rows.clear();
        let mut secondary = panel(PanelSide::Right);
        let mut active = PanelSide::Left;
        let mut input_mode = InputMode::Normal;
        let mut pending_confirm = None;

        let outcome = with_state(
            &mut primary,
            &mut secondary,
            &mut active,
            &mut input_mode,
            &mut pending_confirm,
            BrowserCommand::NewDirectory,
            "nd",
        );

        assert_eq!(outcome.status.as_deref(), Some("new directory: new_dir"));
        assert!(!outcome.reveal_active);
        assert!(matches!(outcome.effect, BrowserCommandEffect::None));
        assert!(matches!(
            input_mode,
            InputMode::NewDirectory { ref parent, ref input }
                if parent == &PathBuf::from("/tmp") && input == "new_dir"
        ));
    }

    #[test]
    fn toggle_hidden_reloads_empty_panel_and_changes_visibility_mode() {
        let mut primary = panel(PanelSide::Left);
        primary.rows.clear();
        let mut secondary = panel(PanelSide::Right);
        let mut active = PanelSide::Left;
        let mut input_mode = InputMode::Normal;
        let mut pending_confirm = None;

        let outcome = with_state(
            &mut primary,
            &mut secondary,
            &mut active,
            &mut input_mode,
            &mut pending_confirm,
            BrowserCommand::ToggleHidden,
            "gh",
        );

        assert!(primary.show_hidden);
        assert_eq!(outcome.status.as_deref(), Some("showing hidden entries"));
        assert!(matches!(
            outcome.effect,
            BrowserCommandEffect::LoadActive { ref path, prefer_name: None }
                if path == &PathBuf::from("/tmp")
        ));
    }

    #[test]
    fn toggle_ignored_reloads_empty_panel_and_changes_visibility_mode() {
        let mut primary = panel(PanelSide::Left);
        primary.rows.clear();
        let mut secondary = panel(PanelSide::Right);
        let mut active = PanelSide::Left;
        let mut input_mode = InputMode::Normal;
        let mut pending_confirm = None;

        let outcome = with_state(
            &mut primary,
            &mut secondary,
            &mut active,
            &mut input_mode,
            &mut pending_confirm,
            BrowserCommand::ToggleIgnored,
            "gH",
        );

        assert!(primary.show_ignored);
        assert_eq!(
            outcome.status.as_deref(),
            Some("showing gitignored entries")
        );
        assert!(matches!(
            outcome.effect,
            BrowserCommandEffect::LoadActive { ref path, prefer_name: None }
                if path == &PathBuf::from("/tmp")
        ));
    }

    #[test]
    fn delete_prepares_confirmation_without_shell_context() {
        let mut primary = panel(PanelSide::Left);
        let mut secondary = panel(PanelSide::Right);
        let mut active = PanelSide::Left;
        let mut input_mode = InputMode::Normal;
        let mut pending_confirm = None;

        let outcome = with_state(
            &mut primary,
            &mut secondary,
            &mut active,
            &mut input_mode,
            &mut pending_confirm,
            BrowserCommand::Delete,
            "dD",
        );

        assert_eq!(
            outcome.status.as_deref(),
            Some("delete 1 item(s)? y/enter to confirm")
        );
        assert!(!outcome.reveal_active);
        assert!(matches!(outcome.effect, BrowserCommandEffect::None));
        assert!(matches!(
            pending_confirm,
            Some(PendingConfirm::Delete(ref targets)) if targets.len() == 1
        ));
    }

    #[test]
    fn opening_directory_returns_load_effect() {
        let mut primary = panel(PanelSide::Left);
        let mut secondary = panel(PanelSide::Right);
        let mut active = PanelSide::Left;
        let mut input_mode = InputMode::Normal;
        let mut pending_confirm = None;

        let outcome = with_state(
            &mut primary,
            &mut secondary,
            &mut active,
            &mut input_mode,
            &mut pending_confirm,
            BrowserCommand::OpenSelected,
            "l",
        );

        assert!(!outcome.reveal_active);
        assert!(matches!(
            outcome.effect,
            BrowserCommandEffect::LoadActive { ref path, prefer_name: None }
                if path == &PathBuf::from("/tmp/alpha")
        ));
    }

    #[test]
    fn preview_returns_selected_file_effect() {
        let mut primary = panel(PanelSide::Left);
        primary.selected_index = 1;
        let mut secondary = panel(PanelSide::Right);
        let mut active = PanelSide::Left;
        let mut input_mode = InputMode::Normal;
        let mut pending_confirm = None;

        let outcome = with_state(
            &mut primary,
            &mut secondary,
            &mut active,
            &mut input_mode,
            &mut pending_confirm,
            BrowserCommand::Preview,
            "gp",
        );

        assert!(!outcome.reveal_active);
        assert!(matches!(
            outcome.effect,
            BrowserCommandEffect::Preview(ref target)
                if target.name == "beta.txt" && target.path == PathBuf::from("/tmp/beta.txt")
        ));
    }

    #[test]
    fn preview_returns_selected_directory_effect_for_shell_toggle() {
        let mut primary = panel(PanelSide::Left);
        let mut secondary = panel(PanelSide::Right);
        let mut active = PanelSide::Left;
        let mut input_mode = InputMode::Normal;
        let mut pending_confirm = None;

        let outcome = with_state(
            &mut primary,
            &mut secondary,
            &mut active,
            &mut input_mode,
            &mut pending_confirm,
            BrowserCommand::Preview,
            "gp",
        );

        assert!(!outcome.reveal_active);
        assert!(matches!(
            outcome.effect,
            BrowserCommandEffect::Preview(ref target) if target.name == "alpha" && target.is_dir
        ));
    }
}
