use std::ptr;

use crate::{action::LeaderState, attach_session, keymap};

pub(crate) fn is_root(state: &LeaderState) -> bool {
    std::ptr::eq(state.nodes.as_ptr(), keymap::KEYMAP.as_ptr())
}

pub(crate) fn is_session_subgroup(state: &LeaderState) -> bool {
    ptr::eq(state.nodes.as_ptr(), keymap::SESSION_NODES.as_ptr())
}

/// **Sessions** pill strip — **s** subgroup only (switch target before other session actions).
pub(crate) fn session_pill_strip_visible(state: &LeaderState) -> bool {
    is_session_subgroup(state) && state.pending_input.is_none()
}

pub(crate) fn is_input_mode(state: &LeaderState) -> bool {
    state.pending_input.is_some()
}

pub(crate) fn is_pane_subgroup(state: &LeaderState) -> bool {
    ptr::eq(state.nodes.as_ptr(), keymap::PANE_NODES.as_ptr())
}

/// **Panes** pill strip (`p` group only).
pub(crate) fn pane_section_visible(state: &LeaderState) -> bool {
    if !is_pane_subgroup(state) {
        return false;
    }
    state.pending_input.is_none()
}

/// **Windows** pill strip + Tab / Enter / 1–9 — root only.
pub(crate) fn window_tab_strip_visible(state: &LeaderState) -> bool {
    is_root(state) && state.pending_input.is_none()
}

pub(crate) fn is_attach_session_group(state: &LeaderState) -> bool {
    ptr::eq(state.nodes.as_ptr(), attach_session::NODES.as_ptr())
}

/// Session pills in the **A** (attach to session) view.
pub(crate) fn attach_session_section_visible(state: &LeaderState) -> bool {
    is_attach_session_group(state) && state.pending_input.is_none()
}
