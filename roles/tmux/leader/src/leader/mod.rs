//! Tmux leader overlay: ratatui UX backed by `tmux(1)`.

mod context;
mod dividers;
mod event_loop;
mod layout;
mod palette;
mod pick;
mod pills;
mod render;
mod term;
mod theme;

pub use pick::{pick, PickGroup, PickItem};

pub fn run() -> anyhow::Result<()> {
    let target = crate::tmux::target_pane();
    let startup_panes = crate::tmux::list_panes_for_target(&target)?;
    let my_pane = std::env::var("TMUX_PANE").unwrap_or_default();
    if crate::tmux::other_leader_running(&my_pane, &startup_panes) {
        return Ok(());
    }
    let mut terminal = term::try_init().map_err(|e| anyhow::anyhow!("terminal: {e}"))?;
    let mut state = crate::action::LeaderState::from_tmux(startup_panes);
    let result = event_loop::run(&mut terminal, &mut state);
    term::restore_global();
    result
}
