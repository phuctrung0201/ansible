mod action;
mod keymap;
mod leader;
mod tmux;

fn main() {
    if let Err(e) = leader::run() {
        let msg = format!("leader error: {}", e);
        let _ = std::process::Command::new("tmux")
            .args(["display-message", "-d", "4000", &msg])
            .status();
    }
}
