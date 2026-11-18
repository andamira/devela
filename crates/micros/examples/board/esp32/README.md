# ESP32 board examples

Bare-metal `no_std` examples for ESP32-C3/C6 (RISC-V) and ESP32-S3 (Xtensa).

## Requirements

On Debian, Ubuntu, or Linux Mint:

````sh
sudo apt-get install -y gcc build-essential curl pkg-config libudev-dev
cargo install espflash --locked
````

**C3/C6** use upstream Rust RISC-V targets. See each board README
for its specific target and the binary utilities needed to create flash images.

**S3** requires Espressif's Xtensa Rust toolchain and GCC/binutils.
Install both with [`espup`](https://github.com/esp-rs/espup):

````sh
cargo install espup --locked
espup install --name esp --targets esp32s3 --toolchain-version 1.100.0.0
````

Choose a version compatible with devela's MSRV (currently Rust 1.100.0).
The S3 runner loads `~/export-esp.sh` itself;
no manual sourcing is needed when using `example.sh`.

## Build and flash

From any board's example directory:

````sh
./example.sh build  # Compile only
./example.sh run    # Build and flash
````

See each board README for available programs, device ports, and inspection commands.

## Source-build fallback (S3)

If a matching prebuilt Xtensa Rust release is unavailable, from this directory:

````sh
mkdir -p /path/to/build-dir
../../tools/build-xtensa-rust.sh 1.100.0.0 /path/to/build-dir
````

The builder installs Rust under the versioned name `esp-1.100.0.0` by default;
select it from an S3 example with `TOOLCHAIN=esp-1.100.0.0 ./example.sh build`.

It does **not** install GCC/binutils: provide them via `espup`
or a separate Xtensa GCC installation.
