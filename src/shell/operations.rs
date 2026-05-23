use std::path::PathBuf;

use gpui::Context;

use super::FilemanShell;
use crate::{
    core,
    features::file_browser::{BrowserCommandState, FileOperation, PanelSide},
};

impl FilemanShell {
    pub(super) fn load_panel(
        &mut self,
        side: PanelSide,
        path: PathBuf,
        prefer_name: Option<String>,
        cx: &mut Context<Self>,
    ) {
        let generation = {
            let panel = self.panel_mut(side);
            panel.load_generation = panel.load_generation.wrapping_add(1);
            panel.loading = true;
            panel.error = None;
            panel.path = path.clone();
            panel.rows.clear();
            panel.selected_index = 0;
            panel.load_generation
        };
        self.status = format!("loading {}", path.display());

        cx.spawn(async move |shell, cx| {
            let load_path = path.clone();
            let result = cx
                .background_executor()
                .spawn(async move { core::read_fs_directory(&load_path) })
                .await;

            cx.update(|cx| {
                let _ = shell.update(cx, |shell, cx| {
                    shell.apply_loaded_panel(side, path, prefer_name, generation, result);
                    cx.notify();
                });
            })
        })
        .detach();
    }

    fn reload_panels_after_operation(&mut self, cx: &mut Context<Self>) {
        let left = self.primary.path.clone();
        let right = self.secondary.path.clone();
        self.load_panel(PanelSide::Left, left, None, cx);
        self.load_panel(PanelSide::Right, right, None, cx);
    }

    pub(super) fn run_operation(&mut self, operation: FileOperation, cx: &mut Context<Self>) {
        if self.operation_in_flight {
            self.status = "operation already running".to_string();
            return;
        }

        self.operation_in_flight = true;
        self.status = operation.pending_status();

        cx.spawn(async move |shell, cx| {
            let result = cx
                .background_executor()
                .spawn(async move { operation.run() })
                .await;

            cx.update(|cx| {
                let _ = shell.update(cx, |shell, cx| {
                    shell.operation_in_flight = false;
                    match result {
                        Ok(status) => {
                            shell.status = status;
                            shell.active_panel_mut().marked.clear();
                            shell.reload_panels_after_operation(cx);
                        }
                        Err(error) => shell.status = error.to_string(),
                    }
                    cx.notify();
                });
            })
        })
        .detach();
    }

    fn apply_loaded_panel(
        &mut self,
        side: PanelSide,
        path: PathBuf,
        prefer_name: Option<String>,
        generation: u64,
        result: anyhow::Result<Vec<core::DirEntry>>,
    ) {
        let panel = self.panel_mut(side);
        if let Some(status) =
            BrowserCommandState::apply_loaded(panel, path, prefer_name, generation, result)
        {
            self.status = status;
        }
    }
}
