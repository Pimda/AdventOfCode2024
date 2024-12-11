#!/bin/bash

# Make sure we have an up-to-date release build without older builds
cargo clean
cargo build --release

# Set the target directory
target_dir="target/release"

# Loop through all files named day_* in the target/release directory
for exe in "$target_dir"/day_*; do
    # Check if the file exists and is executable
    if [ -f "$exe" ] && [ -x "$exe" ]; then
        echo "---$exe---"
        "$exe"
    fi
done