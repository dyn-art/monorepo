#!/bin/sh

# Exit immediately if a command exits with a non-zero status
set -e

echo "🦀 Preparing Rust environment..."

# Change directory to the script's directory
cd "$(dirname "$0")"

RUST_ENV_FILE="$HOME/temp/rust/env.sh"

# Execute the scripts and source them to ensure environment changes affect the current shell
chmod +x ./install_rust.sh
source ./install_rust.sh
chmod +x ./install_wasm-pack.sh
source ./install_wasm-pack.sh

# Write the Rust-specific environment variables into a file
{
    echo "export HOME=\"$HOME\""
    echo "RUST_PROFILE=\"$HOME/.cargo/env\""
} >"$ENV_FILE"

# Append a command to source the Rust profile to the file
echo "if [ -s \"\$RUST_PROFILE\" ]; then source \"\$RUST_PROFILE\"; fi" >>"$ENV_FILE"

echo "✅ - 🦀 Rust environment is ready."
