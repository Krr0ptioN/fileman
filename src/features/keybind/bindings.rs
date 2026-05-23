pub struct KeybindGroup {
    pub title: &'static str,
    pub bindings: &'static [KeybindHelp],
}

pub struct KeybindHelp {
    pub keys: &'static str,
    pub action: &'static str,
}

pub const KEYBIND_GROUPS: &[KeybindGroup] = &[
    KeybindGroup {
        title: "Help",
        bindings: &[
            KeybindHelp {
                keys: "; / Space",
                action: "Open help",
            },
            KeybindHelp {
                keys: "Esc / q",
                action: "Close popup",
            },
        ],
    },
    KeybindGroup {
        title: "Navigation",
        bindings: &[
            KeybindHelp {
                keys: "j / k",
                action: "Move down / up",
            },
            KeybindHelp {
                keys: "h / l",
                action: "Parent / open",
            },
            KeybindHelp {
                keys: "gg / G / 0",
                action: "Top / bottom / first",
            },
            KeybindHelp {
                keys: "J / K",
                action: "Page down / up",
            },
            KeybindHelp {
                keys: "Tab / Ctrl-I / w",
                action: "Switch pane",
            },
        ],
    },
    KeybindGroup {
        title: "Selection",
        bindings: &[
            KeybindHelp {
                keys: "v / V",
                action: "Toggle mark / all marks",
            },
            KeybindHelp {
                keys: "uv",
                action: "Clear marks",
            },
        ],
    },
    KeybindGroup {
        title: "Operations",
        bindings: &[
            KeybindHelp {
                keys: "yy / yp / yn / yc",
                action: "Copy item / path / name / contents",
            },
            KeybindHelp {
                keys: "dd / pp",
                action: "Move / paste",
            },
            KeybindHelp {
                keys: "dD / x",
                action: "Delete",
            },
            KeybindHelp {
                keys: "cw / C",
                action: "Rename",
            },
            KeybindHelp {
                keys: "r / R",
                action: "Reload",
            },
            KeybindHelp {
                keys: "s",
                action: "Toggle layout",
            },
        ],
    },
];
