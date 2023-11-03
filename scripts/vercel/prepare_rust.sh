#!/bin/sh

# Exit immediately if a command exits with a non-zero status
set -e

echo "Preparing Rust environment..."

# Make sure the scripts are executable
DIR="$(dirname "$0")"
chmod +x "$DIR/install_rust.sh"
chmod +x "$DIR/install_wasm-pack.sh"

# Execute the scripts and source it to ensure environment changes affect the current shell
source "$DIR/install_rust.sh"
source "$DIR/install_wasm-pack.sh"

echo "Rust environment is ready."