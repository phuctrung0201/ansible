mod action;
mod keymap;
mod kitty;
mod leader;

fn main() -> anyhow::Result<()> {
    leader::run()
}
