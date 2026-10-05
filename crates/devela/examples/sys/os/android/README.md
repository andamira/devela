# Android examples

Small Android experiments for devela, developed from the native Rust/NDK side.

The goal is to keep Android as a thin host substrate: application state,
rendering models, events, and runtime abstractions should remain portable
where possible, while Android-specific code stays at the platform boundary.

## Progression

The examples are intended to grow in small vertical slices:

1. `raw` — compile and run a normal Rust/devela executable through `adb`.
2. logging — write through Android's native log facility.
3. native activity — load a Rust `cdylib` from a minimal Android activity shell.
4. window — acquire an `ANativeWindow` and draw a software raster buffer.
5. input — receive Android input and normalize it into devela events.
6. lifecycle — suspend, resume, destroy, and reconstruct application state cleanly.
7. packaging — make APK assembly/install/run reproducible from repository tooling.

The first slice deliberately does not require Android Studio, Gradle, Java, or Kotlin.

## Code boundaries

Android-specific foundations belong in a few distinct places:

```text
src/sys/os/android/                 Android host/OS concepts and native interfaces
src/sys/device/display/android/     Android windows, surfaces, and display buffers
src/run/                            portable application lifecycle/runtime concepts
src/ui/event/                       normalized input/event vocabulary
```

Raw Android activity/input/window handles may originate in `sys`, but portable
application lifecycle and normalized UI events should not become Android-shaped.

## Host requirements

The minimal native path needs:

- a Rust Android target;
- the Android NDK;
- `adb` from Android Platform-Tools;
- `cargo-ndk` as a thin helper around Cargo/NDK environment setup.

Rust supports `aarch64-linux-android` with `std`.
For a modern physical Android phone, ARM64 is the first target to install.

```sh
rustup target add aarch64-linux-android
cargo install cargo-ndk --locked
```

### NDK

Use the current Android NDK LTS release. The initial devela Android bring-up is
pinned/tested against:

```text
NDK r30
30.0.16248370
```

The NDK can be installed without Android Studio. Download the Linux archive from:

```text
https://developer.android.com/ndk/downloads
```

For example, unpack it under a local tools directory and point `cargo-ndk` at it:

```sh
mkdir -p "$HOME/.local/opt"
unzip ~/Downloads/android-ndk-r30-linux.zip -d "$HOME/.local/opt"
export ANDROID_NDK_HOME="$HOME/.local/opt/android-ndk-r30"
```

Add the export to your shell configuration once the location is settled.

### adb / Platform-Tools

Download the current Android SDK Platform-Tools for Linux from:

```text
https://developer.android.com/tools/releases/platform-tools
```

Unpack the archive somewhere stable, for example:

```sh
mkdir -p "$HOME/.local/opt/android"
unzip ~/Downloads/platform-tools-latest-linux.zip -d "$HOME/.local/opt/android"
export PATH="$HOME/.local/opt/android/platform-tools:$PATH"
```

No complete Android SDK installation is required for the first raw-binary step.
Later APK-building examples will document any additional SDK/build-tool packages
they actually require.

## Prepare a phone

On the phone:

1. enable Developer options;
2. enable USB debugging;
3. connect the phone by USB;
4. accept the debugging/RSA authorization prompt for the development machine.

Verify the connection:

```sh
adb devices
```

Inspect the device ABI:

```sh
adb shell getprop ro.product.cpu.abi
adb shell uname -m
```

A typical current ARM64 phone reports `arm64-v8a` / `aarch64`.

## First example

Enter the raw example directory:

```sh
cd raw
```

Check it without linking:

```sh
cargo check --target aarch64-linux-android
```

Build it through the NDK:

```sh
cargo ndk -t arm64-v8a build
```

Run it on the connected phone:

```sh
cargo ndk -t arm64-v8a run
```

The local `.cargo/config.toml` uses `cargo ndk-runner`, which pushes the built
binary through `adb` and executes it in the Android shell.

The helper script provides the same first operations:

```sh
./run.sh check
./run.sh build
./run.sh run
```

`run` is the default.

## Environment checks

Useful diagnostics:

```sh
rustc --print target-list | grep android
cargo ndk-env -t arm64-v8a
adb devices -l
adb shell getprop ro.build.version.sdk
adb shell getprop ro.product.cpu.abilist
```

## Later SDK tooling

When the examples begin producing APKs, prefer installing only the SDK packages
that the packaging step actually uses. Android's current command-line tooling
can manage packages without requiring Android Studio.

Keep the NDK version explicit for reproducible builds; update it deliberately
rather than silently following whatever happens to be newest on a host machine.
