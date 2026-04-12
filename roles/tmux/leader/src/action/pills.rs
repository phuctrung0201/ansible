//! Git branch and Kubernetes context strings for the leader header pills.

use std::path::{Path, PathBuf};
use std::process::Command;

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
        let Some(rest) = line.strip_prefix("gitdir:") else {
            continue;
        };
        let rest = rest.trim();
        if rest.is_empty() {
            continue;
        }
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
        let name = rest.split('/').next_back()?.trim();
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
    // Intentionally no full `PATH` walk: that was O(dirs) syscalls every leader open.
    // `git` first plus fixed paths covers normal setups; exec still resolves PATH.
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
    if h.is_empty() {
        None
    } else {
        Some(h)
    }
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

pub(crate) fn git_branch_pill_for_leader(raw_cwd: Option<&str>) -> Option<String> {
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

fn home_dir() -> Option<PathBuf> {
    std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .map(PathBuf::from)
}

/// Paths to check for kubeconfig files: `KUBECONFIG` entries (in order), then `~/.kube/config`.
fn kubeconfig_search_paths() -> Vec<PathBuf> {
    let mut out = Vec::new();
    if let Ok(paths) = std::env::var("KUBECONFIG") {
        if !paths.is_empty() {
            for part in paths.split(kubeconfig_path_sep()) {
                if part.is_empty() {
                    continue;
                }
                out.push(PathBuf::from(part));
            }
        }
    }
    if let Some(h) = home_dir() {
        out.push(h.join(".kube/config"));
    }
    out
}

fn kube_config_present() -> bool {
    kubeconfig_search_paths().iter().any(|p| p.is_file())
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
    kubeconfig_search_paths().into_iter().find(|p| p.is_file())
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
