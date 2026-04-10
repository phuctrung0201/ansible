//! Shared key-tree types for [`crate::keymap`] and [`crate::launcher`].

pub struct KeyNode {
    pub key: char,
    pub label: &'static str,
    pub kind: KeyNodeKind,
}

pub enum KeyNodeKind {
    Action(fn() -> anyhow::Result<()>),
    /// Root **tab** — list sessions (replaces the action grid until Esc).
    SessionList,
    /// **w k** — close current window; toast in-popup if it is the only window in the session.
    CloseWindow,
    /// Opens an inline text-input prompt within the TUI popup before running the action.
    PromptAction {
        prompt: &'static str,
        /// Called at key-press time to populate the initial value (e.g. current name).
        initial_fn: fn() -> String,
        /// Called with the user's confirmed input to execute the rename/create.
        confirm_fn: fn(String) -> anyhow::Result<()>,
        /// If false, Enter with an empty value cancels (e.g. rename). If true, confirm still runs.
        allow_empty_confirm: bool,
    },
    /// icon: displayed in the popup header when this group is active.
    Group {
        icon: &'static str,
        nodes: &'static [KeyNode],
    },
}
