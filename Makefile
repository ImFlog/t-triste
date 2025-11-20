.PHONY: help setup-mobile install-targets install-cargo-mobile2 \
        android-build android-run android-release \
        ios-build ios-run ios-release \
        clean-android clean-ios clean

# Default target
help:
	@echo "T-Triste Mobile Build Commands"
	@echo "==============================="
	@echo ""
	@echo "Setup:"
	@echo "  make setup-mobile         - Install all required mobile build tools"
	@echo "  make install-targets      - Install Android and iOS Rust targets"
	@echo "  make install-cargo-mobile2 - Install cargo-mobile2 tool"
	@echo ""
	@echo "Android:"
	@echo "  make android-build        - Build Android APK (debug)"
	@echo "  make android-release      - Build Android APK (release)"
	@echo "  make android-run          - Build and run on Android device/emulator"
	@echo "  make clean-android        - Clean Android build artifacts"
	@echo ""
	@echo "iOS:"
	@echo "  make ios-build            - Build iOS app (debug)"
	@echo "  make ios-release          - Build iOS app (release)"
	@echo "  make ios-run              - Build and run on iOS device/simulator"
	@echo "  make clean-ios            - Clean iOS build artifacts"
	@echo ""
	@echo "Other:"
	@echo "  make clean                - Clean all build artifacts"
	@echo ""

# ============================================================================
# Setup and Installation
# ============================================================================

setup-mobile: install-cargo-mobile2 install-targets
	@echo "Mobile build tools installed successfully!"

install-cargo-mobile2:
	@echo "Installing cargo-mobile2..."
	cargo install --git https://github.com/tauri-apps/cargo-mobile2

install-targets:
	@echo "Installing Android targets..."
	rustup target add aarch64-linux-android
	rustup target add armv7-linux-androideabi
	rustup target add x86_64-linux-android
	rustup target add i686-linux-android
	@echo "Installing iOS targets..."
	rustup target add aarch64-apple-ios
	rustup target add x86_64-apple-ios
	rustup target add aarch64-apple-ios-sim

# ============================================================================
# Android Build Commands
# ============================================================================

android-build:
	@echo "Building Android APK (debug)..."
	cd t-triste && cargo mobile apk build --dev

android-release:
	@echo "Building Android APK (release)..."
	cd t-triste && cargo mobile apk build --release

android-run:
	@echo "Building and running on Android..."
	cd t-triste && cargo mobile apk run --dev

clean-android:
	@echo "Cleaning Android build artifacts..."
	cd t-triste && cargo mobile android clean

# ============================================================================
# iOS Build Commands
# ============================================================================

ios-build:
	@echo "Building iOS app (debug)..."
	cd t-triste && cargo mobile ios build --dev

ios-release:
	@echo "Building iOS app (release)..."
	cd t-triste && cargo mobile ios build --release

ios-run:
	@echo "Building and running on iOS simulator..."
	cd t-triste && cargo mobile ios run --dev

clean-ios:
	@echo "Cleaning iOS build artifacts..."
	cd t-triste && cargo mobile ios clean

# ============================================================================
# General Commands
# ============================================================================

clean: clean-android clean-ios
	@echo "Cleaning all build artifacts..."
	cargo clean
