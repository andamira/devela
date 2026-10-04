
# Lilygo T-Display-S3 examples

Minimal bare-metal examples for the ESP32-S3 using the Espressif Rust toolchain and `xtensa-lx-rt`.


## Programs

- `blink` — blinks the LCD backlight to verify ESP32-S3 startup, IO-MUX/GPIO
  routing, and digital output without depending on the display controller.


## Requirements

This board uses the ESP32-S3 Xtensa toolchain.
See the [ESP32 example setup](../README.md#esp32-s3)
for installation and toolchain requirements.


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
