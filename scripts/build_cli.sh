#!/bin/sh

# Exit immediately if a command exits with a non-zero status
set -e

echo "Building CLI..."

# Build the CLI package
turbo run build --filter=cli

# Get the absolute path to the CLI's bin directory
CLI_BIN_PATH="$(pwd)/packages/cli/bin"

# Making sure the run.sh script is executable
chmod +x "$CLI_BIN_PATH/run.sh"

# Add CLI's bin directory to the PATH
export PATH="$CLI_BIN_PATH:$PATH"

# Verify if dyn-cli is installed correctly by checking its version
dyn-cli --version