#!/bin/sh

# This script serves as a local execution wrapper for the dyn-cli tool.
# We are using this approach because we couldn't figure out how to
# reliable bind the "dyn-cli" command in the local development environment
# and within the constraints of a PaaS build step.
# It ensures the run.sh script within the CLI's bin directory is directly invoked
# with the necessary arguments and environment variables setup.

# Exit immediately if a command exits with a non-zero status
set -e

DIR="$(dirname "$0")"

# Check if we are running on Vercel (https://vercel.com/docs/projects/environment-variables/system-environment-variables)
if [ -n "$VERCEL_ENV" ]; then
    echo "🔄 - ⬛️ Detected Vercel environment. Setting up..."
    chmod +x "$DIR/vercel/setup_cli.sh"
    source "$DIR/vercel/setup_cli.sh"
else
    echo "🔶 Running locally..."
fi

# Find the absolute path to the monorepo's root directory
chmod +x "$DIR/find_monorepo_root.sh"
source "$DIR/find_monorepo_root.sh"
MONOREPO_ROOT=$(find_monorepo_root)
echo "👉 Monorepo root found at: $MONOREPO_ROOT"

# Get the absolute path to the CLI's bin directory
CLI_BIN_PATH="$MONOREPO_ROOT/node_modules/@ibg/cli/bin"

# Making sure the run.sh script is executable
chmod +x "$CLI_BIN_PATH/run.sh"

# Execute the run.sh script with passed arguments
"$CLI_BIN_PATH/run.sh" "$@"
