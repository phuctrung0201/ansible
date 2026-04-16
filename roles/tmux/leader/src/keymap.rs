//! Leader keyboard layers: root [`KEYMAP`]. Apps live in [`crate::launcher`].
//!
//! Root actions are ordered for the grid: **non-letters first** (e.g. space), then **a–z
//! case-insensitively** (for the same letter, lowercase before uppercase).
//!
//! **Window** actions and the window pill strip are on the root. **Sessions** live under **s**;
//! **S** opens scrollback. Session subgroup: pill strip plus **space** last session, **a** add,
//! **d** detach, **k** / **K** kill, **r** rename, **R** rename to cwd (basename).

use crate::action;
use crate::attach_session;
use crate::keynode::{KeyNode, KeyNodeKind};
use crate::launcher;

/// Session sub-group (**s** on root). Session pills + session-scoped actions.
pub static SESSION_NODES: &[KeyNode] = &[
    KeyNode {
        key: ' ',
        label: "last session",
        kind: KeyNodeKind::Action(action::last_session),
    },
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
        key: 'R',
        label: "rename to cwd",
        kind: KeyNodeKind::Action(action::rename_session_to_pane_folder),
    },
];

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

/// Root keymap (includes window actions; window pill strip on root).
pub static KEYMAP: &[KeyNode] = &[
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
        key: 'A',
        label: "attach to session",
        kind: KeyNodeKind::Group {
            icon: "\u{f233}",
            nodes: attach_session::NODES,
        },
    },
    KeyNode {
        key: 'e',
        label: "edit command",
        kind: KeyNodeKind::Action(action::edit_command),
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
        key: 'l',
        label: "launcher",
        kind: KeyNodeKind::Group {
            icon: "\u{f14de}",
            nodes: launcher::NODES,
        },
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
        label: "rename window",
        kind: KeyNodeKind::PromptAction {
            prompt: "rename window:",
            initial_fn: action::get_window_name,
            confirm_fn: action::do_rename_window,
            allow_empty_confirm: false,
        },
    },
    KeyNode {
        key: 's',
        label: "sessions",
        kind: KeyNodeKind::Group {
            icon: "\u{f233}",
            nodes: SESSION_NODES,
        },
    },
    KeyNode {
        key: 'S',
        label: "scrollback",
        kind: KeyNodeKind::Action(action::open_scrollback),
    },
];
