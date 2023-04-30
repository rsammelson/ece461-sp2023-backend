# ChatGPT for suggesting setting path variable
export PATH=$PATH:/home/461team/.cargo/bin/

export GITHUB_TOKEN="$1"

export GITHUB_API_TOKEN="$1"

export LOG_FILE=log
export LOG_LEVEL=0
cargo build --release
cargo run --release -- URL_FILE
