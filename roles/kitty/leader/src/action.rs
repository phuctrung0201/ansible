use std::path::{Path, PathBuf};
use std::process::Command;

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
pub struct KittyWindowGroup {
    pub windows: Vec<u64>,
}

#[derive(Deserialize)]
pub struct KittyTab {
    pub id: u64,
    pub title: String,
    pub is_focused: bool,
    #[serde(default)]
    pub active_window_history: Vec<u64>,
    #[serde(default)]
    pub groups: Vec<KittyWindowGroup>,
    pub windows: Vec<KittyWindow>,
}

#[derive(Deserialize)]
pub struct KittyForegroundProcess {
    #[serde(default)]
    pub cmdline: Vec<String>,
    #[serde(default)]
    pub cwd: String,
}

#[derive(Deserialize)]
pub struct KittyWindow {
    pub id: u64,
    pub is_self: bool,
    #[serde(default)]
    pub is_active: bool,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub cwd: String,
    #[serde(default)]
    pub is_focused: bool,
    #[serde(default)]
    pub cmdline: Vec<String>,
    #[serde(default)]
    pub foreground_processes: Vec<KittyForegroundProcess>,
}

/// One row in the tab’s window list (overlay excluded). Shown at the top in the window group.
pub struct LeaderWindowRow {
    pub id: u64,
    pub label: String,
    pub focused: bool,
    pub current: bool,
}

fn window_label(w: &KittyWindow) -> String {
    if !w.title.is_empty() {
        w.title.clone()
    } else if !w.cwd.is_empty() {
        Path::new(&w.cwd)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("window")
            .to_string()
    } else {
        format!("window {}", w.id)
    }
}

/// Window under an overlay in the same kitty layout group (`Tab.overlay_parent` in kitty).
fn overlay_parent_window_id(tab: &KittyTab, overlay_id: u64) -> Option<u64> {
    for g in &tab.groups {
        if let Some(pos) = g.windows.iter().position(|&id| id == overlay_id) {
            if pos > 0 {
                return Some(g.windows[pos - 1]);
            }
            return None;
        }
    }
    None
}

/// Which non-overlay window should read as “current” (shell under overlay, or real focus).
///
/// When the leader runs as an overlay, kitty’s `list_windows` marks only the overlay as
/// `is_active`; non-self panes get `is_focused: false` because focus requires `w is active_window`.
/// The stable signal is [`KittyTab::groups`]: the id before `is_self` in that group is the pane
/// beneath the overlay.
fn effective_current_window_id(tab: &KittyTab) -> Option<u64> {
    let real: Vec<u64> = tab
        .windows
        .iter()
        .filter(|w| !w.is_self)
        .map(|w| w.id)
        .collect();

    let overlay_id = tab.windows.iter().find(|w| w.is_self).map(|w| w.id);

    if let Some(oid) = overlay_id {
        if let Some(under) = overlay_parent_window_id(tab, oid) {
            if real.contains(&under) {
                return Some(under);
            }
        }
    }

    if let Some(w) = tab
        .windows
        .iter()
        .find(|w| !w.is_self && w.is_focused)
    {
        return Some(w.id);
    }
    for &id in tab.active_window_history.iter().rev() {
        if real.contains(&id) {
            return Some(id);
        }
    }
    if let Some(w) = tab
        .windows
        .iter()
        .find(|w| !w.is_self && w.is_active)
    {
        return Some(w.id);
    }
    real.first().copied()
}

fn leader_overlay_tab<'a>(os: &'a [KittyOs]) -> Option<&'a KittyTab> {
    let focused_win = os.iter().find(|w| {
        w.tabs
            .iter()
            .any(|t| t.windows.iter().any(|win| win.is_self))
    })?;
    focused_win
        .tabs
        .iter()
        .find(|t| t.windows.iter().any(|w| w.is_self))
}

fn overlay_tab_window_rows_with_os(os_windows: &[KittyOs]) -> anyhow::Result<Vec<LeaderWindowRow>> {
    let tab = match leader_overlay_tab(os_windows) {
        Some(t) => t,
        None => return Ok(Vec::new()),
    };
    let current_id = effective_current_window_id(tab);
    let mut rows = Vec::new();
    for w in tab.windows.iter().filter(|w| !w.is_self) {
        rows.push(LeaderWindowRow {
            id: w.id,
            label: window_label(w),
            // Avoid double-styles: “current” is derived once via `effective_current_window_id`.
            focused: false,
            current: current_id == Some(w.id),
        });
    }
    Ok(rows)
}

fn foreground_exe_basename(arg: &str) -> Option<&str> {
    Path::new(arg.trim())
        .file_name()
        .and_then(|s| s.to_str())
}

fn foreground_looks_like_shell(cmdline: &[String]) -> bool {
    cmdline.iter().any(|a| {
        foreground_exe_basename(a).is_some_and(|n| {
            matches!(
                n,
                "zsh" | "bash" | "fish" | "nu" | "sh" | "dash" | "ksh" | "csh" | "tcsh"
            )
        })
    })
}

/// Prefer a shell’s `cwd` in `foreground_processes`; otherwise first non-empty `cwd` there.
/// Kitty’s window-level `cwd` often stays at session start while children report the real directory.
fn best_cwd_from_kitty_window(w: &KittyWindow) -> Option<&str> {
    for proc in &w.foreground_processes {
        let c = proc.cwd.trim();
        if c.is_empty() || !foreground_looks_like_shell(&proc.cmdline) {
            continue;
        }
        return Some(c);
    }
    for proc in &w.foreground_processes {
        let c = proc.cwd.trim();
        if !c.is_empty() {
            return Some(c);
        }
    }
    let c = w.cwd.trim();
    if !c.is_empty() {
        return Some(c);
    }
    None
}

fn format_cwd_for_display(cwd: &str) -> String {
    let Ok(home) = std::env::var("HOME") else {
        return cwd.to_string();
    };
    if cwd == home.as_str() {
        return "~".to_string();
    }
    let prefix = format!("{}/", home);
    if let Some(rest) = cwd.strip_prefix(&prefix) {
        return format!("~/{}", rest);
    }
    cwd.to_string()
}

fn overlay_effective_cwd_raw_with_os(os_windows: &[KittyOs]) -> anyhow::Result<Option<String>> {
    let tab = match leader_overlay_tab(os_windows) {
        Some(t) => t,
        None => return Ok(None),
    };
    let Some(current_id) = effective_current_window_id(tab) else {
        return Ok(None);
    };
    let w = match tab
        .windows
        .iter()
        .find(|win| win.id == current_id && !win.is_self)
    {
        Some(w) => w,
        None => return Ok(None),
    };
    let Some(cwd) = best_cwd_from_kitty_window(w) else {
        return Ok(None);
    };
    Ok(Some(cwd.to_string()))
}

fn git_branch_in_repo(workdir: &Path) -> Option<String> {
    let inside = Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .current_dir(workdir)
        .output()
        .ok()?;
    if !inside.status.success() {
        return None;
    }
    if String::from_utf8_lossy(&inside.stdout).trim() != "true" {
        return None;
    }
    let sym = Command::new("git")
        .args(["symbolic-ref", "-q", "--short", "HEAD"])
        .current_dir(workdir)
        .output()
        .ok()?;
    if sym.status.success() {
        let b = String::from_utf8_lossy(&sym.stdout).trim().to_string();
        if !b.is_empty() {
            return Some(b);
        }
    }
    let short = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .current_dir(workdir)
        .output()
        .ok()?;
    if !short.status.success() {
        return None;
    }
    let h = String::from_utf8_lossy(&short.stdout).trim().to_string();
    if h.is_empty() { None } else { Some(h) }
}

fn overlay_tab_rows_with_os(os_windows: &[KittyOs]) -> anyhow::Result<Vec<LeaderWindowRow>> {
    let focused_win = match os_windows.iter().find(|w| {
        w.tabs
            .iter()
            .any(|t| t.windows.iter().any(|win| win.is_self))
    }) {
        Some(w) => w,
        None => return Ok(Vec::new()),
    };
    let mut rows = Vec::new();
    for tab in &focused_win.tabs {
        let label = if tab.title.is_empty() {
            format!("tab {}", tab.id)
        } else {
            tab.title.clone()
        };
        rows.push(LeaderWindowRow {
            id: tab.id,
            label,
            focused: false,
            current: tab.windows.iter().any(|w| w.is_self),
        });
    }
    Ok(rows)
}

/// Rows for launcher pills (`id` = index into [`crate::launcher::NODES`]).
pub fn leader_launch_rows() -> Vec<LeaderWindowRow> {
    crate::launcher::NODES
        .iter()
        .enumerate()
        .map(|(i, n)| LeaderWindowRow {
            id: i as u64,
            label: n.label.to_string(),
            focused: false,
            current: false,
        })
        .collect()
}

/// Run launch action by index in [`crate::launcher::NODES`] (after closing overlay if the action does).
pub fn execute_launch_at(index: usize) -> anyhow::Result<()> {
    let node = crate::launcher::NODES
        .get(index)
        .with_context(|| format!("launch index {index} out of range"))?;
    match &node.kind {
        crate::keynode::KeyNodeKind::Action(f) => f(),
        crate::keynode::KeyNodeKind::Group { .. } => anyhow::bail!("launch node must be an action"),
    }
}

pub(crate) fn parse_ls() -> anyhow::Result<Vec<KittyOs>> {
    let raw = kitty::ls().context("kitty ls")?;
    serde_json::from_str(&raw).context("parse kitty ls JSON")
}

fn window_looks_like_leader_binary(w: &KittyWindow) -> bool {
    w.cmdline.iter().any(|arg| {
        Path::new(arg)
            .file_name()
            .and_then(|f| f.to_str())
            .is_some_and(|name| name == "leader")
    })
}

/// Whether to skip starting the TUI: another leader instance is already in this tab (shortcut pressed twice).
pub(crate) fn should_skip_duplicate_leader_launch_from_os(os: &[KittyOs]) -> bool {
    let Some(tab) = os
        .iter()
        .flat_map(|o| &o.tabs)
        .find(|t| t.windows.iter().any(|w| w.is_self))
    else {
        return false;
    };
    tab.windows
        .iter()
        .any(|w| !w.is_self && window_looks_like_leader_binary(w))
}

// ---------------------------------------------------------------------------
// Kubernetes context pill (optional)
// ---------------------------------------------------------------------------

fn kubeconfig_path_sep() -> char {
    if cfg!(windows) {
        ';'
    } else {
        ':'
    }
}

/// True if `KUBECONFIG` points at an existing file or `~/.kube/config` exists.
fn kube_config_present() -> bool {
    if let Ok(paths) = std::env::var("KUBECONFIG") {
        if !paths.is_empty() {
            for part in paths.split(kubeconfig_path_sep()) {
                if part.is_empty() {
                    continue;
                }
                if Path::new(part).is_file() {
                    return true;
                }
            }
        }
    }
    home_dir()
        .map(|h| h.join(".kube/config"))
        .is_some_and(|p| p.is_file())
}

fn home_dir() -> Option<PathBuf> {
    std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .map(PathBuf::from)
}

fn kubectl_current_context() -> Option<String> {
    let out = Command::new("kubectl")
        .args(["config", "current-context"])
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

/// First kubeconfig path: first existing entry in `KUBECONFIG`, else `~/.kube/config`.
fn primary_kubeconfig_path() -> Option<PathBuf> {
    if let Ok(paths) = std::env::var("KUBECONFIG") {
        if !paths.is_empty() {
            for part in paths.split(kubeconfig_path_sep()) {
                if part.is_empty() {
                    continue;
                }
                let p = PathBuf::from(part);
                if p.is_file() {
                    return Some(p);
                }
            }
        }
    }
    let p = home_dir()?.join(".kube/config");
    if p.is_file() {
        Some(p)
    } else {
        None
    }
}

fn current_context_from_kubeconfig_file(path: &Path) -> Option<String> {
    let data = std::fs::read_to_string(path).ok()?;
    for raw in data.lines() {
        let line = raw.trim_start();
        if let Some(rest) = line.strip_prefix("current-context:") {
            let ctx = rest.trim().trim_matches(|c| c == '"' || c == '\'');
            if !ctx.is_empty() {
                return Some(ctx.to_string());
            }
        }
    }
    None
}

/// Current kube context if a kubeconfig file exists and a context name can be read.
pub fn leader_kube_context_display() -> Option<String> {
    if !kube_config_present() {
        return None;
    }
    kubectl_current_context().or_else(|| {
        let path = primary_kubeconfig_path()?;
        current_context_from_kubeconfig_file(&path)
    })
}

// ---------------------------------------------------------------------------
// Leader state
// ---------------------------------------------------------------------------

/// Nerd Fonts / Font Awesome `bolt` (private use). Renders in the title fg color; Unicode ⚡ is emoji-colored and ignores mauve.
const LEADER_HEADER_ICON: &str = "\u{f0e7}";

pub struct LeaderState {
    pub nodes: &'static [crate::keynode::KeyNode],
    pub icon: &'static str,
    pub label: &'static str,
    /// Current tab’s windows (no overlay), snapshot when the leader opens.
    pub window_rows: Vec<LeaderWindowRow>,
    /// Keyboard selection in `window_rows` (window group only).
    pub window_cursor: usize,
    /// OS-window tabs, snapshot when the leader opens.
    pub tab_rows: Vec<LeaderWindowRow>,
    /// Keyboard selection in `tab_rows` (root).
    pub tab_cursor: usize,
    /// Launcher tools (pill list; indices match [`crate::launcher::NODES`]).
    pub launch_rows: Vec<LeaderWindowRow>,
    pub launch_cursor: usize,
    /// Effective shell cwd (tilde‑shortened) for the top pill; snapshot at open.
    pub cwd_pill: Option<String>,
    /// Current Kubernetes context when kubeconfig is present; snapshot at open.
    pub kube_pill: Option<String>,
    /// Git branch (or detached short SHA) when effective cwd is in a git work tree; snapshot at open.
    pub git_pill: Option<String>,
}

impl LeaderState {
    /// Prefer this after a single [`parse_ls`] so startup does not run `kitten @ ls` multiple times.
    pub fn from_kitty_ls(os: Vec<KittyOs>) -> Self {
        let os_ref: &[KittyOs] = os.as_slice();
        let window_rows = overlay_tab_window_rows_with_os(os_ref).unwrap_or_default();
        let window_cursor = if window_rows.is_empty() {
            0
        } else {
            window_rows
                .iter()
                .position(|r| r.current)
                .unwrap_or(0)
                .min(window_rows.len() - 1)
        };
        let tab_rows = overlay_tab_rows_with_os(os_ref).unwrap_or_default();
        let tab_cursor = if tab_rows.is_empty() {
            0
        } else {
            tab_rows
                .iter()
                .position(|r| r.current)
                .unwrap_or(0)
                .min(tab_rows.len() - 1)
        };
        let launch_rows = leader_launch_rows();
        let launch_cursor = 0;
        let raw_cwd = overlay_effective_cwd_raw_with_os(os_ref).ok().flatten();
        let cwd_pill = raw_cwd
            .as_deref()
            .map(|s| format_cwd_for_display(s));
        let git_pill = raw_cwd
            .as_deref()
            .and_then(|p| git_branch_in_repo(Path::new(p)));
        let kube_pill = leader_kube_context_display();
        LeaderState {
            nodes: keymap::KEYMAP,
            icon: LEADER_HEADER_ICON,
            label: "leader",
            window_rows,
            window_cursor,
            tab_rows,
            tab_cursor,
            launch_rows,
            launch_cursor,
            cwd_pill,
            kube_pill,
            git_pill,
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
                crate::keynode::KeyNodeKind::Action(f) => return KeyPress::Execute(*f),
                crate::keynode::KeyNodeKind::Group { icon, nodes } => {
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

pub fn open_link() -> anyhow::Result<()> {
    close_overlay()?;
    kitty::send_action("open_url_with_hints").context("open link")
}

pub fn copy_link() -> anyhow::Result<()> {
    close_overlay()?;
    kitty::send_action("kitten hints --program @").context("copy link")
}

pub fn edit_command() -> anyhow::Result<()> {
    close_overlay()?;
    kitty::send_text("\\x18\\x05").context("edit command")
}

pub fn open_buffer() -> anyhow::Result<()> {
    close_overlay()?;
    kitty::send_action("show_scrollback").context("open buffer")
}

pub fn rename_tab() -> anyhow::Result<()> {
    close_overlay()?;
    kitty::send_action("set_tab_title").context("rename tab")
}

pub fn rename_window() -> anyhow::Result<()> {
    close_overlay()?;
    kitty::send_action("set_window_title").context("rename window")
}

pub fn new_window() -> anyhow::Result<()> {
    close_overlay()?;
    kitty::send_action("launch --type=window --cwd=current").context("new window with current cwd")
}

pub fn close_window_action() -> anyhow::Result<()> {
    close_overlay()?;
    kitty::send_action("close_window").context("close window")
}

pub fn close_other_windows() -> anyhow::Result<()> {
    close_overlay()?;
    kitty::send_action("close_other_windows_in_tab").context("close other windows in tab")
}

pub fn new_tab() -> anyhow::Result<()> {
    close_overlay()?;
    kitty::send_action("launch --type=tab").context("new tab")
}

pub fn detach_tab() -> anyhow::Result<()> {
    close_overlay()?;
    kitty::send_action("detach_tab").context("detach tab")
}

pub fn close_tab() -> anyhow::Result<()> {
    kitty::close_tab_self().context("close tab")
}

pub fn last_tab() -> anyhow::Result<()> {
    close_overlay()?;
    kitty::focus_tab_recent()
}

pub fn last_window() -> anyhow::Result<()> {
    close_overlay()?;
    kitty::send_action("nth_window -1").context("last window")
}

/// Focus a tab window after closing the leader overlay.
pub fn focus_window_from_leader(id: u64) -> anyhow::Result<()> {
    close_overlay()?;
    kitty::focus_window(id).context("focus window")
}

pub fn focus_tab_from_leader(id: u64) -> anyhow::Result<()> {
    close_overlay()?;
    kitty::focus_tab(id).context("focus tab")
}

// ---------------------------------------------------------------------------
// Custom actions using leader::pick
// ---------------------------------------------------------------------------

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

pub fn launch_lazygit() -> anyhow::Result<()> {
    close_overlay()?;
    kitty::send_action("launch --type=overlay --cwd=current lazygit").context("launch lazygit")
}

pub fn launch_k9s() -> anyhow::Result<()> {
    close_overlay()?;
    kitty::send_action("launch --type=overlay --cwd=current k9s").context("launch k9s")
}

pub fn launch_lazysql() -> anyhow::Result<()> {
    close_overlay()?;
    kitty::send_action("launch --type=overlay --cwd=current lazysql").context("launch lazysql")
}

pub fn launch_nb() -> anyhow::Result<()> {
    close_overlay()?;
    kitty::send_action("launch --type=overlay --cwd=current nb -i").context("launch nb")
}

pub fn attach_tab() -> anyhow::Result<()> {
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
        return leader::show_message("attach", "no other windows");
    }

    let items: Vec<leader::PickItem> = items.into_iter()
        .map(|s| leader::PickItem { label: s, focused: false, current: false })
        .collect();
    let groups = vec![leader::PickGroup { label: String::new(), items }];
    let result = leader::pick("󰓩 attach", &groups)?;
    close_overlay()?;
    if let Some((_group_idx, item_idx)) = result {
        kitty::detach_tab_self(target_tab_ids[item_idx]).context("attach tab")?;
    }
    Ok(())
}
