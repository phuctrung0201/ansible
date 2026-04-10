//! Leader keyboard layers: root [`KEYMAP`]. Apps live in [`crate::launcher`].
//!
//! Root actions are ordered for the grid: **non-letters first** (e.g. space, tab), then **a–z
//! case-insensitively** (for the same letter, lowercase before uppercase).

use crate::action;
use crate::keynode::{KeyNode, KeyNodeKind};
use crate::launcher;

/// Tab picker layer (empty key grid; list UI handled in [`crate::leader::render`]).
pub static TAB_LIST_NODES: &[KeyNode] = &[];

pub static KEYMAP: &[KeyNode] = &[
    // --- special (not a–z / A–Z) ---
    KeyNode {
        key: ' ',
        label: "last tab",
        kind: KeyNodeKind::Action(action::last_tab),
    },
    KeyNode {
        key: '\t',
        label: "tab list",
        kind: KeyNodeKind::Group {
            icon: "\u{f04e9}",
            nodes: TAB_LIST_NODES,
        },
    },
    // --- a–z (case-insensitive; lower then upper per letter) ---
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
            icon: "\u{f14de}",
            nodes: launcher::NODES,
        },
    },
    KeyNode {
        key: 'n',
        label: "new tab",
        kind: KeyNodeKind::Action(action::new_tab),
    },
    KeyNode {
        key: 'o',
        label: "open buffer",
        kind: KeyNodeKind::Action(action::open_buffer),
    },
    KeyNode {
        key: 'O',
        label: "open link",
        kind: KeyNodeKind::Action(action::open_link),
    },
    KeyNode {
        key: 'r',
        label: "rename tab",
        kind: KeyNodeKind::Action(action::rename_tab),
    },
    KeyNode {
        key: 'x',
        label: "close tab",
        kind: KeyNodeKind::Action(action::close_tab),
    },
    KeyNode {
        key: 'X',
        label: "close other tabs",
        kind: KeyNodeKind::Action(action::close_other_tabs),
    },
];
