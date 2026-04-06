use anyhow::Context;
use std::process::Command;

fn tmux() -> Command {
    Command::new("tmux")
}

pub fn run(args: &[&str]) -> anyhow::Result<()> {
    tmux().args(args).status().context("tmux command")?;
    Ok(())
}

pub fn output(args: &[&str]) -> anyhow::Result<String> {
    let out = tmux().args(args).output().context("tmux output")?;
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

fn parent_pane() -> anyhow::Result<String> {
    let pane = std::env::var("LEADER_PARENT_PANE").unwrap_or_default();
    if pane.is_empty() {
        return Err(anyhow::anyhow!("LEADER_PARENT_PANE not set"));
    }
    Ok(pane)
}

fn pane_cwd(pane: &str) -> anyhow::Result<String> {
    let raw = output(&["display-message", "-t", pane, "-p", "#{pane_current_path}"])?;
    Ok(raw.trim().to_owned())
}

// ---------------------------------------------------------------------------
// Windows
// ---------------------------------------------------------------------------

pub struct Window {
    pub index: usize,
    pub name: String,
    pub active: bool,
}

pub fn list_windows() -> anyhow::Result<Vec<Window>> {
    let raw = output(&[
        "list-windows",
        "-F",
        "#{window_index}\t#{window_name}\t#{window_active}",
    ])?;
    let mut windows = Vec::new();
    for line in raw.lines() {
        let mut parts = line.splitn(3, '\t');
        let index: usize = parts.next().unwrap_or("0").trim().parse().unwrap_or(0);
        let name = parts.next().unwrap_or("").trim().to_owned();
        let active = parts.next().unwrap_or("0").trim() == "1";
        windows.push(Window { index, name, active });
    }
    Ok(windows)
}

pub fn select_window(index: usize) -> anyhow::Result<()> {
    run(&["select-window", "-t", &format!(":{index}")])
}

pub fn new_window() -> anyhow::Result<()> {
    run(&["new-window"])
}

pub fn close_window() -> anyhow::Result<()> {
    let raw = output(&["list-windows", "-F", "x"])?;
    if raw.lines().count() <= 1 {
        run(&["display-message", "-d", "3000", "cannot close the last window"])?;
        return Ok(());
    }
    run(&["kill-window"])
}

pub fn last_window() -> anyhow::Result<()> {
    run(&["last-window"])
}

pub fn move_window_to_session(dst_session: &str) -> anyhow::Result<()> {
    let raw = output(&["list-windows", "-t", dst_session, "-F", "#{window_index}"])?;
    let max_idx: usize = raw.lines()
        .filter_map(|l| l.trim().parse().ok())
        .max()
        .unwrap_or(0);
    run(&["move-window", "-t", &format!("{dst_session}:{}", max_idx + 1)])
}

// ---------------------------------------------------------------------------
// Panes
// ---------------------------------------------------------------------------

pub fn split_h() -> anyhow::Result<()> {
    run(&["split-window", "-h", "-c", "#{pane_current_path}"])
}

pub fn split_v() -> anyhow::Result<()> {
    run(&["split-window", "-v", "-c", "#{pane_current_path}"])
}

pub fn close_pane() -> anyhow::Result<()> {
    let raw = output(&["list-panes", "-F", "x"])?;
    if raw.lines().count() <= 1 {
        run(&["display-message", "-d", "3000", "cannot close the last pane"])?;
        return Ok(());
    }
    run(&["kill-pane"])
}

pub struct Pane {
    pub index: usize,
    pub title: String,
    pub active: bool,
}

pub fn list_panes() -> anyhow::Result<Vec<Pane>> {
    let pane = parent_pane()?;
    let raw = output(&[
        "list-panes",
        "-t", &pane,
        "-F", "#{pane_index}\t#{pane_title}\t#{pane_active}",
    ])?;
    let mut panes = Vec::new();
    for line in raw.lines() {
        let mut parts = line.splitn(3, '\t');
        let index: usize = parts.next().unwrap_or("0").trim().parse().unwrap_or(0);
        let title = parts.next().unwrap_or("").trim().to_owned();
        let active = parts.next().unwrap_or("0").trim() == "1";
        panes.push(Pane { index, title, active });
    }
    Ok(panes)
}

pub fn select_pane(index: usize) -> anyhow::Result<()> {
    run(&["select-pane", "-t", &format!(".{index}")])
}

pub fn last_pane() -> anyhow::Result<()> {
    run(&["last-pane"])
}

pub fn edit_command() -> anyhow::Result<()> {
    let pane = parent_pane()?;
    // Trigger zle _tmux_edit_command_line in the parent pane after the leader popup closes
    run(&["run-shell", "-b", &format!("tmux send-keys -t {pane} C-x C-e")])
}

pub fn open_buffer() -> anyhow::Result<()> {
    let pane = parent_pane()?;
    // Capture scrollback first, then open in a popup — same file-based deferred mechanism
    // as exec_in_popup so the new popup opens cleanly after the leader popup closes.
    std::fs::write(
        "/tmp/tmux-leader-action",
        format!(
            "tmux capture-pane -t {pane} -p -S -10000 \
             | perl -0 -pe 's/\\s+$/\\n/' > /tmp/tmux-scrollback.txt; \
             tmux display-popup -t {pane} -E -b none -w 100% -h 100% \
             'nvim -R -c \"normal G\" /tmp/tmux-scrollback.txt'\n"
        ),
    )?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Sessions
// ---------------------------------------------------------------------------

pub struct Session {
    pub name: String,
    pub attached: bool,
}

pub fn list_sessions() -> anyhow::Result<Vec<Session>> {
    let raw = output(&[
        "list-sessions",
        "-F",
        "#{session_name}\t#{session_attached}",
    ])?;
    let mut sessions = Vec::new();
    for line in raw.lines() {
        let mut parts = line.splitn(2, '\t');
        let name = parts.next().unwrap_or("").trim().to_owned();
        let attached = parts.next().unwrap_or("0").trim() != "0";
        if !name.is_empty() {
            sessions.push(Session { name, attached });
        }
    }
    Ok(sessions)
}

pub fn switch_session(name: &str) -> anyhow::Result<()> {
    run(&["switch-client", "-t", name])
}

pub fn last_session() -> anyhow::Result<()> {
    run(&["switch-client", "-l"])
}

pub fn new_session() -> anyhow::Result<()> {
    let name = output(&["new-session", "-d", "-P", "-F", "#{session_name}"])?;
    let name = name.trim();
    if !name.is_empty() {
        run(&["switch-client", "-t", name])?;
    }
    Ok(())
}

pub fn rename_session_to(name: &str) -> anyhow::Result<()> {
    run(&["rename-session", "--", name])
}

pub fn current_session() -> anyhow::Result<String> {
    let out = output(&["display-message", "-p", "#{session_name}"])?;
    Ok(out.trim().to_owned())
}

pub fn kill_session(name: &str) -> anyhow::Result<()> {
    run(&["kill-session", "-t", name])
}

pub fn session_names() -> anyhow::Result<Vec<String>> {
    Ok(list_sessions()?.into_iter().map(|s| s.name).collect())
}

// ---------------------------------------------------------------------------
// Launch
// ---------------------------------------------------------------------------

pub fn exec_in_popup(program: &str, args: &[&str]) -> anyhow::Result<()> {
    let pane = parent_pane()?;
    let cwd = pane_cwd(&pane).unwrap_or_default();
    let cmd = std::iter::once(program)
        .chain(args.iter().copied())
        .collect::<Vec<_>>()
        .join(" ");
    // Write the command to a file. The keybinding's run-shell wrapper executes it after
    // display-popup (leader) returns, i.e. after the leader popup has closed.
    std::fs::write(
        "/tmp/tmux-leader-action",
        format!("tmux display-popup -t {pane} -d '{cwd}' -E -b none -w 100% -h 100% '{cmd}'\n"),
    )?;
    Ok(())
}
