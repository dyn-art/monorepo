#!/bin/sh

set -e # Exit immediately if a command exits with a non-zero status.

echo "Building CLI..."

# Build the CLI
turbo run build --filter=cli

# Add CLI's bin directory to PATH
CLI_PATH="$(pwd)/packages/cli/bin"
export PATH="$CLI_PATH:$PATH"

# Verify if Rust is installed correctly by checking the version
dyn-cli --version