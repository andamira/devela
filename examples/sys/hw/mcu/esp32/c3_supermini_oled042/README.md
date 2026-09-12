<!-- devela/examples/sys/hw/mcu/esp32/c3_supermini_oled_042/README.md -->

# ESP32-C3 SuperMini OLED 0.42 examples

Minimal bare-metal examples for an ESP32-C3 SuperMini board with a 0.42-inch OLED.

They boot through the ESP32-C3 ROM direct-boot path
and use devela's low-level MCU support, without an ESP HAL or runtime crate.

## Examples

* `led` — Turns on the board's active-low blue LED on GPIO8.
* `usb_serial_tx` — Sends `hello from devela` over the native USB Serial/JTAG interface.
* `usb_serial_chat` — Runs a small interactive command console over native USB serial.

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

On Linux, the USB serial device is typically `/dev/ttyACM0`.
The user needs permission to access it, commonly through the `dialout` group.

## Build and flash

The runner defaults to the `led` binary:

```sh
./run.sh
```

Select another example with the second argument:

```sh
./run.sh run usb_serial_tx
./run.sh run usb_serial_chat
```

Build without flashing:

```sh
./run.sh build usb_serial_chat
```

The serial port can be overridden:

```sh
PORT=/dev/ttyACM1 ./run.sh run usb_serial_chat
```

The runner builds a release ELF, converts it to a raw binary, verifies the
ESP32-C3 direct-boot header, and writes it directly at flash address `0x0`.

Flashing replaces the firmware stored at the beginning of flash.
Back up any factory firmware first if it needs to be preserved.

## USB serial

The USB serial examples use the ESP32-C3's native USB Serial/JTAG peripheral,
exposed on Linux through the same `/dev/ttyACM*` device used for flashing.

Because flashing and console access share this connection, the firmware can
begin running before a terminal such as `picocom` has opened the serial port.
The examples therefore wait for the first received byte before sending their
initial output. Checking for received data does not consume that byte, so
`usb_serial_chat` still processes it as the first byte of the command line.

For example:

```sh
picocom /dev/ttyACM0
```

Then type a command such as:

```text
ping
```

The initial synchronization happens once after reset. Disconnecting and
reopening the terminal does not restart the firmware or repeat the welcome
message; an already-running `usb_serial_chat` continues accepting commands.

USB Serial/JTAG transports bytes over USB rather than a UART bitstream,
so there is no device-side baud-rate configuration.

## Direct boot

The examples use devela's `esp32_c3_direct_boot!` macro for the minimal
ESP32-C3 startup sequence.

Before entering the supplied Rust function, it establishes the RISC-V stack
and global pointer, initializes `.data` and `.bss`, and disables the watchdog
states left active by ROM flash boot.

devela also provides the matching `esp32_c3_direct_boot.x` linker script.
Its build script makes the linker resource available for the
`riscv32imc-unknown-none-elf` target, and this example selects it from
`.cargo/config.toml`.

## Size

The runner reports the raw image size after each build.

Exact sizes may vary with the example, compiler, and toolchain version.
