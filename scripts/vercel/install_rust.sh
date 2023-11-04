#!/bin/sh

# Exit immediately if a command exits with a non-zero status
set -e

echo "🦀 Installing Rustup..."

# Install Rustup (the Rust toolchain installer)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Source the Rust environment script to add Rust binaries to PATH
RUST_PROFILE="$HOME/.cargo/env"
if [ -f "$RUST_PROFILE" ]; then
    source "$RUST_PROFILE"
else
    echo "🛑 - 🦀 Rust profile script does not exist. The installation may have failed."
    exit 1
fi

# Verify if Rust and Cargo are installed correctly by checking the versions
rustc --version
cargo --version

echo "✅ - 🦀 Rust installation complete."
