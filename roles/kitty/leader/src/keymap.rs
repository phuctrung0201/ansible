use crate::action;

pub struct KeyNode {
    pub key: char,
    pub label: &'static str,
    pub kind: KeyNodeKind,
}

pub enum KeyNodeKind {
    Action(fn() -> anyhow::Result<()>),
    /// icon: displayed in the popup header when this group is active
    Group {
        icon: &'static str,
        nodes: &'static [KeyNode],
    },
}

pub static KEYMAP: &[KeyNode] = &[
    KeyNode {
        key: 'u',
        label: "yank url",
        kind: KeyNodeKind::Action(action::copy_url),
    },
    KeyNode {
        key: 'o',
        label: "open url",
        kind: KeyNodeKind::Action(action::open_url),
    },
    KeyNode {
        key: 'f',
        label: "yank file path",
        kind: KeyNodeKind::Action(action::copy_file_path),
    },
    KeyNode {
        key: 'c',
        label: "edit command",
        kind: KeyNodeKind::Action(action::edit_command),
    },
    KeyNode {
        key: 'h',
        label: "search history",
        kind: KeyNodeKind::Action(action::search_history),
    },
    KeyNode {
        key: 'l',
        label: "yank last out",
        kind: KeyNodeKind::Action(action::copy_last_output),
    },
    KeyNode {
        key: 's',
        label: "scrollback",
        kind: KeyNodeKind::Action(action::open_scrollback),
    },
    KeyNode {
        key: 'k',
        label: "kube ctx",
        kind: KeyNodeKind::Action(action::kube_context_switch),
    },
    KeyNode {
        key: 't',
        label: "tabs",
        kind: KeyNodeKind::Group {
            icon: "󰓩",
            nodes: &[
                KeyNode {
                    key: 'n',
                    label: "new",
                    kind: KeyNodeKind::Action(action::new_tab),
                },
                KeyNode {
                    key: 'h',
                    label: "new from here",
                    kind: KeyNodeKind::Action(action::new_tab_here),
                },
                KeyNode {
                    key: 's',
                    label: "switch",
                    kind: KeyNodeKind::Action(action::tab_switch),
                },
                KeyNode {
                    key: 'x',
                    label: "close",
                    kind: KeyNodeKind::Action(action::close_tab_self),
                },
                KeyNode {
                    key: 'X',
                    label: "close others",
                    kind: KeyNodeKind::Action(action::close_other_tabs),
                },
                KeyNode {
                    key: 'd',
                    label: "detach",
                    kind: KeyNodeKind::Action(action::detach_tab),
                },
                KeyNode {
                    key: 'a',
                    label: "attach",
                    kind: KeyNodeKind::Action(action::move_tab_to_window),
                },
            ],
        },
    },
];
