use gpui::Rgba;

const fn rgb(hex: u32) -> Rgba {
    Rgba {
        r: ((hex >> 16) & 0xff) as f32 / 255.0,
        g: ((hex >> 8) & 0xff) as f32 / 255.0,
        b: (hex & 0xff) as f32 / 255.0,
        a: 1.0,
    }
}

const fn rgba(hex: u32, a: f32) -> Rgba {
    Rgba {
        r: ((hex >> 16) & 0xff) as f32 / 255.0,
        g: ((hex >> 8) & 0xff) as f32 / 255.0,
        b: (hex & 0xff) as f32 / 255.0,
        a,
    }
}

pub(crate) const BG_CANVAS: Rgba = rgb(0x0a0a0a);
pub(crate) const BG_PANEL: Rgba = rgb(0x111111);
pub(crate) const BG_PANEL_RAISED: Rgba = rgb(0x171717);
pub(crate) const BORDER_SUBTLE: Rgba = rgb(0x262626);
pub(crate) const BORDER_FOCUS: Rgba = rgb(0x3b82f6);
pub(crate) const TEXT_PRIMARY: Rgba = rgb(0xfafafa);
pub(crate) const TEXT_SECONDARY: Rgba = rgb(0xa1a1aa);
pub(crate) const TEXT_MUTED: Rgba = rgb(0x71717a);
pub(crate) const ROW_HOVER: Rgba = rgb(0x1f1f1f);
pub(crate) const ROW_BORDER_CLEAR: Rgba = rgba(0xffffff, 0.0);
pub(crate) const ROW_SELECTED_ACTIVE: Rgba = rgba(0x4f9cf9, 0.22);
pub(crate) const ROW_SELECTED_ACTIVE_BORDER: Rgba = rgba(0xdbeafe, 0.34);
pub(crate) const ROW_SELECTED_INACTIVE: Rgba = rgba(0xffffff, 0.08);
pub(crate) const ROW_SELECTED_INACTIVE_BORDER: Rgba = rgba(0xffffff, 0.16);
pub(crate) const ROW_MARKED: Rgba = rgba(0x2563eb, 0.16);
pub(crate) const ROW_COPY: Rgba = rgba(0x0891b2, 0.16);
pub(crate) const ROW_MOVE: Rgba = rgba(0xa855f7, 0.16);
pub(crate) const ROW_DELETE: Rgba = rgba(0xef4444, 0.16);
pub(crate) const ACCENT: Rgba = rgb(0x3b82f6);
pub(crate) const ICON_COPY: Rgba = rgb(0x22d3ee);
pub(crate) const ICON_MOVE: Rgba = rgb(0xc084fc);
pub(crate) const ICON_DELETE: Rgba = rgb(0xfb7185);
pub(crate) const ICON_EXECUTABLE: Rgba = rgb(0x22c55e);
pub(crate) const ICON_DIRECTORY: Rgba = rgb(0xfbbf24);
pub(crate) const ICON_SYMLINK: Rgba = rgb(0x38bdf8);
pub(crate) const ICON_SOCKET: Rgba = rgb(0xa78bfa);
pub(crate) const ICON_PIPE: Rgba = rgb(0x22c55e);
pub(crate) const ICON_DEVICE: Rgba = rgb(0xf97316);
pub(crate) const ICON_ARCHIVE: Rgba = rgb(0xeab308);
pub(crate) const ICON_AUDIO: Rgba = rgb(0xec4899);
pub(crate) const ICON_BINARY: Rgba = rgb(0x94a3b8);
pub(crate) const ICON_CODE: Rgba = rgb(0x34d399);
pub(crate) const ICON_IMAGE: Rgba = rgb(0x60a5fa);
pub(crate) const ICON_PDF: Rgba = rgb(0xf87171);
pub(crate) const ICON_TEXT: Rgba = rgb(0xcbd5e1);
pub(crate) const ICON_VIDEO: Rgba = rgb(0xc084fc);
pub(crate) const ICON_FILE: Rgba = rgb(0xa1a1aa);
pub(crate) const ICON_OTHER: Rgba = rgb(0xfacc15);
