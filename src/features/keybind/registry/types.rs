#[derive(Clone, Copy)]
pub struct KeybindArgs {
    pub count: usize,
    pub explicit_count: bool,
}

#[derive(Clone)]
pub struct KeybindGroup {
    pub title: String,
    pub bindings: Vec<KeybindHelp>,
}

#[derive(Clone)]
pub struct KeybindHelp {
    pub keys: String,
    pub action: String,
}

#[derive(Clone)]
pub struct LeaderContinuation {
    pub key: String,
    pub command: String,
    pub description: Option<String>,
}

pub struct KeybindSpec<C> {
    pub sequence: &'static str,
    pub name: &'static str,
    pub description: Option<&'static str>,
    pub group: &'static str,
    pub general: bool,
    pub handler: fn(KeybindArgs) -> C,
}
