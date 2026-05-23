use gpui::KeyDownEvent;

use super::{HelpAction, command_char_from_key, help_action};

pub enum KeyCommandAction {
    HandledResetNavigation,
    HandledKeepNavigation,
    Ignored,
}

pub trait AppKeyHandler<Cx> {
    fn modal_key(&mut self, event: &KeyDownEvent, cx: &mut Cx) -> bool;
    fn control_key(&mut self, event: &KeyDownEvent) -> bool;
    fn cancel_key(&mut self, event: &KeyDownEvent) -> bool;
    fn help_key(&mut self, action: HelpAction) -> bool;
    fn help_open(&self) -> bool;
    fn leader_open(&self) -> bool;
    fn open_leader(&mut self);
    fn close_leader(&mut self);
    fn has_pending_vim(&self) -> bool;
    fn navigation_key(&mut self, event: &KeyDownEvent) -> bool;
    fn vim_char(&mut self, ch: char, cx: &mut Cx) -> bool;
}

pub fn handle_key_command<H, Cx>(
    handler: &mut H,
    event: &KeyDownEvent,
    cx: &mut Cx,
) -> KeyCommandAction
where
    H: AppKeyHandler<Cx>,
{
    let reset_navigation = handler.modal_key(event, cx)
        || handler.control_key(event)
        || handler.cancel_key(event)
        || handle_help_key(handler, event)
        || handle_leader_key(handler, event, cx);

    match reset_navigation {
        true => KeyCommandAction::HandledResetNavigation,
        false if handler.navigation_key(event) => KeyCommandAction::HandledKeepNavigation,
        false if handle_vim_key(handler, event, cx) => KeyCommandAction::HandledResetNavigation,
        false => KeyCommandAction::Ignored,
    }
}

fn handle_help_key<H, Cx>(handler: &mut H, event: &KeyDownEvent) -> bool
where
    H: AppKeyHandler<Cx>,
{
    match help_action(event, handler.help_open()) {
        Some(action) => handler.help_key(action),
        None => handler.help_open(),
    }
}

fn handle_leader_key<H, Cx>(handler: &mut H, event: &KeyDownEvent, cx: &mut Cx) -> bool
where
    H: AppKeyHandler<Cx>,
{
    match leader_key_action(handler, event) {
        LeaderKeyAction::Open => {
            handler.open_leader();
            true
        }
        LeaderKeyAction::CloseAndApply(ch) => {
            handler.close_leader();
            ch.is_none_or(|ch| handler.vim_char(ch, cx))
        }
        LeaderKeyAction::ConsumeOpen => true,
        LeaderKeyAction::Ignore => false,
    }
}

fn handle_vim_key<H, Cx>(handler: &mut H, event: &KeyDownEvent, cx: &mut Cx) -> bool
where
    H: AppKeyHandler<Cx>,
{
    handler.close_leader();
    command_char_from_key(event).is_some_and(|ch| handler.vim_char(ch, cx))
}

enum LeaderKeyAction {
    Open,
    CloseAndApply(Option<char>),
    ConsumeOpen,
    Ignore,
}

fn leader_key_action<H, Cx>(handler: &H, event: &KeyDownEvent) -> LeaderKeyAction
where
    H: AppKeyHandler<Cx>,
{
    match (
        event.is_held,
        event.keystroke.modifiers.modified(),
        handler.help_open(),
        handler.leader_open(),
        event.keystroke.key.as_str(),
        handler.has_pending_vim(),
    ) {
        (true, _, _, true, _, _) | (_, true, _, true, _, _) | (_, _, true, true, _, _) => {
            LeaderKeyAction::ConsumeOpen
        }
        (true, _, _, false, _, _) | (_, true, _, false, _, _) | (_, _, true, false, _, _) => {
            LeaderKeyAction::Ignore
        }
        (false, false, false, false, ";" | "space", false) => LeaderKeyAction::Open,
        (false, false, false, true, _, _) => {
            LeaderKeyAction::CloseAndApply(command_char_from_key(event))
        }
        _ => LeaderKeyAction::Ignore,
    }
}
