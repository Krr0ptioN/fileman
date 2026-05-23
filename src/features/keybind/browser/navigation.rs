use super::{BrowserCommand, Registry};

pub(super) fn register(registry: &mut Registry) {
    super::bind(registry, "j", "move down", "Navigation", false, |args| {
        BrowserCommand::Move(args.count.max(1) as isize)
    });
    super::bind(registry, "k", "move up", "Navigation", false, |args| {
        BrowserCommand::Move(-(args.count.max(1) as isize))
    });
    super::bind(registry, "J", "page down", "Navigation", false, |args| {
        BrowserCommand::MovePage((args.count.max(1) * 8) as isize)
    });
    super::bind(registry, "K", "page up", "Navigation", false, |args| {
        BrowserCommand::MovePage(-((args.count.max(1) * 8) as isize))
    });
    super::bind(registry, "gg", "go to top", "Navigation", false, |args| {
        BrowserCommand::Line(if args.explicit_count {
            args.count.saturating_sub(1)
        } else {
            0
        })
    });
    super::bind(registry, "G", "go to bottom", "Navigation", false, |args| {
        if args.explicit_count {
            BrowserCommand::Line(args.count.saturating_sub(1))
        } else {
            BrowserCommand::Last
        }
    });
    super::bind(registry, "0", "first row", "Navigation", false, |_| {
        BrowserCommand::First
    });
    super::bind(registry, "h", "open parent", "Navigation", false, |_| {
        BrowserCommand::OpenParent
    });
    super::bind(registry, "l", "open selected", "Navigation", false, |_| {
        BrowserCommand::OpenSelected
    });
}
