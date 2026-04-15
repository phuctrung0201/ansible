//! Run `tmux(1)` against the server inherited from the environment (`TMUX`).
//!
//! Session / window / pane scope comes from **CLI arguments** (see `tmux.conf`
//! `display-popup … -E /path/tmux-leader "#{session_id}" …` — separate argv tokens, not one `-E '…'` string),
//! not from `TMUX_LEADER_*` env vars.

use std::process::Command;
use std::sync::OnceLock;

pub struct LeaderContext {
    /// `#{session_id}` e.g. `$0`
    pub session_id: String,
    /// Numeric part of `#{window_id}` for the pane under the popup
    pub window_id: u64,
    /// `#{pane_id}` e.g. `%1`
    pub pane_id: String,
    /// Character-cell size from tmux formats (e.g. `#{client_width}` × `#{client_height}`) when passed on the CLI.
    pub client_size: Option<(u16, u16)>,
}

static CTX: OnceLock<LeaderContext> = OnceLock::new();

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ParsedArgv {
    pub session_id: String,
    pub window_id: u64,
    pub pane_id: String,
    pub client_size: Option<(u16, u16)>,
}

fn parse_wh(raw_w: &str, raw_h: &str) -> anyhow::Result<(u16, u16)> {
    let w: u16 = raw_w
        .trim()
        .parse()
        .map_err(|_| anyhow::anyhow!("client width {:?} is not a number", raw_w.trim()))?;
    let h: u16 = raw_h
        .trim()
        .parse()
        .map_err(|_| anyhow::anyhow!("client height {:?} is not a number", raw_h.trim()))?;
    anyhow::ensure!(
        w >= 1 && h >= 1,
        "client size must be at least 1×1, got {w}×{h}"
    );
    Ok((w, h))
}

/// Parse `argv` after the program name (pure; for tests and [`init_from_args`]).
pub(crate) fn parse_argv(argv: &[String]) -> anyhow::Result<ParsedArgv> {
    if argv.iter().any(|a| a.contains("#{")) {
        anyhow::bail!(
            "tmux passed literal #{{…}} placeholders (args: {:?}). \
             Formats were not expanded — usually `#` started a comment in tmux.conf.",
            argv
        );
    }

    let parsed = if argv.len() == 1 {
        parse_targets_from_combined_e_arg(&argv[0])?
    } else if argv.len() == 6 && looks_like_binary_path(&argv[0]) {
        ParsedArgv {
            session_id: argv[1].trim().to_string(),
            window_id: parse_window_id(argv[2].trim())?,
            pane_id: argv[3].trim().to_string(),
            client_size: Some(parse_wh(&argv[4], &argv[5])?),
        }
    } else if argv.len() == 4 && looks_like_binary_path(&argv[0]) {
        ParsedArgv {
            session_id: argv[1].trim().to_string(),
            window_id: parse_window_id(argv[2].trim())?,
            pane_id: argv[3].trim().to_string(),
            client_size: None,
        }
    } else if argv.len() == 5 && !looks_like_binary_path(&argv[0]) {
        ParsedArgv {
            session_id: argv[0].trim().to_string(),
            window_id: parse_window_id(argv[1].trim())?,
            pane_id: argv[2].trim().to_string(),
            client_size: Some(parse_wh(&argv[3], &argv[4])?),
        }
    } else if argv.len() == 3 {
        ParsedArgv {
            session_id: argv[0].trim().to_string(),
            window_id: parse_window_id(argv[1].trim())?,
            pane_id: argv[2].trim().to_string(),
            client_size: None,
        }
    } else {
        anyhow::bail!(
            "tmux-leader: got {} argument(s) after the program name: {:?}\n\
             \n\
             Need 3: session_id @window %pane\n\
             or 5: … plus client_width client_height (decimals)\n\
             or 1: one shell-style line: /path/tmux-leader $sess @win %pane [w h]\n\
             \n\
             tmux (see `tmux list-commands display-popup`): separate tokens after -E, each format in double quotes:\n\
 display-popup … -E /path/tmux-leader \"#{{session_id}}\" \"#{{window_id}}\" \"#{{pane_id}}\"\n\
             Unquoted # in tmux.conf starts a comment — \"#{{…}}\" so tmux expands formats.",
            argv.len(),
            argv
        );
    };

    anyhow::ensure!(
        !parsed.session_id.is_empty() && !parsed.pane_id.is_empty(),
        "session_id and pane_id must be non-empty"
    );
    Ok(parsed)
}

/// Preferred init path: no CLI args needed.
///
/// First tries `LEADER_SID` / `LEADER_WID` / `LEADER_PID` env vars set by
/// `display-popup -e "LEADER_SID=#{session_id}"` etc. — evaluated against the originating
/// pane at key-press time, so `$`/`@`/`%` ids are never shell-interpolated.
///
/// Falls back to `TMUX_PANE` (auto-set by tmux) + a `display-message` query for the
/// window's active pane (the pre-popup pane, not the popup overlay itself).
fn init_from_env() -> anyhow::Result<()> {
    // --- path 1: explicit env vars from display-popup -e ---
    let sid_env = std::env::var("LEADER_SID").ok();
    let wid_env = std::env::var("LEADER_WID").ok();
    let pid_env = std::env::var("LEADER_PID").ok();

    let is_expanded = |v: &str| !v.contains("#{");

    if let (Some(sid), Some(wid), Some(pid)) = (&sid_env, &wid_env, &pid_env) {
        if is_expanded(sid) && is_expanded(wid) && is_expanded(pid) {
            crate::diag::log_to_file("init_path", "env-vars (LEADER_SID/WID/PID)");
            let window_id = parse_window_id(wid.trim())?;
            anyhow::ensure!(
                !sid.is_empty() && !pid.is_empty(),
                "LEADER_* env vars are empty"
            );
            return CTX
                .set(LeaderContext {
                    session_id: sid.trim().to_string(),
                    window_id,
                    pane_id: pid.trim().to_string(),
                    client_size: None,
                })
                .map_err(|_| anyhow::anyhow!("leader context already initialized"));
        }
        crate::diag::log_to_file(
            "init_path",
            &format!("env-vars unexpanded (LEADER_SID={sid:?}), falling back to TMUX_PANE"),
        );
    }

    // --- path 2: TMUX_PANE (auto-set by tmux for every popup) ---
    crate::diag::log_to_file("init_path", "TMUX_PANE fallback");
    let popup_pane = std::env::var("TMUX_PANE")
        .map_err(|_| anyhow::anyhow!("TMUX_PANE not set — is tmux-leader running inside tmux?"))?;

    // Get session + window from the popup pane.
    let raw = output_lossy(&[
        "display-message",
        "-t",
        &popup_pane,
        "-p",
        "#{session_id}\t#{window_id}",
    ])?;
    let mut parts = raw.trim().splitn(2, '\t');
    let session_id = parts.next().unwrap_or("").to_string();
    let window_id_raw = parts.next().unwrap_or("").trim().to_string();
    let window_id = parse_window_id(&window_id_raw)?;
    anyhow::ensure!(
        !session_id.is_empty(),
        "display-message returned empty session_id"
    );

    // Use the window's currently-active pane as the target (the originating pane that was
    // active before the popup overlay, not the popup pane itself).
    let window_target = format!("{}:{}", session_id, window_id_raw);
    let pane_id = output_lossy(&["display-message", "-t", &window_target, "-p", "#{pane_id}"])?
        .trim()
        .to_string();
    anyhow::ensure!(
        !pane_id.is_empty(),
        "could not resolve active pane for window {window_target}"
    );

    CTX.set(LeaderContext {
        session_id,
        window_id,
        pane_id,
        client_size: None,
    })
    .map_err(|_| anyhow::anyhow!("leader context already initialized"))?;
    Ok(())
}

/// Init from CLI argv (legacy / explicit override) or fall back to [`init_from_env`].
///
/// Preferred tmux.conf form — no format args needed, avoids `#` comment / quoting issues:
/// ```text
/// bind-key -n F12 display-popup -w 90% -h 30% -b rounded -E '/path/tmux-leader'
/// ```
pub fn init_from_args(argv: &[String]) -> anyhow::Result<()> {
    if argv.is_empty() {
        return init_from_env();
    }
    let p = parse_argv(argv)?;
    CTX.set(LeaderContext {
        session_id: p.session_id,
        window_id: p.window_id,
        pane_id: p.pane_id,
        client_size: p.client_size,
    })
    .map_err(|_| anyhow::anyhow!("leader context already initialized"))?;
    Ok(())
}

/// Viewport size from tmux CLI formats (`#{client_width}` × `#{client_height}`), if provided.
pub fn client_viewport_wh() -> Option<(u16, u16)> {
    CTX.get().and_then(|c| c.client_size)
}

fn looks_like_binary_path(s: &str) -> bool {
    let t = s.trim();
    t.contains("tmux-leader")
        || (t.contains('/') && !t.starts_with('$') && !t.starts_with('@') && !t.starts_with('%'))
}

/// Last tokens: `$session`, `@window`, `%pane`, optional `width` `height`; leading tokens are the binary path.
fn parse_targets_from_combined_e_arg(combined: &str) -> anyhow::Result<ParsedArgv> {
    let parts: Vec<&str> = combined.split_whitespace().collect();
    anyhow::ensure!(
        parts.len() >= 4,
        "expected '/path/tmux-leader $session @window %pane [width height]', got {} token(s)",
        parts.len()
    );
    let mut n = parts.len();
    let client_size = if n >= 6 {
        match parse_wh(parts[n - 2], parts[n - 1]) {
            Ok(wh) => {
                n -= 2;
                Some(wh)
            }
            Err(_) => None,
        }
    } else {
        None
    };
    let pane = parts[n - 1].trim();
    let win = parts[n - 2].trim();
    let sess = parts[n - 3].trim();
    anyhow::ensure!(pane.starts_with('%'), "pane id should be %n, got {pane:?}");
    anyhow::ensure!(win.starts_with('@'), "window id should be @n, got {win:?}");
    anyhow::ensure!(
        sess.starts_with('$'),
        "session id should be $n, got {sess:?}"
    );
    Ok(ParsedArgv {
        session_id: sess.to_string(),
        window_id: parse_window_id(win)?,
        pane_id: pane.to_string(),
        client_size,
    })
}

fn ctx() -> &'static LeaderContext {
    CTX.get()
        .expect("tmux-leader: context not initialized (missing CLI args?)")
}

pub fn target_pane() -> String {
    ctx().pane_id.clone()
}

pub fn session_id() -> &'static str {
    ctx().session_id.as_str()
}

/// Window id (`#{window_id}`) for the invoked pane — from argv, not a live query.
pub fn initial_window_id() -> u64 {
    ctx().window_id
}

/// Brief notice in the **client** status line (not a popup). Escapes `#` for `display-message`.
/// Ignores failure (e.g. not running under tmux).
pub fn notify_client(msg: &str) {
    let flat: String = msg.split_whitespace().collect::<Vec<_>>().join(" ");
    let mut tail: String = flat.chars().take(200).collect();
    if tail.len() < flat.len() {
        tail.push('…');
    }
    let escaped = tail.replace('#', "##");
    let _ = Command::new("tmux")
        .args(["display-message", "-d", "5000", &escaped])
        .status();
}

pub fn run_status(args: &[&str]) -> anyhow::Result<()> {
    let st = Command::new("tmux").args(args).status()?;
    anyhow::ensure!(st.success(), "tmux {:?} exited with {:?}", args, st.code());
    Ok(())
}

pub fn output_lossy(args: &[&str]) -> anyhow::Result<String> {
    let out = Command::new("tmux").args(args).output()?;
    anyhow::ensure!(
        out.status.success(),
        "tmux {:?} failed: {}",
        args,
        String::from_utf8_lossy(&out.stderr)
    );
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

pub fn pane_cwd(target: &str) -> anyhow::Result<String> {
    let s = output_lossy(&[
        "display-message",
        "-t",
        target,
        "-p",
        "#{pane_current_path}",
    ])?;
    Ok(s.trim().to_string())
}

pub fn window_id_for_pane(_target: &str) -> anyhow::Result<u64> {
    Ok(initial_window_id())
}

fn parse_window_id(raw: &str) -> anyhow::Result<u64> {
    raw.trim_start_matches('@')
        .parse::<u64>()
        .map_err(|e| anyhow::anyhow!("window id {raw:?}: {e}"))
}

/// `-t` for window-scoped commands: `$session:@n`
pub fn window_target(window_id: u64) -> String {
    format!("{}:@{}", ctx().session_id, window_id)
}

pub fn session_name_for_pane(target: &str) -> anyhow::Result<String> {
    let s = output_lossy(&["display-message", "-t", target, "-p", "#{session_name}"])?;
    Ok(s.trim().to_string())
}

/// `true` if `tmux has-session -t` succeeds.
pub fn has_session(target: &str) -> anyhow::Result<bool> {
    let st = Command::new("tmux")
        .args(["has-session", "-t", target])
        .status()
        .map_err(|e| anyhow::anyhow!("tmux has-session: {e}"))?;
    Ok(st.success())
}

pub fn list_session_names() -> anyhow::Result<Vec<String>> {
    let raw = output_lossy(&["list-sessions", "-F", "#{session_name}"])?;
    Ok(raw
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect())
}

pub fn list_windows_for_target() -> anyhow::Result<Vec<WindowLine>> {
    const FMT: &str = concat!(
        "#{window_id}",
        "\u{1f}",
        "#{window_index}",
        "\u{1f}",
        "#{window_name}",
        "\u{1f}",
        "#{?window_active,1,0}"
    );
    let list_t = session_id();
    let raw = output_lossy(&["list-windows", "-t", list_t, "-F", FMT])?;
    let mut out = Vec::new();
    for line in raw.lines() {
        let parts: Vec<&str> = line.split('\u{1f}').collect();
        if parts.len() != 4 {
            continue;
        }
        let id = parse_window_id(parts[0].trim())?;
        let index: u64 = parts[1].trim().parse().unwrap_or(0);
        let name = parts[2].to_string();
        let active = parts[3] == "1";
        out.push(WindowLine {
            id,
            index,
            name,
            active,
        });
    }
    Ok(out)
}

pub struct WindowLine {
    pub id: u64,
    pub index: u64,
    pub name: String,
    pub active: bool,
}

/// One row from `list-panes` for the current window (`window_target(initial_window_id())`).
pub struct PaneLine {
    pub pane_id: String,
    pub index: u64,
    pub command: String,
    pub active: bool,
}

/// List panes for any valid tmux `-t` target (pane id, window, or session).
pub fn list_panes_for_target(target: &str) -> anyhow::Result<Vec<PaneLine>> {
    const FMT: &str = concat!(
        "#{pane_id}",
        "\u{1f}",
        "#{pane_index}",
        "\u{1f}",
        "#{pane_current_command}",
        "\u{1f}",
        "#{?pane_active,1,0}"
    );
    let raw = output_lossy(&["list-panes", "-t", target, "-F", FMT])?;
    let mut out = Vec::new();
    for line in raw.lines() {
        let parts: Vec<&str> = line.split('\u{1f}').collect();
        if parts.len() != 4 {
            continue;
        }
        let pane_id = parts[0].trim().to_string();
        let index: u64 = parts[1].trim().parse().unwrap_or(0);
        let command = parts[2].trim().to_string();
        let active = parts[3].trim() == "1";
        if !pane_id.is_empty() && pane_id.starts_with('%') {
            out.push(PaneLine {
                pane_id,
                index,
                command,
                active,
            });
        }
    }
    Ok(out)
}

pub fn list_panes_for_window() -> anyhow::Result<Vec<PaneLine>> {
    let wt = window_target(initial_window_id());
    list_panes_for_target(&wt)
}

pub struct SessionLine {
    pub name: String,
    pub active: bool,
}

pub fn list_sessions() -> anyhow::Result<Vec<SessionLine>> {
    const FMT: &str = concat!("#{session_name}", "\u{1f}", "#{?session_active,1,0}");
    let raw = output_lossy(&["list-sessions", "-F", FMT])?;
    let mut out = Vec::new();
    for line in raw.lines() {
        let mut parts = line.splitn(2, '\u{1f}');
        let name = parts.next().unwrap_or("").to_string();
        let active = parts.next().unwrap_or("").trim() == "1";
        if !name.is_empty() {
            out.push(SessionLine { name, active });
        }
    }
    Ok(out)
}

/// Same order and names as [`list_sessions`], but `active` is reconciled with the session this
/// client is attached to (`session_name_for_pane`), matching the session pill strip in tmux-leader.
pub fn list_sessions_reconciled_for_pane(target_pane: &str) -> anyhow::Result<Vec<SessionLine>> {
    let mut sessions = list_sessions()?;
    if let Ok(here) = session_name_for_pane(target_pane) {
        let name = here.trim();
        if !name.is_empty() {
            for s in sessions.iter_mut() {
                s.active = s.name.trim() == name;
            }
        }
    }
    Ok(sessions)
}

/// True if another pane in `lines` (typically from [`list_panes_for_target`]) is running tmux-leader.
pub fn other_leader_running(my_pane: &str, lines: &[PaneLine]) -> bool {
    for pl in lines {
        if pl.pane_id == my_pane {
            continue;
        }
        if pl.command.contains("tmux-leader") {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_argv_session_window_pane() {
        let args = vec!["$0".into(), "@2".into(), "%1".into()];
        let p = parse_argv(&args).unwrap();
        assert_eq!(p.session_id, "$0");
        assert_eq!(p.window_id, 2);
        assert_eq!(p.pane_id, "%1");
        assert_eq!(p.client_size, None);
    }

    #[test]
    fn five_argv_with_client_size() {
        let args = vec![
            "$0".into(),
            "@2".into(),
            "%1".into(),
            "120".into(),
            "40".into(),
        ];
        let p = parse_argv(&args).unwrap();
        assert_eq!(p.client_size, Some((120, 40)));
    }

    #[test]
    fn five_argv_session_id_is_dollar_one() {
        let args = vec![
            "$1".into(),
            "@2".into(),
            "%1".into(),
            "80".into(),
            "24".into(),
        ];
        let p = parse_argv(&args).unwrap();
        assert_eq!(p.session_id, "$1");
        assert_eq!(p.window_id, 2);
        assert_eq!(p.pane_id, "%1");
        assert_eq!(p.client_size, Some((80, 24)));
    }

    #[test]
    fn four_argv_with_binary_path() {
        let args = vec![
            "/opt/bin/tmux-leader".into(),
            "$3".into(),
            "@0".into(),
            "%5".into(),
        ];
        let p = parse_argv(&args).unwrap();
        assert_eq!(p.session_id, "$3");
        assert_eq!(p.window_id, 0);
        assert_eq!(p.pane_id, "%5");
        assert_eq!(p.client_size, None);
    }

    #[test]
    fn six_argv_with_binary_path_and_size() {
        let args = vec![
            "/opt/bin/tmux-leader".into(),
            "$3".into(),
            "@0".into(),
            "%5".into(),
            "80".into(),
            "24".into(),
        ];
        let p = parse_argv(&args).unwrap();
        assert_eq!(p.client_size, Some((80, 24)));
    }

    #[test]
    fn one_argv_combined() {
        let args = vec!["/home/u/.config/tmux/tmux-leader $0 @1 %2".into()];
        let p = parse_argv(&args).unwrap();
        assert_eq!(p.session_id, "$0");
        assert_eq!(p.window_id, 1);
        assert_eq!(p.pane_id, "%2");
        assert_eq!(p.client_size, None);
    }

    #[test]
    fn one_argv_combined_with_size() {
        let args = vec!["/home/u/.config/tmux/tmux-leader $0 @1 %2 100 30".into()];
        let p = parse_argv(&args).unwrap();
        assert_eq!(p.client_size, Some((100, 30)));
    }

    #[test]
    fn literal_placeholder_errors() {
        let args = vec!["#{session_id}".into(), "@1".into(), "%0".into()];
        assert!(parse_argv(&args).is_err());
    }
}
