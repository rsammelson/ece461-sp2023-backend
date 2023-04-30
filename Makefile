i.PHONY: check verify_format lint test

check: init-fake-submodules verify-format lint test

verify-format:
	cargo fmt --check

lint: verify-format
	cargo clippy --tests -- -Dwarnings

test:
	cargo build
	cargo test

init-fake-submodules:
	cp -r tests/multiple_commits_repo/git tests/multiple_commits_repo/.git
	cp -r tests/single_commit_repo/git tests/single_commit_repo/.git

copy-hooks:
	cp hooks/* .git/hooks
  
runmain: 
	export GITHUB_TOKEN=$(TOKEN); export GITHUB_API_TOKEN=$(TOKEN); export LOG_FILE=log; export LOG_LEVEL=0; ./run build; ./run URL_FILE > tmp_file.txt
