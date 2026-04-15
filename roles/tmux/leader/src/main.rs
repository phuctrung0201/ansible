mod action;
mod attach_session;
mod diag;
mod keymap;
mod keynode;
mod launcher;
mod leader;
mod tmux;

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    let tail = &argv[1..];

    // Log every invocation so pressing F12 leaves a breadcrumb even if the popup
    // closes instantly. Check: cat "$TMPDIR/tmux-leader-errors.log"
    diag::log_to_file("argv", &format!("{argv:?}"));

    // `display-popup … -E /path/tmux-leader --print-params …` usually puts the flag in argv[1],
    // but accept `--print-params` after a redundant path token if tmux ever passes one.
    if let Some(i) = tail.iter().position(|s| s == "--print-params") {
        if let Err(e) = diag::print_params(&tail[i + 1..]) {
            eprintln!("{e:#}");
            std::process::exit(1);
        }
        return;
    }

    if let Err(e) = tmux::init_from_args(tail) {
        let msg = e.to_string();
        diag::log_to_file("init_error", &msg);
        eprintln!("tmux-leader: {msg}");
        tmux::notify_client(&format!("tmux-leader: {msg}"));
        std::process::exit(1);
    }
    if let Err(e) = leader::run() {
        let msg = e.to_string();
        diag::log_to_file("run_error", &msg);
        eprintln!("tmux-leader: {msg}");
        tmux::notify_client(&format!("tmux-leader: {msg}"));
        std::process::exit(1);
    }
}
