#!/bin/bash

set -e # Exit immediately if a command exits with a non-zero status.

echo "Installing Rustup..."

# Check if curl is installed
if ! command -v curl &> /dev/null; then
    echo "curl could not be found, please install it first."
    exit 1
fi

# Install Rustup (the Rust toolchain installer)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Adding binaries to the path by modifying the profile file
RUST_PROFILE="$HOME/.cargo/env"
if [ -f "$RUST_PROFILE" ]; then
    source "$RUST_PROFILE"
else
    echo "Rust profile script does not exist. The installation may have failed."
    exit 1
fi

echo "Rust installation complete."
