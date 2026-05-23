use std::path::PathBuf;

use gpui::{App, AppContext, Application, Bounds, WindowBounds, WindowOptions, px, size};
use gpui_component::Root;

use super::FilemanShell;
use crate::features::{
    clipboard::ClipboardState, file_browser::FilemanAssets, layout::LayoutState,
};

pub fn run(start_path: Option<PathBuf>) {
    Application::new()
        .with_assets(FilemanAssets)
        .run(move |app: &mut App| {
            gpui_component::init(app);
            app.set_global(ClipboardState::default());
            app.set_global(LayoutState::default());
            let start_path = start_path.clone();

            let bounds = Bounds::centered(None, size(px(1180.0), px(720.0)), app);
            app.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    app_id: Some("com.fileman.gpui".to_string()),
                    window_min_size: Some(size(px(820.0), px(520.0))),
                    ..Default::default()
                },
                |window, cx| {
                    window.set_window_title("FileMan GPUI");
                    let shell = cx.new(|cx| FilemanShell::new(cx.focus_handle(), start_path, cx));
                    shell.read(cx).focus_handle.focus(window);
                    cx.new(|cx| Root::new(shell, window, cx))
                },
            )
            .expect("failed to open GPUI window");

            app.activate(true);
        });
}
