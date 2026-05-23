use crate::features::clipboard::ClipboardKind;

use super::{
    BrowserCommand, BrowserCommandEffect, BrowserCommandOutcome, BrowserCommandState,
    effective_targets, parent_navigation, selected_navigation, selected_target, start_rename,
    toggle_all_marks, toggle_marked,
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
        BrowserCommand::OpenParent => navigation_outcome(parent_navigation(state.active_panel())),
        BrowserCommand::OpenSelected => {
            navigation_outcome(selected_navigation(state.active_panel()))
        }
        BrowserCommand::ToggleMark(count) => {
            let marked = toggle_marked(state.active_panel_mut(), count);
            BrowserCommandOutcome::status(format!("{marked} marked"))
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
        BrowserCommand::CopyPath => BrowserCommandOutcome::effect(BrowserCommandEffect::CopyPath(
            selected_target(state.active_panel()),
        )),
        BrowserCommand::CopyName => BrowserCommandOutcome::effect(BrowserCommandEffect::CopyName(
            selected_target(state.active_panel()),
        )),
        BrowserCommand::CopyFileContents => BrowserCommandOutcome::effect(
            BrowserCommandEffect::CopyFileContents(selected_target(state.active_panel())),
        ),
        BrowserCommand::Paste => BrowserCommandOutcome::effect(BrowserCommandEffect::PasteInto(
            state.active_panel().path.clone(),
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
        BrowserCommand::TogglePaneMode => {
            BrowserCommandOutcome::effect(BrowserCommandEffect::TogglePaneMode)
        }
        BrowserCommand::SwitchPanel => BrowserCommandOutcome::status(state.switch_panel()),
        BrowserCommand::OpenHelp => BrowserCommandOutcome::effect(BrowserCommandEffect::OpenHelp),
        BrowserCommand::Reload => BrowserCommandOutcome::effect(BrowserCommandEffect::ReloadActive),
    }
}

fn clipboard_outcome(
    state: &BrowserCommandState<'_>,
    kind: ClipboardKind,
) -> BrowserCommandOutcome {
    BrowserCommandOutcome::effect(BrowserCommandEffect::PrepareClipboard {
        kind,
        targets: effective_targets(state.active_panel()),
    })
}

fn navigation_outcome(navigation: super::PanelNavigation) -> BrowserCommandOutcome {
    match navigation {
        super::PanelNavigation::Load { path, prefer_name } => {
            BrowserCommandOutcome::effect(BrowserCommandEffect::LoadActive { path, prefer_name })
        }
        super::PanelNavigation::Status(status) => BrowserCommandOutcome::status(status),
    }
}
