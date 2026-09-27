
# Waveshare ESP32-C6 Touch LCD 1.47 examples

Minimal bare-metal examples for the Waveshare ESP32-C6-Touch-LCD-1.47
(ESP32-C6FH8).

They boot through the ESP32-C6 ROM direct-boot path and use devela's low-level
MCU support without ESP-IDF, an ESP HAL, or a runtime crate.


## Programs

- `blink` — blinks the LCD backlight to verify direct boot, IO-MUX/GPIO
  routing, and digital output without depending on the display controller.
- `solid_color` — initializes SPI2 and the JD9853 controller, writes a full
  RGB565 frame, and then enables the backlight.


## Requirements

Install the upstream Rust target:

```sh
rustup target add riscv32imac-unknown-none-elf
```

Install the binary utilities used to produce the raw flash image:

```sh
cargo install cargo-binutils
rustup component add llvm-tools-preview
```

Install `espflash` for flashing:

```sh
cargo install espflash --locked
```

On Linux, the USB serial device is typically `/dev/ttyACM0`.
The user needs permission to access it, commonly through the `dialout` group.


## Build and flash

The script defaults to flashing the `blink` binary:

```sh
./flash.sh
```

Build without flashing:

```sh
./flash.sh build blink
```

The serial port can be overridden:

```sh
PORT=/dev/ttyACM1 ./flash.sh flash blink
```

The script builds a release ELF, converts it to a raw binary, verifies the
ESP32-C6 direct-boot header, and writes it directly at flash address `0x0`.

Flashing replaces the firmware stored at the beginning of flash.
Back up any factory firmware first if it needs to be preserved.


## Inspect and dump

For a concise ELF overview:

```sh
./flash.sh inspect blink
```

For file and section headers, the complete symbol table, and disassembly:


```sh
./flash.sh dump blink
```

When stdout is interactive and `$EDITOR` is set, the dump is saved beside the
ELF and opened in the editor. Otherwise it is written to stdout.


## Direct boot

The example uses devela's `esp32_c6_direct_boot!` macro for the minimal
ESP32-C6 startup sequence. Before entering `main`, it establishes the RISC-V
stack and global pointer, initializes `.data` and `.bss`, and hands off the ROM
boot watchdog state.

devela also provides the matching `esp32_c6_direct_boot.x` linker script.
The build script makes it available for `riscv32imac-unknown-none-elf`, and
this example selects it from `.cargo/config.toml`.

The board contains 8 MiB of in-package flash. The generic C6 direct-boot linker
currently limits the executable image to the first 4 MiB. Direct boot requires
the chip's `DIS_DIRECT_BOOT` eFuse to remain unset.
