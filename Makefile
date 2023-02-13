.PHONY: check verify_format lint test

check: verify_format lint test

verify_format:
	cargo fmt --check

lint:
	cargo clippy -- -Dwarnings

test:
	cargo test

init-fake-submodules:
	cp -r tests/multiple_commits_repo/git tests/multiple_commits_repo/.git
	cp -r tests/single_commit_repo/git tests/single_commit_repo/.git
