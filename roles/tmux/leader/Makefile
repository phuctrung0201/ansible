.PHONY: run build install

run:
	cargo run

build:
	cargo build --release

install: build
	cp target/release/tmux-leader ~/.config/tmux/leader
