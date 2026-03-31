.PHONY: run build install

run:
	cargo run

build:
	cargo build --release

install: build
	cp target/release/leader ~/.config/kitty/leader
