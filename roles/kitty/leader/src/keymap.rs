//! Leader keyboard layers: root [`KEYMAP`], window group [`WINDOW_GROUP_NODES`]. Apps live in [`crate::launcher`].

use crate::action;
use crate::keynode::{KeyNode, KeyNodeKind};
use crate::launcher;

/// Window actions subgroup (`w` → window). Used to detect window group for the window strip TUI.
pub static WINDOW_GROUP_NODES: &[KeyNode] = &[
    KeyNode {
        key: ' ',
        label: "last window",
        kind: KeyNodeKind::Action(action::last_window),
    },
    KeyNode {
        key: 'r',
        label: "rename window",
        kind: KeyNodeKind::Action(action::rename_window),
    },
    KeyNode {
        key: 'w',
        label: "new window",
        kind: KeyNodeKind::Action(action::new_window),
    },
    KeyNode {
        key: 'x',
        label: "close window",
        kind: KeyNodeKind::Action(action::close_window_action),
    },
    KeyNode {
        key: 'X',
        label: "close other windows",
        kind: KeyNodeKind::Action(action::close_other_windows),
    },
];

pub static KEYMAP: &[KeyNode] = &[
    KeyNode {
        key: ' ',
        label: "last tab",
        kind: KeyNodeKind::Action(action::last_tab),
    },
    KeyNode {
        key: 'a',
        label: "attach tab",
        kind: KeyNodeKind::Action(action::attach_tab),
    },
    KeyNode {
        key: 'c',
        label: "copy link",
        kind: KeyNodeKind::Action(action::copy_link),
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
        label: "launcher",
        kind: KeyNodeKind::Group {
            icon: "󱓞",
            nodes: launcher::NODES,
        },
    },
    KeyNode {
        key: 'o',
        label: "open buffer",
        kind: KeyNodeKind::Action(action::open_buffer),
    },
    KeyNode {
        key: 'r',
        label: "rename tab",
        kind: KeyNodeKind::Action(action::rename_tab),
    },
    KeyNode {
        key: 't',
        label: "new tab",
        kind: KeyNodeKind::Action(action::new_tab),
    },
    KeyNode {
        key: 'w',
        label: "window",
        kind: KeyNodeKind::Group {
            icon: "\u{f2d0}",
            nodes: WINDOW_GROUP_NODES,
        },
    },
    KeyNode {
        key: 'x',
        label: "close tab",
        kind: KeyNodeKind::Action(action::close_tab),
    },
    KeyNode {
        key: 'O',
        label: "open link",
        kind: KeyNodeKind::Action(action::open_link),
    },
    KeyNode {
        key: 'X',
        label: "close other tabs",
        kind: KeyNodeKind::Action(action::close_other_tabs),
    },
];
