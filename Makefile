.PHONY: generate build check test clean

# Regenerate API code from OpenAPI schemas
generate:
	cargo run -p lolzteam-codegen -- schemas/forum.json schemas/market.json src

build: generate
	cargo build

check:
	cargo check

test:
	cargo test

fmt:
	cargo fmt

clippy:
	cargo clippy --all-targets -- -D warnings

clean:
	cargo clean
