mod command;
mod executor;
mod general;
mod navigation;
mod operations;
mod selection;
mod target;
#[cfg(test)]
mod tests;
mod vim;

use super::{KeybindArgs, KeybindRegistry, KeybindSpec};

pub use command::BrowserCommand;
pub use executor::execute_browser_sequence;
pub use target::BrowserCommandExecutor;
pub use vim::{BrowserVimInput, apply_browser_vim_char};

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
