#!/bin/bash

# Make sure we have an up-to-date release build
cargo build --release

# Set the target directory
target_dir="target/release"

# Loop through all files named day_*.exe in the target/release directory
for exe in "$target_dir"/day_*.exe; do
    # Check if the file exists and is executable
    if [ -f "$exe" ] && [ -x "$exe" ]; then
        echo "---$exe---"
        "$exe"
    fi
done