//! Kitty leader overlay: ratatui UI, key routing, and secondary pick dialogs.

mod context;
mod dividers;
mod event_loop;
mod layout;
mod message;
mod pick;
mod pills;
mod render;
mod palette;
pub(crate) mod tab_filter;
mod theme;

pub use message::show_message;
pub use pick::{pick, PickGroup, PickItem};

pub fn run() -> anyhow::Result<()> {
    let os = crate::action::parse_ls().unwrap_or_default();
    if crate::action::should_skip_duplicate_leader_launch_from_os(&os) {
        crate::kitty::close_window_self()?;
        return Ok(());
    }
    let mut terminal = ratatui::init();
    let mut state = crate::action::LeaderState::from_kitty_ls(os);
    let result = event_loop::run(&mut terminal, &mut state);
    ratatui::restore();
    result
}
