.PHONY: check verify_format lint test

check: init-fake-submodules verify-format lint test

verify-format:
	cargo fmt --check


# python linting command from https://stackoverflow.com/questions/36873096/run-pylint-for-all-python-files-in-a-directory-and-all-subdirectories
lint: verify-format
	cargo clippy --tests -- -Dwarnings
	find . -type f -name "*.py" | xargs pylint

test:
	cargo build
	cargo test

init-fake-submodules:
	cp -r tests/multiple_commits_repo/git tests/multiple_commits_repo/.git
	cp -r tests/single_commit_repo/git tests/single_commit_repo/.git

copy-hooks:
	cp hooks/* .git/hooks
