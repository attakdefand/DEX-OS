#!/bin/bash

# Script to push DEX-OS repository to GitHub

echo "DEX-OS Repository Push Script"
echo "============================"

# Check if we're in a git repository
if [ ! -d ".git" ]; then
    echo "Error: Not in a git repository"
    exit 1
fi

# Check current branch
CURRENT_BRANCH=$(git branch --show-current)
echo "Current branch: $CURRENT_BRANCH"

# Check if origin is set
if ! git remote get-url origin > /dev/null 2>&1; then
    echo "Setting up remote origin..."
    git remote add origin https://github.com/flodecentralizedchat-source/DEX-OS.git
fi

# Show remote URL
echo "Remote URL: $(git remote get-url origin)"

# Check if we're already up to date
echo "Checking repository status..."
git status

echo ""
echo "To push to GitHub, you may need to:"
echo "1. Use a personal access token if you haven't already"
echo "2. Or ensure you have write access to the repository"
echo ""
echo "Run the following command to push:"
echo "git push -u origin main"
echo ""
echo "If you get authentication errors, try:"
echo "git push -u https://<username>:<personal-access-token>@github.com/flodecentralizedchat-source/DEX-OS.git main"