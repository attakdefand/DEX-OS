#!/bin/bash

# Script to set up and push to a new GitHub repository

echo "DEX-OS Repository Setup Script"
echo "============================="

echo ""
echo "Instructions:"
echo "1. First, create a new repository on GitHub at https://github.com/new"
echo "2. Name it \"DEX-REPO\""
echo "3. Do NOT initialize with a README"
echo "4. Click \"Create repository\""
echo ""
echo "After creating the repository, this script will push your code."

echo ""
read -p "Press Enter to continue with the push process..."

echo ""
echo "Setting remote URL to your repository..."
cd d:\DEX-OS\dex-os\dex-os
git remote set-url origin https://github.com/attakdefand/DEX-REPO.git

echo ""
echo "Pushing code to your new repository..."
git push -u origin main

echo ""
echo "Push completed! Your repository should now be available at:"
echo "https://github.com/attakdefand/DEX-REPO"

echo ""
echo "If you get authentication errors, you may need to:"
echo "1. Use a personal access token instead of password"
echo "2. Go to GitHub Settings > Developer settings > Personal access tokens"
echo "3. Generate a new token with \"repo\" scope"
echo "4. Use this token when prompted for password"