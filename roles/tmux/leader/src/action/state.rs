//! Leader UI state: window/session/launcher rows, filters, and pill metadata.

use tui_input::Input;

use crate::tmux;

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

pub struct LeaderState {
    pub nodes: &'static [crate::keynode::KeyNode],
    pub icon: &'static str,
    pub label: &'static str,
    pub tab_rows: Vec<LeaderWindowRow>,
    pub session_rows: Vec<LeaderWindowRow>,
    pub session_cursor: usize,
    pub pending_input: Option<PendingInput>,
    pub root_window_cursor: usize,
    pub pane_rows: Vec<LeaderPaneRow>,
    pub root_pane_cursor: usize,
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
            // `list-sessions` can include trailing spaces in `#{session_name}`; `session_name_for_pane`
            // returns trimmed. Without normalizing, `r.label == name` misses (e.g. second session),
            // every `current` becomes false, and the cursor stays on pill 0.
            label: s.name.trim().to_string(),
            focused: false,
            current: s.active,
        })
        .collect()
}

impl LeaderState {
    /// `startup_panes` comes from a single `list-panes` in [`crate::leader::run`] (avoids a second query).
    pub fn from_tmux(startup_panes: Vec<tmux::PaneLine>) -> Self {
        let target = tmux::target_pane();
        let target_sessions = target.clone();
        let (windows_res, sessions_res) = std::thread::scope(|s| {
            let w = s.spawn(|| tmux::list_windows_for_target());
            let se = s.spawn(move || tmux::list_sessions_reconciled_for_pane(&target_sessions));
            (
                w.join().expect("list windows join"),
                se.join().expect("list sessions join"),
            )
        });
        let tab_rows = windows_res
            .map(leader_rows_from_windows)
            .unwrap_or_default();
        let session_rows = sessions_res
            .map(leader_rows_from_sessions)
            .unwrap_or_default();
        let mut state = LeaderState {
            nodes: crate::keymap::KEYMAP,
            icon: LEADER_HEADER_ICON,
            label: "tmux leader",
            tab_rows,
            session_rows,
            session_cursor: 0,
            pending_input: None,
            root_window_cursor: 0,
            pane_rows: Vec::new(),
            root_pane_cursor: 0,
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

    pub fn refresh_session_rows(&mut self) {
        let target = tmux::target_pane();
        match tmux::list_sessions_reconciled_for_pane(&target) {
            Ok(sessions) => {
                self.session_rows = leader_rows_from_sessions(sessions);
            }
            Err(_) => self.session_rows.clear(),
        }
    }

    pub fn root_pane_cursor_follow_active(&mut self) {
        let pos = self.pane_rows.iter().position(|r| r.current);
        pill_strip_cursor_follow(self.pane_rows.len(), pos, &mut self.root_pane_cursor);
    }

    pub fn session_cursor_follow_active(&mut self) {
        // Same idea as window pills: keep tmux list order; pick the row for *this* client by name
        // (session_active can be 1 for every attached session, so name beats the flag).
        let t = tmux::target_pane();
        let pos = tmux::session_name_for_pane(&t)
            .ok()
            .map(|n| n.trim().to_string())
            .filter(|n| !n.is_empty())
            .and_then(|name| self.session_rows.iter().position(|r| r.label.trim() == name.as_str()))
            .or_else(|| self.session_rows.iter().position(|r| r.current));
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
        self.session_cursor_follow_active();
        self.root_window_cursor_follow_active();
    }

    /// Leave **w m** (move-session view) and restore the windows keymap.
    pub fn return_to_windows(&mut self) {
        self.nodes = crate::keymap::WINDOW_NODES;
        self.icon = "\u{f04e9}";
        self.label = "windows";
        self.pending_input = None;
        self.notice = None;
        self.root_window_cursor_follow_active();
    }
}
