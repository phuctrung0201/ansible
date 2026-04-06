use crate::{keymap, leader, tmux};

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
    Input {
        prompt: &'static str,
        prefill: String,
        action: fn(String) -> anyhow::Result<()>,
    },
    Unrecognised,
}

pub fn press_key(state: &mut LeaderState, key: char) -> KeyPress {
    for node in state.nodes {
        if node.key == key {
            match &node.kind {
                keymap::KeyNodeKind::Action(f) => return KeyPress::Execute(*f),
                keymap::KeyNodeKind::InputAction { prompt, action } => {
                    let prefill = tmux::current_session().unwrap_or_default();
                    return KeyPress::Input { prompt, prefill, action: *action };
                }
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
// Window actions
// ---------------------------------------------------------------------------

pub fn window_list() -> anyhow::Result<()> {
    let windows = tmux::list_windows()?;
    if windows.is_empty() {
        return leader::show_message("windows", "no windows");
    }
    let current_idx = windows.iter().position(|w| w.active).unwrap_or(0);
    let indices: Vec<usize> = windows.iter().map(|w| w.index).collect();
    let items: Vec<leader::PickItem> = windows
        .iter()
        .map(|w| leader::PickItem {
            label: format!("{}  {}", w.index, w.name),
            current: w.active,
        })
        .collect();
    let groups = vec![leader::PickGroup {
        label: String::new(),
        items,
        initial_cursor: current_idx,
    }];
    if let Some((_gi, ii)) = leader::pick(" list windows", &groups)? {
        tmux::select_window(indices[ii])?;
    }
    Ok(())
}

pub fn last_window() -> anyhow::Result<()> {
    tmux::last_window()
}

pub fn new_window() -> anyhow::Result<()> {
    tmux::new_window()
}

pub fn close_window() -> anyhow::Result<()> {
    tmux::close_window()
}

pub fn zoom_pane() -> anyhow::Result<()> {
    tmux::zoom_pane()
}

pub fn open_buffer() -> anyhow::Result<()> {
    tmux::open_buffer()
}

pub fn move_window_to_session() -> anyhow::Result<()> {
    let sessions = tmux::list_sessions()?;
    let current = tmux::current_session()?;
    let other: Vec<_> = sessions.into_iter().filter(|s| s.name != current).collect();
    if other.is_empty() {
        return leader::show_message("move window", "no other sessions");
    }
    let items = other.iter().map(|s| leader::PickItem {
        label: s.name.clone(),
        current: false,
    }).collect();
    let groups = vec![leader::PickGroup { label: String::new(), items, initial_cursor: 0 }];
    if let Some((_gi, ii)) = leader::pick("󱂬 move window to session", &groups)? {
        tmux::move_window_to_session(&other[ii].name)?;
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Pane actions
// ---------------------------------------------------------------------------

pub fn split_h() -> anyhow::Result<()> {
    tmux::split_h()
}

pub fn split_v() -> anyhow::Result<()> {
    tmux::split_v()
}

pub fn close_pane() -> anyhow::Result<()> {
    tmux::close_pane()
}

pub fn pane_list() -> anyhow::Result<()> {
    let panes = tmux::list_panes()?;
    if panes.is_empty() {
        return leader::show_message("panes", "no panes");
    }
    let current_idx = panes.iter().position(|p| p.active).unwrap_or(0);
    let indices: Vec<usize> = panes.iter().map(|p| p.index).collect();
    let items: Vec<leader::PickItem> = panes
        .iter()
        .map(|p| leader::PickItem {
            label: format!("{}  {}", p.index, p.title),
            current: p.active,
        })
        .collect();
    let groups = vec![leader::PickGroup {
        label: String::new(),
        items,
        initial_cursor: current_idx,
    }];
    if let Some((_gi, ii)) = leader::pick(" list panes", &groups)? {
        tmux::select_pane(indices[ii])?;
    }
    Ok(())
}

pub fn last_pane() -> anyhow::Result<()> {
    tmux::last_pane()
}

// ---------------------------------------------------------------------------
// Session actions
// ---------------------------------------------------------------------------

pub fn session_list() -> anyhow::Result<()> {
    let sessions = tmux::list_sessions()?;
    if sessions.is_empty() {
        return leader::show_message("sessions", "no sessions");
    }

    let current_idx = sessions.iter().position(|s| s.attached).unwrap_or(0);
    let items: Vec<leader::PickItem> = sessions
        .iter()
        .map(|s| leader::PickItem {
            label: s.name.clone(),
            current: s.attached,
        })
        .collect();
    let names: Vec<String> = sessions.iter().map(|s| s.name.clone()).collect();
    let groups = vec![leader::PickGroup {
        label: String::new(),
        items,
        initial_cursor: current_idx,
    }];

    if let Some((_gi, ii)) = leader::pick("󱂬 switch session", &groups)? {
        tmux::switch_session(&names[ii])?;
    }
    Ok(())
}

pub fn new_session() -> anyhow::Result<()> {
    tmux::new_session()
}

pub fn last_session() -> anyhow::Result<()> {
    tmux::last_session()
}

pub fn rename_session_to(name: String) -> anyhow::Result<()> {
    tmux::rename_session_to(&name)
}

pub fn delete_session() -> anyhow::Result<()> {
    let sessions = tmux::session_names()?;
    if sessions.len() <= 1 {
        return leader::show_message("close session", "cannot close the last session");
    }
    let current = tmux::current_session()?;
    tmux::last_session()?;
    tmux::kill_session(&current)
}

pub fn session_cleanup() -> anyhow::Result<()> {
    let names = tmux::session_names()?;
    if names.is_empty() {
        return Ok(());
    }
    let main = names.iter().find(|n| n.as_str() == "main")
        .or_else(|| names.first())
        .cloned()
        .unwrap();
    tmux::switch_session(&main)?;
    for name in &names {
        if name != &main {
            let _ = tmux::kill_session(name);
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Launch actions
// ---------------------------------------------------------------------------

pub fn launch_lazygit() -> anyhow::Result<()> {
    tmux::exec_in_popup("lazygit", &[])
}

pub fn launch_lazysql() -> anyhow::Result<()> {
    tmux::exec_in_popup("lazysql", &[])
}

pub fn launch_k9s() -> anyhow::Result<()> {
    tmux::exec_in_popup("k9s", &[])
}

pub fn launch_nb() -> anyhow::Result<()> {
    tmux::exec_in_popup("nb", &["-i"])
}

pub fn launch_edit() -> anyhow::Result<()> {
    tmux::edit_command()
}
