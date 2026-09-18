# ESP32-S3 bring-up example

Minimal bare-metal `no_std` bring-up for the ESP32-S3 using the Espressif Rust toolchain and `xtensa-lx-rt`.

## Requirements

On Debian, Ubuntu, or Linux Mint:
```sh
sudo apt-get install -y gcc build-essential curl pkg-config libudev-dev
```

Install the Espressif Rust tooling and flashing utility:
```sh
cargo install espup --locked
cargo install espflash --locked
```

Install the ESP32-S3 Rust toolchain:
```sh
espup install --name esp --targets esp32s3 --toolchain-version 1.98.1.0
```

`espup` writes the environment setup to `~/export-esp.sh`. Load it before building so the Xtensa compiler and related tools are available.

For Bash and compatible shells:
```sh
. "$HOME/export-esp.sh"
```

From fish, start a fish shell with that environment imported:
```sh
bash -lc '. "$HOME/export-esp.sh"; exec fish'
```

You can verify the setup with:
```sh
rustc +esp -Vv
command -v xtensa-esp32s3-elf-gcc
espflash --version
```

## Run the example

Build:
```sh
cargo +esp build --release
```

Flash:
```sh
espflash flash target/xtensa-esp32s3-none-elf/release/esp32s3_bringup
```

Connect:
```sh
espflash monitor --port /dev/ttyACM0
```

Then reset the chip with `Ctrl + R` and press any key. The example should print:
```txt
hello from devela on esp32-s3
```
