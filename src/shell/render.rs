use gpui::prelude::FluentBuilder;
use gpui::{Context, InteractiveElement, IntoElement, ParentElement, Render, Styled, Window};
use gpui_component::v_flex;

use super::FilemanShell;
use crate::features::file_browser::{
    CommandBar, HelpPopup, LeaderMap, PanelLayout, TitleBar, tokens,
};

impl Render for FilemanShell {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let panel_region = PanelLayout::new(
            &self.primary,
            &self.secondary,
            self.active,
            self.pending_confirm.as_ref(),
        );
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

        v_flex()
            .id("fileman-shell")
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(Self::on_key_down))
            .relative()
            .size_full()
            .bg(tokens::BG_CANVAS)
            .text_color(tokens::TEXT_PRIMARY)
            .font_family("Berkeley Mono")
            .child(TitleBar)
            .child(panel_region)
            .child(CommandBar::new(
                self.command_mode_label(cx),
                self.status.as_str(),
            ))
            .when(show_leader_map, |this| {
                this.child(LeaderMap::new(leader_prefix, leader_entries))
            })
            .when(self.help_popup_open, |this| {
                this.child(HelpPopup::new(self.keybinds.help_groups()))
            })
    }
}
