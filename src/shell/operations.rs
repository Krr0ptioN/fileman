use std::{path::PathBuf, time::Duration};

use gpui::{Context, Timer};

use super::{StiffShell, state::ShellPaneFocus};
use crate::{
    core,
    features::file_browser::{
        BrowserCommandState, FileOperation, FileTarget, PanelSide, PreviewBody, PreviewCacheEntry,
        PreviewPreloadDecision, PreviewRequest, PreviewState, load_local_preview,
        preview_preload_decision, read_visible_fs_directory,
    },
};

const PREVIEW_PRELOAD_DELAY: Duration = Duration::from_millis(750);
const PREVIEW_SCROLL_STEP: usize = 1;
const PREVIEW_EXTENSION_BASELINE_LINES: usize = 5;
const PREVIEW_PAGE_EXTENSION_MULTIPLIER: usize = 2;
const STATUS_DEBOUNCE_DELAY: Duration = Duration::from_millis(80);

#[derive(Clone, Copy)]
enum PreviewScrollStrategy {
    Line,
    Page { page_lines: usize },
}

impl StiffShell {
    pub(super) fn load_panel(
        &mut self,
        side: PanelSide,
        path: PathBuf,
        prefer_name: Option<String>,
        cx: &mut Context<Self>,
    ) {
        self.flush_preview_memory();
        let (generation, show_hidden, show_ignored) = {
            let panel = self.panel_mut(side);
            (
                BrowserCommandState::start_loading(panel, path.clone()),
                panel.show_hidden,
                panel.show_ignored,
            )
        };
        self.status = format!("loading {}", path.display());

        cx.spawn(async move |shell, cx| {
            let load_path = path.clone();
            let result = cx
                .background_executor()
                .spawn(
                    async move { read_visible_fs_directory(&load_path, show_hidden, show_ignored) },
                )
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
                            shell.active_panel_mut().clear_marks();
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

    pub(super) fn set_status_debounced(&mut self, status: String, cx: &mut Context<Self>) {
        self.status_debounce_generation = self.status_debounce_generation.wrapping_add(1).max(1);
        let generation = self.status_debounce_generation;
        let previous_status = self.status.clone();

        cx.spawn(async move |shell, cx| {
            Timer::after(STATUS_DEBOUNCE_DELAY).await;

            cx.update(|cx| {
                let _ = shell.update(cx, |shell, cx| {
                    if shell.status_debounce_generation == generation
                        && shell.status == previous_status
                    {
                        shell.status = status;
                        cx.notify();
                    }
                });
            })
        })
        .detach();
    }

    pub(super) fn toggle_preview(&mut self, target: FileTarget, cx: &mut Context<Self>) {
        if self
            .preview
            .as_ref()
            .is_some_and(|preview| preview.target() == &target)
        {
            self.hide_preview_pane();
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
        self.reset_preview_line_extension_sequence();
        if let Some(preload) = self
            .preview_preload
            .as_ref()
            .filter(|preload| preload.matches_target(&target))
        {
            self.preview = Some(PreviewState::loaded(
                generation,
                request,
                preload.body.clone(),
            ));
            self.pane_focus = ShellPaneFocus::Preview;
            self.status = format!("preview {}", target.name);
            return;
        }

        self.status = format!("previewing {}", target.name);
        self.preview = Some(PreviewState::loading(generation, request.clone()));
        self.pane_focus = ShellPaneFocus::Preview;

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

    pub(super) fn hide_preview_pane(&mut self) {
        if let Some(preview) = self.preview.as_ref() {
            self.preview_preload = Some(PreviewCacheEntry::new(
                preview.request.clone(),
                preview.body.clone(),
            ));
        }
        self.preview = None;
        self.pane_focus = ShellPaneFocus::Browser;
        self.pane_focus_prefix = false;
        self.preview_pending_scroll_line = None;
    }

    pub(super) fn flush_preview_memory(&mut self) {
        self.hide_preview_pane();
        self.preview_preload = None;
        self.preview_preload_generation = self.preview_preload_generation.wrapping_add(1).max(1);
        self.preview_generation = self.preview_generation.wrapping_add(1).max(1);
        self.preview_extension_generation =
            self.preview_extension_generation.wrapping_add(1).max(1);
        self.preview_extension_start_line = None;
        self.reset_preview_line_extension_sequence();
    }

    pub(super) fn focus_browser_pane(&mut self) {
        self.pane_focus = ShellPaneFocus::Browser;
        self.pane_focus_prefix = false;
        self.status = "browser pane".to_string();
    }

    pub(super) fn focus_preview_pane(&mut self) {
        self.pane_focus_prefix = false;
        if self.preview.is_some() {
            self.pane_focus = ShellPaneFocus::Preview;
            self.status = "preview pane".to_string();
        } else {
            self.pane_focus = ShellPaneFocus::Browser;
            self.status = "no preview pane".to_string();
        }
    }

    pub(super) fn focus_next_pane(&mut self) {
        self.pane_focus_prefix = false;
        match (self.pane_focus, self.preview.is_some()) {
            (ShellPaneFocus::Browser, true) => self.focus_preview_pane(),
            _ => self.focus_browser_pane(),
        }
    }

    pub(super) fn preview_pane_focused(&self) -> bool {
        self.pane_focus == ShellPaneFocus::Preview && self.preview.is_some()
    }

    pub(super) fn scroll_preview_lines(&mut self, delta: isize, cx: &mut Context<Self>) -> bool {
        self.scroll_preview_by_strategy(delta, PreviewScrollStrategy::Line, cx)
    }

    pub(super) fn scroll_preview_page(&mut self, direction: isize, cx: &mut Context<Self>) -> bool {
        let page = self
            .preview
            .as_ref()
            .map(|preview| preview.request.viewport.visible_lines / 2)
            .unwrap_or(0)
            .max(PREVIEW_SCROLL_STEP);
        self.scroll_preview_by_strategy(
            direction.saturating_mul(page as isize),
            PreviewScrollStrategy::Page { page_lines: page },
            cx,
        )
    }

    fn scroll_preview_by_strategy(
        &mut self,
        delta: isize,
        strategy: PreviewScrollStrategy,
        cx: &mut Context<Self>,
    ) -> bool {
        if !self.preview_pane_focused() {
            return false;
        }

        let Some(preview) = self.preview.as_ref() else {
            return false;
        };
        let mut request = preview.request.clone();
        request.scroll_line = if delta.is_negative() {
            request.scroll_line.saturating_sub(delta.unsigned_abs())
        } else {
            request.scroll_line.saturating_add(delta as usize)
        };
        if request.scroll_line == preview.request.scroll_line {
            return true;
        }

        if self.preview_body_can_cover(&request) {
            if let Some(preview) = self.preview.as_mut() {
                preview.request = request;
            }
            self.maybe_extend_visible_preview(strategy, cx);
        } else if self.preview_can_extend_text() {
            self.preview_pending_scroll_line = Some(request.scroll_line);
            self.maybe_extend_visible_preview(strategy, cx);
        } else {
            self.reload_visible_preview(request, cx);
        }
        true
    }

    fn reload_visible_preview(&mut self, request: PreviewRequest, cx: &mut Context<Self>) {
        self.preview_generation = self.preview_generation.wrapping_add(1).max(1);
        let generation = self.preview_generation;
        self.reset_preview_line_extension_sequence();

        if let Some(preload) = self
            .preview_preload
            .as_ref()
            .filter(|preload| preload.matches_request(&request))
        {
            self.preview = Some(PreviewState::loaded(
                generation,
                preload.request.clone(),
                preload.body.clone(),
            ));
            return;
        }

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

    fn preview_body_can_cover(&self, request: &PreviewRequest) -> bool {
        let Some(preview) = self.preview.as_ref() else {
            return false;
        };
        match preview.body {
            PreviewBody::Text(ref text) => text.contains_line(request.scroll_line),
            PreviewBody::Listing(ref listing) => request.scroll_line < listing.entries.len(),
            _ => false,
        }
    }

    fn preview_can_extend_text(&self) -> bool {
        self.preview.as_ref().is_some_and(
            |preview| matches!(preview.body, PreviewBody::Text(ref text) if text.truncated),
        )
    }

    fn maybe_extend_visible_preview(
        &mut self,
        strategy: PreviewScrollStrategy,
        cx: &mut Context<Self>,
    ) {
        let Some(preview) = self.preview.as_ref() else {
            return;
        };
        let PreviewBody::Text(ref text) = preview.body else {
            return;
        };
        if !text.truncated {
            return;
        }

        let threshold = preview
            .request
            .scroll_line
            .saturating_add(preview.request.viewport.visible_lines)
            .saturating_add(PREVIEW_EXTENSION_BASELINE_LINES);
        let loaded_end = text.loaded_end_line();
        if threshold < loaded_end {
            return;
        }
        if self.preview_extension_start_line == Some(loaded_end) {
            return;
        }

        let mut request = preview.request.clone();
        request.scroll_line = loaded_end;
        request.viewport.visible_lines = self.extension_lines_for_strategy(strategy);
        request.viewport.preload_lines = 0;
        self.extend_visible_preview(request, cx);
    }

    fn extension_lines_for_strategy(&mut self, strategy: PreviewScrollStrategy) -> usize {
        match strategy {
            PreviewScrollStrategy::Line => self.next_preview_line_extension_lines(),
            PreviewScrollStrategy::Page { page_lines } => page_lines
                .saturating_mul(PREVIEW_PAGE_EXTENSION_MULTIPLIER)
                .max(PREVIEW_EXTENSION_BASELINE_LINES),
        }
    }

    fn next_preview_line_extension_lines(&mut self) -> usize {
        let lines = self.preview_line_extension_next.max(1);
        let next = self
            .preview_line_extension_prev
            .saturating_add(self.preview_line_extension_next)
            .max(lines);
        self.preview_line_extension_prev = self.preview_line_extension_next;
        self.preview_line_extension_next = next;
        lines
    }

    fn reset_preview_line_extension_sequence(&mut self) {
        self.preview_line_extension_prev = 1;
        self.preview_line_extension_next = 2;
    }

    fn extend_visible_preview(&mut self, request: PreviewRequest, cx: &mut Context<Self>) {
        self.preview_extension_generation =
            self.preview_extension_generation.wrapping_add(1).max(1);
        let generation = self.preview_extension_generation;
        self.preview_extension_start_line = Some(request.scroll_line);

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
                    if shell.preview_extension_generation != generation {
                        return;
                    }
                    shell.preview_extension_start_line = None;
                    let Some(preview) = shell.preview.as_mut() else {
                        return;
                    };
                    if preview.target() != &request.target {
                        return;
                    }
                    if preview.body.merge_extension(body) {
                        if let Some(pending_scroll_line) = shell.preview_pending_scroll_line {
                            let can_cover_pending = match preview.body {
                                PreviewBody::Text(ref text) => {
                                    text.contains_line(pending_scroll_line)
                                }
                                PreviewBody::Listing(ref listing) => {
                                    pending_scroll_line < listing.entries.len()
                                }
                                _ => false,
                            };
                            if can_cover_pending {
                                preview.request.scroll_line = pending_scroll_line;
                                shell.preview_pending_scroll_line = None;
                            }
                        }
                        shell.preview_preload = Some(PreviewCacheEntry::new(
                            preview.request.clone(),
                            preview.body.clone(),
                        ));
                        cx.notify();
                    }
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
                    async move {
                        match preview_preload_decision(&request.target) {
                            PreviewPreloadDecision::Preload => Some(load_local_preview(request)),
                            PreviewPreloadDecision::SkipGitIgnored => None,
                        }
                    }
                })
                .await;
            let Some(body) = body else {
                return;
            };

            let _ = cx.update(|cx| {
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
            });
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
