#!/bin/bash

# Array of directories to look in
dirs_to_check=("packages" "apps")

# Array of dependencies to update
dependencies=(
    "@ibg/cli"
    "@ibg/config"
    "feature-fetch"
    "feature-logger"
    "feature-state"
    "feature-state-react"
    "figma-connect"
    "google-webfonts-client"
    "openapi-express"
    "@ibg/utils"
)

# Find the absolute path to the monorepo's root directory
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
chmod +x "$DIR/find_monorepo_root.sh"
source "$DIR/find_monorepo_root.sh"
MONOREPO_ROOT=$(find_monorepo_root)
echo "👉 Monorepo root found at: $MONOREPO_ROOT"

# Function to update dependencies in a given package
update_dependencies() {
    local sd_path=$1
    echo "🔄 Updating dependencies in $sd_path"
    if (cd "$sd_path" && pnpm update "${dependencies[@]/%/@latest}"); then
        echo "✅ Successfully updated dependencies in $sd_path"
    else
        echo "❌ Failed to update dependencies in $sd_path"
    fi
}

# Loop through specified directories and update dependencies in each sub dir
for dir in "${dirs_to_check[@]}"; do
    sub_dir=$(ls -d "$MONOREPO_ROOT/$dir"/* 2>/dev/null)
    for sd in $sub_dir; do
        if [ -d "$sd" ]; then
            echo "📦 Processing package: $sd"
            if [ -f "$sd/package.json" ]; then
                update_dependencies "$sd"
            else
                echo "⚠️ No package.json found in $sd, skipping..."
            fi
        else
            echo "⚠️ Directory $sd does not exist, skipping..."
        fi
    done
done

echo "✅ Dependency updates completed."
