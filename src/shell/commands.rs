use gpui::{Context, UpdateGlobal};

use super::StiffShell;
use crate::features::{
    clipboard::{
        ClipboardState, PastePlan, apply_clipboard_effect as apply_clipboard_runtime_effect,
    },
    file_browser::{
        BrowserCommand, BrowserCommandEffect, BrowserCommandOutcome, BrowserCommandState,
        FileOperation, PanelSide, execute_browser_command,
    },
    layout::{LayoutState, PaneMode},
};

impl StiffShell {
    pub(super) fn execute_browser_command(
        &mut self,
        command: BrowserCommand,
        sequence: &str,
        cx: &mut Context<Self>,
    ) -> bool {
        let outcome = {
            let mut state = BrowserCommandState {
                primary: &mut self.primary,
                secondary: &mut self.secondary,
                active: &mut self.active,
                input_mode: &mut self.input_mode,
                pending_confirm: &mut self.pending_confirm,
            };
            execute_browser_command(&mut state, command, sequence)
        };
        self.apply_browser_outcome_with_status_mode(outcome, command.reports_selection(), cx);
        true
    }

    pub(super) fn apply_browser_outcome(
        &mut self,
        outcome: BrowserCommandOutcome,
        cx: &mut Context<Self>,
    ) {
        self.apply_browser_outcome_with_status_mode(outcome, false, cx);
    }

    fn apply_browser_outcome_with_status_mode(
        &mut self,
        outcome: BrowserCommandOutcome,
        debounce_status: bool,
        cx: &mut Context<Self>,
    ) {
        let reveal_active = outcome.reveal_active;

        if let Some(status) = outcome.status {
            if debounce_status {
                self.set_status_debounced(status, cx);
            } else {
                self.status = status;
            }
        }

        match outcome.effect {
            BrowserCommandEffect::None => {}
            BrowserCommandEffect::LoadActive { path, prefer_name } => {
                self.load_panel(self.active, path, prefer_name, cx);
            }
            BrowserCommandEffect::SearchActive { root, query } => {
                self.search_panel(self.active, root, query, cx);
            }
            BrowserCommandEffect::OpenWithSystem(path) => cx.open_with_system(&path),
            BrowserCommandEffect::Clipboard(effect) => self.apply_clipboard_effect(effect, cx),
            BrowserCommandEffect::RunOperation(operation) => self.run_operation(operation, cx),
            BrowserCommandEffect::Preview(target) => self.toggle_preview(target, cx),
            BrowserCommandEffect::TogglePaneMode => {
                let pane_mode =
                    LayoutState::update_global(cx, |layout, _| layout.toggle_pane_mode());
                if pane_mode == PaneMode::Dual {
                    self.ensure_panel_loaded(PanelSide::Left, cx);
                    self.ensure_panel_loaded(PanelSide::Right, cx);
                }
                self.status = format!("{} pane mode", pane_mode.label());
            }
            BrowserCommandEffect::OpenHelp => {
                self.help_popup_open = true;
                self.leader_map_open = false;
                self.status = "help".to_string();
            }
            BrowserCommandEffect::ReloadActive => {
                let path = self.active_panel().path.clone();
                self.load_panel(self.active, path, None, cx);
            }
            BrowserCommandEffect::CancelActiveTask => self.cancel_active_task(),
        }

        if reveal_active {
            self.ensure_panel_loaded(self.active, cx);
            self.active_panel().reveal_selected();
            self.hide_preview_pane();
            self.schedule_preview_preload(cx);
        }
    }

    fn apply_clipboard_effect(
        &mut self,
        effect: crate::features::clipboard::ClipboardEffect,
        cx: &mut Context<Self>,
    ) {
        let outcome = apply_clipboard_runtime_effect(effect, cx);
        if !outcome.status.is_empty() {
            self.status = outcome.status;
        }
        if let Some(paste) = outcome.paste {
            self.handle_paste_plan(paste, cx);
        }
    }

    pub(super) fn handle_paste_plan(&mut self, plan: PastePlan, cx: &mut Context<Self>) {
        match plan {
            PastePlan::Empty => self.status = "clipboard empty".to_string(),
            PastePlan::Cancelled => {
                self.pending_paste = None;
                self.status = "paste cancelled".to_string();
            }
            PastePlan::Conflict { conflict, pending } => {
                self.status = format!(
                    "{} exists: s skip, o overwrite, r rename, c cancel; Shift applies all",
                    conflict.destination.display()
                );
                self.pending_paste = Some((conflict, pending));
            }
            PastePlan::Ready(batch) => {
                self.pending_paste = None;
                if batch.items.is_empty() {
                    self.status = "paste skipped".to_string();
                    return;
                }
                if batch.clear_after_paste {
                    ClipboardState::update_global(cx, |clipboard, _| clipboard.clear());
                }
                self.run_operation(
                    FileOperation::Paste {
                        kind: batch.kind,
                        items: batch.items,
                    },
                    cx,
                );
            }
        }
    }
}
