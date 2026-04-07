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

/// Launcher subgroup (`l` → launcher). Used for pill strip UI.
pub static LAUNCH_GROUP_NODES: &[KeyNode] = &[
    KeyNode { key: 'g', label: "lazygit", kind: KeyNodeKind::Action(action::launch_lazygit) },
    KeyNode { key: 'k', label: "k9s", kind: KeyNodeKind::Action(action::launch_k9s) },
    KeyNode { key: 'n', label: "nb", kind: KeyNodeKind::Action(action::launch_nb) },
    KeyNode { key: 's', label: "lazysql", kind: KeyNodeKind::Action(action::launch_lazysql) },
];

/// Tab actions subgroup (`t` → tab). Used to detect tab group for the tab strip UI.
pub static TAB_GROUP_NODES: &[KeyNode] = &[
    KeyNode { key: ' ',  label: "last tab",        kind: KeyNodeKind::Action(action::last_tab)        },
    KeyNode { key: 't', label: "new tab",          kind: KeyNodeKind::Action(action::new_tab)          },
    KeyNode { key: 'r', label: "rename tab",       kind: KeyNodeKind::Action(action::rename_tab)       },
    KeyNode { key: 'a', label: "attach tab",       kind: KeyNodeKind::Action(action::attach_tab)       },
    KeyNode { key: 'c', label: "clone tab",        kind: KeyNodeKind::Action(action::clone_tab)        },
    KeyNode { key: 'd', label: "detach tab",       kind: KeyNodeKind::Action(action::detach_tab)       },
    KeyNode { key: 'x', label: "close tab",        kind: KeyNodeKind::Action(action::close_tab)        },
    KeyNode { key: 'X', label: "close other tabs", kind: KeyNodeKind::Action(action::close_other_tabs) },
];

pub static KEYMAP: &[KeyNode] = &[
    // special — root: tab → cycle window above; enter → focus (handled in event loop)
    KeyNode { key: ' ',  label: "last window",  kind: KeyNodeKind::Action(action::last_window)   },
    // a-z
    KeyNode { key: 'c',  label: "copy link",    kind: KeyNodeKind::Action(action::copy_link)    },
    KeyNode { key: 'e',  label: "edit command", kind: KeyNodeKind::Action(action::edit_command) },
    KeyNode {
        key: 'l',
        label: "launcher",
        kind: KeyNodeKind::Group {
            icon: "󱓞",
            nodes: LAUNCH_GROUP_NODES,
        },
    },
    KeyNode { key: 'o',  label: "open buffer",  kind: KeyNodeKind::Action(action::open_buffer)  },
    KeyNode { key: 'r',  label: "rename window", kind: KeyNodeKind::Action(action::rename_window) },
    KeyNode {
        key: 't',
        label: "tab",
        kind: KeyNodeKind::Group {
            icon: "󰓩",
            nodes: TAB_GROUP_NODES,
        },
    },
    KeyNode { key: 'w',  label: "new window",   kind: KeyNodeKind::Action(action::new_window)    },
    KeyNode { key: 'x',  label: "close window", kind: KeyNodeKind::Action(action::close_window_action) },
    // A-Z
    KeyNode { key: 'O',  label: "open link",          kind: KeyNodeKind::Action(action::open_link)              },
    KeyNode { key: 'X',  label: "close other windows", kind: KeyNodeKind::Action(action::close_other_windows) },
];
