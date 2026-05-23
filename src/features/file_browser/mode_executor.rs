use super::{
    BrowserCommandEffect, BrowserCommandOutcome, ConfirmModeAction, FileOperation, InputMode,
    PendingConfirm, RenameModeAction,
};

pub fn apply_rename_action(
    input_mode: &mut InputMode,
    action: RenameModeAction,
) -> Option<BrowserCommandOutcome> {
    let InputMode::Rename { target, input } = input_mode else {
        return None;
    };

    Some(match action {
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
    })
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
