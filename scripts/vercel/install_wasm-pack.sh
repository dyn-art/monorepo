#!/bin/sh

set -e # Exit immediately if a command exits with a non-zero status.

echo "Installing wasm-pack..."

# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh -s -- -y

echo "wasm-pack installation complete."
