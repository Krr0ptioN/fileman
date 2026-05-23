use super::BrowserCommand;
use super::target::BrowserCommandExecutor;
use crate::features::keybind::KeybindArgs;

pub fn execute_browser_sequence<T, Cx>(
    target: &mut T,
    sequence: &str,
    count: usize,
    explicit_count: bool,
    cx: &mut Cx,
) -> bool
where
    T: BrowserCommandExecutor<Cx>,
{
    let Some(command) = target.command_for(
        sequence,
        KeybindArgs {
            count,
            explicit_count,
        },
    ) else {
        return false;
    };

    if command.requires_rows() && !target.has_active_rows() {
        target.set_status("empty".to_string());
        return true;
    }

    execute_browser_command(target, command, cx);
    if command.reports_selection() {
        target.set_status(format!("{sequence} -> {}", target.selected_name()));
    }
    true
}

fn execute_browser_command<T, Cx>(target: &mut T, command: BrowserCommand, cx: &mut Cx)
where
    T: BrowserCommandExecutor<Cx>,
{
    match command {
        BrowserCommand::Move(delta) | BrowserCommand::MovePage(delta) => {
            target.select_relative(delta)
        }
        BrowserCommand::First => target.select_first(),
        BrowserCommand::Last => target.select_last(),
        BrowserCommand::Line(line) => target.select_line(line),
        BrowserCommand::OpenParent => {
            let _ = target.open_parent(cx);
        }
        BrowserCommand::OpenSelected => {
            let _ = target.open_selected(cx);
        }
        BrowserCommand::ToggleMark(count) => {
            let marked = target.toggle_marked(count);
            target.set_status(format!("{marked} marked"));
        }
        BrowserCommand::ToggleAllMarks => {
            let status = target.toggle_all_marks();
            target.set_status(status);
        }
        BrowserCommand::ClearMarks => {
            target.clear_marks();
            target.set_status("marks cleared".to_string());
        }
        BrowserCommand::Copy => {
            let _ = target.prepare_copy(cx);
        }
        BrowserCommand::CopyPath => {
            let _ = target.copy_path(cx);
        }
        BrowserCommand::CopyName => {
            let _ = target.copy_name(cx);
        }
        BrowserCommand::CopyFileContents => {
            let _ = target.copy_file_contents(cx);
        }
        BrowserCommand::MoveSelection => {
            let _ = target.prepare_move(cx);
        }
        BrowserCommand::Paste => {
            let _ = target.paste(cx);
        }
        BrowserCommand::Delete => {
            let _ = target.prepare_delete();
        }
        BrowserCommand::Rename => {
            let _ = target.start_rename();
        }
        BrowserCommand::TogglePaneMode => target.toggle_pane_mode(cx),
        BrowserCommand::SwitchPanel => {
            let _ = target.switch_panel();
        }
        BrowserCommand::OpenHelp => {
            let _ = target.open_help();
        }
        BrowserCommand::Reload => target.reload(cx),
    }
}
