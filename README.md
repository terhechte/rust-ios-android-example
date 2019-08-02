This is an old fork of [https://github.com/rust-windowing/android-rs-glue](https://github.com/rust-windowing/android-rs-glue) in order to add Swift / iOS support. Ignore the Android parts (use the above-linked android-rs-glue instead). The Swift/iOS part is maybe useful. What it does is the following:

- Add `C` bindings to Rust types (via Repr(C) and `cbindgen`) in [ios_c_headers.rs](https://github.com/terhechte/rust-ios-android-example/blob/master/src/ios_c_headers.rs)
- Add Xcode Project [MobileApp](MobileApp)
- Add a [build script to the Xcode project that automatically compiles Rust as a dependency](MobileApp/https://github.com/terhechte/rust-ios-android-example/blob/master/MobileApp/build-rust-xcode.sh) (Shamelessly pulled from XI-Mac)
- Configure the Rust [build.rs](https://github.com/terhechte/rust-ios-android-example/blob/master/build.rs) script to also create a C header file in `target/mobileapp-ios.h` (created on each compile)
- Add that `mobileapp-ios.h` to the Xcode project and import it
- Finally, use the imported Rust in the [ViewController](MobileApp/MobileApp/ViewController.swift)

This is all a bit undocumented. I always wanted to clean it up, but never did so.

# Prerequisites

## iOS
```
rustup target add x86_64-apple-ios
rustup target add aarch64-apple-ios 
```

## Android

1. Android SDK (w/ Android Studio)
2. Android NDK (might not be necessary, just a 500mb download, no installation required)
3. the following env needs to be set:
```
ANDROID_HOME=/Users/terhechte/Library/Android/sdk/
```
(or wherever the android SDK is installed) i.e.  `set -x ANDROID_HOME /sdjkl`
4. `PATH` should also contain `/Users/terhechte/Library/Android/sdk/platform-tools/`
(i.e. sdk platform-tools) so that the `adb` command is in path

5. Do all the stuff explained below

6. Make sure you have a running emulator
```
/Users/terhechte/Library/Android/sdk/emulator
./emulator -list-avds
./emulator -avd Nexus_5X_API_28
```
(or where you installed the sdk, and which emulator you installed via android studio)

7. To run on the device do `./run-on-android.sh` This will build, install, run

# Android Example

This example shows off how to use rust to build a native library from android
and use it through an automatically generated JNI wrapper.

## Project Structure

The file `build.rs` defines how rust_swig generates wrapper code. It
automatically finds a suitable `jni.h` and generates a rust source file against
it. Then, the script recursively looks for files ending in `.rs.in` in the
source directory and uses rust_swig to generate a JNI wrapper both in Rust and
in Java.

This build script is intended to be launched from gradle through e.g. `./gradlew
aR`. The gradle build files contain definitions on how to build the Rust
libraries and where to find them for inclusion in the apk.

## Building

### Prerequisites
To build the demo, you will need the latest version of Cargo and at least rustc
1.16. You will also need to add support for android targets:

``` shell
rustup target add arm-linux-androideabi
rustup target add aarch64-linux-android
rustup target add i686-linux-android
rustup target add x86_64-linux-android
```

To link the libraries, you will need the android NDK and generate standalone
toolchains for each target (edit the install dir as required):

Some env variables need to be present:
``` shell
# The Android NDK
set -x ANDROID_NDK ~/Desktop/android-ndk-r18/
```

``` shell
# Where you want the Android toolchains to be installed
# folder has to be created
set -x ANDROID_TOOLCHAINS ~/Desktop/Archive/mobilecore-rust/xtoolchains/
```
(use eval on fish)
``` shell
# ARM
$ANDROID_NDK/build/tools/make-standalone-toolchain.sh --platform="android-27" --toolchain=arm-linux-androideabi-4.9 --install-dir=$ANDROID_TOOLCHAINS/android-27-arm-linux-androideabi-4.9  --arch=arm
$ANDROID_NDK/build/tools/make-standalone-toolchain.sh --platform="android-27" --toolchain=aarch64-linux-android-4.9 --install-dir=$ANDROID_TOOLCHAINS/android-27-aarch64-linux-android-4.9  --arch=aarch64

# x86
$ANDROID_NDK/build/tools/make-standalone-toolchain.sh --platform="android-27" --toolchain=x86-4.9 --install-dir=$ANDROID_TOOLCHAINS/android-27-x86-4.9  --arch=x86
$ANDROID_NDK/build/tools/make-standalone-toolchain.sh --platform="android-27" --toolchain=x86_64-4.9 --install-dir=$ANDROID_TOOLCHAINS/android-27-x86_64-4.9  --arch=x86_64
```

Then edit `.cargo/config` to point to the toolchains you just generated.

Finally, wrapping Rust around `jni.h` depends on bindgen, for which you will
need libclang installed on your machine.

### Invocation

Gradle will take care of building and deploying the Rust sources. Thus, to build
the project in release mode, simply call `./gradlew androidRelease`.

To build only the rust libraries for a specific target, call cargo as usual, e.g.
`cargo build --target arm-linux-androideabi`.
