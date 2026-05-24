use gpui::{Context, UpdateGlobal};

use super::FilemanShell;
use crate::features::{
    clipboard::apply_clipboard_effect as apply_clipboard_runtime_effect,
    file_browser::{
        BrowserCommand, BrowserCommandEffect, BrowserCommandOutcome, BrowserCommandState,
        FileOperation, execute_browser_command,
    },
    layout::LayoutState,
};

impl FilemanShell {
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
        self.apply_browser_outcome(outcome, cx);
        true
    }

    pub(super) fn apply_browser_outcome(
        &mut self,
        outcome: BrowserCommandOutcome,
        cx: &mut Context<Self>,
    ) {
        let reveal_active = outcome.reveal_active;

        if let Some(status) = outcome.status {
            self.status = status;
        }

        match outcome.effect {
            BrowserCommandEffect::None => {}
            BrowserCommandEffect::LoadActive { path, prefer_name } => {
                self.load_panel(self.active, path, prefer_name, cx);
            }
            BrowserCommandEffect::OpenWithSystem(path) => cx.open_with_system(&path),
            BrowserCommandEffect::Clipboard(effect) => self.apply_clipboard_effect(effect, cx),
            BrowserCommandEffect::RunOperation(operation) => self.run_operation(operation, cx),
            BrowserCommandEffect::TogglePaneMode => {
                let pane_mode =
                    LayoutState::update_global(cx, |layout, _| layout.toggle_pane_mode());
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
        }

        if reveal_active {
            self.active_panel().reveal_selected();
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
            self.run_operation(
                FileOperation::Paste {
                    kind: paste.kind,
                    targets: paste.targets,
                    dst_dir: paste.dst_dir,
                },
                cx,
            );
        }
    }
}
