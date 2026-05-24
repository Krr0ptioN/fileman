use std::{collections::HashSet, path::PathBuf};

use gpui::{Context, FocusHandle};

use crate::features::{
    file_browser::{
        BrowserCommand, BrowserPanel, InputMode, PanelSide, PendingConfirm, PreviewCacheEntry,
        PreviewState,
    },
    keybind::{HeldNavigation, KeybindRegistry, VimCommandState, file_manager_keybinds},
    layout::LayoutState,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ShellPaneFocus {
    Browser,
    Preview,
}

pub(crate) struct FilemanShell {
    pub(super) primary: BrowserPanel,
    pub(super) secondary: BrowserPanel,
    pub(super) active: PanelSide,
    pub(crate) focus_handle: FocusHandle,
    pub(super) vim_command: VimCommandState,
    pub(super) input_mode: InputMode,
    pub(super) pending_confirm: Option<PendingConfirm>,
    pub(super) held_navigation: HeldNavigation,
    pub(super) keybinds: KeybindRegistry<BrowserCommand>,
    pub(super) help_popup_open: bool,
    pub(super) leader_map_open: bool,
    pub(super) operation_in_flight: bool,
    pub(super) preview: Option<PreviewState>,
    pub(super) preview_generation: u64,
    pub(super) preview_preload: Option<PreviewCacheEntry>,
    pub(super) preview_preload_generation: u64,
    pub(super) pane_focus: ShellPaneFocus,
    pub(super) pane_focus_prefix: bool,
    pub(super) status: String,
}

impl FilemanShell {
    pub(crate) fn new(
        focus_handle: FocusHandle,
        start_path: Option<PathBuf>,
        cx: &mut Context<Self>,
    ) -> Self {
        let start_path = start_path
            .or_else(|| std::env::current_dir().ok())
            .unwrap_or_else(|| PathBuf::from("."));
        let mut shell = Self {
            primary: Self::panel_factory(&start_path, "Primary", PanelSide::Left),
            secondary: Self::panel_factory(&start_path, "Secondary", PanelSide::Right),
            active: PanelSide::Left,
            focus_handle,
            vim_command: VimCommandState::default(),
            input_mode: InputMode::Normal,
            pending_confirm: None,
            held_navigation: HeldNavigation::default(),
            keybinds: file_manager_keybinds(),
            help_popup_open: false,
            leader_map_open: false,
            operation_in_flight: false,
            preview: None,
            preview_generation: 0,
            preview_preload: None,
            preview_preload_generation: 0,
            pane_focus: ShellPaneFocus::Browser,
            pane_focus_prefix: false,
            status: "normal".to_string(),
        };
        shell.load_panel(PanelSide::Left, start_path, None, cx);
        shell
    }

    fn panel_factory(start_path: &PathBuf, title: &'static str, side: PanelSide) -> BrowserPanel {
        BrowserPanel {
            side,
            title,
            path: start_path.clone(),
            selected_index: 0,
            rows: Vec::new(),
            marked: HashSet::new(),
            loading: false,
            error: None,
            load_generation: 0,
            scroll_handle: Default::default(),
        }
    }

    pub(super) fn active_panel(&self) -> &BrowserPanel {
        match self.active {
            PanelSide::Left => &self.primary,
            PanelSide::Right => &self.secondary,
        }
    }

    pub(super) fn active_panel_mut(&mut self) -> &mut BrowserPanel {
        match self.active {
            PanelSide::Left => &mut self.primary,
            PanelSide::Right => &mut self.secondary,
        }
    }

    pub(super) fn panel_mut(&mut self, side: PanelSide) -> &mut BrowserPanel {
        match side {
            PanelSide::Left => &mut self.primary,
            PanelSide::Right => &mut self.secondary,
        }
    }

    pub(super) fn command_mode_label(&self, cx: &Context<Self>) -> String {
        match (&self.input_mode, &self.pending_confirm) {
            (InputMode::Rename { .. }, _) => "rename".to_string(),
            (InputMode::NewDirectory { .. }, _) => "mkdir".to_string(),
            (InputMode::QuickJump { .. }, _) => "jump".to_string(),
            (_, Some(_)) => "confirm".to_string(),
            _ if self.help_popup_open => "keys".to_string(),
            _ if self.leader_map_open => "leader".to_string(),
            _ => self
                .vim_command
                .display()
                .unwrap_or_else(|| cx.global::<LayoutState>().pane_mode().label().to_string()),
        }
    }
}
