use std::{path::PathBuf, time::Duration};

use gpui::{Context, Timer};

use super::FilemanShell;
use crate::{
    core,
    features::file_browser::{
        BrowserCommandState, FileOperation, FileTarget, PanelSide, PreviewCacheEntry,
        PreviewRequest, PreviewState, load_local_preview,
    },
};

const PREVIEW_PRELOAD_DELAY: Duration = Duration::from_millis(750);

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
                    shell.schedule_preview_preload(cx);
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
        if let Some(preload) = self
            .preview_preload
            .as_ref()
            .filter(|preload| preload.matches_target(&target))
        {
            self.preview = Some(PreviewState::loaded(
                generation,
                preload.request.clone(),
                preload.body.clone(),
            ));
            self.status = format!("preview {}", target.name);
            return;
        }

        let request = PreviewRequest::initial(target.clone());
        self.status = format!("previewing {}", target.name);
        self.preview = Some(PreviewState::loading(generation, request.clone()));

        cx.spawn(async move |shell, cx| {
            let body = cx
                .background_executor()
                .spawn({
                    let request = request.clone();
                    async move { load_local_preview(request) }
                })
                .await;

            cx.update(|cx| {
                let _ = shell.update(cx, |shell, cx| {
                    let cache_entry = PreviewCacheEntry::new(request, body.clone());
                    shell.preview_preload = Some(cache_entry);
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

    pub(super) fn schedule_preview_preload(&mut self, cx: &mut Context<Self>) {
        let Some(target) = self.selected_preview_target() else {
            return;
        };
        if target.is_dir {
            return;
        }
        if self
            .preview_preload
            .as_ref()
            .is_some_and(|preload| preload.matches_target(&target))
        {
            return;
        }

        self.preview_preload_generation = self.preview_preload_generation.wrapping_add(1).max(1);
        let generation = self.preview_preload_generation;

        cx.spawn(async move |shell, cx| {
            Timer::after(PREVIEW_PRELOAD_DELAY).await;

            cx.update(|cx| {
                let _ = shell.update(cx, |shell, cx| {
                    shell.start_preview_preload(generation, target, cx);
                });
            })
        })
        .detach();
    }

    fn start_preview_preload(
        &mut self,
        generation: u64,
        target: FileTarget,
        cx: &mut Context<Self>,
    ) {
        if generation != self.preview_preload_generation {
            return;
        }
        if !self
            .selected_preview_target()
            .is_some_and(|selected| selected == target)
        {
            return;
        }

        let request = PreviewRequest::initial(target);
        cx.spawn(async move |shell, cx| {
            let body = cx
                .background_executor()
                .spawn({
                    let request = request.clone();
                    async move { load_local_preview(request) }
                })
                .await;

            cx.update(|cx| {
                let _ = shell.update(cx, |shell, cx| {
                    if generation != shell.preview_preload_generation {
                        return;
                    }

                    let entry = PreviewCacheEntry::new(request, body);
                    if let Some(preview) = shell.preview.as_mut()
                        && entry.matches_target(preview.target())
                        && matches!(
                            preview.body,
                            crate::features::file_browser::PreviewBody::Loading { .. }
                        )
                    {
                        preview.body = entry.body.clone();
                        shell.status = format!("preview {}", preview.target().name);
                    }
                    shell.preview_preload = Some(entry);
                    cx.notify();
                });
            })
        })
        .detach();
    }

    fn selected_preview_target(&self) -> Option<FileTarget> {
        self.active_panel().selected_row().map(FileTarget::from_row)
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
