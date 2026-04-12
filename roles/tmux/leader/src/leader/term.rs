//! Tmux `display-popup -E` often leaves **stdio** disconnected from the real TTY. Ratatui’s default
//! [`Terminal::new`] calls `backend.size()` → crossterm’s global `TIOCGWINSZ`, which can return
//! **ENXIO** (“Device not configured”) on macOS even when `/dev/tty` is usable for I/O.
//!
//! We enter the alternate screen on `/dev/tty` but build the viewport with [`Viewport::Fixed`].
//! Prefer **`client_viewport_wh`** (tmux expands `#{client_width}` × `#{client_height}` on the
//! `display-popup -E` line), then **`COLUMNS` / `LINES`**, then crossterm `size()`, then **80×24**.

use std::fs::OpenOptions;
use std::io;

use ratatui::crossterm::{
    execute,
    terminal::{
        self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use ratatui::{backend::CrosstermBackend, layout::Rect, Terminal, TerminalOptions, Viewport};

#[cfg(unix)]
use std::fs::File;

#[cfg(unix)]
pub type LeaderTerminal = Terminal<CrosstermBackend<File>>;

#[cfg(not(unix))]
pub type LeaderTerminal = ratatui::DefaultTerminal;

#[cfg(unix)]
fn viewport_rect() -> Rect {
    let from_env = || -> Option<(u16, u16)> {
        let w: u16 = std::env::var("COLUMNS").ok()?.parse().ok()?;
        let h: u16 = std::env::var("LINES").ok()?.parse().ok()?;
        Some((w.max(1), h.max(1)))
    };
    let (w, h) = crate::tmux::client_viewport_wh()
        .map(|(w, h)| (w.max(1), h.max(1)))
        .or_else(from_env)
        .or_else(|| terminal::size().ok())
        .unwrap_or((80, 24));
    Rect::new(0, 0, w, h)
}

/// Initialize full-screen TUI on the real terminal.
pub fn try_init() -> io::Result<LeaderTerminal> {
    #[cfg(unix)]
    {
        set_panic_hook();
        let rect = viewport_rect();
        let mut tty = OpenOptions::new().read(true).write(true).open("/dev/tty")?;
        if let Err(e) = enable_raw_mode() {
            return Err(e);
        }
        if let Err(e) = execute!(&mut tty, EnterAlternateScreen) {
            let _ = disable_raw_mode();
            return Err(e);
        }
        let backend = CrosstermBackend::new(tty);
        match Terminal::with_options(
            backend,
            TerminalOptions {
                viewport: Viewport::Fixed(rect),
            },
        ) {
            Ok(t) => Ok(t),
            Err(e) => {
                restore_global();
                Err(e)
            }
        }
    }
    #[cfg(not(unix))]
    {
        ratatui::try_init()
    }
}

/// Restore tty after [`try_init`]. Safe to call more than once (e.g. before returning from nested actions).
pub fn restore_global() {
    #[cfg(unix)]
    {
        let _ = disable_raw_mode();
        if let Ok(mut tty) = OpenOptions::new().read(true).write(true).open("/dev/tty") {
            let _ = execute!(&mut tty, LeaveAlternateScreen);
        }
    }
    #[cfg(not(unix))]
    {
        ratatui::restore();
    }
}

fn set_panic_hook() {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        restore_global();
        hook(info);
    }));
}
