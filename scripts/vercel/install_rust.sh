#!/bin/sh

set -e # Exit immediately if a command exits with a non-zero status.

echo "Installing Rustup..."

# Explicitly set the HOME environment variable to the euid-obtained home directory
export HOME=/root

# Install Rustup (the Rust toolchain installer)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Source the Rust environment script if it exists to add Rust binaries to PATH
RUST_PROFILE="$HOME/.cargo/env"
if [ -f "$RUST_PROFILE" ]; then
    source "$RUST_PROFILE"
else
    echo "Rust profile script does not exist. The installation may have failed."
    exit 1
fi

# Verify if Rust is installed correctly by checking the version
rustc --version

# Verify if Cargo is installed correctly by checking the version
cargo --version

echo "Rust installation complete."
