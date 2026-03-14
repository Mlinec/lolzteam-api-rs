.PHONY: generate build check test clean

# Regenerate API code from OpenAPI schemas
generate:
	cargo run -p lolzteam-codegen -- schemas/forum.json schemas/market.json src/generated
	cp src/generated/models.rs src/models.rs
	cp -r src/generated/forum/* src/forum/
	cp -r src/generated/market/* src/market/

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
	rm -rf src/generated
