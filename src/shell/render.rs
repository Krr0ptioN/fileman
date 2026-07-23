use gpui::prelude::FluentBuilder;
use gpui::{Context, InteractiveElement, IntoElement, ParentElement, Render, Styled, Window, px};
use gpui_component::{h_flex, v_flex};

use super::StiffShell;
use crate::features::{
    file_browser::{
        CommandBar, FilePanel, HelpPopup, LayoutVariant, LeaderMap, PanelLayout, PreviewPanel,
        TitleBar, tokens,
    },
    layout::PaneMode,
};

impl Render for StiffShell {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let panel_region = match self.preview.as_ref() {
            Some(preview) => {
                let preview_active = self.preview_pane_focused();
                let browser_active = !preview_active;
                match LayoutVariant::resolve(window.viewport_size(), PaneMode::Dual) {
                    LayoutVariant::SingleActive | LayoutVariant::DualStacked => v_flex()
                        .flex_grow()
                        .min_h(px(0.0))
                        .gap_2()
                        .p_2()
                        .child(FilePanel::new(
                            self.active_panel(),
                            browser_active,
                            (
                                self.pane(self.active).active_number(),
                                self.pane(self.active).tab_count(),
                            ),
                            self.pending_confirm.as_ref(),
                        ))
                        .child(PreviewPanel::new(preview, preview_active))
                        .into_any_element(),
                    LayoutVariant::DualSplit => h_flex()
                        .flex_grow()
                        .min_h(px(0.0))
                        .gap_2()
                        .p_2()
                        .child(FilePanel::new(
                            self.active_panel(),
                            browser_active,
                            (
                                self.pane(self.active).active_number(),
                                self.pane(self.active).tab_count(),
                            ),
                            self.pending_confirm.as_ref(),
                        ))
                        .child(PreviewPanel::new(preview, preview_active))
                        .into_any_element(),
                }
            }
            None => PanelLayout::new(
                self.primary.active(),
                self.secondary.active(),
                (self.primary.active_number(), self.primary.tab_count()),
                (self.secondary.active_number(), self.secondary.tab_count()),
                self.active,
                self.pending_confirm.as_ref(),
            )
            .into_any_element(),
        };
        let leader_prefix = match self.leader_map_open {
            true => String::new(),
            false => self.vim_command.pending.clone(),
        };
        let leader_entries = match (
            self.help_popup_open,
            self.leader_map_open,
            leader_prefix.is_empty(),
        ) {
            (true, _, _) | (false, false, true) => Vec::new(),
            _ => self.keybinds.leader_continuations(leader_prefix.as_str()),
        };
        let show_leader_map = !leader_entries.is_empty();
        let task_status = self.task_queue.status_line();
        let status = match task_status.is_empty() {
            true => self.status.clone(),
            false => format!("{} | {task_status}", self.status),
        };

        v_flex()
            .id("stiff-shell")
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(Self::on_key_down))
            .relative()
            .size_full()
            .bg(tokens::BG_CANVAS)
            .text_color(tokens::TEXT_PRIMARY)
            .font_family("Berkeley Mono")
            .child(TitleBar)
            .child(panel_region)
            .child(CommandBar::new(self.command_mode_label(cx), status))
            .when(show_leader_map, |this| {
                this.child(LeaderMap::new(leader_prefix, leader_entries))
            })
            .when(self.help_popup_open, |this| {
                this.child(HelpPopup::new(self.keybinds.help_groups()))
            })
    }
}
