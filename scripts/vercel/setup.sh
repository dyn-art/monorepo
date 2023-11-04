#!/bin/sh

# Make sure the script stops if there are any errors
set -e

echo "⬛️ Preparing Vercel environment"

DIR="$(dirname "$0")"

# Check whether to build
chmod +x "$DIR/check_build.sh"
source "$DIR/check_build.sh"

# Prepare Rust
chmod +x "$DIR/setup_rust.sh"
source "$DIR/setup_rust.sh"

echo "✅ - ⬛️ Vercel environment is ready."
