//! Result of handling a single character key in the leader grid.

pub enum KeyPress {
    Redraw,
    Execute(fn() -> anyhow::Result<()>),
    /// Show in-popup notice; redraw without closing the leader.
    Notice(String),
    OpenInput {
        prompt: &'static str,
        initial: String,
        confirm: fn(String) -> anyhow::Result<()>,
        allow_empty_confirm: bool,
    },
    Unrecognised,
}
