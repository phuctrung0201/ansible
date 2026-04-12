//! RGB palette loaded from `leader_theme.yml`. Falls back to the embedded default.

use std::path::PathBuf;
use std::sync::OnceLock;

use anyhow::{Context, Result};
use ratatui::style::Color;
use serde::Deserialize;

static PALETTE: OnceLock<Palette> = OnceLock::new();

const EMBEDDED_YML: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/leader_theme.yml"));

fn default_pill_fg_hex() -> String {
    "#282a36".to_string()
}

#[derive(Debug, Deserialize)]
struct LeaderThemeFile {
    purple: String,
    orange: String,
    cyan: String,
    green: String,
    yellow: String,
    pink: String,
    bright_cyan: String,
    fg: String,
    comment: String,
    comment_bright: String,
    bg: String,
    selection: String,
    /// Dark text on bright pill bodies (banner + window pills). Separate from `bg` so popup can use `reset`.
    #[serde(default = "default_pill_fg_hex")]
    pill_fg: String,
}

#[derive(Debug, Clone)]
pub(crate) struct Palette {
    pub mauve: Color,
    pub orange: Color,
    pub teal: Color,
    /// Reserved (e.g. cwd pill); kept so `green` stays in theme YAML.
    #[allow(dead_code)]
    pub green: Color,
    pub yellow: Color,
    pub pink: Color,
    pub fg: Color,
    pub comment: Color,
    pub comment_bright: Color,
    pub dracula_bg: Color,
    /// Foreground for text on colored pill fills (always RGB; see `pill_fg` in theme YAML).
    pub pill_fg: Color,
    pub pill_bg: Color,
    pub kube_pill_bg: Color,
}

impl Palette {
    fn from_entry(entry: LeaderThemeFile) -> Result<Self> {
        let kube_pill_bg = parse_hex(&entry.bright_cyan).context("bright_cyan")?;
        Ok(Self {
            mauve: parse_hex(&entry.purple).context("purple")?,
            orange: parse_hex(&entry.orange).context("orange")?,
            teal: parse_hex(&entry.cyan).context("cyan")?,
            green: parse_hex(&entry.green).context("green")?,
            yellow: parse_hex(&entry.yellow).context("yellow")?,
            pink: parse_hex(&entry.pink).context("pink")?,
            fg: parse_hex(&entry.fg).context("fg")?,
            comment: parse_hex(&entry.comment).context("comment")?,
            comment_bright: parse_hex(&entry.comment_bright).context("comment_bright")?,
            dracula_bg: parse_bg(&entry.bg).context("bg")?,
            pill_fg: parse_hex(&entry.pill_fg).context("pill_fg")?,
            pill_bg: parse_hex(&entry.selection).context("selection")?,
            kube_pill_bg,
        })
    }
}

fn parse_hex(raw: &str) -> Result<Color> {
    let s = raw.trim().trim_start_matches('#');
    anyhow::ensure!(s.len() == 6, "expected #RRGGBB, got {raw:?}");
    let r = u8::from_str_radix(&s[0..2], 16).context("red channel")?;
    let g = u8::from_str_radix(&s[2..4], 16).context("green channel")?;
    let b = u8::from_str_radix(&s[4..6], 16).context("blue channel")?;
    Ok(Color::Rgb(r, g, b))
}

/// Popup / panel background: `#RRGGBB` or `transparent` / `reset` / `default` / `none` for terminal default (see-through in many setups).
fn parse_bg(raw: &str) -> Result<Color> {
    let s = raw.trim();
    match s.to_ascii_lowercase().as_str() {
        "transparent" | "reset" | "default" | "none" => Ok(Color::Reset),
        _ => parse_hex(s),
    }
}

fn config_paths() -> impl Iterator<Item = PathBuf> {
    let mut v: Vec<PathBuf> = Vec::new();
    if let Ok(p) = std::env::var("LEADER_THEME") {
        v.push(PathBuf::from(p));
    }
    if let Ok(home) = std::env::var("HOME") {
        v.push(PathBuf::from(home).join(".config/tmux/leader_theme.yml"));
    }
    v.into_iter()
}

fn load_yaml_bytes(bytes: &[u8]) -> Result<Palette> {
    let entry: LeaderThemeFile = serde_yaml::from_slice(bytes).context("parse leader_theme.yml")?;
    Palette::from_entry(entry)
}

fn load_or_embedded() -> Result<Palette> {
    for path in config_paths() {
        if path.as_os_str().is_empty() || !path.is_file() {
            continue;
        }
        let bytes = std::fs::read(&path)
            .with_context(|| format!("read leader theme {}", path.display()))?;
        return load_yaml_bytes(&bytes)
            .with_context(|| format!("invalid leader theme {}", path.display()));
    }
    load_yaml_bytes(EMBEDDED_YML.as_bytes()).context("embedded leader_theme.yml")
}

/// Initialized lazily on first use; prefers `$LEADER_THEME`, then `~/.config/tmux/leader_theme.yml`, then embedded defaults.
pub(crate) fn palette() -> &'static Palette {
    PALETTE.get_or_init(|| match load_or_embedded() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("tmux-leader: theme load failed ({e}), using embedded defaults");
            load_yaml_bytes(EMBEDDED_YML.as_bytes()).expect("embedded leader_theme.yml")
        }
    })
}
