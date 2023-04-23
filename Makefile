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

copy-hooks:
	cp hooks/* .git/hooks

exports: 
	export GITHUB_TOKEN=$(TOKEN)
	export GITHUB_API_TOKEN=$(TOKEN)
	export LOG_FILE=log
	export LOG_LEVEL=2

run: exports
	./run build
	./run <file_with_repo_url> > tmp_file.txt