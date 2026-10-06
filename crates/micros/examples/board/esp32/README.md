# ESP32 board examples

Bare-metal `no_std` examples for ESP32-family boards supported by devela.


## Requirements

On Debian, Ubuntu, or Linux Mint:

```sh
sudo apt-get install -y gcc build-essential curl pkg-config libudev-dev
```

Install the flashing utility:

```sh
cargo install espflash --locked
```


## ESP32-C3 and ESP32-C6

The RISC-V ESP32 targets use upstream Rust targets.
See each board README for the target and any additional tooling it requires.


## ESP32-S3

ESP32-S3 uses Xtensa and requires the Espressif Rust toolchain.

Install `espup`:

```sh
cargo install espup --locked
```

`espup` installs the Xtensa Rust toolchain together
with the GCC/binutils environment used during linking.

The default Xtensa Rust version selected by `espup` may lag behind devela's
MSRV. If necessary, install a matching prebuilt toolchain explicitly:

```sh
espup install --name esp --targets esp32s3 --toolchain-version 1.99.0.0
```

Replace `1.99.0.0` with the Espressif Rust release corresponding to devela's
current MSRV.

If the `esp` toolchain is already installed, update it instead:

```sh
espup update --name esp --targets esp32s3 --toolchain-version 1.99.0.0
```

If replacing an existing installation fails, a clean reinstall can be used:

```sh
espup uninstall --name esp
espup install --name esp --targets esp32s3 --toolchain-version 1.99.0.0
```

devela tracks stable Rust closely, and occasionally its MSRV may advance before
a matching prebuilt Espressif Xtensa toolchain is available. In that case, the
matching Rust toolchain can be built locally with the provided helper.

Choose a build directory with substantial free space;
building Rust and LLVM may temporarily consume tens of gigabytes:

```sh
../../tools/build-xtensa-rust.sh 1.99.0.0 /path/to/build-dir
```

On one x86_64 Linux system the build downloaded about 5 GB
and grew to about 43 GB, taking roughly one hour.

A custom named toolchain can be selected when running an S3 example:

```sh
TOOLCHAIN=esp-1.99.0.0 ./example.sh build
```
