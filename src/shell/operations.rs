use std::path::PathBuf;

use gpui::Context;

use super::FilemanShell;
use crate::{
    core,
    features::file_browser::{
        BrowserCommandState, FileOperation, FileTarget, PanelSide, PreviewRequest, PreviewState,
        load_local_preview,
    },
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
            BrowserCommandState::start_loading(panel, path.clone())
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

    pub(super) fn ensure_panel_loaded(&mut self, side: PanelSide, cx: &mut Context<Self>) {
        let panel = self.panel_mut(side);
        if panel.load_generation != 0 || panel.loading {
            return;
        }

        let path = panel.path.clone();
        self.load_panel(side, path, None, cx);
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

    pub(super) fn toggle_preview(&mut self, target: FileTarget, cx: &mut Context<Self>) {
        if self.preview.is_some() {
            self.preview = None;
            self.status = "preview closed".to_string();
            return;
        }

        if target.is_dir {
            self.status = "cannot preview directory".to_string();
            return;
        }

        self.preview_generation = self.preview_generation.wrapping_add(1).max(1);
        let generation = self.preview_generation;
        let request = PreviewRequest::initial(target.clone());
        self.status = format!("previewing {}", target.name);
        self.preview = Some(PreviewState::loading(generation, request.clone()));

        cx.spawn(async move |shell, cx| {
            let body = cx
                .background_executor()
                .spawn(async move { load_local_preview(request) })
                .await;

            cx.update(|cx| {
                let _ = shell.update(cx, |shell, cx| {
                    if let Some(preview) = shell.preview.as_mut()
                        && preview.apply_result(generation, body)
                    {
                        shell.status = format!("preview {}", preview.target().name);
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
            self.panel_mut(side).reveal_selected();
        }
    }
}
