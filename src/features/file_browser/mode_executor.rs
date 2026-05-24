use super::{
    BrowserCommandEffect, BrowserCommandOutcome, ConfirmModeAction, FileOperation, InputMode,
    PendingConfirm, RenameModeAction,
};

pub fn apply_rename_action(
    input_mode: &mut InputMode,
    action: RenameModeAction,
) -> Option<BrowserCommandOutcome> {
    match *input_mode {
        InputMode::Rename { .. } => Some(apply_rename_mode_action(input_mode, action)),
        InputMode::NewDirectory { .. } => Some(apply_new_directory_action(input_mode, action)),
        InputMode::QuickJump { .. } => Some(apply_quick_jump_action(input_mode, action)),
        InputMode::Normal => None,
    }
}

fn apply_rename_mode_action(
    input_mode: &mut InputMode,
    action: RenameModeAction,
) -> BrowserCommandOutcome {
    let InputMode::Rename {
        ref target,
        ref mut input,
    } = *input_mode
    else {
        unreachable!("rename mode action requires rename input mode");
    };

    match action {
        RenameModeAction::Cancel => {
            *input_mode = InputMode::Normal;
            BrowserCommandOutcome::status("rename cancelled")
        }
        RenameModeAction::Backspace => {
            input.pop();
            BrowserCommandOutcome::status(format!("rename: {input}"))
        }
        RenameModeAction::Submit => {
            let target = target.clone();
            let new_name = input.trim().to_string();
            *input_mode = InputMode::Normal;
            match new_name.is_empty() || new_name == target.name {
                true => BrowserCommandOutcome::status("rename unchanged"),
                false => BrowserCommandOutcome::effect(BrowserCommandEffect::RunOperation(
                    FileOperation::Rename { target, new_name },
                )),
            }
        }
        RenameModeAction::Insert(ch) => {
            input.push(ch);
            BrowserCommandOutcome::status(format!("rename: {input}"))
        }
        RenameModeAction::Consume => BrowserCommandOutcome::effect(BrowserCommandEffect::None),
    }
}

fn apply_new_directory_action(
    input_mode: &mut InputMode,
    action: RenameModeAction,
) -> BrowserCommandOutcome {
    let InputMode::NewDirectory {
        ref parent,
        ref mut input,
    } = *input_mode
    else {
        unreachable!("new directory action requires new-directory input mode");
    };

    match action {
        RenameModeAction::Cancel => {
            *input_mode = InputMode::Normal;
            BrowserCommandOutcome::status("new directory cancelled")
        }
        RenameModeAction::Backspace => {
            input.pop();
            BrowserCommandOutcome::status(format!("new directory: {input}"))
        }
        RenameModeAction::Submit => {
            let parent = parent.clone();
            let name = input.trim().to_string();
            *input_mode = InputMode::Normal;
            match directory_name_error(name.as_str()) {
                Some(status) => BrowserCommandOutcome::status(status),
                None => BrowserCommandOutcome::effect(BrowserCommandEffect::RunOperation(
                    FileOperation::NewDirectory {
                        path: parent.join(name),
                    },
                )),
            }
        }
        RenameModeAction::Insert(ch) => {
            input.push(ch);
            BrowserCommandOutcome::status(format!("new directory: {input}"))
        }
        RenameModeAction::Consume => BrowserCommandOutcome::effect(BrowserCommandEffect::None),
    }
}

fn directory_name_error(name: &str) -> Option<&'static str> {
    match name {
        "" => Some("new directory unchanged"),
        "." | ".." => Some("invalid directory name"),
        _ if name.contains(std::path::MAIN_SEPARATOR) => Some("invalid directory name"),
        _ => None,
    }
}

fn apply_quick_jump_action(
    input_mode: &mut InputMode,
    action: RenameModeAction,
) -> BrowserCommandOutcome {
    let InputMode::QuickJump {
        ref base,
        ref mut input,
    } = *input_mode
    else {
        unreachable!("quick jump action requires quick-jump input mode");
    };

    match action {
        RenameModeAction::Cancel => {
            *input_mode = InputMode::Normal;
            BrowserCommandOutcome::status("jump cancelled")
        }
        RenameModeAction::Backspace => {
            input.pop();
            BrowserCommandOutcome::status(format!("jump: {input}"))
        }
        RenameModeAction::Submit => {
            let base = base.clone();
            let target = input.trim().to_string();
            *input_mode = InputMode::Normal;
            match target.is_empty() {
                true => BrowserCommandOutcome::status("jump unchanged"),
                false => BrowserCommandOutcome::status_effect(
                    format!("jumping to {target}"),
                    BrowserCommandEffect::LoadActive {
                        path: quick_jump_path(base, target.as_str()),
                        prefer_name: None,
                    },
                ),
            }
        }
        RenameModeAction::Insert(ch) => {
            input.push(ch);
            BrowserCommandOutcome::status(format!("jump: {input}"))
        }
        RenameModeAction::Consume => BrowserCommandOutcome::effect(BrowserCommandEffect::None),
    }
}

fn quick_jump_path(base: std::path::PathBuf, input: &str) -> std::path::PathBuf {
    let path = match input {
        "~" => std::env::var_os("HOME")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|| std::path::PathBuf::from(input)),
        _ if input.starts_with("~/") => std::env::var_os("HOME")
            .map(std::path::PathBuf::from)
            .map(|home| home.join(&input[2..]))
            .unwrap_or_else(|| std::path::PathBuf::from(input)),
        _ => std::path::PathBuf::from(input),
    };

    match path.is_absolute() {
        true => path,
        false => base.join(path),
    }
}

pub fn apply_confirm_action(
    pending_confirm: &mut Option<PendingConfirm>,
    action: ConfirmModeAction,
) -> Option<BrowserCommandOutcome> {
    let confirm = pending_confirm.clone()?;

    match action {
        ConfirmModeAction::Cancel => {
            *pending_confirm = None;
            Some(BrowserCommandOutcome::status("cancelled"))
        }
        ConfirmModeAction::Confirm => {
            *pending_confirm = None;
            Some(confirm_effect(confirm))
        }
        ConfirmModeAction::Consume => {
            Some(BrowserCommandOutcome::effect(BrowserCommandEffect::None))
        }
        ConfirmModeAction::Ignore => None,
    }
}

fn confirm_effect(confirm: PendingConfirm) -> BrowserCommandOutcome {
    match confirm {
        PendingConfirm::Delete(targets) => BrowserCommandOutcome::effect(
            BrowserCommandEffect::RunOperation(FileOperation::Delete { targets }),
        ),
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::features::file_browser::{FileTarget, state::InputMode};

    fn target() -> FileTarget {
        FileTarget {
            path: PathBuf::from("/tmp/old.txt"),
            name: "old.txt".to_string(),
            is_dir: false,
        }
    }

    #[test]
    fn rename_submit_runs_operation_for_changed_name() {
        let mut mode = InputMode::Rename {
            target: target(),
            input: "new.txt".to_string(),
        };

        let outcome = apply_rename_action(&mut mode, RenameModeAction::Submit)
            .expect("rename mode should handle submit");

        assert!(matches!(mode, InputMode::Normal));
        assert!(matches!(
            outcome.effect,
            BrowserCommandEffect::RunOperation(FileOperation::Rename {
                ref target,
                ref new_name,
            }) if target.name == "old.txt" && new_name == "new.txt"
        ));
    }

    #[test]
    fn new_directory_submit_runs_operation_for_valid_name() {
        let mut mode = InputMode::NewDirectory {
            parent: PathBuf::from("/tmp"),
            input: "photos".to_string(),
        };

        let outcome = apply_rename_action(&mut mode, RenameModeAction::Submit)
            .expect("new-directory mode should handle submit");

        assert!(matches!(mode, InputMode::Normal));
        assert!(matches!(
            outcome.effect,
            BrowserCommandEffect::RunOperation(FileOperation::NewDirectory { ref path })
                if path == &PathBuf::from("/tmp/photos")
        ));
    }

    #[test]
    fn new_directory_rejects_empty_and_path_names() {
        let mut empty = InputMode::NewDirectory {
            parent: PathBuf::from("/tmp"),
            input: " ".to_string(),
        };
        let outcome = apply_rename_action(&mut empty, RenameModeAction::Submit)
            .expect("new-directory mode should handle submit");
        assert_eq!(outcome.status.as_deref(), Some("new directory unchanged"));

        let mut nested = InputMode::NewDirectory {
            parent: PathBuf::from("/tmp"),
            input: "nested/path".to_string(),
        };
        let outcome = apply_rename_action(&mut nested, RenameModeAction::Submit)
            .expect("new-directory mode should handle submit");
        assert_eq!(outcome.status.as_deref(), Some("invalid directory name"));
    }

    #[test]
    fn new_directory_cancel_returns_to_normal_mode() {
        let mut mode = InputMode::NewDirectory {
            parent: PathBuf::from("/tmp"),
            input: "new_dir".to_string(),
        };

        let outcome = apply_rename_action(&mut mode, RenameModeAction::Cancel)
            .expect("new-directory mode should handle cancel");

        assert!(matches!(mode, InputMode::Normal));
        assert_eq!(outcome.status.as_deref(), Some("new directory cancelled"));
    }

    #[test]
    fn quick_jump_submit_loads_absolute_path() {
        let mut mode = InputMode::QuickJump {
            base: PathBuf::from("/tmp"),
            input: "/var/log".to_string(),
        };

        let outcome = apply_rename_action(&mut mode, RenameModeAction::Submit)
            .expect("quick-jump mode should handle submit");

        assert!(matches!(mode, InputMode::Normal));
        assert_eq!(outcome.status.as_deref(), Some("jumping to /var/log"));
        assert!(matches!(
            outcome.effect,
            BrowserCommandEffect::LoadActive { ref path, prefer_name: None }
                if path == &PathBuf::from("/var/log")
        ));
    }

    #[test]
    fn quick_jump_submit_loads_relative_path_from_base() {
        let mut mode = InputMode::QuickJump {
            base: PathBuf::from("/tmp"),
            input: "project/src".to_string(),
        };

        let outcome = apply_rename_action(&mut mode, RenameModeAction::Submit)
            .expect("quick-jump mode should handle submit");

        assert_eq!(outcome.status.as_deref(), Some("jumping to project/src"));
        assert!(matches!(
            outcome.effect,
            BrowserCommandEffect::LoadActive { ref path, prefer_name: None }
                if path == &PathBuf::from("/tmp/project/src")
        ));
    }

    #[test]
    fn quick_jump_cancel_and_empty_submit_do_not_load() {
        let mut cancelled = InputMode::QuickJump {
            base: PathBuf::from("/tmp"),
            input: "project".to_string(),
        };
        let outcome = apply_rename_action(&mut cancelled, RenameModeAction::Cancel)
            .expect("quick-jump mode should handle cancel");
        assert!(matches!(cancelled, InputMode::Normal));
        assert_eq!(outcome.status.as_deref(), Some("jump cancelled"));
        assert!(matches!(outcome.effect, BrowserCommandEffect::None));

        let mut empty = InputMode::QuickJump {
            base: PathBuf::from("/tmp"),
            input: " ".to_string(),
        };
        let outcome = apply_rename_action(&mut empty, RenameModeAction::Submit)
            .expect("quick-jump mode should handle submit");
        assert_eq!(outcome.status.as_deref(), Some("jump unchanged"));
        assert!(matches!(outcome.effect, BrowserCommandEffect::None));
    }
}
