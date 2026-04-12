//! Stage 1: print what tmux (or your shell) passes — no TUI, no `tmux` subprocess.
//! Also provides [`log_to_file`]: append-only structured log at `$TMPDIR/tmux-leader-errors.log`
//! for diagnosing F12 popup failures that are otherwise invisible (popup closes before you read them).

use std::io::IsTerminal;

// ---------------------------------------------------------------------------
// File logger
// ---------------------------------------------------------------------------

fn log_file_path() -> String {
    let dir = std::env::var("TMPDIR").unwrap_or_else(|_| "/tmp".to_string());
    let dir = dir.trim_end_matches('/').to_string();
    format!("{dir}/tmux-leader-errors.log")
}

fn now_epoch_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// Append one line to `$TMPDIR/tmux-leader-errors.log`.
/// Silent on write errors so a broken TMPDIR never crashes the leader.
pub fn log_to_file(context: &str, msg: &str) {
    use std::io::Write as _;
    let path = log_file_path();
    let ts = now_epoch_secs();
    let line = format!("[{ts}] [{context}] {msg}\n");
    if let Ok(mut f) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
    {
        let _ = f.write_all(line.as_bytes());
    }
}

/// Print argv, env, tty probes, and [`crate::tmux::parse_argv`] result. Exits successfully only if parsing succeeds.
pub fn print_params(argv_after_flag: &[String]) -> anyhow::Result<()> {
    println!("tmux-leader --print-params");
    println!("==========");
    println!("full argv:");
    for (i, a) in std::env::args().enumerate() {
        println!("  [{i}] {a:?}");
    }
    println!("==========");
    println!("argv after --print-params (what should be session / window / pane / [w h]):");
    if argv_after_flag.is_empty() {
        println!("  (empty)");
    } else {
        for (i, a) in argv_after_flag.iter().enumerate() {
            println!("  [{i}] {a:?}");
        }
    }
    println!("==========");
    println!("environment (leader cares about TMUX / TMUX_PANE):");
    for key in ["TMUX", "TMUX_PANE", "TERM", "COLUMNS", "LINES"] {
        println!("  {key}={:?}", std::env::var(key));
    }
    println!("==========");
    println!(
        "is_terminal: stdin={} stdout={} stderr={}",
        std::io::stdin().is_terminal(),
        std::io::stdout().is_terminal(),
        std::io::stderr().is_terminal(),
    );
    println!("==========");
    println!("parse_argv result:");
    match crate::tmux::parse_argv(argv_after_flag) {
        Ok(p) => {
            println!("  session_id: {:?}", p.session_id);
            println!("  window_id: {}", p.window_id);
            println!("  pane_id: {:?}", p.pane_id);
            println!(
                "  client_size (w×h from tmux, if passed): {:?}",
                p.client_size
            );
        }
        Err(e) => {
            println!("  FAILED: {e:#}");
            return Err(e);
        }
    }
    println!("==========");
    println!("Next: if the above matches your session, drop --print-params in tmux.conf and bind the real leader.");
    Ok(())
}
