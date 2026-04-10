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
    #[allow(dead_code)]
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

/// Tab or launcher row in the leader UI.
pub struct LeaderWindowRow {
    pub id: u64,
    pub label: String,
    pub focused: bool,
    pub current: bool,
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

fn foreground_exe_basename(arg: &str) -> Option<&str> {
    let name = Path::new(arg.trim())
        .file_name()
        .and_then(|s| s.to_str())?;
    // Login shells use argv0 like `-zsh`.
    Some(name.strip_prefix('-').unwrap_or(name))
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
        let c = tab
            .windows
            .iter()
            .find(|win| win.is_self)
            .map(|w| w.cwd.trim())
            .filter(|s| !s.is_empty());
        return Ok(c.map(|s| s.to_string()));
    };
    let w = match tab
        .windows
        .iter()
        .find(|win| win.id == current_id && !win.is_self)
    {
        Some(w) => w,
        None => {
            // `current_id` can be the overlay in some layouts; the overlay still has `cwd` from
            // `launch --cwd=current` in kitty.conf.
            let Some(overlay) = tab.windows.iter().find(|win| win.is_self) else {
                return Ok(None);
            };
            let c = overlay.cwd.trim();
            return Ok((!c.is_empty()).then(|| c.to_string()));
        }
    };
    let Some(cwd) = best_cwd_from_kitty_window(w) else {
        let c = tab
            .windows
            .iter()
            .find(|win| win.is_self)
            .map(|ow| ow.cwd.trim())
            .filter(|c| !c.is_empty());
        return Ok(c.map(|s| s.to_string()));
    };
    Ok(Some(cwd.to_string()))
}

/// Shell cwd strings from kitty may use `~`; [`Path::new`] alone does not expand that.
fn leader_path_for_shell(s: &str) -> PathBuf {
    if s == "~" {
        return std::env::var("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from(s));
    }
    if let Some(rest) = s.strip_prefix("~/") {
        if let Ok(h) = std::env::var("HOME") {
            return Path::new(&h).join(rest);
        }
    }
    PathBuf::from(s)
}

/// `.git` as a directory, or a **file** (`gitdir:`) for linked worktrees / submodules.
fn resolve_git_dir(worktree: &Path) -> Option<PathBuf> {
    let marker = worktree.join(".git");
    if marker.is_dir() {
        return Some(marker);
    }
    if !marker.is_file() {
        return None;
    }
    let text = std::fs::read_to_string(&marker).ok()?;
    for raw in text.lines() {
        let line = raw.trim();
        let rest = line.strip_prefix("gitdir:")?.trim();
        let p = Path::new(rest);
        let resolved = if p.is_absolute() {
            p.to_path_buf()
        } else {
            worktree.join(p)
        };
        return Some(std::fs::canonicalize(&resolved).unwrap_or(resolved));
    }
    None
}

/// Read `HEAD`: branch name, short SHA if detached, or last segment for other refs.
fn parse_git_head(contents: &str) -> Option<String> {
    let line = contents.lines().find(|l| !l.trim().is_empty())?;
    let line = line.trim();
    if let Some(rest) = line.strip_prefix("ref: refs/heads/") {
        let name = rest.trim();
        return (!name.is_empty()).then(|| name.to_string());
    }
    if let Some(rest) = line.strip_prefix("ref: ") {
        let name = rest.split('/').last()?.trim();
        return (!name.is_empty()).then(|| name.to_string());
    }
    if line.len() >= 7 && line.chars().all(|c| c.is_ascii_hexdigit()) {
        return Some(line.chars().take(7).collect());
    }
    None
}

/// Same search order as before PATH issues were debugged: bare `git`, Homebrew, mise, `PATH` dirs.
fn git_executable_candidates() -> Vec<PathBuf> {
    let mut out: Vec<PathBuf> = Vec::new();
    out.push(PathBuf::from("git"));
    #[cfg(target_os = "macos")]
    {
        for p in [
            "/opt/homebrew/bin/git",
            "/usr/local/bin/git",
            "/usr/bin/git",
        ] {
            out.push(PathBuf::from(p));
        }
    }
    #[cfg(not(target_os = "macos"))]
    {
        out.push(PathBuf::from("/usr/bin/git"));
        out.push(PathBuf::from("/usr/local/bin/git"));
    }
    if let Ok(home) = std::env::var("HOME") {
        for rel in [
            ".local/share/mise/shims/git",
            ".local/bin/git",
            ".nix-profile/bin/git",
        ] {
            out.push(Path::new(&home).join(rel));
        }
    }
    if let Ok(path_var) = std::env::var("PATH") {
        for dir in std::env::split_paths(&path_var) {
            let g = dir.join("git");
            if g.is_file() {
                out.push(g);
            }
        }
    }
    let mut deduped: Vec<PathBuf> = Vec::new();
    for p in out {
        if !deduped.iter().any(|q| q == &p) {
            deduped.push(p);
        }
    }
    deduped
}

/// Pre–filesystem-rewrite behavior: ask `git` (this matched your setup). Tries several `git` paths.
fn git_branch_via_cli(workdir: &Path) -> Option<String> {
    if !workdir.is_dir() {
        return None;
    }
    let mut git_exe: Option<PathBuf> = None;
    for candidate in git_executable_candidates() {
        let Some(inside) = Command::new(&candidate)
            .arg("-C")
            .arg(workdir)
            .args(["rev-parse", "--is-inside-work-tree"])
            .output()
            .ok()
        else {
            continue;
        };
        if !inside.status.success() {
            continue;
        }
        if String::from_utf8_lossy(&inside.stdout).trim() != "true" {
            continue;
        }
        git_exe = Some(candidate);
        break;
    }
    let exe = git_exe?;
    let sym = Command::new(&exe)
        .arg("-C")
        .arg(workdir)
        .args(["symbolic-ref", "-q", "--short", "HEAD"])
        .output()
        .ok()?;
    if sym.status.success() {
        let b = String::from_utf8_lossy(&sym.stdout).trim().to_string();
        if !b.is_empty() {
            return Some(b);
        }
    }
    let short = Command::new(&exe)
        .arg("-C")
        .arg(workdir)
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()?;
    if !short.status.success() {
        return None;
    }
    let h = String::from_utf8_lossy(&short.stdout).trim().to_string();
    if h.is_empty() { None } else { Some(h) }
}

fn branch_label_at_worktree(worktree: &Path) -> Option<String> {
    let git_dir = resolve_git_dir(worktree)?;
    let head = std::fs::read_to_string(git_dir.join("HEAD")).ok()?;
    parse_git_head(&head)
}

/// Walk `start` and parents until a `.git` is found; no `git` binary or `PATH` required.
fn git_branch_from_ancestors(mut start: PathBuf) -> Option<String> {
    loop {
        if let Some(label) = branch_label_at_worktree(&start) {
            return Some(label);
        }
        if !start.pop() {
            break;
        }
    }
    None
}

fn git_branch_pill_for_leader(raw_cwd: Option<&str>) -> Option<String> {
    let mut bases: Vec<PathBuf> = Vec::new();
    if let Some(s) = raw_cwd {
        if !s.is_empty() {
            bases.push(leader_path_for_shell(s));
        }
    }
    if let Ok(pwd) = std::env::var("PWD") {
        if !pwd.is_empty() {
            bases.push(PathBuf::from(pwd));
        }
    }
    if let Ok(c) = std::env::current_dir() {
        bases.push(c);
    }
    let mut seen: Vec<PathBuf> = Vec::new();
    for p in bases {
        if seen.iter().any(|q| q == &p) {
            continue;
        }
        seen.push(p.clone());
        let start = if p.is_dir() {
            p
        } else if let Some(pa) = p.parent() {
            pa.to_path_buf()
        } else {
            continue;
        };
        // Prefer `git` CLI (restores pre–rewrite behavior); fall back to reading `.git/HEAD`.
        if let Some(b) = git_branch_via_cli(&start) {
            return Some(b);
        }
        if let Some(b) = git_branch_from_ancestors(start) {
            return Some(b);
        }
    }
    None
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
    /// OS-window tabs, snapshot when the leader opens.
    pub tab_rows: Vec<LeaderWindowRow>,
    /// Indices into `tab_rows` after fuzzy filter (keyboard order).
    pub tab_filtered_indices: Vec<usize>,
    /// Keyboard selection in `tab_filtered_indices` (root).
    pub tab_cursor: usize,
    /// Fuzzy filter for tab names (subsequence match, case-insensitive; type in tab list).
    pub tab_filter: String,
    /// Launcher tools (pill list; indices match [`crate::launcher::NODES`]).
    pub launch_rows: Vec<LeaderWindowRow>,
    pub launch_cursor: usize,
    /// Effective shell cwd (tilde‑shortened) for the top pill; snapshot at open.
    pub cwd_pill: Option<String>,
    /// Current Kubernetes context when kubeconfig is present; snapshot at open.
    pub kube_pill: Option<String>,
    /// Git branch (or detached short SHA) via `git` CLI first, then `.git/HEAD`; snapshot at open.
    pub git_pill: Option<String>,
}

impl LeaderState {
    /// Prefer this after a single [`parse_ls`] so startup does not run `kitten @ ls` multiple times.
    pub fn from_kitty_ls(os: Vec<KittyOs>) -> Self {
        let os_ref: &[KittyOs] = os.as_slice();
        let tab_rows = overlay_tab_rows_with_os(os_ref).unwrap_or_default();
        let launch_rows = leader_launch_rows();
        let launch_cursor = 0;
        // Prefer cwd from the shell under the overlay; fall back to this process cwd (kitty sets
        // it with `launch --cwd=current`) so git/cwd pills still work when @ ls omits usable cwd.
        let raw_cwd = overlay_effective_cwd_raw_with_os(os_ref)
            .ok()
            .flatten()
            .or_else(|| std::env::var("PWD").ok().filter(|s| !s.is_empty()))
            .or_else(|| {
                std::env::current_dir()
                    .ok()
                    .map(|p| p.to_string_lossy().into_owned())
            });
        let cwd_pill = raw_cwd
            .as_deref()
            .map(|s| format_cwd_for_display(s));
        // Branch pill: read `.git/HEAD` by walking from kitty cwd and from this process cwd (no
        // `git` binary; works when GUI apps have a minimal PATH).
        let git_pill = git_branch_pill_for_leader(raw_cwd.as_deref());
        let kube_pill = leader_kube_context_display();
        let mut state = LeaderState {
            nodes: keymap::KEYMAP,
            icon: LEADER_HEADER_ICON,
            label: "leader",
            tab_rows,
            tab_filtered_indices: Vec::new(),
            tab_cursor: 0,
            tab_filter: String::new(),
            launch_rows,
            launch_cursor,
            cwd_pill,
            kube_pill,
            git_pill,
        };
        state.recompute_tab_filter();
        state.tab_cursor_follow_kitty_focus();
        state
    }

    /// Rebuild [`Self::tab_filtered_indices`] from [`Self::tab_rows`] and [`Self::tab_filter`].
    pub fn recompute_tab_filter(&mut self) {
        let mut scored: Vec<(usize, u32)> = Vec::new();
        for (i, r) in self.tab_rows.iter().enumerate() {
            if let Some(s) =
                crate::leader::tab_filter::fuzzy_match_score(&self.tab_filter, &r.label)
            {
                scored.push((i, s));
            }
        }
        scored.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0)));
        self.tab_filtered_indices = scored.into_iter().map(|(i, _)| i).collect();
        if self.tab_filtered_indices.is_empty() {
            self.tab_cursor = 0;
            return;
        }
        self.tab_cursor = self
            .tab_cursor
            .min(self.tab_filtered_indices.len().saturating_sub(1));
    }

    /// Move tab cursor so the focused kitty tab stays selected after the filter changes.
    pub fn tab_cursor_follow_kitty_focus(&mut self) {
        if self.tab_filtered_indices.is_empty() {
            return;
        }
        if let Some(pos) = self.tab_filtered_indices.iter().position(|&i| self.tab_rows[i].current) {
            self.tab_cursor = pos;
        }
    }

    #[inline]
    pub fn selected_tab_id(&self) -> Option<u64> {
        self.tab_filtered_indices
            .get(self.tab_cursor)
            .map(|&i| self.tab_rows[i].id)
    }

    /// After changing `tab_filter`, keep the same tab selected when it still matches.
    pub fn recompute_tab_filter_keep(&mut self, keep_id: Option<u64>) {
        self.recompute_tab_filter();
        if let Some(id) = keep_id {
            if let Some(p) = self
                .tab_filtered_indices
                .iter()
                .position(|&i| self.tab_rows[i].id == id)
            {
                self.tab_cursor = p;
                return;
            }
        }
        self.tab_cursor_follow_kitty_focus();
    }

    pub fn return_to_root(&mut self) {
        self.nodes = keymap::KEYMAP;
        self.icon = LEADER_HEADER_ICON;
        self.label = "leader";
        self.tab_filter.clear();
        self.recompute_tab_filter_keep(None);
        self.tab_cursor_follow_kitty_focus();
    }

    /// Tab picker layer (same as Tab → tab list in [`keymap::KEYMAP`]; icon must stay in sync).
    pub fn enter_tab_list_picker(&mut self) {
        self.nodes = keymap::TAB_LIST_NODES;
        self.icon = "\u{f04e9}";
        self.label = "tab list";
        self.tab_filter.clear();
        self.recompute_tab_filter_keep(None);
        self.tab_cursor_follow_kitty_focus();
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
                    if std::ptr::eq(nodes.as_ptr(), keymap::TAB_LIST_NODES.as_ptr()) {
                        state.enter_tab_list_picker();
                    } else {
                        state.nodes = nodes;
                        state.icon = icon;
                        state.label = node.label;
                    }
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
