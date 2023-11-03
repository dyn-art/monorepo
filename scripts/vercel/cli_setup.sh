#!/bin/sh

# This script sets up the environment for the dyn-cli tool when running on Vercel.

# Exit immediately if a command exits with a non-zero status
set -e

# Add Rust's cargo bin directory to PATH for Vercel environment
export PATH="/root/.cargo/bin:$PATH"

# Set the default Rust toolchain to stable
rustup default stable

echo "Vercel Rust environment setup complete."
