//! Tmux leader UI; terminal RGB values come from [`super::palette`] (`leader_theme.yml`).

pub(crate) use super::palette::palette;

pub(crate) const COLS: usize = 4;
pub(crate) const KEY_WIDTH: usize = 5; // widest key label is "space" (5 chars)

/// Nerd Fonts / Powerline Extra. Both caps use `bg(dracula_bg)` / `fg` = pill fill color.
/// Left/right glyphs swapped vs airline defaults so curves match this layout.
pub(crate) const ROUND_CAP_L: &str = "\u{e0b6}";
pub(crate) const ROUND_CAP_R: &str = "\u{e0b4}";

/// Font Awesome `bolt` — `─── actions` title (same glyph as `LEADER_HEADER_ICON` in `action.rs`).
pub(crate) const ACTIONS_TITLE_ICON: &str = "\u{f0e7}";

/// Tab strip section (nerdfont tab icon).
pub(crate) const TABS_SECTION_ICON: &str = "󰓩";

/// Session picker (root **Tab**); Font Awesome `server`.
pub(crate) const SESSIONS_SECTION_ICON: &str = "\u{f233}";

/// Pane strip (`p` group); matches panes group icon in `keymap` (fa-columns `\u{f0db}`).
pub(crate) const PANES_SECTION_ICON: &str = "\u{f0db}";

/// Matches launcher group icon in `keymap` (`󱓞`).
pub(crate) const LAUNCHER_SECTION_ICON: &str = "󱓞";

/// Nerd Fonts Devicons `nf-dev-kubernetes` (`U+E81D`). Use a patched Nerd Font in the terminal.
pub(crate) const KUBE_PILL_ICON: &str = "\u{e81d}";

/// Nerd Fonts Devicons `nf-dev-git_branch` (`U+E725`).
pub(crate) const GIT_PILL_ICON: &str = "\u{e725}";
