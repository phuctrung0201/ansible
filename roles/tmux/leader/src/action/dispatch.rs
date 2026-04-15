//! Map keymap nodes to [`KeyPress`] outcomes and update [`LeaderState`](super::state::LeaderState).

use crate::{attach_session, keymap};

use super::commands::close_window_keypress;
use super::state::LeaderState;

pub use super::key_press::KeyPress;

pub fn press_key(state: &mut LeaderState, key: char) -> KeyPress {
    for node in state.nodes {
        if node.key == key {
            match &node.kind {
                crate::keynode::KeyNodeKind::Action(f) => {
                    return KeyPress::Execute(*f);
                }
                crate::keynode::KeyNodeKind::CloseWindow => return close_window_keypress(),
                crate::keynode::KeyNodeKind::PromptAction {
                    prompt,
                    initial_fn,
                    confirm_fn,
                    allow_empty_confirm,
                } => {
                    return KeyPress::OpenInput {
                        prompt,
                        initial: initial_fn(),
                        confirm: *confirm_fn,
                        allow_empty_confirm: *allow_empty_confirm,
                    };
                }
                crate::keynode::KeyNodeKind::Group { icon, nodes } => {
                    if std::ptr::eq(nodes.as_ptr(), keymap::SESSION_NODES.as_ptr())
                        || std::ptr::eq(nodes.as_ptr(), attach_session::NODES.as_ptr())
                    {
                        state.refresh_session_rows();
                        state.session_cursor_follow_active();
                    }
                    state.nodes = nodes;
                    state.icon = icon;
                    state.label = node.label;
                    if std::ptr::eq(nodes.as_ptr(), keymap::PANE_NODES.as_ptr()) {
                        state.refresh_pane_rows();
                        state.root_pane_cursor_follow_active();
                    }
                    return KeyPress::Redraw;
                }
            }
        }
    }
    KeyPress::Unrecognised
}
