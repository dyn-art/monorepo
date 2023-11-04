#!/bin/sh

# Exit immediately if a command exits with a non-zero status
set -e

echo "🦀 Preparing Rust environment..."

DIR="$(dirname "$0")"
RUST_ENV_FILE="$HOME/temp/rust/env.sh"

# Execute the scripts and source them to ensure environment changes affect the current shell
chmod +x "$DIR/install_rust.sh"
source "$DIR/install_rust.sh"
chmod +x "$DIR/install_wasm-pack.sh"
source "$DIR/install_wasm-pack.sh"

# Write the Rust-specific environment variables into a file
{
    echo "export HOME=\"$HOME\""
    echo "RUST_PROFILE=\"$HOME/.cargo/env\""
} >"$RUST_ENV_FILE"

# Append a command to source the Rust profile to the file
echo "if [ -s \"\$RUST_PROFILE\" ]; then source \"\$RUST_PROFILE\"; fi" >>"$RUST_ENV_FILE"

echo "✅ - 🦀 Rust environment is ready."
