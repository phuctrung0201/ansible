//! Map keymap nodes to [`KeyPress`] outcomes and update [`LeaderState`](super::state::LeaderState).

use crate::{keymap, launcher};

use super::commands::{attach_tab, close_window_keypress, other_session_names_for_move_window};
use super::state::LeaderState;

pub use super::key_press::KeyPress;

pub fn press_key(state: &mut LeaderState, key: char) -> KeyPress {
    for node in state.nodes {
        if node.key == key {
            match &node.kind {
                crate::keynode::KeyNodeKind::Action(f) => {
                    if std::ptr::fn_addr_eq(*f, attach_tab as fn() -> anyhow::Result<()>) {
                        match other_session_names_for_move_window() {
                            Ok(names) if names.is_empty() => {
                                return KeyPress::Notice(
                                    "move window: no other sessions".to_string(),
                                );
                            }
                            Ok(_) => {}
                            Err(e) => {
                                return KeyPress::Notice(format!("move window: {e:#}"));
                            }
                        }
                    }
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
                    state.nodes = nodes;
                    state.icon = icon;
                    state.label = node.label;
                    if std::ptr::eq(nodes.as_ptr(), keymap::PANE_NODES.as_ptr()) {
                        state.refresh_pane_rows();
                        state.root_pane_cursor_follow_active();
                    }
                    if std::ptr::eq(nodes.as_ptr(), launcher::NODES.as_ptr()) {
                        state.launch_cursor = 0;
                    }
                    return KeyPress::Redraw;
                }
            }
        }
    }
    KeyPress::Unrecognised
}
