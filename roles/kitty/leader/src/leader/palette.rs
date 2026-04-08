//! RGB palette loaded from `leader_theme.yml` (see crate root). Falls back to the embedded default.

use std::path::PathBuf;
use std::sync::OnceLock;

use anyhow::{Context, Result};
use ratatui::style::Color;
use serde::Deserialize;

static PALETTE: OnceLock<Palette> = OnceLock::new();

const EMBEDDED_YML: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/leader_theme.yml"));

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
}

#[derive(Debug, Clone)]
pub(crate) struct Palette {
    pub mauve: Color,
    pub orange: Color,
    pub teal: Color,
    pub green: Color,
    pub yellow: Color,
    pub pink: Color,
    pub fg: Color,
    pub comment: Color,
    pub comment_bright: Color,
    pub dracula_bg: Color,
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
            dracula_bg: parse_hex(&entry.bg).context("bg")?,
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

fn config_paths() -> impl Iterator<Item = PathBuf> {
    let mut v: Vec<PathBuf> = Vec::new();
    if let Ok(p) = std::env::var("LEADER_THEME") {
        v.push(PathBuf::from(p));
    }
    if let Ok(home) = std::env::var("HOME") {
        v.push(PathBuf::from(home).join(".config/kitty/leader_theme.yml"));
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
        return load_yaml_bytes(&bytes).with_context(|| format!("invalid leader theme {}", path.display()));
    }
    load_yaml_bytes(EMBEDDED_YML.as_bytes()).context("embedded leader_theme.yml")
}

/// Initialized lazily on first use; prefers `$LEADER_THEME`, then `~/.config/kitty/leader_theme.yml`, then the embedded default.
pub(crate) fn palette() -> &'static Palette {
    PALETTE.get_or_init(|| match load_or_embedded() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("leader: theme load failed ({e}), using embedded defaults");
            load_yaml_bytes(EMBEDDED_YML.as_bytes()).expect("embedded leader_theme.yml")
        }
    })
}
