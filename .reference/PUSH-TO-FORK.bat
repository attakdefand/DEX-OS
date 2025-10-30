@echo off
REM Script to fork and push to a new GitHub repository

echo DEX-OS Fork and Push Script
echo =========================

echo.
echo Instructions:
echo 1. First, fork the repository on GitHub:
echo    - Go to https://github.com/flodecentralizedchat-source/DEX-OS
echo    - Click the "Fork" button
echo    - Choose your GitHub account as the destination
echo.
echo 2. After forking, this script will push your code to your fork.

echo.
set /p github_username="Enter your GitHub username: "

echo.
echo Press any key to continue with the push process...
pause

echo.
echo Setting remote URL to your fork...
cd /d d:\DEX-OS\dex-os\dex-os
git remote set-url origin https://github.com/%github_username%/DEX-OS.git

echo.
echo Pushing code to your fork...
git push -u origin main

echo.
echo Push completed! Your fork should now be available at:
echo https://github.com/%github_username%/DEX-OS

echo.
echo Your commits will appear as "Verified" because your GPG key is already set up on GitHub.

echo.
echo After pushing to your fork, you can create a pull request to contribute your changes to the original repository.

pause