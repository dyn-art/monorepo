#!/bin/sh

set -e # Exit immediately if a command exits with a non-zero status.

echo "Installing wasm-pack..."

# Check if curl is installed
if ! command -v curl &> /dev/null; then
    echo "curl could not be found, please install it first."
    exit 1
fi

# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh -s -- -y

echo "wasm-pack installation complete."
