use crate::{action::LeaderState, keymap};

pub(crate) fn is_root(state: &LeaderState) -> bool {
    std::ptr::eq(state.nodes.as_ptr(), keymap::KEYMAP.as_ptr())
}

pub(crate) fn is_tab_group(state: &LeaderState) -> bool {
    std::ptr::eq(state.nodes.as_ptr(), keymap::TAB_GROUP_NODES.as_ptr())
}

pub(crate) fn is_launch_group(state: &LeaderState) -> bool {
    std::ptr::eq(state.nodes.as_ptr(), keymap::LAUNCH_GROUP_NODES.as_ptr())
}
