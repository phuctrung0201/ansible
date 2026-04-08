//! Kitty leader UI; terminal RGB values come from [`super::theme_colors`] (generated from `share.yml`).

pub(crate) use super::theme_colors::{
    COMMENT, COMMENT_BRIGHT, DRACULA_BG, FG, GREEN, KUBE_PILL_BG, MAUVE, ORANGE, PILL_BG, PINK, TEAL,
    YELLOW,
};

pub(crate) const COLS: usize = 4;
pub(crate) const KEY_WIDTH: usize = 5; // widest key label is "space" (5 chars)

/// Nerd Fonts / Powerline Extra. Both caps use `bg(DRACULA_BG)` / `fg` = pill fill.
/// Left/right glyphs swapped vs airline defaults so curves match this layout.
pub(crate) const ROUND_CAP_L: &str = "\u{e0b6}";
pub(crate) const ROUND_CAP_R: &str = "\u{e0b4}";

/// Font Awesome `bolt` — `─── actions` title (same glyph as `LEADER_HEADER_ICON` in `action.rs`).
pub(crate) const ACTIONS_TITLE_ICON: &str = "\u{f0e7}";

/// Font Awesome `window-maximize` (same PUA style as `LEADER_HEADER_ICON` in `action.rs`).
pub(crate) const WINDOWS_SECTION_ICON: &str = "\u{f2d0}";

/// Tab strip section (nerdfont tab icon).
pub(crate) const TABS_SECTION_ICON: &str = "󰓩";

/// Matches launcher group icon in `keymap` (`󱓞`).
pub(crate) const LAUNCHER_SECTION_ICON: &str = "󱓞";

/// Font Awesome `folder-open` (PUA); pair with Nerd Fonts / merged FA like other leader icons.
pub(crate) const CWD_PILL_ICON: &str = "\u{f07c}";

/// Nerd Fonts Devicons `nf-dev-kubernetes` (`U+E81D`). Use a patched Nerd Font in kitty.
pub(crate) const KUBE_PILL_ICON: &str = "\u{e81d}";

/// Nerd Fonts Devicons `nf-dev-git_branch` (`U+E725`).
pub(crate) const GIT_PILL_ICON: &str = "\u{e725}";
