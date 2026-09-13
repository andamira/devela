<!-- devela/examples/sys/hw/mcu/esp32/s3_bringup/README.md -->

# ESP32-S3 bring-up example

## Requirements

On Debian, Ubuntu, or Linux Mint:

```sh
sudo apt-get install -y gcc build-essential curl pkg-config libudev-dev
```

Install the binary utilities used to produce the raw flash image:

```sh
cargo install espup --locked
espup install --targets esp32s3
cargo install espflash --locked
```

Install the Rust target:

```sh
espup install --name esp --targets esp32s3 --toolchain-version 1.98.1.0
```

## Run the example

Build
```sh
cargo +esp build --release
```

Flash
```sh
espflash flash target/xtensa-esp32s3-none-elf/release/esp32s3_bringup
```

Connect
```sh
espflash monitor --port /dev/ttyACM0
```

Then reset the chip with `Ctrl + R`, press any key,
and the example text should appear:

```txt
hello from devela on esp32-s3
```
