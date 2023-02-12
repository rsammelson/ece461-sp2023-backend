#!/bin/bash

flags="-C instrument-coverage"
profdata="coverage/backend.profdata"

files=$( \
    RUSTFLAGS=$flags \
    cargo test --tests --no-run --message-format=json \
    | jq -r "select(.profile.test == true) | .filenames[]" \
    | grep -v dSYM - \
)

RUSTFLAGS=$flags \
    cargo test

rm coverage/*
mv default_*.profraw coverage
llvm-profdata merge -sparse coverage/default*.profraw -o "$profdata"

print_files () {
    for file in $files; do \
        printf "%s %s " -object $file; \
    done
}

if [ "$1" == "show" ] ; then
    llvm-cov show \
        $(print_files) \
        --instr-profile="$profdata" \
        --ignore-filename-regex='/.cargo/registry' \
        --ignore-filename-regex='tests.rs' \
        --ignore-filename-regex='rustc' \
        --use-color  --Xdemangler=rustfilt

else
    llvm-cov report \
        $(print_files) \
        --instr-profile="$profdata" \
        --ignore-filename-regex='/.cargo/registry' \
        --ignore-filename-regex='tests.rs' \
        --ignore-filename-regex='rustc' \
        --show-region-summary=false --show-branch-summary=false \
        --use-color \
        | less -SEXIER
fi
