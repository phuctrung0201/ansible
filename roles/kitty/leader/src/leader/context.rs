use crate::{action::LeaderState, keymap, launcher};

pub(crate) fn is_root(state: &LeaderState) -> bool {
    std::ptr::eq(state.nodes.as_ptr(), keymap::KEYMAP.as_ptr())
}

pub(crate) fn is_launch_group(state: &LeaderState) -> bool {
    std::ptr::eq(state.nodes.as_ptr(), launcher::NODES.as_ptr())
}

pub(crate) fn is_tab_list_group(state: &LeaderState) -> bool {
    std::ptr::eq(state.nodes.as_ptr(), keymap::TAB_LIST_NODES.as_ptr())
}
