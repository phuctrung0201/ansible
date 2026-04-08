use ratatui::style::Color;

// ---------------------------------------------------------------------------
// Dracula palette (https://spec.draculatheme.com/)
// ---------------------------------------------------------------------------
pub(crate) const MAUVE: Color = Color::Rgb(189, 147, 249); // purple
pub(crate) const ORANGE: Color = Color::Rgb(255, 184, 108); // #ffb86c — git branch pill
pub(crate) const TEAL: Color = Color::Rgb(139, 233, 253); // cyan
pub(crate) const GREEN: Color = Color::Rgb(80, 250, 123); // #50fa7b — cwd pill
pub(crate) const YELLOW: Color = Color::Rgb(241, 250, 140); // yellow
pub(crate) const FG: Color = Color::Rgb(248, 248, 242); // foreground
pub(crate) const COMMENT: Color = Color::Rgb(98, 114, 164); // comment
pub(crate) const DRACULA_BG: Color = Color::Rgb(40, 42, 54); // #282a36
pub(crate) const PILL_BG: Color = Color::Rgb(68, 71, 90); // #44475a (selection / inactive window pills)
/// Dracula Cyan (`#8BE9FD`) — kube context pill fill; cwd pill uses `GREEN` in `cwd_pill_spans`.
pub(crate) const CWD_PILL_BG: Color = TEAL;

pub(crate) const COLS: usize = 4;
pub(crate) const KEY_WIDTH: usize = 5; // widest key label is "space" (5 chars)

/// Nerd Fonts / Powerline Extra. Both caps use `bg(DRACULA_BG)` / `fg` = pill fill.
/// Left/right glyphs swapped vs airline defaults so curves match this layout.
pub(crate) const ROUND_CAP_L: &str = "\u{e0b6}";
pub(crate) const ROUND_CAP_R: &str = "\u{e0b4}";

/// Font Awesome `bolt` — `─── actions ───` title (same glyph as `LEADER_HEADER_ICON` in `action.rs`).
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
