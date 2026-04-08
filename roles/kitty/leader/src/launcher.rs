//! Applications launched from the leader (`l` → launcher). Keeps TUI apps out of [`crate::keymap`].
//!
//! In the launcher layer, apps are chosen only by **index** (number keys 1–9, Tab / Shift-Tab, Enter).
//! The `key` field on each [`KeyNode`] is a placeholder (`'\0'`) and is not used for dispatch.

use crate::action;
use crate::keynode::{KeyNode, KeyNodeKind};

pub static NODES: &[KeyNode] = &[
    KeyNode {
        key: '\0',
        label: "lazygit",
        kind: KeyNodeKind::Action(action::launch_lazygit),
    },
    KeyNode {
        key: '\0',
        label: "k9s",
        kind: KeyNodeKind::Action(action::launch_k9s),
    },
    KeyNode {
        key: '\0',
        label: "nb",
        kind: KeyNodeKind::Action(action::launch_nb),
    },
    KeyNode {
        key: '\0',
        label: "lazysql",
        kind: KeyNodeKind::Action(action::launch_lazysql),
    },
];
