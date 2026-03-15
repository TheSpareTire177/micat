.PHONY: build run init fmt lint

# Build the Micat binary
build:
	cd core && . "$$HOME/.cargo/env" && cargo build --bin micat

# Run Micat (default command: show help)
run:
	cd core && . "$$HOME/.cargo/env" && cargo run --bin micat -- --help

# Initialise Micat config for the two Docker servers
init:
	cd core && . "$$HOME/.cargo/env" && cargo run --bin micat -- init

# Format Rust code
fmt:
	cd core && . "$$HOME/.cargo/env" && cargo fmt

# Run clippy lints
lint:
	cd core && . "$$HOME/.cargo/env" && cargo clippy --all-targets --all-features -- -D warnings
