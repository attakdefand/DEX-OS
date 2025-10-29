@echo off
REM Script to push DEX-OS repository to GitHub

echo DEX-OS Repository Push Script
echo ============================

REM Check if we're in a git repository
if not exist ".git" (
    echo Error: Not in a git repository
    exit /b 1
)

REM Check current branch
for /f %%i in ('git branch --show-current') do set CURRENT_BRANCH=%%i
echo Current branch: %CURRENT_BRANCH%

REM Check if origin is set
git remote get-url origin >nul 2>&1
if errorlevel 1 (
    echo Setting up remote origin...
    git remote add origin https://github.com/flodecentralizedchat-source/DEX-OS.git
)

REM Show remote URL
for /f %%i in ('git remote get-url origin') do set REMOTE_URL=%%i
echo Remote URL: %REMOTE_URL%

REM Check repository status
echo.
echo Checking repository status...
git status

echo.
echo To push to GitHub, you may need to:
echo 1. Use a personal access token if you haven't already
echo 2. Or ensure you have write access to the repository
echo.
echo Run the following command to push:
echo git push -u origin main
echo.
echo If you get authentication errors, try:
echo git push -u https://^<username^>:^<personal-access-token^>@github.com/flodecentralizedchat-source/DEX-OS.git main