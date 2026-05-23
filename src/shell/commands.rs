use std::path::PathBuf;

use gpui::{Context, UpdateGlobal};

use super::FilemanShell;
use crate::features::{
    clipboard::{
        ClipboardState, PastePlan, copy_file_contents, copy_target_name, copy_target_path,
        plan_paste, prepare_clipboard,
    },
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

    fn apply_browser_outcome(&mut self, outcome: BrowserCommandOutcome, cx: &mut Context<Self>) {
        if let Some(status) = outcome.status {
            self.status = status;
        }

        match outcome.effect {
            BrowserCommandEffect::None => {}
            BrowserCommandEffect::LoadActive { path, prefer_name } => {
                self.load_panel(self.active, path, prefer_name, cx);
            }
            BrowserCommandEffect::PrepareClipboard { kind, targets } => {
                self.status = ClipboardState::update_global(cx, |clipboard, _| {
                    prepare_clipboard(clipboard, kind, targets)
                });
            }
            BrowserCommandEffect::CopyPath(target) => self.status = copy_target_path(target, cx),
            BrowserCommandEffect::CopyName(target) => self.status = copy_target_name(target, cx),
            BrowserCommandEffect::CopyFileContents(target) => {
                self.status = copy_file_contents(target, cx);
            }
            BrowserCommandEffect::PasteInto(dst_dir) => self.paste_into(dst_dir, cx),
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
    }

    fn paste_into(&mut self, dst_dir: PathBuf, cx: &mut Context<Self>) {
        let plan = ClipboardState::update_global(cx, |clipboard, _| plan_paste(clipboard, dst_dir));
        match plan {
            PastePlan::Empty => self.status = "clipboard empty".to_string(),
            PastePlan::Ready {
                kind,
                targets,
                dst_dir,
                clear_after_paste,
            } => {
                self.run_operation(
                    FileOperation::Paste {
                        kind,
                        targets,
                        dst_dir,
                    },
                    cx,
                );
                if clear_after_paste {
                    ClipboardState::update_global(cx, |clipboard, _| clipboard.clear());
                }
            }
        }
    }
}
