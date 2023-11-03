#!/bin/sh

set -e # Exit immediately if a command exits with a non-zero status.

echo "Preparing Rust environment..."

# Make sure the scripts are executable
DIR="$(dirname "$0")"
chmod +x "$DIR/install_rust.sh"
chmod +x "$DIR/install_wasm-pack.sh"

# Execute the scripts
"$DIR/install_rust.sh"
"$DIR/install_wasm-pack.sh"

echo "Rust environment is ready."