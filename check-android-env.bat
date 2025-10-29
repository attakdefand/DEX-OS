@echo off
echo DEX-OS Android Environment Check
echo ===============================
echo.

echo Checking Android SDK...
if defined ANDROID_HOME (
    echo ANDROID_HOME = %ANDROID_HOME%
) else (
    echo WARNING: ANDROID_HOME is not set
)

echo.
echo Checking Android NDK...
if defined ANDROID_NDK_HOME (
    echo ANDROID_NDK_HOME = %ANDROID_NDK_HOME%
) else (
    echo WARNING: ANDROID_NDK_HOME is not set
)

echo.
echo Checking ADB...
adb version >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo WARNING: ADB is not installed or not in PATH
) else (
    echo ADB is installed
)

echo.
echo Checking Rust Android targets...
rustup target list | findstr android >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo WARNING: No Android targets found
) else (
    echo Android targets are installed
    rustup target list | findstr installed | findstr android
)

echo.
echo Android environment check completed!