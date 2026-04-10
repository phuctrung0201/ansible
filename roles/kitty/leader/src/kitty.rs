use anyhow::Context;
use std::process::Command;

/// Prepend /opt/homebrew/bin to PATH for kitten invocations.
fn kitten_cmd() -> Command {
    let path = format!(
        "/opt/homebrew/bin:{}",
        std::env::var("PATH").unwrap_or_default()
    );
    let mut cmd = Command::new("kitten");
    cmd.env("PATH", path);
    cmd
}

/// Ignore SIGHUP so the process survives after the overlay window is closed.
fn ignore_hup() {
    unsafe { libc::signal(libc::SIGHUP, libc::SIG_IGN); }
}

pub fn ls() -> anyhow::Result<String> {
    let out = kitten_cmd()
        .args(["@", "ls"])
        .output()
        .context("kitten @ ls")?;
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

pub fn send_action(cmd: &str) -> anyhow::Result<()> {
    kitten_cmd()
        .args(["@", "action", cmd])
        .status()
        .context("kitten @ action")?;
    Ok(())
}

pub fn send_text(code: &str) -> anyhow::Result<()> {
    kitten_cmd()
        .args(["@", "send-text", "--", code])
        .status()
        .context("kitten @ send-text")?;
    Ok(())
}

pub fn close_window_self() -> anyhow::Result<()> {
    ignore_hup();
    kitten_cmd()
        .args(["@", "close-window", "--self"])
        .status()
        .context("kitten @ close-window --self")?;
    Ok(())
}

pub fn close_tab_self() -> anyhow::Result<()> {
    ignore_hup();
    kitten_cmd()
        .args(["@", "close-tab", "--self"])
        .status()
        .context("kitten @ close-tab --self")?;
    Ok(())
}

pub fn focus_tab_recent() -> anyhow::Result<()> {
    kitten_cmd()
        .args(["@", "focus-tab", "--match", "recent:1"])
        .status()
        .context("focus recent tab")?;
    Ok(())
}

pub fn focus_tab(id: u64) -> anyhow::Result<()> {
    kitten_cmd()
        .args(["@", "focus-tab", "--match", &format!("id:{id}")])
        .status()
        .context("kitten @ focus-tab")?;
    Ok(())
}

pub fn close_tab(id: u64) -> anyhow::Result<()> {
    kitten_cmd()
        .args(["@", "close-tab", "--match", &format!("id:{id}")])
        .status()
        .context("kitten @ close-tab")?;
    Ok(())
}

pub fn detach_tab_self(target_tab_id: u64) -> anyhow::Result<()> {
    kitten_cmd()
        .args([
            "@",
            "detach-tab",
            "--self",
            "--target-tab",
            &format!("id:{target_tab_id}"),
        ])
        .status()
        .context("kitten @ detach-tab")?;
    Ok(())
}
