@echo off
echo DEX-OS Environment Check
echo ======================
echo.

echo Checking Rust installation...
rustc --version
if %ERRORLEVEL% NEQ 0 (
    echo ERROR: Rust is not installed or not in PATH
    exit /b 1
)

echo.
echo Checking Cargo installation...
cargo --version
if %ERRORLEVEL% NEQ 0 (
    echo ERROR: Cargo is not installed or not in PATH
    exit /b 1
)

echo.
echo Checking Git installation...
git --version >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo WARNING: Git is not installed or not in PATH
) else (
    echo Git is installed
)

echo.
echo Environment check completed successfully!
echo.
echo Next steps:
echo 1. Run 'cargo build' to build the project
echo 2. Run 'cargo test' to run tests