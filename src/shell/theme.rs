use std::rc::Rc;

use gpui::App;
use gpui_component::{Theme, ThemeConfig, ThemeConfigColors, ThemeMode};

pub(super) fn install_fileman_theme(cx: &mut App) {
    let mut colors = ThemeConfigColors::default();
    colors.background = Some("#0a0a0a".into());
    colors.foreground = Some("#fafafa".into());
    colors.border = Some("#262626".into());
    colors.primary = Some("#3b82f6".into());
    colors.primary_foreground = Some("#fafafa".into());
    colors.secondary = Some("#171717".into());
    colors.secondary_foreground = Some("#a1a1aa".into());
    colors.secondary_hover = Some("#1f1f1f".into());
    colors.accent = Some("#3b82f6".into());
    colors.accent_foreground = Some("#fafafa".into());
    colors.muted = Some("#171717".into());
    colors.muted_foreground = Some("#71717a".into());
    colors.list = Some("#111111".into());
    colors.list_active = Some("#0f2a4a".into());
    colors.list_active_border = Some("#3b82f6".into());
    colors.list_head = Some("#171717".into());
    colors.list_hover = Some("#1f1f1f".into());
    colors.popover = Some("#171717".into());
    colors.popover_foreground = Some("#fafafa".into());
    colors.title_bar = Some("#171717".into());
    colors.title_bar_border = Some("#262626".into());
    colors.input = Some("#262626".into());
    colors.ring = Some("#3b82f6".into());
    colors.selection = Some("#0f2a4a".into());
    colors.danger = Some("#ef4444".into());
    colors.warning = Some("#f59e0b".into());
    colors.success = Some("#22c55e".into());
    colors.scrollbar = Some("#111111".into());
    colors.scrollbar_thumb = Some("#262626".into());
    colors.scrollbar_thumb_hover = Some("#3b82f6".into());
    colors.window_border = Some("#262626".into());

    let theme = Rc::new(ThemeConfig {
        is_default: true,
        name: "FileMan Dark".into(),
        mode: ThemeMode::Dark,
        font_size: Some(13.0),
        font_family: Some(".SystemUIFont".into()),
        mono_font_family: Some("Berkeley Mono".into()),
        mono_font_size: Some(12.0),
        radius: Some(6),
        radius_lg: Some(8),
        shadow: Some(false),
        colors,
        highlight: None,
    });

    Theme::global_mut(cx).apply_config(&theme);
}
