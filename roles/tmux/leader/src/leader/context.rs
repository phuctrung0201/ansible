use std::ptr;

use crate::{action::LeaderState, keymap, launcher};

pub(crate) fn is_root(state: &LeaderState) -> bool {
    std::ptr::eq(state.nodes.as_ptr(), keymap::KEYMAP.as_ptr())
}

pub(crate) fn is_launch_group(state: &LeaderState) -> bool {
    std::ptr::eq(state.nodes.as_ptr(), launcher::NODES.as_ptr())
}

/// **Sessions** pill strip — root grid only (above launcher / windows / actions).
pub(crate) fn root_session_section_visible(state: &LeaderState) -> bool {
    is_root(state) && state.pending_input.is_none()
}

pub(crate) fn is_input_mode(state: &LeaderState) -> bool {
    state.pending_input.is_some()
}

pub(crate) fn is_window_subgroup(state: &LeaderState) -> bool {
    ptr::eq(state.nodes.as_ptr(), keymap::WINDOW_NODES.as_ptr())
}

pub(crate) fn is_pane_subgroup(state: &LeaderState) -> bool {
    ptr::eq(state.nodes.as_ptr(), keymap::PANE_NODES.as_ptr())
}

/// **Panes** pill strip (`p` group only).
pub(crate) fn pane_section_visible(state: &LeaderState) -> bool {
    if !is_pane_subgroup(state) || is_launch_group(state) {
        return false;
    }
    state.pending_input.is_none()
}

/// **Windows** pill strip + Tab / Enter / 1–9 — only inside **`w`** (windows group).
pub(crate) fn window_tab_strip_visible(state: &LeaderState) -> bool {
    if is_launch_group(state) || !is_window_subgroup(state) {
        return false;
    }
    state.pending_input.is_none()
}
