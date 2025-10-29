# Android Installation Guide for DEX-OS

## Prerequisites

1. Android device with root access (recommended)
2. ADB (Android Debug Bridge) installed on your development machine
3. Android NDK installed

## Building for Android

To build DEX-OS for Android, you'll need to:

1. Install the Android target for Rust:
   ```
   rustup target add aarch64-linux-android
   ```

2. Set up the Android NDK environment variables:
   ```
   export ANDROID_NDK_HOME=/path/to/your/android/ndk
   export PATH=$PATH:$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin
   ```

3. Create a standalone toolchain:
   ```
   $ANDROID_NDK_HOME/build/tools/make_standalone_toolchain.py \
       --arch arm64 --api 28 --install-dir /tmp/ndk-arm64
   ```

4. Build the project for Android:
   ```
   cargo build --target aarch64-linux-android --release
   ```

## Installation Steps

1. Connect your Android device via USB and enable USB debugging

2. Push the binary to your device:
   ```
   adb push target/aarch64-linux-android/release/kernel /data/local/tmp/dexos
   ```

3. Set appropriate permissions:
   ```
   adb shell chmod +x /data/local/tmp/dexos
   ```

4. Run the DEX-OS kernel:
   ```
   adb shell /data/local/tmp/dexos
   ```

## Notes

- Root access may be required for certain operations
- Some features may not work on all Android devices
- Performance may vary depending on the device hardware