//! Leader keyboard layers: root [`KEYMAP`]. Apps live in [`crate::launcher`].
//!
//! Root actions are ordered for the grid: **non-letters first** (e.g. space, tab), then **a–z
//! case-insensitively** (for the same letter, lowercase before uppercase).
//!
//! **tab** = list sessions only (no session block on the root grid). In that view: type to filter,
//! **Tab** / Shift-Tab move the cursor, **Enter** attaches, **Esc** returns to the grid.

use crate::action;
use crate::keynode::{KeyNode, KeyNodeKind};
use crate::launcher;

/// Empty grid while the session picker (root **Tab**) is active.
pub static SESSION_LIST_NODES: &[KeyNode] = &[];

/// Pane sub-group (`p` on root). Pane strip + **h** / **v** splits + **r** rename pane.
pub static PANE_NODES: &[KeyNode] = &[
    KeyNode {
        key: 'h',
        label: "split horizontal",
        kind: KeyNodeKind::Action(action::split_pane_horizontal),
    },
    KeyNode {
        key: 'v',
        label: "split vertical",
        kind: KeyNodeKind::Action(action::split_pane_vertical),
    },
    KeyNode {
        key: 'r',
        label: "rename pane",
        kind: KeyNodeKind::PromptAction {
            prompt: "rename pane:",
            initial_fn: action::get_pane_title,
            confirm_fn: action::do_rename_pane,
            allow_empty_confirm: false,
        },
    },
];

/// Window-scoped sub-group (`w` on root). **w a** = add window; **w m** = move window to session + switch.
pub static WINDOW_NODES: &[KeyNode] = &[
    // --- special (not a–z / A–Z) ---
    KeyNode {
        key: ' ',
        label: "last window",
        kind: KeyNodeKind::Action(action::last_tab),
    },
    // --- a–z (case-insensitive; lower then upper per letter) ---
    KeyNode {
        key: 'a',
        label: "add window",
        kind: KeyNodeKind::Action(action::add_window),
    },
    KeyNode {
        key: 'k',
        label: "close window",
        kind: KeyNodeKind::CloseWindow,
    },
    KeyNode {
        key: 'K',
        label: "close other windows",
        kind: KeyNodeKind::Action(action::close_other_tabs),
    },
    KeyNode {
        key: 'm',
        label: "move window to session",
        kind: KeyNodeKind::Action(action::attach_tab),
    },
    KeyNode {
        key: 'r',
        label: "rename window",
        kind: KeyNodeKind::PromptAction {
            prompt: "rename window:",
            initial_fn: action::get_window_name,
            confirm_fn: action::do_rename_window,
            allow_empty_confirm: false,
        },
    },
];

/// Root keymap — session-focused.
pub static KEYMAP: &[KeyNode] = &[
    // --- special (not a–z / A–Z) ---
    KeyNode {
        key: ' ',
        label: "last session",
        kind: KeyNodeKind::Action(action::last_session),
    },
    KeyNode {
        key: '\t',
        label: "list sessions",
        kind: KeyNodeKind::SessionList,
    },
    // --- a–z (case-insensitive; lower then upper per letter) ---
    KeyNode {
        key: 'a',
        label: "add session",
        kind: KeyNodeKind::Action(action::add_session),
    },
    KeyNode {
        key: 'd',
        label: "detach session",
        kind: KeyNodeKind::Action(action::detach_session),
    },
    KeyNode {
        key: 'e',
        label: "edit command",
        kind: KeyNodeKind::Action(action::edit_command),
    },
    KeyNode {
        key: 'k',
        label: "kill session",
        kind: KeyNodeKind::Action(action::kill_session),
    },
    KeyNode {
        key: 'K',
        label: "kill other sessions",
        kind: KeyNodeKind::Action(action::kill_other_sessions),
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
        key: 'o',
        label: "open buffer",
        kind: KeyNodeKind::Action(action::open_buffer),
    },
    KeyNode {
        key: 'p',
        label: "panes",
        kind: KeyNodeKind::Group {
            icon: "\u{f0db}",
            nodes: PANE_NODES,
        },
    },
    KeyNode {
        key: 'r',
        label: "rename session",
        kind: KeyNodeKind::PromptAction {
            prompt: "rename session:",
            initial_fn: action::get_session_name,
            confirm_fn: action::do_rename_session,
            allow_empty_confirm: false,
        },
    },
    KeyNode {
        key: 'w',
        label: "windows",
        kind: KeyNodeKind::Group {
            icon: "\u{f04e9}",
            nodes: WINDOW_NODES,
        },
    },
];
