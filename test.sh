#!/bin/bash

output=$(./coverage.sh 2> /dev/null)

pass=$(echo "$output" | grep -oE '[0-9]+ passed' | sed "s/ passed//")
fail=$(echo "$output" | grep -oE '[0-9]+ failed' | sed "s/ failed//")

total=$(expr $pass + $fail)

percent=$(echo "$output" | grep -oE "[0-9]+\.[0-9]+%\$" | tail -n 1)

echo "$pass/$total test cases passed. $percent line coverage achieved."
