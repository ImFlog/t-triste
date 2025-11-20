# Mobile Build Guide for T-Triste

This guide explains how to build T-Triste for iOS and Android platforms.

## Prerequisites

### For Android:
- Android SDK and NDK installed
- `ANDROID_HOME` environment variable set
- `ANDROID_NDK_ROOT` environment variable set
- Android device or emulator running

### For iOS (macOS only):
- Xcode installed with command-line tools
- iOS device connected or simulator available

## Quick Start

### 1. Install Mobile Build Tools

Run the setup command to install all necessary tools:

```bash
make setup-mobile
```

This will:
- Install `cargo-mobile2` (the mobile build tool)
- Install all required Rust targets for Android and iOS

You can also install these separately:
```bash
make install-cargo-mobile2  # Install cargo-mobile2 only
make install-targets        # Install Rust targets only
```

### 2. Build for Android

#### Debug Build
```bash
make android-build
```

#### Release Build
```bash
make android-release
```

#### Build and Run on Device/Emulator
```bash
make android-run
```

The APK will be generated in:
- Debug: `t-triste/target/android/debug/apk/`
- Release: `t-triste/target/android/release/apk/`

### 3. Build for iOS

#### Debug Build
```bash
make ios-build
```

#### Release Build
```bash
make ios-release
```

#### Build and Run on Simulator/Device
```bash
make ios-run
```

The iOS app will be generated in:
- Debug: `t-triste/target/ios/debug/`
- Release: `t-triste/target/ios/release/`

## Troubleshooting

### Android Build Issues

1. **NDK not found**: Ensure `ANDROID_NDK_ROOT` is set correctly
   ```bash
   export ANDROID_NDK_ROOT=$ANDROID_HOME/ndk/<version>
   ```

2. **SDK not found**: Ensure `ANDROID_HOME` is set correctly
   ```bash
   export ANDROID_HOME=$HOME/Android/Sdk  # Linux
   export ANDROID_HOME=$HOME/Library/Android/sdk  # macOS
   ```

3. **Build targets missing**: Run `make install-targets`

### iOS Build Issues

1. **Xcode not configured**: Run `xcode-select --install`

2. **Signing issues**: Open the generated Xcode project and configure signing:
   ```bash
   open t-triste/gen/apple/t-triste.xcodeproj
   ```

3. **Simulator not found**: List available simulators:
   ```bash
   xcrun simctl list devices
   ```

## Configuration

Mobile-specific configuration is in `t-triste/Cargo.toml`:

### Android Configuration
- **Package name**: `com.ttrieste.game`
- **Min SDK**: 28 (Android 9.0)
- **Target SDK**: 34 (Android 14)
- **Build targets**: ARM64 and ARMv7

### iOS Configuration
- **Bundle identifier**: `com.ttrieste.game`
- **Bundle name**: T-Triste
- **Version**: 0.1.0

You can modify these values in the `[package.metadata.android]` and `[package.metadata.ios]` sections.

## Cleaning Build Artifacts

```bash
make clean-android   # Clean Android builds only
make clean-ios       # Clean iOS builds only
make clean          # Clean all builds
```

## Additional Resources

- [Bevy Mobile Guide](https://bevyengine.org/learn/book/getting-started/setup/#mobile)
- [cargo-mobile2 Documentation](https://github.com/tauri-apps/cargo-mobile2)

## Available Make Commands

Run `make help` to see all available commands:

```bash
make help
```
