#!/bin/bash

flags="-C instrument-coverage"
profdata="coverage/backend.profdata"

files=$( \
    RUSTFLAGS=$flags \
    cargo test --tests --no-run --message-format=json \
    | tail -n 2 | grep -oE '"filenames":\[.*\]' | sed 's/.*\["\(.*\)"\]/\1/'
    # | jq -r "select(.profile.test == true) | .filenames[]" \
    # | grep -v dSYM - \
)

RUSTFLAGS=$flags \
    cargo build

make init-fake-submodules

RUSTFLAGS=$flags \
    cargo test

[ -d coverage ] || mkdir coverage

rm -f coverage/*
mv default_*.profraw coverage
llvm-profdata-14 merge -sparse coverage/default*.profraw -o "$profdata"

print_files () {
    for file in $files; do \
        printf "%s %s " -object $file; \
    done
}

if [ "$1" == "show" ] ; then
    llvm-cov-14 show \
        $(print_files) \
        --instr-profile="$profdata" \
        --ignore-filename-regex='/.cargo/registry' \
        --ignore-filename-regex='tests.rs' \
        --ignore-filename-regex='rustc' \
        --use-color  --Xdemangler=rustfilt

else
    llvm-cov-14 report \
        $(print_files) \
        --instr-profile="$profdata" \
        --ignore-filename-regex='/.cargo/registry' \
        --ignore-filename-regex='tests.rs' \
        --ignore-filename-regex='rustc' \
        --show-region-summary=false --show-branch-summary=false \
        $( \
        if [ "$1" == "--color" ] || [ "$1" == "-c" ]; then \
            printf "%s" "--use-color"; \
            fi \
            ) \
            | less -SEXIER
fi
