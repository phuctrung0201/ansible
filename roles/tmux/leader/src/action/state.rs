//! Leader UI state: window/session/launcher rows, filters, and pill metadata.

use anyhow::Context;
use tui_input::Input;

use crate::{launcher, tmux};

use super::pills::{git_branch_pill_for_leader, leader_kube_context_display};

/// Full-screen rename-style prompt within the popup (section rule, hint, bordered field).
pub struct PendingInput {
    pub prompt: &'static str,
    pub input: Input,
    pub confirm: fn(String) -> anyhow::Result<()>,
    pub allow_empty_confirm: bool,
}

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
const PILL_STRIP_CAP: usize = 24;

fn pill_strip_cursor_follow(rows_len: usize, current_idx: Option<usize>, cursor: &mut usize) {
    let n = rows_len.min(PILL_STRIP_CAP);
    if n == 0 {
        *cursor = 0;
        return;
    }
    if let Some(i) = current_idx {
        *cursor = i.min(n - 1);
    } else {
        *cursor = (*cursor).min(n - 1);
    }
}

pub(crate) fn window_rows() -> anyhow::Result<Vec<LeaderWindowRow>> {
    let lines = tmux::list_windows_for_target()?;
    let mut rows = Vec::new();
    for w in lines {
        let label = if w.name.is_empty() {
            format!("window {}", w.index)
        } else {
            w.name.clone()
        };
        rows.push(LeaderWindowRow {
            id: w.id,
            label,
            focused: false,
            current: w.active,
        });
    }
    Ok(rows)
}

pub fn leader_launch_rows() -> Vec<LeaderWindowRow> {
    launcher::NODES
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
    let node = launcher::NODES
        .get(index)
        .with_context(|| format!("launch index {index} out of range"))?;
    match &node.kind {
        crate::keynode::KeyNodeKind::Action(f) => f(),
        crate::keynode::KeyNodeKind::PromptAction { .. }
        | crate::keynode::KeyNodeKind::Group { .. }
        | crate::keynode::KeyNodeKind::CloseWindow => {
            anyhow::bail!("launch node must be an action")
        }
    }
}

pub struct LeaderState {
    pub nodes: &'static [crate::keynode::KeyNode],
    pub icon: &'static str,
    pub label: &'static str,
    pub tab_rows: Vec<LeaderWindowRow>,
    pub session_rows: Vec<LeaderWindowRow>,
    pub session_cursor: usize,
    pub pending_input: Option<PendingInput>,
    pub launch_rows: Vec<LeaderWindowRow>,
    pub launch_cursor: usize,
    pub root_window_cursor: usize,
    pub pane_rows: Vec<LeaderPaneRow>,
    pub root_pane_cursor: usize,
    pub kube_pill: Option<String>,
    pub git_pill: Option<String>,
    pub notice: Option<String>,
}

fn leader_rows_from_windows(lines: Vec<tmux::WindowLine>) -> Vec<LeaderWindowRow> {
    let mut rows = Vec::new();
    for w in lines {
        let label = if w.name.is_empty() {
            format!("window {}", w.index)
        } else {
            w.name.clone()
        };
        rows.push(LeaderWindowRow {
            id: w.id,
            label,
            focused: false,
            current: w.active,
        });
    }
    rows
}

fn leader_rows_from_sessions(sessions: Vec<tmux::SessionLine>) -> Vec<LeaderWindowRow> {
    sessions
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

impl LeaderState {
    /// `startup_panes` comes from a single `list-panes` in [`crate::leader::run`] (avoids a second query).
    pub fn from_tmux(startup_panes: Vec<tmux::PaneLine>) -> Self {
        let target = tmux::target_pane();
        let (windows_res, sessions_res, cwd_res) = std::thread::scope(|s| {
            let w = s.spawn(|| tmux::list_windows_for_target());
            let se = s.spawn(|| tmux::list_sessions());
            let cwd = s.spawn(|| tmux::pane_cwd(&target));
            (
                w.join().expect("list windows join"),
                se.join().expect("list sessions join"),
                cwd.join().expect("pane cwd join"),
            )
        });
        let tab_rows = windows_res
            .map(leader_rows_from_windows)
            .unwrap_or_default();
        let session_rows = sessions_res
            .map(leader_rows_from_sessions)
            .unwrap_or_default();
        let launch_rows = leader_launch_rows();
        let raw_cwd = cwd_res.ok().filter(|s| !s.is_empty());
        let (git_pill, kube_pill) = std::thread::scope(|s| {
            let g = s.spawn(|| git_branch_pill_for_leader(raw_cwd.as_deref()));
            let k = s.spawn(|| leader_kube_context_display());
            (
                g.join().expect("git pill join"),
                k.join().expect("kube pill join"),
            )
        });
        let mut state = LeaderState {
            nodes: crate::keymap::KEYMAP,
            icon: LEADER_HEADER_ICON,
            label: "tmux leader",
            tab_rows,
            session_rows,
            session_cursor: 0,
            pending_input: None,
            launch_rows,
            launch_cursor: 0,
            root_window_cursor: 0,
            pane_rows: Vec::new(),
            root_pane_cursor: 0,
            kube_pill,
            git_pill,
            notice: None,
        };
        state.session_cursor_follow_active();
        state.root_window_cursor_follow_active();
        state.hydrate_pane_rows_from_tmux_lines(startup_panes);
        state.root_pane_cursor_follow_active();
        state
    }

    pub fn root_window_cursor_follow_active(&mut self) {
        let pos = self.tab_rows.iter().position(|r| r.current);
        pill_strip_cursor_follow(self.tab_rows.len(), pos, &mut self.root_window_cursor);
    }

    fn hydrate_pane_rows_from_tmux_lines(&mut self, mut rows: Vec<tmux::PaneLine>) {
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

    pub fn refresh_pane_rows(&mut self) {
        let Ok(rows) = tmux::list_panes_for_window() else {
            self.pane_rows.clear();
            return;
        };
        self.hydrate_pane_rows_from_tmux_lines(rows);
    }

    pub fn root_pane_cursor_follow_active(&mut self) {
        let pos = self.pane_rows.iter().position(|r| r.current);
        pill_strip_cursor_follow(self.pane_rows.len(), pos, &mut self.root_pane_cursor);
    }

    pub fn session_cursor_follow_active(&mut self) {
        let pos = self.session_rows.iter().position(|r| r.current);
        pill_strip_cursor_follow(self.session_rows.len(), pos, &mut self.session_cursor);
    }

    #[inline]
    pub fn selected_session_name(&self) -> Option<String> {
        let n = self.session_rows.len().min(PILL_STRIP_CAP);
        if n == 0 {
            return None;
        }
        self.session_rows
            .get(self.session_cursor.min(n - 1))
            .map(|r| r.label.clone())
    }

    pub fn selected_launch_index(&self) -> Option<usize> {
        let n = self.launch_rows.len().min(PILL_STRIP_CAP);
        if n == 0 {
            return None;
        }
        self.launch_rows
            .get(self.launch_cursor.min(n - 1))
            .map(|r| r.id as usize)
    }

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
        self.nodes = crate::keymap::KEYMAP;
        self.icon = LEADER_HEADER_ICON;
        self.label = "tmux leader";
        self.pending_input = None;
        self.notice = None;
        self.root_window_cursor_follow_active();
    }
}
