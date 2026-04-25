.PHONY: run build release check clean

run:
	cargo run

build:
	cargo build

release:
	cargo build --release

check:
	cargo check && cargo clippy

clean:
	cargo clean
