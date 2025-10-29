# DEX-OS Makefile

# Variables
RUST_TOOLCHAIN = 1.80
ANDROID_TARGET = aarch64-linux-android

# Default target
.PHONY: all
all: build

# Build targets
.PHONY: build
build:
	cargo build

.PHONY: build-release
build-release:
	cargo build --release

.PHONY: build-android
build-android:
	rustup target add $(ANDROID_TARGET)
	cargo build --target $(ANDROID_TARGET) --release

# Test targets
.PHONY: test
test:
	cargo test

.PHONY: test-unit
test-unit:
	cargo test --lib

.PHONY: test-integration
test-integration:
	cargo test --test '*'

# Code quality
.PHONY: fmt
fmt:
	cargo fmt

.PHONY: clippy
clippy:
	cargo clippy

.PHONY: check
check: fmt clippy

# Docker
.PHONY: docker-build
docker-build:
	docker-compose build

.PHONY: docker-up
docker-up:
	docker-compose up -d

.PHONY: docker-down
docker-down:
	docker-compose down

# Android
.PHONY: android-install
android-install:
	adb install ui/android/app/build/outputs/apk/debug/app-debug.apk

.PHONY: android-run
android-run: android-install
	adb shell am start -n com.dexos.app/.MainActivity

# Clean
.PHONY: clean
clean:
	cargo clean
	rm -rf ui/android/app/build

# Help
.PHONY: help
help:
	@echo "DEX-OS Makefile"
	@echo ""
	@echo "Build targets:"
	@echo "  build           - Build the project"
	@echo "  build-release   - Build the project in release mode"
	@echo "  build-android   - Build for Android"
	@echo ""
	@echo "Test targets:"
	@echo "  test            - Run all tests"
	@echo "  test-unit       - Run unit tests"
	@echo "  test-integration - Run integration tests"
	@echo ""
	@echo "Code quality:"
	@echo "  fmt             - Format code"
	@echo "  clippy          - Run clippy lints"
	@echo "  check           - Run fmt and clippy"
	@echo ""
	@echo "Docker:"
	@echo "  docker-build    - Build Docker images"
	@echo "  docker-up       - Start Docker containers"
	@echo "  docker-down     - Stop Docker containers"
	@echo ""
	@echo "Android:"
	@echo "  android-install - Install on connected device"
	@echo "  android-run     - Install and run on connected device"
	@echo ""
	@echo "Clean:"
	@echo "  clean           - Clean build artifacts"