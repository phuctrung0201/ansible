//! Shared key-tree types for [`crate::keymap`] and [`crate::launcher`].

pub struct KeyNode {
    pub key: char,
    pub label: &'static str,
    pub kind: KeyNodeKind,
}

pub enum KeyNodeKind {
    Action(fn() -> anyhow::Result<()>),
    /// icon: displayed in the popup header when this group is active
    Group {
        icon: &'static str,
        nodes: &'static [KeyNode],
    },
}
