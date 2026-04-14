//! Applications launched from the leader (`l` → launcher). Keeps TUI apps out of [`crate::keymap`].
//! Each entry opens a **new tmux window** named after the app, in the pane’s working directory.
//!
//! The launcher UI matches the root session pills: wrapping **pills** (like window tabs), **1–9** /
//! **Tab** / **Shift+Tab**, and **Enter**.
//! The `key` field on each [`KeyNode`] is a placeholder (`'\0'`) and is not used for dispatch.

use crate::action;
use crate::keynode::{KeyNode, KeyNodeKind};

pub static NODES: &[KeyNode] = &[
    KeyNode {
        key: 'c',
        label: "caffeinate",
        kind: KeyNodeKind::Action(action::launch_caffeinate),
    },
    KeyNode {
        key: 'g',
        label: "lazygit",
        kind: KeyNodeKind::Action(action::launch_lazygit),
    },
    KeyNode {
        key: 'k',
        label: "k9s",
        kind: KeyNodeKind::Action(action::launch_k9s),
    },
    KeyNode {
        key: 'n',
        label: "nvim",
        kind: KeyNodeKind::Action(action::launch_nvim),
    },
    KeyNode {
        key: 's',
        label: "lazysql",
        kind: KeyNodeKind::Action(action::launch_lazysql),
    },
    KeyNode {
        key: 'w',
        label: "wiki",
        kind: KeyNodeKind::Action(action::launch_wiki),
    },
];
