use anyhow::Context;
use serde::Deserialize;

use crate::{keymap, kitty, leader};

// ---------------------------------------------------------------------------
// JSON structs for kitten @ ls
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
pub struct KittyOs {
    pub tabs: Vec<KittyTab>,
}

#[derive(Deserialize)]
pub struct KittyTab {
    pub id: u64,
    pub title: String,
    pub is_focused: bool,
    #[serde(default)]
    #[allow(dead_code)]
    pub active_window_history: Vec<u64>,
    #[allow(dead_code)]
    pub windows: Vec<KittyWindow>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct KittyWindow {
    pub id: u64,
    pub is_self: bool,
}

fn parse_ls() -> anyhow::Result<Vec<KittyOs>> {
    let raw = kitty::ls().context("kitty ls")?;
    serde_json::from_str(&raw).context("parse kitty ls JSON")
}

// ---------------------------------------------------------------------------
// Leader state
// ---------------------------------------------------------------------------

pub struct LeaderState {
    pub nodes: &'static [keymap::KeyNode],
    pub icon: &'static str,
    pub label: &'static str,
}

impl LeaderState {
    pub fn new() -> Self {
        LeaderState {
            nodes: keymap::KEYMAP,
            icon: "⚡",
            label: "leader",
        }
    }
}

pub enum KeyPress {
    Redraw,
    Execute(fn() -> anyhow::Result<()>),
    Unrecognised,
}

/// Match key against current nodes. Group match → update state, return Redraw.
/// Action match → return Execute. No match → return Unrecognised.
pub fn press_key(state: &mut LeaderState, key: char) -> KeyPress {
    for node in state.nodes {
        if node.key == key {
            match &node.kind {
                keymap::KeyNodeKind::Action(f) => return KeyPress::Execute(*f),
                keymap::KeyNodeKind::Group { icon, nodes } => {
                    state.nodes = nodes;
                    state.icon = icon;
                    state.label = node.label;
                    return KeyPress::Redraw;
                }
            }
        }
    }
    KeyPress::Unrecognised
}

// ---------------------------------------------------------------------------
// Named action functions
// ---------------------------------------------------------------------------

/// Close the overlay window (self) so the action can run in the parent shell.
fn close_overlay() -> anyhow::Result<()> {
    kitty::close_window_self().context("close overlay")
}

pub fn open_url() -> anyhow::Result<()> {
    close_overlay()?;
    kitty::send_action("open_url_with_hints").context("open url")
}

pub fn copy_url() -> anyhow::Result<()> {
    close_overlay()?;
    kitty::send_action("kitten hints --program @").context("copy url")
}

pub fn copy_file_path() -> anyhow::Result<()> {
    close_overlay()?;
    kitty::send_action("kitten hints --type=path --program=@").context("copy file path")
}

pub fn edit_command() -> anyhow::Result<()> {
    close_overlay()?;
    kitty::send_text("\\x18\\x05").context("edit command")
}

pub fn search_history() -> anyhow::Result<()> {
    close_overlay()?;
    kitty::send_text("\\x12").context("search history (ctrl-r)")
}

pub fn copy_last_output() -> anyhow::Result<()> {
    close_overlay()?;
    kitty::send_action("copy_last_command_output").context("copy last output")
}

pub fn open_scrollback() -> anyhow::Result<()> {
    close_overlay()?;
    kitty::send_action("show_scrollback").context("open scrollback")
}

pub fn new_tab() -> anyhow::Result<()> {
    close_overlay()?;
    kitty::send_action("launch --type=tab").context("new tab")
}

pub fn new_tab_here() -> anyhow::Result<()> {
    close_overlay()?;
    kitty::send_action("launch --type=tab --cwd=current").context("new tab here")
}

pub fn detach_tab() -> anyhow::Result<()> {
    close_overlay()?;
    kitty::send_action("detach_tab").context("detach tab")
}

pub fn close_tab_self() -> anyhow::Result<()> {
    kitty::close_tab_self().context("close tab")
}

// ---------------------------------------------------------------------------
// Custom actions using leader::pick
// ---------------------------------------------------------------------------

pub fn tab_switch() -> anyhow::Result<()> {
    let os_windows = parse_ls()?;

    // Find the OS window and tab that contains this process (is_self window).
    // Using is_self is more reliable than is_focused when running inside an overlay.
    let focused_win = os_windows.iter().find(|w| {
        w.tabs.iter().any(|t| t.windows.iter().any(|win| win.is_self))
    });
    let focused_win = match focused_win {
        Some(w) => w,
        None => return Ok(()),
    };
    let focused_tab_id = focused_win
        .tabs
        .iter()
        .find(|t| t.windows.iter().any(|win| win.is_self))
        .map(|t| t.id);

    // List other tabs in the same window.
    let other_tabs: Vec<(u64, &str)> = focused_win
        .tabs
        .iter()
        .filter(|t| Some(t.id) != focused_tab_id)
        .map(|t| (t.id, t.title.as_str()))
        .collect();

    if other_tabs.is_empty() {
        return Ok(());
    }

    let items: Vec<String> = other_tabs.iter().map(|(_, title)| title.to_string()).collect();
    let ids: Vec<u64> = other_tabs.iter().map(|(id, _)| *id).collect();
    let groups = vec![leader::PickGroup { label: String::new(), items }];

    let result = leader::pick("󰓩 switch tab", &groups)?;
    close_overlay()?;
    if let Some((_group_idx, item_idx)) = result {
        kitty::focus_tab(ids[item_idx]).context("focus tab")?;
    }
    Ok(())
}

pub fn close_other_tabs() -> anyhow::Result<()> {
    let os_windows = parse_ls()?;

    let mut other_tab_ids: Vec<u64> = Vec::new();
    for os_win in &os_windows {
        let self_tab = os_win.tabs.iter().find(|t| t.windows.iter().any(|w| w.is_self));
        if let Some(self_tab) = self_tab {
            for tab in &os_win.tabs {
                if tab.id != self_tab.id {
                    other_tab_ids.push(tab.id);
                }
            }
            break;
        }
    }

    close_overlay()?;
    for tab_id in other_tab_ids {
        kitty::close_tab(tab_id).ok();
    }
    Ok(())
}

pub fn move_tab_to_window() -> anyhow::Result<()> {
    let os_windows = parse_ls()?;

    let focused_os_win_idx: Option<usize> = os_windows
        .iter()
        .position(|w| w.tabs.iter().any(|t| t.windows.iter().any(|win| win.is_self)));

    // Flat list of other OS windows; label each by its active tab's title.
    let mut items: Vec<String> = Vec::new();
    let mut target_tab_ids: Vec<u64> = Vec::new();

    for (win_idx, os_win) in os_windows.iter().enumerate() {
        if Some(win_idx) == focused_os_win_idx {
            continue;
        }
        let target_tab = os_win.tabs.iter().find(|t| t.is_focused)
            .or_else(|| os_win.tabs.first());
        if let Some(tab) = target_tab {
            items.push(tab.title.clone());
            target_tab_ids.push(tab.id);
        }
    }

    if items.is_empty() {
        return Ok(());
    }

    let groups = vec![leader::PickGroup { label: String::new(), items }];
    let result = leader::pick("󰓩 attach", &groups)?;
    close_overlay()?;
    if let Some((_group_idx, item_idx)) = result {
        kitty::detach_tab_self(target_tab_ids[item_idx]).context("detach tab to window")?;
    }
    Ok(())
}
