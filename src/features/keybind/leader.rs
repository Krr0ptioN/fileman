pub struct LeaderContinuation {
    pub key: &'static str,
    pub command: &'static str,
}

pub fn continuations_for(prefix: &str) -> Option<&'static [LeaderContinuation]> {
    match prefix {
        "y" => Some(YANK),
        "d" => Some(DELETE),
        "g" => Some(GO),
        "c" => Some(CHANGE),
        "u" => Some(UNDO),
        "p" => Some(PASTE),
        "z" => Some(VIEW),
        "n" => Some(NEW),
        _ => None,
    }
}

const YANK: &[LeaderContinuation] = &[
    LeaderContinuation {
        key: "y",
        command: "copy selection",
    },
    LeaderContinuation {
        key: "p",
        command: "copy path",
    },
    LeaderContinuation {
        key: "n",
        command: "copy name",
    },
    LeaderContinuation {
        key: "c",
        command: "copy file contents",
    },
];

const DELETE: &[LeaderContinuation] = &[
    LeaderContinuation {
        key: "d",
        command: "mark for move",
    },
    LeaderContinuation {
        key: "D",
        command: "delete",
    },
];

const GO: &[LeaderContinuation] = &[LeaderContinuation {
    key: "g",
    command: "go to top",
}];

const CHANGE: &[LeaderContinuation] = &[LeaderContinuation {
    key: "w",
    command: "rename",
}];

const UNDO: &[LeaderContinuation] = &[
    LeaderContinuation {
        key: "v",
        command: "clear marks",
    },
    LeaderContinuation {
        key: "V",
        command: "clear marks",
    },
];

const PASTE: &[LeaderContinuation] = &[LeaderContinuation {
    key: "p",
    command: "paste",
}];

const VIEW: &[LeaderContinuation] = &[
    LeaderContinuation {
        key: "t",
        command: "selection top",
    },
    LeaderContinuation {
        key: "z",
        command: "selection center",
    },
    LeaderContinuation {
        key: "b",
        command: "selection bottom",
    },
];

const NEW: &[LeaderContinuation] = &[
    LeaderContinuation {
        key: "f",
        command: "new file",
    },
    LeaderContinuation {
        key: "d",
        command: "new directory",
    },
];
