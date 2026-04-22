//! Tmux subprocess actions invoked from key bindings and pickers.

use std::process::Command;

use anyhow::Context;

use crate::tmux;

fn target() -> String {
    tmux::target_pane()
}

/// Shared `tmux new-window -a -t … -c …` builder; add `-n` and command args as needed.
fn tmux_new_window_after_current(cwd: &str) -> Command {
    let wt = tmux::window_target(tmux::initial_window_id());
    let mut cmd = Command::new("tmux");
    cmd.arg("new-window")
        .arg("-a")
        .arg("-t")
        .arg(&wt)
        .arg("-c")
        .arg(cwd);
    cmd
}

/// `new-window -a` after the **last** window in the session (bottom of the window list).
fn tmux_new_window_after_session_last(cwd: &str) -> anyhow::Result<Command> {
    let sid = tmux::session_id();
    let windows = tmux::list_windows_for_target().context("list-windows (scrollback placement)")?;
    let last_index = windows
        .iter()
        .map(|w| w.index)
        .max()
        .with_context(|| format!("session {sid:?} has no windows"))?;
    let target = format!("{sid}:{last_index}");
    let mut cmd = Command::new("tmux");
    cmd.arg("new-window")
        .arg("-a")
        .arg("-t")
        .arg(&target)
        .arg("-c")
        .arg(cwd);
    Ok(cmd)
}

pub fn edit_command() -> anyhow::Result<()> {
    tmux::run_status(&["send-keys", "-t", &target(), "C-x", "C-e"])
}

/// Remove leading / trailing whitespace-only lines from `capture-pane` output (pane padding).
fn trim_scrollback_capture(text: &str) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let mut start = 0usize;
    let mut end = lines.len();
    while start < end && lines[start].trim().is_empty() {
        start += 1;
    }
    while start < end && lines[end - 1].trim().is_empty() {
        end -= 1;
    }
    if start >= end {
        return String::new();
    }
    let body = lines[start..end].join("\n");
    if text.ends_with('\n') {
        body + "\n"
    } else {
        body
    }
}

/// Dumps pane scrollback to a temp file and opens it in **nvim** in a **new window** at the **end**
/// of the session’s window list (`new-window -a -t $session:<last-index>`).
pub fn open_scrollback() -> anyhow::Result<()> {
    let t = target();
    let cwd = tmux::pane_cwd(&t).unwrap_or_default();
    let capture = tmux::output_lossy(&["capture-pane", "-p", "-S", "-", "-E", "-", "-t", &t])
        .context("capture-pane")?;
    let capture = trim_scrollback_capture(&capture);
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let path = std::env::temp_dir().join(format!("tmux-leader-scrollback-{stamp}.txt"));
    std::fs::write(&path, capture.as_bytes()).context("write scrollback snapshot")?;
    let mut cmd = tmux_new_window_after_session_last(&cwd)?;
    cmd.arg("-n").arg("scrollback");
    // `+"…"` runs after the first file is loaded (:help +cmd): no number gutter, then EOF.
    // statuscolumn= clears any plugin-set statuscolumn (e.g. snacks.nvim) that survives the other flags.
    cmd.arg("nvim").arg(concat!(
        "+setlocal nonumber relativenumber signcolumn=no foldcolumn=0 statuscolumn= nowrap nomodifiable",
        " | silent! normal! G",
    ));
    cmd.arg(path.as_os_str());
    let st = cmd.status().context("tmux new-window nvim (scrollback)")?;
    anyhow::ensure!(st.success(), "tmux new-window exited with {:?}", st.code());
    Ok(())
}

pub fn get_window_name() -> String {
    let t = target();
    tmux::output_lossy(&["display-message", "-t", &t, "-p", "#{window_name}"])
        .unwrap_or_default()
        .trim()
        .to_string()
}

pub fn do_rename_window(name: String) -> anyhow::Result<()> {
    let wid = tmux::initial_window_id();
    let wt = tmux::window_target(wid);
    tmux::run_status(&["rename-window", "-t", &wt, &name])
}

pub fn new_window() -> anyhow::Result<()> {
    do_new_window(String::new())
}

fn do_new_window(name: String) -> anyhow::Result<()> {
    let cwd = tmux::pane_cwd(&target()).unwrap_or_default();
    let name = name.trim().to_string();
    let mut cmd = tmux_new_window_after_current(&cwd);
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

use super::key_press::KeyPress;

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

pub fn get_pane_title() -> String {
    let t = target();
    tmux::output_lossy(&["display-message", "-t", &t, "-p", "#{pane_title}"])
        .unwrap_or_default()
        .trim()
        .to_string()
}

pub fn do_rename_pane(name: String) -> anyhow::Result<()> {
    let t = target();
    tmux::run_status(&["select-pane", "-t", &t, "-T", name.trim()])
}

pub fn close_other_tabs() -> anyhow::Result<()> {
    let t = target();
    let cur = tmux::window_id_for_pane(&t)?;
    let windows = tmux::list_windows_for_target()?;
    for w in windows {
        if w.id != cur {
            let wt = tmux::window_target(w.id);
            tmux::run_status(&["kill-window", "-t", &wt]).ok();
        }
    }
    Ok(())
}

pub fn split_pane_horizontal() -> anyhow::Result<()> {
    let t = target();
    tmux::run_status(&["split-window", "-h", "-t", &t])
}

pub fn split_pane_vertical() -> anyhow::Result<()> {
    let t = target();
    tmux::run_status(&["split-window", "-v", "-t", &t])
}

fn launch_app_in_new_window(window_name: &str, app_argv: &[&str]) -> anyhow::Result<()> {
    let cwd = tmux::pane_cwd(&target()).context("pane cwd for launcher")?;
    let mut cmd = tmux_new_window_after_current(&cwd);
    cmd.arg("-n").arg(window_name);
    for a in app_argv {
        cmd.arg(a);
    }
    let st = cmd.status().context("tmux new-window (launcher)")?;
    anyhow::ensure!(st.success(), "tmux new-window exited with {:?}", st.code());
    Ok(())
}

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

pub fn launch_nvim() -> anyhow::Result<()> {
    launch_app_in_new_window("nvim", &["nvim"])
}

pub fn launch_wiki() -> anyhow::Result<()> {
    let home = std::env::var("HOME").unwrap_or_default();
    let wiki_dir = format!("{home}/wiki");
    let mut cmd = tmux_new_window_after_current(&wiki_dir);
    cmd.arg("-n").arg("wiki").arg("zsh").arg("-ic").arg("wiki");
    let st = cmd.status().context("tmux new-window (wiki)")?;
    anyhow::ensure!(st.success(), "tmux new-window exited with {:?}", st.code());
    Ok(())
}

pub fn last_session() -> anyhow::Result<()> {
    tmux::run_status(&["switch-client", "-l"])
}


pub fn do_new_session(name: String) -> anyhow::Result<()> {
    let name = name.trim();
    if name.is_empty() {
        let raw = tmux::output_lossy(&["new-session", "-d", "-P", "-F", "#{session_name}"])
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

pub fn new_session() -> anyhow::Result<()> {
    do_new_session(String::new())
}

pub fn get_session_name() -> String {
    let t = target();
    tmux::session_name_for_pane(&t).unwrap_or_default()
}

pub fn do_rename_session(name: String) -> anyhow::Result<()> {
    tmux::run_status(&["rename-session", "-t", tmux::session_id(), &name])
}

/// Rename the current session to the basename of the active pane’s working directory.
pub fn rename_session_to_pane_folder() -> anyhow::Result<()> {
    let t = target();
    let cwd = tmux::pane_cwd(&t).context("pane cwd")?;
    let name = std::path::Path::new(cwd.trim())
        .file_name()
        .map(|s| s.to_string_lossy().trim().to_string())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| anyhow::anyhow!("cannot derive folder name from cwd {cwd:?}"))?;
    do_rename_session(name)
}

pub fn kill_session() -> anyhow::Result<()> {
    let t = target();
    let cur = tmux::session_name_for_pane(&t)?;
    let all = tmux::list_session_names()?;
    if let Some(other) = all.iter().find(|n| *n != &cur).cloned() {
        tmux::run_status(&["switch-client", "-t", other.as_str()])?;
    }
    tmux::run_status(&["kill-session", "-t", &cur])
}

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

pub fn attach_session_from_leader(name: String) -> anyhow::Result<()> {
    let wt = tmux::window_target(tmux::initial_window_id());
    tmux::run_status(&["move-window", "-s", &wt, "-t", &name])?;
    tmux::run_status(&["switch-client", "-t", &name])
}
