.PHONY: check verify_format lint test

check: verify_format lint test

verify_format:
	cargo fmt --check

lint:
	cargo clippy -- -Dwarnings

test:
	cargo test
