use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::Context;
use tui_input::Input;

use crate::{keymap, leader, tmux};

/// Root grid has **tab** → list sessions; the picker is not shown on the root grid itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeaderView {
    Normal,
    SessionList,
}

/// Full-screen rename-style prompt within the popup (section rule, hint, bordered field).
pub struct PendingInput {
    pub prompt: &'static str,
    pub input: Input,
    pub confirm: fn(String) -> anyhow::Result<()>,
    pub allow_empty_confirm: bool,
}

// ---------------------------------------------------------------------------
// Git / kube pills (same behavior as kitty leader)
// ---------------------------------------------------------------------------

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

fn git_branch_via_cli(workdir: &Path) -> Option<String> {
    if !workdir.is_dir() {
        return None;
    }
    let mut git_exe: Option<PathBuf> = None;
    for candidate in git_executable_candidates() {
        let inside = Command::new(&candidate)
            .arg("-C")
            .arg(workdir)
            .args(["rev-parse", "--is-inside-work-tree"])
            .output()
            .ok()?;
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
        if let Some(b) = git_branch_via_cli(&start) {
            return Some(b);
        }
        if let Some(b) = git_branch_from_ancestors(start) {
            return Some(b);
        }
    }
    None
}

fn kubeconfig_path_sep() -> char {
    if cfg!(windows) {
        ';'
    } else {
        ':'
    }
}

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
// Rows & state
// ---------------------------------------------------------------------------

#[derive(Clone)]
pub struct LeaderWindowRow {
    pub id: u64,
    pub label: String,
    pub focused: bool,
    pub current: bool,
}

pub struct LeaderPaneRow {
    pub pane_id: String,
    pub label: String,
    pub current: bool,
}

const LEADER_HEADER_ICON: &str = "\u{f0e7}";

pub struct LeaderState {
    pub nodes: &'static [crate::keynode::KeyNode],
    pub icon: &'static str,
    pub label: &'static str,
    pub tab_rows: Vec<LeaderWindowRow>,
    pub session_rows: Vec<LeaderWindowRow>,
    pub session_filtered_indices: Vec<usize>,
    pub session_cursor: usize,
    pub session_filter: String,
    pub view: LeaderView,
    // pending inline text input
    pub pending_input: Option<PendingInput>,
    // launcher (fuzzy list + pills, same UX as session picker)
    pub launch_rows: Vec<LeaderWindowRow>,
    pub launch_filtered_indices: Vec<usize>,
    pub launch_cursor: usize,
    pub launch_filter: String,
    /// Root “windows” pill strip: Tab cycles `0..min(tab_rows.len(), 24)` like launcher.
    pub root_window_cursor: usize,
    /// **p** group: pane pills (Tab / Enter / 1–9), same idea as windows.
    pub pane_rows: Vec<LeaderPaneRow>,
    pub root_pane_cursor: usize,
    // pills
    pub kube_pill: Option<String>,
    pub git_pill: Option<String>,
    /// One-line message in the popup (e.g. blocked or invalid action); cleared on the next key.
    pub notice: Option<String>,
}

fn window_rows() -> anyhow::Result<Vec<LeaderWindowRow>> {
    let lines = tmux::list_windows_for_target()?;
    let mut rows = Vec::new();
    for w in lines {
        let label = if w.name.is_empty() {
            format!("window {}", w.index)
        } else {
            w.name.clone()
        };
        rows.push(LeaderWindowRow { id: w.id, label, focused: false, current: w.active });
    }
    Ok(rows)
}

fn session_rows_data() -> Vec<LeaderWindowRow> {
    tmux::list_sessions()
        .unwrap_or_default()
        .into_iter()
        .enumerate()
        .map(|(i, s)| LeaderWindowRow {
            id: i as u64,
            label: s.name,
            focused: false,
            current: s.active,
        })
        .collect()
}

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

pub fn execute_launch_at(index: usize) -> anyhow::Result<()> {
    let node = crate::launcher::NODES
        .get(index)
        .with_context(|| format!("launch index {index} out of range"))?;
    match &node.kind {
        crate::keynode::KeyNodeKind::Action(f) => f(),
        crate::keynode::KeyNodeKind::PromptAction { .. }
        | crate::keynode::KeyNodeKind::Group { .. }
        | crate::keynode::KeyNodeKind::SessionList
        | crate::keynode::KeyNodeKind::CloseWindow => anyhow::bail!("launch node must be an action"),
    }
}

impl LeaderState {
    pub fn from_tmux() -> Self {
        let target = tmux::target_pane();
        let tab_rows = window_rows().unwrap_or_default();
        let session_rows = session_rows_data();
        let launch_rows = leader_launch_rows();
        let raw_cwd = tmux::pane_cwd(&target).ok().filter(|s| !s.is_empty());
        let git_pill = git_branch_pill_for_leader(raw_cwd.as_deref());
        let kube_pill = leader_kube_context_display();
        let mut state = LeaderState {
            nodes: keymap::KEYMAP,
            icon: LEADER_HEADER_ICON,
            label: "tmux leader",
            tab_rows,
            session_rows,
            session_filtered_indices: Vec::new(),
            session_cursor: 0,
            session_filter: String::new(),
            view: LeaderView::Normal,
            pending_input: None,
            launch_rows,
            launch_filtered_indices: Vec::new(),
            launch_cursor: 0,
            launch_filter: String::new(),
            root_window_cursor: 0,
            pane_rows: Vec::new(),
            root_pane_cursor: 0,
            kube_pill,
            git_pill,
            notice: None,
        };
        state.recompute_session_filter();
        state.session_cursor_follow_active();
        state.root_window_cursor_follow_active();
        state.refresh_pane_rows();
        state.root_pane_cursor_follow_active();
        state.recompute_launch_filter();
        state
    }

    /// Align root window pill selection with the active window (first 24 windows only).
    pub fn root_window_cursor_follow_active(&mut self) {
        let n = self.tab_rows.len().min(24);
        if n == 0 {
            self.root_window_cursor = 0;
            return;
        }
        if let Some(i) = self.tab_rows.iter().position(|r| r.current) {
            self.root_window_cursor = i.min(n - 1);
        } else {
            self.root_window_cursor = self.root_window_cursor.min(n - 1);
        }
    }

    pub fn refresh_pane_rows(&mut self) {
        let Ok(mut rows) = tmux::list_panes_for_window() else {
            self.pane_rows.clear();
            return;
        };
        let my_pane = std::env::var("TMUX_PANE").unwrap_or_default();
        rows.retain(|r| r.pane_id != my_pane && !r.command.contains("tmux-leader"));
        self.pane_rows = rows
            .into_iter()
            .map(|r| LeaderPaneRow {
                pane_id: r.pane_id,
                label: format!("{} {}", r.index, r.command),
                current: r.active,
            })
            .collect();
    }

    pub fn root_pane_cursor_follow_active(&mut self) {
        let n = self.pane_rows.len().min(24);
        if n == 0 {
            self.root_pane_cursor = 0;
            return;
        }
        if let Some(i) = self.pane_rows.iter().position(|r| r.current) {
            self.root_pane_cursor = i.min(n - 1);
        } else {
            self.root_pane_cursor = self.root_pane_cursor.min(n - 1);
        }
    }

    pub fn recompute_session_filter(&mut self) {
        let mut scored: Vec<(usize, u32)> = Vec::new();
        for (i, r) in self.session_rows.iter().enumerate() {
            if let Some(s) =
                crate::leader::tab_filter::fuzzy_match_score(&self.session_filter, &r.label)
            {
                scored.push((i, s));
            }
        }
        scored.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0)));
        self.session_filtered_indices = scored.into_iter().map(|(i, _)| i).collect();
        if self.session_filtered_indices.is_empty() {
            self.session_cursor = 0;
            return;
        }
        let n = self.session_filtered_indices.len();
        self.session_cursor = self.session_cursor.min(n.saturating_sub(1));
    }

    pub fn session_cursor_follow_active(&mut self) {
        let n = self.session_filtered_indices.len();
        if n == 0 {
            return;
        }
        if let Some(pos) = self
            .session_filtered_indices
            .iter()
            .position(|&i| self.session_rows[i].current)
        {
            self.session_cursor = pos.min(n - 1);
        }
    }

    #[inline]
    pub fn selected_session_name(&self) -> Option<String> {
        self.session_filtered_indices
            .get(self.session_cursor)
            .map(|&i| self.session_rows[i].label.clone())
    }

    pub fn recompute_session_filter_keep(&mut self, keep_name: Option<&str>) {
        self.recompute_session_filter();
        if let Some(name) = keep_name {
            if let Some(p) = self
                .session_filtered_indices
                .iter()
                .position(|&i| self.session_rows[i].label == name)
            {
                let n = self.session_filtered_indices.len();
                if n > 0 {
                    self.session_cursor = p.min(n - 1);
                }
                return;
            }
        }
        self.session_cursor_follow_active();
    }

    pub fn enter_session_list_picker(&mut self) {
        self.nodes = keymap::SESSION_LIST_NODES;
        self.icon = "";
        self.label = "list sessions";
        self.view = LeaderView::SessionList;
        self.session_filter.clear();
        self.recompute_session_filter_keep(None);
        self.session_cursor_follow_active();
    }

    pub fn recompute_launch_filter(&mut self) {
        let mut scored: Vec<(usize, u32)> = Vec::new();
        for (i, r) in self.launch_rows.iter().enumerate() {
            if let Some(s) =
                crate::leader::tab_filter::fuzzy_match_score(&self.launch_filter, &r.label)
            {
                scored.push((i, s));
            }
        }
        scored.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0)));
        self.launch_filtered_indices = scored.into_iter().map(|(i, _)| i).collect();
        if self.launch_filtered_indices.is_empty() {
            self.launch_cursor = 0;
            return;
        }
        let n = self.launch_filtered_indices.len();
        self.launch_cursor = self.launch_cursor.min(n.saturating_sub(1));
    }

    pub fn selected_launch_index(&self) -> Option<usize> {
        self.launch_filtered_indices
            .get(self.launch_cursor)
            .map(|&i| self.launch_rows[i].id as usize)
    }

    pub fn selected_launch_label(&self) -> Option<String> {
        self.launch_filtered_indices
            .get(self.launch_cursor)
            .map(|&i| self.launch_rows[i].label.clone())
    }

    pub fn recompute_launch_filter_keep(&mut self, keep_label: Option<&str>) {
        self.recompute_launch_filter();
        if let Some(label) = keep_label {
            if let Some(p) = self
                .launch_filtered_indices
                .iter()
                .position(|&i| self.launch_rows[i].label == label)
            {
                let n = self.launch_filtered_indices.len();
                if n > 0 {
                    self.launch_cursor = p.min(n - 1);
                }
                return;
            }
        }
    }

    // --- shared ---

    pub fn enter_input_mode(
        &mut self,
        prompt: &'static str,
        initial: String,
        confirm: fn(String) -> anyhow::Result<()>,
        allow_empty_confirm: bool,
    ) {
        self.notice = None;
        self.pending_input = Some(PendingInput {
            prompt,
            input: Input::new(initial),
            confirm,
            allow_empty_confirm,
        });
    }

    pub fn return_to_root(&mut self) {
        self.nodes = keymap::KEYMAP;
        self.icon = LEADER_HEADER_ICON;
        self.label = "tmux leader";
        self.view = LeaderView::Normal;
        self.pending_input = None;
        self.notice = None;
        self.session_filter.clear();
        self.recompute_session_filter_keep(None);
        self.launch_filter.clear();
        self.recompute_launch_filter_keep(None);
        self.root_window_cursor_follow_active();
    }
}

pub enum KeyPress {
    Redraw,
    Execute(fn() -> anyhow::Result<()>),
    /// Show in-popup notice; redraw without closing the leader.
    Notice(String),
    OpenInput {
        prompt: &'static str,
        initial: String,
        confirm: fn(String) -> anyhow::Result<()>,
        allow_empty_confirm: bool,
    },
    Unrecognised,
}

pub fn press_key(state: &mut LeaderState, key: char) -> KeyPress {
    for node in state.nodes {
        if node.key == key {
            match &node.kind {
                crate::keynode::KeyNodeKind::Action(f) => {
                    if std::ptr::fn_addr_eq(*f, attach_tab as fn() -> anyhow::Result<()>) {
                        match other_session_names_for_move_window() {
                            Ok(names) if names.is_empty() => {
                                return KeyPress::Notice(
                                    "move window: no other sessions".to_string(),
                                );
                            }
                            Ok(_) => {}
                            Err(e) => {
                                return KeyPress::Notice(format!("move window: {e:#}"));
                            }
                        }
                    }
                    return KeyPress::Execute(*f);
                }
                crate::keynode::KeyNodeKind::SessionList => {
                    state.enter_session_list_picker();
                    return KeyPress::Redraw;
                }
                crate::keynode::KeyNodeKind::CloseWindow => return close_window_keypress(),
                crate::keynode::KeyNodeKind::PromptAction {
                    prompt,
                    initial_fn,
                    confirm_fn,
                    allow_empty_confirm,
                } => {
                    return KeyPress::OpenInput {
                        prompt,
                        initial: initial_fn(),
                        confirm: *confirm_fn,
                        allow_empty_confirm: *allow_empty_confirm,
                    };
                }
                crate::keynode::KeyNodeKind::Group { icon, nodes } => {
                    state.nodes = nodes;
                    state.icon = icon;
                    state.label = node.label;
                    if std::ptr::eq(nodes.as_ptr(), keymap::PANE_NODES.as_ptr()) {
                        state.refresh_pane_rows();
                        state.root_pane_cursor_follow_active();
                    }
                    if std::ptr::eq(nodes.as_ptr(), crate::launcher::NODES.as_ptr()) {
                        state.launch_filter.clear();
                        state.recompute_launch_filter_keep(None);
                    }
                    return KeyPress::Redraw;
                }
            }
        }
    }
    KeyPress::Unrecognised
}

// ---------------------------------------------------------------------------
// Actions (tmux)
// ---------------------------------------------------------------------------

fn target() -> String {
    tmux::target_pane()
}

pub fn edit_command() -> anyhow::Result<()> {
    tmux::run_status(&[
        "send-keys",
        "-t",
        &target(),
        "C-x",
        "C-e",
    ])
}

/// Dumps pane scrollback to a temp file and opens it in **nvim** in a **new pane below** (`split-window -h`).
pub fn open_buffer() -> anyhow::Result<()> {
    let t = target();
    let cwd = tmux::pane_cwd(&t).unwrap_or_default();
    let capture = tmux::output_lossy(&[
        "capture-pane",
        "-p",
        "-S",
        "-",
        "-E",
        "-",
        "-t",
        &t,
    ])
    .context("capture-pane")?;
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let path = std::env::temp_dir().join(format!("tmux-leader-buffer-{stamp}"));
    std::fs::write(&path, capture.as_bytes()).context("write scrollback snapshot")?;
    // After plugins run: no number/sign/fold gutter; close common left-tree UIs.
    const NVIM_BUFFER_UI: &str = concat!(
        "autocmd VimEnter * ++once ",
        "setlocal nonumber norelativenumber signcolumn=no foldcolumn=0 | ",
        "silent! NvimTreeClose | silent! Neotree close | silent! NeoTree close",
    );
    let mut cmd = Command::new("tmux");
    cmd.arg("split-window")
        .arg("-h")
        .arg("-t")
        .arg(&t)
        .arg("-c")
        .arg(&cwd)
        .arg("nvim")
        .arg("-c")
        .arg(NVIM_BUFFER_UI)
        .arg(path.as_os_str());
    let st = cmd.status().context("tmux split-window nvim")?;
    anyhow::ensure!(st.success(), "tmux split-window exited with {:?}", st.code());
    Ok(())
}

/// Called at key-press time to read the current window name as the initial input value.
pub fn get_window_name() -> String {
    let t = target();
    tmux::output_lossy(&["display-message", "-t", &t, "-p", "#{window_name}"])
        .unwrap_or_default()
        .trim()
        .to_string()
}

/// Confirm callback: rename the current window to the user-supplied name.
pub fn do_rename_window(name: String) -> anyhow::Result<()> {
    let wid = tmux::initial_window_id();
    let wt = tmux::window_target(wid);
    tmux::run_status(&["rename-window", "-t", &wt, &name])
}

/// Create a new window after the current one (`new-window -a`), no name prompt.
pub fn add_window() -> anyhow::Result<()> {
    do_new_window(String::new())
}

/// Create a new window after the current one (`new-window -a`), optionally named.
fn do_new_window(name: String) -> anyhow::Result<()> {
    let t = target();
    let cwd = tmux::pane_cwd(&t).unwrap_or_default();
    let wt = tmux::window_target(tmux::initial_window_id());
    let name = name.trim().to_string();
    let mut cmd = Command::new("tmux");
    cmd.arg("new-window")
        .arg("-a")
        .arg("-t")
        .arg(&wt)
        .arg("-c")
        .arg(&cwd);
    if !name.is_empty() {
        cmd.arg("-n").arg(&name);
    }
    let st = cmd.status().context("tmux new-window")?;
    anyhow::ensure!(st.success(), "tmux new-window exited with {:?}", st.code());
    Ok(())
}

const CLOSE_WINDOW_ONLY_ONE_MSG: &str = "close window: only window in this session";

fn perform_close_tab() -> anyhow::Result<()> {
    let t = target();
    let wid = tmux::window_id_for_pane(&t)?;
    let wt = tmux::window_target(wid);
    tmux::run_status(&["select-window", "-l"])?;
    tmux::run_status(&["kill-window", "-t", &wt])
}

/// **w k** — close window or in-popup notice if this is the sole window.
pub fn close_window_keypress() -> KeyPress {
    let windows = match tmux::list_windows_for_target() {
        Ok(w) => w,
        Err(e) => {
            return KeyPress::Notice(format!("close window: {e:#}"));
        }
    };
    if windows.len() <= 1 {
        return KeyPress::Notice(CLOSE_WINDOW_ONLY_ONE_MSG.to_string());
    }
    KeyPress::Execute(perform_close_tab)
}

pub fn last_tab() -> anyhow::Result<()> {
    tmux::run_status(&["select-window", "-l"])
}

pub fn focus_tab_from_leader(id: u64) -> anyhow::Result<()> {
    let wt = tmux::window_target(id);
    tmux::run_status(&["select-window", "-t", &wt])
}

pub fn focus_pane_from_leader(pane_id: &str) -> anyhow::Result<()> {
    tmux::run_status(&["select-pane", "-t", pane_id])
}

/// Initial value for **p r** — current pane title (`#{pane_title}`), may be empty.
pub fn get_pane_title() -> String {
    let t = target();
    tmux::output_lossy(&["display-message", "-t", &t, "-p", "#{pane_title}"])
        .unwrap_or_default()
        .trim()
        .to_string()
}

/// Set the target pane’s title (`select-pane -T`).
pub fn do_rename_pane(name: String) -> anyhow::Result<()> {
    let t = target();
    tmux::run_status(&["select-pane", "-t", &t, "-T", name.trim()])
}

pub fn close_other_tabs() -> anyhow::Result<()> {
    let t = target();
    let cur = tmux::window_id_for_pane(&t)?;
    let rows = window_rows()?;
    for r in rows {
        if r.id != cur {
            let wt = tmux::window_target(r.id);
            tmux::run_status(&["kill-window", "-t", &wt]).ok();
        }
    }
    Ok(())
}

/// Split the target pane left/right (`split-window -h`).
pub fn split_pane_horizontal() -> anyhow::Result<()> {
    let t = target();
    tmux::run_status(&["split-window", "-h", "-t", &t])
}

/// Split the target pane top/bottom (`split-window -v`).
pub fn split_pane_vertical() -> anyhow::Result<()> {
    let t = target();
    tmux::run_status(&["split-window", "-v", "-t", &t])
}

/// Sessions other than the one for the leader’s target pane (for **w m** move-window).
pub(crate) fn other_session_names_for_move_window() -> anyhow::Result<Vec<String>> {
    let t = target();
    let here = tmux::session_name_for_pane(&t)?;
    let all = tmux::list_session_names()?;
    Ok(all.into_iter().filter(|s| s != &here).collect())
}

pub fn attach_tab() -> anyhow::Result<()> {
    let names = other_session_names_for_move_window()?;
    if names.is_empty() {
        return Ok(());
    }
    let labels = names.clone();
    let items: Vec<leader::PickItem> = labels
        .into_iter()
        .map(|s| leader::PickItem {
            label: s,
            focused: false,
            current: false,
        })
        .collect();
    let groups = vec![leader::PickGroup {
        label: String::new(),
        items,
    }];
    let result = leader::pick("move window to session", &groups)?;
    if let Some((_g, i)) = result {
        let name = names
            .get(i)
            .with_context(|| format!("session pick index {i}"))?;
        let src = tmux::window_target(tmux::initial_window_id());
        let dst = format!("{}:", name);
        tmux::run_status(&["move-window", "-s", &src, "-t", &dst])?;
        tmux::run_status(&["switch-client", "-t", name.as_str()])?;
    }
    Ok(())
}

/// Open `app_argv` in a new window after the current one (`new-window -a`), with tmux window name `window_name`.
fn launch_app_in_new_window(window_name: &str, app_argv: &[&str]) -> anyhow::Result<()> {
    let t = target();
    let cwd = tmux::pane_cwd(&t).context("pane cwd for launcher")?;
    let wt = tmux::window_target(tmux::initial_window_id());
    let mut cmd = Command::new("tmux");
    cmd.arg("new-window")
        .arg("-a")
        .arg("-t")
        .arg(&wt)
        .arg("-c")
        .arg(&cwd)
        .arg("-n")
        .arg(window_name);
    for a in app_argv {
        cmd.arg(a);
    }
    let st = cmd.status().context("tmux new-window (launcher)")?;
    anyhow::ensure!(st.success(), "tmux new-window exited with {:?}", st.code());
    Ok(())
}

/// macOS: print a line, then `caffeinate -i` until the pane exits.
pub fn launch_caffeinate() -> anyhow::Result<()> {
    launch_app_in_new_window(
        "caffeinate",
        &[
            "sh",
            "-c",
            r#"echo "caffeinate -i: idle sleep disabled until this pane exits (Ctrl+C or close window)."; exec caffeinate -i"#,
        ],
    )
}

pub fn launch_lazygit() -> anyhow::Result<()> {
    launch_app_in_new_window("lazygit", &["lazygit"])
}

pub fn launch_k9s() -> anyhow::Result<()> {
    launch_app_in_new_window("k9s", &["k9s"])
}

pub fn launch_lazysql() -> anyhow::Result<()> {
    launch_app_in_new_window("lazysql", &["lazysql"])
}

pub fn launch_nb() -> anyhow::Result<()> {
    launch_app_in_new_window("nb", &["sh", "-c", "nb -i"])
}

pub fn launch_nvim() -> anyhow::Result<()> {
    launch_app_in_new_window("nvim", &["nvim"])
}

// ---------------------------------------------------------------------------
// Actions (session)
// ---------------------------------------------------------------------------

pub fn last_session() -> anyhow::Result<()> {
    tmux::run_status(&["switch-client", "-l"])
}

/// Detach the current tmux client (leave sessions running in the background).
pub fn detach_session() -> anyhow::Result<()> {
    tmux::run_status(&["detach-client"])
}

/// Create or reuse a session **detached** (`new-session -d`), then `switch-client` so the real
/// client (not the `display-popup` pane) attaches — same as “outside the popup”.
pub fn do_new_session(name: String) -> anyhow::Result<()> {
    let name = name.trim();
    if name.is_empty() {
        let raw = tmux::output_lossy(&[
            "new-session",
            "-d",
            "-P",
            "-F",
            "#{session_name}",
        ])
        .context("new-session -d (unnamed)")?;
        let sname = raw.trim();
        anyhow::ensure!(!sname.is_empty(), "new-session returned empty session name");
        tmux::run_status(&["switch-client", "-t", sname])?;
        Ok(())
    } else {
        if !tmux::has_session(name)? {
            tmux::run_status(&["new-session", "-d", "-s", name])?;
        }
        tmux::run_status(&["switch-client", "-t", name])?;
        Ok(())
    }
}

/// Add session: `new-session` with no name (tmux assigns one); leader key **a** — no prompt.
pub fn add_session() -> anyhow::Result<()> {
    do_new_session(String::new())
}

/// Called at key-press time to read the current session name as the initial input value.
pub fn get_session_name() -> String {
    let t = target();
    tmux::session_name_for_pane(&t).unwrap_or_default()
}

/// Confirm callback: rename the originating session to the user-supplied name.
pub fn do_rename_session(name: String) -> anyhow::Result<()> {
    tmux::run_status(&["rename-session", "-t", tmux::session_id(), &name])
}

/// Kill the current session. If another session exists, switch the client there first; otherwise kill in place (detach / exit tmux).
pub fn kill_session() -> anyhow::Result<()> {
    let t = target();
    let cur = tmux::session_name_for_pane(&t)?;
    let all = tmux::list_session_names()?;
    if let Some(other) = all.iter().find(|n| *n != &cur).cloned() {
        tmux::run_status(&["switch-client", "-t", other.as_str()])?;
    }
    tmux::run_status(&["kill-session", "-t", &cur])
}

/// Kill every session except the one attached to this pane (current client).
pub fn kill_other_sessions() -> anyhow::Result<()> {
    let t = target();
    let cur = tmux::session_name_for_pane(&t)?;
    let names = tmux::list_session_names()?;
    for name in names {
        if name != cur {
            tmux::run_status(&["kill-session", "-t", name.as_str()]).ok();
        }
    }
    Ok(())
}

pub fn focus_session_from_leader(name: String) -> anyhow::Result<()> {
    tmux::run_status(&["switch-client", "-t", &name])
}
