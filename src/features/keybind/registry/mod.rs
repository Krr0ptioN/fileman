mod types;

pub use types::{KeybindArgs, KeybindGroup, KeybindHelp, KeybindSpec, LeaderContinuation};

pub struct KeybindRegistry<C> {
    bindings: Vec<KeybindBinding<C>>,
}

struct KeybindBinding<C> {
    sequence: &'static str,
    name: &'static str,
    description: Option<&'static str>,
    group: &'static str,
    general: bool,
    handler: fn(KeybindArgs) -> C,
}

impl<C> Default for KeybindRegistry<C> {
    fn default() -> Self {
        Self { bindings: vec![] }
    }
}

impl<C> KeybindRegistry<C> {
    pub fn register(&mut self, spec: KeybindSpec<C>) {
        self.bindings.push(KeybindBinding {
            sequence: spec.sequence,
            name: spec.name,
            description: spec.description,
            group: spec.group,
            general: spec.general,
            handler: spec.handler,
        });
    }

    pub fn command_for(&self, sequence: &str, args: KeybindArgs) -> Option<C> {
        self.bindings
            .iter()
            .find(|binding| binding.sequence == sequence)
            .map(|binding| (binding.handler)(args))
    }

    pub fn is_prefix(&self, prefix: &str) -> bool {
        self.bindings.iter().any(|binding| {
            binding.sequence.starts_with(prefix) && binding.sequence.len() > prefix.len()
        })
    }

    pub fn leader_continuations(&self, prefix: &str) -> Vec<LeaderContinuation> {
        match prefix {
            "" => self.general_continuations(),
            _ => self.prefixed_continuations(prefix),
        }
    }

    pub fn help_groups(&self) -> Vec<KeybindGroup> {
        let mut groups = Vec::<KeybindGroup>::new();
        for binding in &self.bindings {
            push_help_binding(&mut groups, binding);
        }
        groups
    }

    fn general_continuations(&self) -> Vec<LeaderContinuation> {
        self.bindings
            .iter()
            .filter(|binding| binding.general)
            .map(continuation_for_binding)
            .collect()
    }

    fn prefixed_continuations(&self, prefix: &str) -> Vec<LeaderContinuation> {
        self.bindings
            .iter()
            .filter(|binding| binding.sequence.starts_with(prefix))
            .filter_map(|binding| continuation_after_prefix(binding, prefix))
            .collect()
    }
}

fn push_help_binding<C>(groups: &mut Vec<KeybindGroup>, binding: &KeybindBinding<C>) {
    let help = KeybindHelp {
        keys: binding.sequence.to_string(),
        action: binding.name.to_string(),
    };
    match groups.iter_mut().find(|group| group.title == binding.group) {
        Some(group) => group.bindings.push(help),
        None => groups.push(KeybindGroup {
            title: binding.group.to_string(),
            bindings: vec![help],
        }),
    }
}

fn continuation_for_binding<C>(binding: &KeybindBinding<C>) -> LeaderContinuation {
    LeaderContinuation {
        key: binding.sequence.to_string(),
        command: binding.name.to_string(),
        description: binding.description.map(str::to_string),
    }
}

fn continuation_after_prefix<C>(
    binding: &KeybindBinding<C>,
    prefix: &str,
) -> Option<LeaderContinuation> {
    let suffix = binding.sequence.strip_prefix(prefix)?;
    let key = suffix.chars().next()?;
    Some(LeaderContinuation {
        key: key.to_string(),
        command: binding.name.to_string(),
        description: binding.description.map(str::to_string),
    })
}
