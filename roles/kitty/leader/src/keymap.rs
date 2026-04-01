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
    // actions
    KeyNode {
        key: 'a',
        label: "attach tab",
        kind: KeyNodeKind::Action(action::move_tab_to_window),
    },
    KeyNode {
        key: 'b',
        label: "browse history",
        kind: KeyNodeKind::Action(action::search_history),
    },
    KeyNode {
        key: 'd',
        label: "detach tab",
        kind: KeyNodeKind::Action(action::detach_tab),
    },
    KeyNode {
        key: 'e',
        label: "edit command",
        kind: KeyNodeKind::Action(action::edit_command),
    },
    KeyNode {
        key: 'l',
        label: "list tabs",
        kind: KeyNodeKind::Action(action::tab_list),
    },
    KeyNode {
        key: 'p',
        label: "previous tab",
        kind: KeyNodeKind::Action(action::previous_tab),
    },
    KeyNode {
        key: 'n',
        label: "new tab",
        kind: KeyNodeKind::Action(action::new_tab_here),
    },
    KeyNode {
        key: 'x',
        label: "close tab",
        kind: KeyNodeKind::Action(action::close_tab_self),
    },
    KeyNode {
        key: 'X',
        label: "close other tabs",
        kind: KeyNodeKind::Action(action::close_other_tabs),
    },
    // groups
    KeyNode {
        key: 'o',
        label: "open",
        kind: KeyNodeKind::Group {
            icon: "󰏌",
            nodes: &[
                KeyNode {
                    key: 'u',
                    label: "url",
                    kind: KeyNodeKind::Action(action::open_url),
                },
                KeyNode {
                    key: 's',
                    label: "scrollback",
                    kind: KeyNodeKind::Action(action::open_scrollback),
                },
            ],
        },
    },
    KeyNode {
        key: 'y',
        label: "yank",
        kind: KeyNodeKind::Group {
            icon: "󰆒",
            nodes: &[
                KeyNode {
                    key: 'u',
                    label: "url",
                    kind: KeyNodeKind::Action(action::copy_url),
                },
                KeyNode {
                    key: 'f',
                    label: "file path",
                    kind: KeyNodeKind::Action(action::copy_file_path),
                },
                KeyNode {
                    key: 'w',
                    label: "word",
                    kind: KeyNodeKind::Action(action::copy_word),
                },
                KeyNode {
                    key: 'l',
                    label: "line",
                    kind: KeyNodeKind::Action(action::copy_line),
                },
                KeyNode {
                    key: 'h',
                    label: "hash",
                    kind: KeyNodeKind::Action(action::copy_hash),
                },
            ],
        },
    },
];
