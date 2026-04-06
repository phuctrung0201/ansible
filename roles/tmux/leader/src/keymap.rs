use crate::action;

pub struct KeyNode {
    pub key: char,
    pub label: &'static str,
    pub kind: KeyNodeKind,
}

pub enum KeyNodeKind {
    Action(fn() -> anyhow::Result<()>),
    InputAction {
        prompt: &'static str,
        action: fn(String) -> anyhow::Result<()>,
    },
    Group {
        icon: &'static str,
        nodes: &'static [KeyNode],
    },
}

pub static KEYMAP: &[KeyNode] = &[
    // Window
    KeyNode { key: '\t', label: "list windows",  kind: KeyNodeKind::Action(action::window_list)  },
    KeyNode { key: ' ',  label: "last window",   kind: KeyNodeKind::Action(action::last_window)   },
    KeyNode { key: 'w',  label: "new window",    kind: KeyNodeKind::Action(action::new_window)    },
    KeyNode { key: 'x',  label: "kill window",   kind: KeyNodeKind::Action(action::close_window)  },
    KeyNode { key: 'X',  label: "kill others",   kind: KeyNodeKind::Action(action::kill_other_windows) },
    KeyNode { key: 'm',  label: "move to session", kind: KeyNodeKind::Action(action::move_window_to_session) },
    // Misc
    KeyNode { key: 'e',  label: "edit command",   kind: KeyNodeKind::Action(action::launch_edit)  },
    KeyNode { key: 'o',  label: "open buffer",   kind: KeyNodeKind::Action(action::open_buffer)  },
    // Groups
    KeyNode {
        key: 's',
        label: "session",
        kind: KeyNodeKind::Group {
            icon: "󱂬",
            nodes: &[
                KeyNode { key: '\t', label: "switch session", kind: KeyNodeKind::Action(action::session_list) },
                KeyNode { key: ' ',  label: "last session",   kind: KeyNodeKind::Action(action::last_session) },
                KeyNode { key: 's',  label: "new session",    kind: KeyNodeKind::Action(action::new_session)  },
                KeyNode { key: 'r',  label: "rename session", kind: KeyNodeKind::InputAction { prompt: "rename session", action: action::rename_session_to } },
                KeyNode { key: 'x',  label: "kill session",    kind: KeyNodeKind::Action(action::delete_session)  },
                KeyNode { key: 'X',  label: "kill others",    kind: KeyNodeKind::Action(action::kill_other_sessions) },
                KeyNode { key: 'd',  label: "detach session", kind: KeyNodeKind::Action(action::detach_session)  },
            ],
        },
    },
    KeyNode {
        key: 'p',
        label: "pane",
        kind: KeyNodeKind::Group {
            icon: "󰅱",
            nodes: &[
                KeyNode { key: '\t', label: "list panes",        kind: KeyNodeKind::Action(action::pane_list)   },
                KeyNode { key: ' ',  label: "last pane",        kind: KeyNodeKind::Action(action::last_pane)   },
                KeyNode { key: 'h',  label: "split horizontal", kind: KeyNodeKind::Action(action::split_h)    },
                KeyNode { key: 'v',  label: "split vertical",   kind: KeyNodeKind::Action(action::split_v)    },
                KeyNode { key: 'x',  label: "kill pane",         kind: KeyNodeKind::Action(action::close_pane) },
                KeyNode { key: 'X',  label: "kill others",       kind: KeyNodeKind::Action(action::kill_other_panes) },
            ],
        },
    },
    KeyNode {
        key: 'l',
        label: "launch",
        kind: KeyNodeKind::Group {
            icon: "󱓞",
            nodes: &[
                KeyNode { key: 'g', label: "lazygit", kind: KeyNodeKind::Action(action::launch_lazygit) },
                KeyNode { key: 's', label: "lazysql", kind: KeyNodeKind::Action(action::launch_lazysql) },
                KeyNode { key: 'k', label: "k9s",     kind: KeyNodeKind::Action(action::launch_k9s)     },
                KeyNode { key: 'n', label: "nb",      kind: KeyNodeKind::Action(action::launch_nb)      },
            ],
        },
    },
];
