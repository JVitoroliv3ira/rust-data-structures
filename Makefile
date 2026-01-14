.PHONY: test check fmt clippy

test:
	cargo test --workspace --lib --tests

check:
	cargo check --workspace

fmt:
	cargo fmt --all

clippy:
	cargo clippy --workspace -- -D warnings
