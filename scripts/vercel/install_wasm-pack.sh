#!/bin/sh

# Exit immediately if a command exits with a non-zero status
set -e

echo "🔄 - 🦀 Installing wasm-pack..."

# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh -s -- -y

# Verify if wasm-pack is installed correctly by checking its version
wasm-pack --version

echo "✅ - 🦀 wasm-pack installation complete."
