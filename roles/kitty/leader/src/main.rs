mod action;
mod keymap;
mod kitty;
mod leader;

fn main() {
    if let Err(e) = leader::run() {
        let _ = leader::show_message("error", &e.to_string());
    }
}
