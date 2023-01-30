.PHONY: check verify_format lint

check: verify_format lint

verify_format:
	cargo fmt --check

lint:
	cargo clippy
