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
    // special
    KeyNode { key: '\t', label: "list tabs",    kind: KeyNodeKind::Action(action::tab_list)     },
    KeyNode { key: ' ',  label: "last tab",     kind: KeyNodeKind::Action(action::last_tab)     },
    // a-z
    KeyNode { key: 'c',  label: "copy link",    kind: KeyNodeKind::Action(action::copy_link)    },
    KeyNode { key: 'e',  label: "edit command", kind: KeyNodeKind::Action(action::edit_command) },
    KeyNode { key: 'o',  label: "open buffer",  kind: KeyNodeKind::Action(action::open_buffer)  },
    KeyNode { key: 't',  label: "new tab",      kind: KeyNodeKind::Action(action::new_tab)      },
    KeyNode { key: 'x',  label: "close tab",    kind: KeyNodeKind::Action(action::close_tab)    },
    // A-Z
    KeyNode {
        key: 'L',
        label: "launch",
        kind: KeyNodeKind::Group {
            icon: "󱓞",
            nodes: &[
                KeyNode { key: 'g', label: "lazygit",  kind: KeyNodeKind::Action(action::launch_lazygit) },
                KeyNode { key: 'k', label: "k9s",      kind: KeyNodeKind::Action(action::launch_k9s)     },
                KeyNode { key: 'n', label: "nb",       kind: KeyNodeKind::Action(action::launch_nb)      },
                KeyNode { key: 's', label: "lazysql",  kind: KeyNodeKind::Action(action::launch_lazysql) },
            ],
        },
    },
    KeyNode { key: 'O',  label: "open link",    kind: KeyNodeKind::Action(action::open_link)    },
    KeyNode {
        key: 'T',
        label: "tab",
        kind: KeyNodeKind::Group {
            icon: "󰓩",
            nodes: &[
                KeyNode { key: 'a', label: "attach tab",       kind: KeyNodeKind::Action(action::attach_tab)       },
                KeyNode { key: 'c', label: "clone tab",        kind: KeyNodeKind::Action(action::clone_tab)        },
                KeyNode { key: 'd', label: "detach tab",       kind: KeyNodeKind::Action(action::detach_tab)       },
                KeyNode { key: 'x', label: "close other tabs", kind: KeyNodeKind::Action(action::close_other_tabs) },
            ],
        },
    },
];
