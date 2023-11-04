#!/bin/sh

# Make sure the script stops if there are any errors
set -e

echo "⬛️ Preparing Vercel environment"

# Check whether to build
chmod +x ./check_build.sh
source ./check_build.sh

# Prepare Rust
chmod +x ./setup_rust.sh
source ./setup_rust.sh

echo "✅ - ⬛️ Vercel environment is ready."
