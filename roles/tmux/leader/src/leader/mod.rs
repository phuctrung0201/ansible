//! Tmux leader overlay: same ratatui UX as the kitty leader, backed by `tmux(1)`.

mod context;
mod dividers;
mod event_loop;
mod layout;
mod pick;
mod pills;
mod render;
mod palette;
pub(crate) mod tab_filter;
mod term;
mod theme;

pub use pick::{pick, PickGroup, PickItem};

pub fn run() -> anyhow::Result<()> {
    let target = crate::tmux::target_pane();
    if crate::tmux::should_skip_duplicate_leader(&target)? {
        return Ok(());
    }
    let mut terminal = term::try_init().map_err(|e| anyhow::anyhow!("terminal: {e}"))?;
    let mut state = crate::action::LeaderState::from_tmux();
    let result = event_loop::run(&mut terminal, &mut state);
    term::restore_global();
    result
}
