# DEX-OS

A decentralized exchange operating system built with Rust.

## Architecture

- **Kernel**: Core VM and consensus engine
- **Chains**: Multi-chain adapters (EVM, SVM, CosmWasm, IBC)
- **Services**: Off-chain microservices
- **UI**: Progressive web app, desktop application, and mobile app
- **Security**: Protection layer with ZK proofs and signature verification

## Development

### Prerequisites

- Rust 1.80+
- Docker (for local development environment)
- Android SDK and NDK (for Android deployment)
- Visual Studio Build Tools (for Windows development)

### Environment Variables

Create a `.env` file in the project root based on `.env.example`:

```bash
cp .env.example .env
```

Then edit the `.env` file to set your configuration values.

### Building

```bash
# Build for desktop
cargo build

# Build for Android
cargo build --target aarch64-linux-android
```

### Testing

```bash
# Run all tests
cargo test

# Run unit tests only
cargo test --lib

# Run integration tests only
cargo test --test '*'
```

## Deployment

### Desktop

```bash
# Build release version
cargo build --release

# Run the application
cargo run --release
```

### Android

1. Install Android SDK and NDK
2. Set environment variables:
   ```bash
   export ANDROID_HOME=/path/to/android/sdk
   export ANDROID_NDK_HOME=/path/to/android/ndk
   ```
3. Build for Android target:
   ```bash
   cargo build --target aarch64-linux-android --release
   ```
4. Package as APK using Android tooling
5. Install on device using ADB:
   ```bash
   adb install app-release.apk
   ```

## Documentation

- [Project Structure](docs/project-structure.md)
- [Build Verification](docs/build-verification.md)
- [Android Setup Verification](docs/android-setup-verification.md)
- [Comprehensive Testing Plan](tests/comprehensive-testing-plan.md)
- [Android Testing Plan](tests/android-testing-plan.md)
- [Android Installation Guide](infra/android-installation.md)

## CI/CD

The project uses GitHub Actions for continuous integration. All tests are run automatically on every commit.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for your changes
5. Run the test suite
6. Commit your changes
7. Push to your fork
8. Create a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.