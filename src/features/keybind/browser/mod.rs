mod general;
mod navigation;
mod operations;
mod selection;
#[cfg(test)]
mod tests;
mod vim;

use super::{KeybindArgs, KeybindRegistry, KeybindSpec};

pub use crate::features::file_browser::BrowserCommand;
pub use vim::{BrowserVimOutcome, apply_browser_vim_char};

type Registry = KeybindRegistry<BrowserCommand>;

pub fn file_manager_keybinds() -> Registry {
    let mut registry = Registry::default();
    navigation::register(&mut registry);
    selection::register(&mut registry);
    operations::register(&mut registry);
    general::register(&mut registry);
    registry
}

fn bind(
    registry: &mut Registry,
    sequence: &'static str,
    name: &'static str,
    group: &'static str,
    general: bool,
    handler: fn(KeybindArgs) -> BrowserCommand,
) {
    registry.register(KeybindSpec {
        sequence,
        name,
        description: None,
        group,
        general,
        handler,
    });
}
