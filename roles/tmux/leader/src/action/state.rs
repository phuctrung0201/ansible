//! Leader UI state: window/session/launcher rows, filters, and pill metadata.

use anyhow::Context;
use tui_input::Input;

use crate::{keymap, launcher, tmux};

use super::pills::{git_branch_pill_for_leader, leader_kube_context_display};

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

fn fuzzy_filtered_indices(filter: &str, rows: &[LeaderWindowRow]) -> Vec<usize> {
    let mut scored: Vec<(usize, u32)> = Vec::new();
    for (i, r) in rows.iter().enumerate() {
        if let Some(s) =
            crate::leader::tab_filter::fuzzy_match_score(filter, &r.label)
        {
            scored.push((i, s));
        }
    }
    scored.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0)));
    scored.into_iter().map(|(i, _)| i).collect()
}

fn clamp_picker_cursor(len: usize, cursor: &mut usize) {
    if len == 0 {
        *cursor = 0;
    } else {
        *cursor = (*cursor).min(len.saturating_sub(1));
    }
}

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

fn position_in_filtered_pick(
    filtered_indices: &[usize],
    rows: &[LeaderWindowRow],
    needle: &str,
) -> Option<usize> {
    filtered_indices
        .iter()
        .position(|&i| rows[i].label == needle)
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
        | crate::keynode::KeyNodeKind::SessionList
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
    pub session_filtered_indices: Vec<usize>,
    pub session_cursor: usize,
    pub session_filter: String,
    pub view: LeaderView,
    pub pending_input: Option<PendingInput>,
    pub launch_rows: Vec<LeaderWindowRow>,
    pub launch_filtered_indices: Vec<usize>,
    pub launch_cursor: usize,
    pub launch_filter: String,
    pub root_window_cursor: usize,
    pub pane_rows: Vec<LeaderPaneRow>,
    pub root_pane_cursor: usize,
    pub kube_pill: Option<String>,
    pub git_pill: Option<String>,
    pub notice: Option<String>,
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
            nodes: crate::keymap::KEYMAP,
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

    pub fn root_window_cursor_follow_active(&mut self) {
        let pos = self.tab_rows.iter().position(|r| r.current);
        pill_strip_cursor_follow(self.tab_rows.len(), pos, &mut self.root_window_cursor);
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
        let pos = self.pane_rows.iter().position(|r| r.current);
        pill_strip_cursor_follow(self.pane_rows.len(), pos, &mut self.root_pane_cursor);
    }

    pub fn recompute_session_filter(&mut self) {
        self.session_filtered_indices =
            fuzzy_filtered_indices(&self.session_filter, &self.session_rows);
        clamp_picker_cursor(self.session_filtered_indices.len(), &mut self.session_cursor);
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
            if let Some(p) = position_in_filtered_pick(
                &self.session_filtered_indices,
                &self.session_rows,
                name,
            ) {
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
        self.launch_filtered_indices =
            fuzzy_filtered_indices(&self.launch_filter, &self.launch_rows);
        clamp_picker_cursor(self.launch_filtered_indices.len(), &mut self.launch_cursor);
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
            if let Some(p) = position_in_filtered_pick(
                &self.launch_filtered_indices,
                &self.launch_rows,
                label,
            ) {
                let n = self.launch_filtered_indices.len();
                if n > 0 {
                    self.launch_cursor = p.min(n - 1);
                }
            }
        }
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
