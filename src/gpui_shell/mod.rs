use gpui::{
    App, AppContext, Application, Bounds, Context, FocusHandle, InteractiveElement, IntoElement,
    KeyDownEvent, ParentElement, Render, Styled, Window, WindowBounds, WindowOptions, div, px,
    size, uniform_list,
};
use gpui_component::{Root, h_flex, v_flex};

use crate::features::vim_keys::{VimCommandState, VimCommandStep};

pub fn run() {
    Application::new().run(|cx: &mut App| {
        gpui_component::init(cx);

        let bounds = Bounds::centered(None, size(px(1180.0), px(720.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                app_id: Some("com.fileman.gpui".to_string()),
                window_min_size: Some(size(px(820.0), px(520.0))),
                ..Default::default()
            },
            |window, cx| {
                window.set_window_title("FileMan GPUI");
                let shell = cx.new(|cx| FilemanShell::demo(cx.focus_handle()));
                shell.read(cx).focus_handle.focus(window);
                cx.new(|cx| Root::new(shell, window, cx))
            },
        )
        .expect("failed to open GPUI window");

        cx.activate(true);
    });
}

struct FilemanShell {
    left: BrowserPanel,
    right: BrowserPanel,
    active: PanelSide,
    focus_handle: FocusHandle,
    vim_command: VimCommandState,
    status: String,
}

impl FilemanShell {
    fn demo(focus_handle: FocusHandle) -> Self {
        Self {
            left: BrowserPanel {
                side: PanelSide::Left,
                title: "Left",
                path: "~/workspace/fileman",
                selected_index: 2,
                rows: demo_rows(),
            },
            right: BrowserPanel {
                side: PanelSide::Right,
                title: "Right",
                path: "~/workspace/fileman/tests/data",
                selected_index: 1,
                rows: vec![
                    FileRow::dir("basic", "3 items"),
                    FileRow::dir("edit_test", "2 items"),
                    FileRow::file("test_archive.zip", "1.7 KB"),
                ],
            },
            active: PanelSide::Left,
            focus_handle,
            vim_command: VimCommandState::default(),
            status: "normal".to_string(),
        }
    }

    fn on_key_down(&mut self, event: &KeyDownEvent, window: &mut Window, cx: &mut Context<Self>) {
        let Some(ch) = vim_char_from_key(event) else {
            return;
        };

        if self.apply_vim_char(ch) {
            window.prevent_default();
            cx.stop_propagation();
            cx.notify();
        }
    }

    fn apply_vim_char(&mut self, ch: char) -> bool {
        match self.vim_command.push(ch) {
            VimCommandStep::Ignored => false,
            VimCommandStep::Pending => {
                self.status = self
                    .vim_command
                    .display()
                    .unwrap_or_else(|| "normal".to_string());
                true
            }
            VimCommandStep::Execute {
                sequence,
                count,
                explicit_count,
                had_pending,
            } => {
                let handled = self.execute_vim_sequence(sequence.as_str(), count, explicit_count);
                if !handled && had_pending {
                    return self.apply_vim_char(ch);
                }
                handled
            }
        }
    }

    fn execute_vim_sequence(&mut self, sequence: &str, count: usize, explicit_count: bool) -> bool {
        let active = self.active_panel_mut();
        let row_count = active.rows.len();
        if row_count == 0 {
            self.status = "empty".to_string();
            return true;
        }

        match sequence {
            "j" => active.select_relative(count as isize),
            "k" => active.select_relative(-(count as isize)),
            "J" => active.select_relative((count * 8) as isize),
            "K" => active.select_relative(-((count * 8) as isize)),
            "gg" => active.select_line(if explicit_count {
                count.saturating_sub(1)
            } else {
                0
            }),
            "G" => {
                if explicit_count {
                    active.select_line(count.saturating_sub(1));
                } else {
                    active.select_last();
                }
            }
            "0" => active.select_line(0),
            "h" => self.status = "parent".to_string(),
            "l" => {
                let name = self.active_panel().selected_name();
                self.status = format!("open {name}");
            }
            _ => return false,
        }

        if !matches!(sequence, "h" | "l") {
            let selected = self.active_panel().selected_name();
            self.status = format!("{sequence} -> {selected}");
        }
        true
    }

    fn active_panel(&self) -> &BrowserPanel {
        match self.active {
            PanelSide::Left => &self.left,
            PanelSide::Right => &self.right,
        }
    }

    fn active_panel_mut(&mut self) -> &mut BrowserPanel {
        match self.active {
            PanelSide::Left => &mut self.left,
            PanelSide::Right => &mut self.right,
        }
    }
}

impl Render for FilemanShell {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .id("fileman-shell")
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(Self::on_key_down))
            .size_full()
            .bg(tokens::BG_CANVAS)
            .text_color(tokens::TEXT_PRIMARY)
            .font_family("Berkeley Mono")
            .child(render_title_bar())
            .child(
                h_flex()
                    .flex_grow()
                    .gap_2()
                    .p_2()
                    .child(self.left.render(self.active == PanelSide::Left))
                    .child(self.right.render(self.active == PanelSide::Right)),
            )
            .child(render_command_bar(
                self.vim_command.display(),
                self.status.as_str(),
            ))
    }
}

#[derive(Clone)]
struct BrowserPanel {
    side: PanelSide,
    title: &'static str,
    path: &'static str,
    selected_index: usize,
    rows: Vec<FileRow>,
}

impl BrowserPanel {
    fn render(&self, active: bool) -> impl IntoElement + use<> {
        let rows = self.rows.clone();
        let selected_index = self.selected_index;
        let row_count = rows.len();
        let list_id = match self.side {
            PanelSide::Left => "left-rows",
            PanelSide::Right => "right-rows",
        };

        v_flex()
            .flex_1()
            .min_w(px(0.0))
            .h_full()
            .bg(tokens::BG_PANEL)
            .border_1()
            .border_color(if active {
                tokens::BORDER_FOCUS
            } else {
                tokens::BORDER_SUBTLE
            })
            .rounded(px(6.0))
            .overflow_hidden()
            .child(render_panel_header(self, active))
            .child(
                div().flex_grow().child(
                    uniform_list(list_id, row_count, move |range, _, _| {
                        range
                            .map(|ix| render_row(ix, rows[ix].clone(), ix == selected_index))
                            .collect::<Vec<_>>()
                    })
                    .h_full(),
                ),
            )
    }

    fn select_relative(&mut self, delta: isize) {
        if self.rows.is_empty() {
            self.selected_index = 0;
            return;
        }

        self.selected_index = if delta.is_negative() {
            self.selected_index.saturating_sub(delta.unsigned_abs())
        } else {
            self.selected_index
                .saturating_add(delta as usize)
                .min(self.rows.len() - 1)
        };
    }

    fn select_line(&mut self, index: usize) {
        if self.rows.is_empty() {
            self.selected_index = 0;
        } else {
            self.selected_index = index.min(self.rows.len() - 1);
        }
    }

    fn select_last(&mut self) {
        if !self.rows.is_empty() {
            self.selected_index = self.rows.len() - 1;
        }
    }

    fn selected_name(&self) -> &'static str {
        self.rows
            .get(self.selected_index)
            .map(|row| row.name)
            .unwrap_or("<none>")
    }
}

#[derive(Clone)]
struct FileRow {
    kind: RowKind,
    name: &'static str,
    detail: &'static str,
}

impl FileRow {
    fn dir(name: &'static str, detail: &'static str) -> Self {
        Self {
            kind: RowKind::Directory,
            name,
            detail,
        }
    }

    fn file(name: &'static str, detail: &'static str) -> Self {
        Self {
            kind: RowKind::File,
            name,
            detail,
        }
    }
}

#[derive(Clone, Copy)]
enum RowKind {
    Directory,
    File,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PanelSide {
    Left,
    Right,
}

fn render_title_bar() -> impl IntoElement {
    h_flex()
        .h(px(42.0))
        .px_3()
        .items_center()
        .justify_between()
        .bg(tokens::BG_PANEL_RAISED)
        .border_b_1()
        .border_color(tokens::BORDER_SUBTLE)
        .child(
            h_flex()
                .items_center()
                .gap_2()
                .child(div().size(px(10.0)).rounded_full().bg(tokens::ACCENT))
                .child(
                    div()
                        .text_color(tokens::TEXT_PRIMARY)
                        .font_weight(gpui::FontWeight::SEMIBOLD)
                        .child("FileMan"),
                ),
        )
        .child(
            div()
                .text_color(tokens::TEXT_SECONDARY)
                .text_size(px(12.0))
                .child("GPUI shell"),
        )
}

fn render_panel_header(panel: &BrowserPanel, active: bool) -> impl IntoElement {
    v_flex()
        .gap_1()
        .p_3()
        .bg(tokens::BG_PANEL_RAISED)
        .border_b_1()
        .border_color(tokens::BORDER_SUBTLE)
        .child(
            h_flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .font_weight(gpui::FontWeight::SEMIBOLD)
                        .text_color(if active {
                            tokens::TEXT_PRIMARY
                        } else {
                            tokens::TEXT_SECONDARY
                        })
                        .child(panel.title),
                )
                .child(
                    div()
                        .text_size(px(11.0))
                        .text_color(tokens::TEXT_MUTED)
                        .child(format!("{} rows", panel.rows.len())),
                ),
        )
        .child(
            div()
                .text_size(px(12.0))
                .text_color(tokens::TEXT_SECONDARY)
                .child(panel.path),
        )
}

fn render_row(ix: usize, row: FileRow, selected: bool) -> impl IntoElement {
    let kind_label = match row.kind {
        RowKind::Directory => "dir",
        RowKind::File => "file",
    };

    h_flex()
        .id(("file-row", ix))
        .h(px(28.0))
        .px_3()
        .items_center()
        .justify_between()
        .bg(if selected {
            tokens::ROW_SELECTED
        } else {
            tokens::BG_PANEL
        })
        .hover(|style| style.bg(tokens::ROW_HOVER))
        .child(
            h_flex()
                .items_center()
                .gap_2()
                .min_w(px(0.0))
                .child(
                    div()
                        .w(px(36.0))
                        .text_size(px(11.0))
                        .text_color(tokens::TEXT_MUTED)
                        .child(kind_label),
                )
                .child(
                    div()
                        .min_w(px(0.0))
                        .text_color(tokens::TEXT_PRIMARY)
                        .child(row.name),
                ),
        )
        .child(
            div()
                .text_size(px(12.0))
                .text_color(tokens::TEXT_SECONDARY)
                .child(row.detail),
        )
}

fn render_command_bar(command: Option<String>, status: &str) -> impl IntoElement {
    let mode = command.unwrap_or_else(|| "normal".to_string());

    h_flex()
        .h(px(34.0))
        .px_3()
        .items_center()
        .justify_between()
        .bg(tokens::BG_PANEL_RAISED)
        .border_t_1()
        .border_color(tokens::BORDER_SUBTLE)
        .child(
            h_flex()
                .items_center()
                .gap_2()
                .child(command_hint("j/k", "move"))
                .child(command_hint("h/l", "parent/open"))
                .child(command_hint("yy/dd", "copy/move"))
                .child(command_hint("cw", "rename")),
        )
        .child(
            h_flex()
                .items_center()
                .gap_2()
                .child(
                    div()
                        .text_size(px(12.0))
                        .text_color(tokens::ACCENT)
                        .child(mode),
                )
                .child(
                    div()
                        .text_size(px(12.0))
                        .text_color(tokens::TEXT_MUTED)
                        .child(status.to_string()),
                ),
        )
}

fn command_hint(key: &'static str, label: &'static str) -> impl IntoElement {
    h_flex()
        .items_center()
        .gap_1()
        .child(
            div()
                .px_1()
                .rounded(px(3.0))
                .border_1()
                .border_color(tokens::BORDER_SUBTLE)
                .text_color(tokens::ACCENT)
                .text_size(px(11.0))
                .child(key),
        )
        .child(
            div()
                .text_size(px(12.0))
                .text_color(tokens::TEXT_SECONDARY)
                .child(label),
        )
}

fn demo_rows() -> Vec<FileRow> {
    vec![
        FileRow::dir("..", "parent"),
        FileRow::dir("src", "18 items"),
        FileRow::dir("tests", "21 items"),
        FileRow::dir("docs", "3 items"),
        FileRow::dir("themes", "2 items"),
        FileRow::file("Cargo.toml", "2.4 KB"),
        FileRow::file("README.md", "4.2 KB"),
        FileRow::file("CHANGELOG.md", "2.7 KB"),
    ]
}

fn vim_char_from_key(event: &KeyDownEvent) -> Option<char> {
    if event.is_held {
        return None;
    }

    let modifiers = event.keystroke.modifiers;
    if modifiers.control || modifiers.alt || modifiers.platform || modifiers.function {
        return None;
    }

    event
        .keystroke
        .key_char
        .as_deref()
        .unwrap_or(event.keystroke.key.as_str())
        .chars()
        .next()
        .filter(|ch| !ch.is_control())
}

mod tokens {
    use gpui::Rgba;

    const fn rgb(hex: u32) -> Rgba {
        Rgba {
            r: ((hex >> 16) & 0xff) as f32 / 255.0,
            g: ((hex >> 8) & 0xff) as f32 / 255.0,
            b: (hex & 0xff) as f32 / 255.0,
            a: 1.0,
        }
    }

    pub const BG_CANVAS: Rgba = rgb(0x0a0a0a);
    pub const BG_PANEL: Rgba = rgb(0x111111);
    pub const BG_PANEL_RAISED: Rgba = rgb(0x171717);
    pub const BORDER_SUBTLE: Rgba = rgb(0x262626);
    pub const BORDER_FOCUS: Rgba = rgb(0x3b82f6);
    pub const TEXT_PRIMARY: Rgba = rgb(0xfafafa);
    pub const TEXT_SECONDARY: Rgba = rgb(0xa1a1aa);
    pub const TEXT_MUTED: Rgba = rgb(0x71717a);
    pub const ROW_HOVER: Rgba = rgb(0x1f1f1f);
    pub const ROW_SELECTED: Rgba = rgb(0x0f2a4a);
    pub const ACCENT: Rgba = rgb(0x3b82f6);
}
