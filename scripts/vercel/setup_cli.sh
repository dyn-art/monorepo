#!/bin/sh

# Exit immediately if a command exits with a non-zero status
set -e

RUST_ENV_FILE="$HOME/temp/rust/env.sh"

# Load the Rust-specific environment variables
source $RUST_ENV_FILE

# Set the default Rust toolchain to stable
rustup default stable

echo "⬛️ Vercel environment setup complete."
