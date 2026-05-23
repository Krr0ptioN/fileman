use super::BrowserCommand;
use crate::features::keybind::KeybindArgs;

pub trait BrowserCommandExecutor<Cx> {
    fn command_for(&self, sequence: &str, args: KeybindArgs) -> Option<BrowserCommand>;
    fn has_active_rows(&self) -> bool;
    fn selected_name(&self) -> String;
    fn set_status(&mut self, status: String);
    fn select_relative(&mut self, delta: isize);
    fn select_first(&mut self);
    fn select_last(&mut self);
    fn select_line(&mut self, line: usize);
    fn open_parent(&mut self, cx: &mut Cx) -> bool;
    fn open_selected(&mut self, cx: &mut Cx) -> bool;
    fn toggle_marked(&mut self, count: usize) -> usize;
    fn toggle_all_marks(&mut self) -> String;
    fn clear_marks(&mut self);
    fn prepare_copy(&mut self, cx: &mut Cx) -> bool;
    fn copy_path(&mut self, cx: &mut Cx) -> bool;
    fn copy_name(&mut self, cx: &mut Cx) -> bool;
    fn copy_file_contents(&mut self, cx: &mut Cx) -> bool;
    fn prepare_move(&mut self, cx: &mut Cx) -> bool;
    fn paste(&mut self, cx: &mut Cx) -> bool;
    fn prepare_delete(&mut self) -> bool;
    fn start_rename(&mut self) -> bool;
    fn toggle_pane_mode(&mut self, cx: &mut Cx);
    fn switch_panel(&mut self) -> bool;
    fn open_help(&mut self) -> bool;
    fn reload(&mut self, cx: &mut Cx);
}
