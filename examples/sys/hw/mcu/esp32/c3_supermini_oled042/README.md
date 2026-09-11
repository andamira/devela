<!-- devela/examples/sys/hw/mcu/esp32/c3_supermini_oled_042/README.md -->

# ESP32-C3 SuperMini OLED 0.42 examples

Minimal bare-metal examples for an ESP32-C3 SuperMini board with a 0.42-inch OLED.

The current example uses devela's ESP32-C3 MMIO definitions directly and boots
through the ESP32-C3 ROM direct-boot path, without an ESP HAL or runtime crate.

## Examples

| Binary | Description                                        |
| ------ | -------------------------------------------------- |
| `led`  | Turns on the board's active-low blue LED on GPIO8. |

## Requirements

Install the Rust target:

```sh
rustup target add riscv32imc-unknown-none-elf
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

On Linux, the serial device is typically `/dev/ttyACM0`.
The user needs permission to access it, commonly through the `dialout` group.

## Build and flash

Build without flashing:

```sh
./run.sh build
```

Build and flash:

```sh
./run.sh
```

The serial port can be overridden:

```sh
PORT=/dev/ttyACM1 ./run.sh
```

The runner builds a release ELF, converts it to a raw binary, verifies the
ESP32-C3 direct-boot header, and writes it directly at flash address `0x0`.

Flashing replaces the firmware stored at the beginning of flash.
Back up any factory firmware first if it needs to be preserved.

## Direct boot

The example currently keeps its minimal startup assembly and linker script local and explicit.

The linker script describes the ESP32-C3 flash and RAM layout and places the
direct-boot header at the beginning of the image. The startup code establishes
the RISC-V stack and global pointer, initializes `.data` and `.bss`, then enters Rust `main`.

These pieces are intentionally visible while the initial runtime support is being developed;
they are candidates for reusable devela support once the interface is settled.

## Size

An initial release build of `led` produced a 128-byte raw direct-boot image.

Exact sizes may vary with compiler and toolchain versions.
