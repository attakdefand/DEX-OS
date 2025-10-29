@echo off
REM Script to push to your forked repository

echo DEX-OS Push to Fork Script
echo =======================

echo.
echo This script will push your DEX-OS code to your forked repository.
echo.

set /p github_username="Enter your GitHub username: "

echo.
echo Setting remote URL to your fork...
cd /d d:\DEX-OS\dex-os\dex-os
git remote set-url origin https://github.com/%github_username%/DEX-OS.git

echo.
echo Pushing code to your fork...
git push -u origin main

echo.
echo Push completed! Your repository should now be available at:
echo https://github.com/%github_username%/DEX-OS

echo.
echo Your commits will appear as "Verified" because your GPG key is already set up on GitHub.

pause