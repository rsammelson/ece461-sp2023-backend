.PHONY: check verify_format lint test

check: init-fake-submodules verify-format lint test

verify-format:
	cargo fmt --check

lint:
	cargo clippy --tests -- -Dwarnings

test:
	cargo build
	cargo test

init-fake-submodules:
	cp -r tests/multiple_commits_repo/git tests/multiple_commits_repo/.git
	cp -r tests/single_commit_repo/git tests/single_commit_repo/.git
