
# Lilygo T-Display-S3 examples

Minimal bare-metal examples for the ESP32-S3 using the Espressif Rust toolchain and `xtensa-lx-rt`.


## Programs

- `blink` — blinks the LCD backlight to verify ESP32-S3 startup, IO-MUX/GPIO
  routing, and digital output without depending on the display controller.


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

## Build and flash

The script defaults to flashing the `blink` binary:

```sh
./example.sh
```

Build without flashing:

```sh
./example.sh build blink
```

The serial port can be overridden:

```sh
PORT=/dev/ttyACM1 ./example.sh flash blink
```

Flashing replaces the firmware stored at the beginning of flash.
Back up any factory firmware first if it needs to be preserved.


## Inspect and dump

For a concise ELF overview:

```sh
./example.sh inspect blink
```

For file and section headers, the complete symbol table, and disassembly:


```sh
./example.sh dump blink
```

When stdout is interactive and `$EDITOR` is set, the dump is saved beside the
ELF and opened in the editor. Otherwise it is written to stdout.
