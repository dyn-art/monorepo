#!/bin/sh

# Exit immediately if a command exits with a non-zero status
set -e

# List of branches to allow builds for
build_branches="develop main 9-dyn-40-build-dtom-mvp-bhag"

# Helper function to check if the current branch is in the build branches list
is_build_branch() {
    for branch in $build_branches; do
        if [ "$branch" = "$1" ]; then
            return 0
        fi
    done
    return 1
}

# Proceed with build if the commit reference is a build branch
if is_build_branch "$VERCEL_GIT_COMMIT_REF"; then
    echo "✅ - Build can proceed"
    exit 1
else
    echo "🛑 - Build cancelled for branch $VERCEL_GIT_COMMIT_REF"
    exit 0
fi
