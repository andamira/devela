
# ESP32-C3 SuperMini OLED 0.42 examples

Minimal bare-metal examples for an ESP32-C3 SuperMini board with a 0.42-inch OLED.

They boot through the ESP32-C3 ROM direct-boot path
and use devela's low-level MCU support, without an ESP HAL or runtime crate.


## Programs

- `blink` — repeatedly drives the board's built-in active-low blue LED on GPIO8.
- `oled` — initializes the onboard 72×40 OLED and draws a test pattern over I²C.
- `uart_echo` — tests UART0 TX/RX through GPIO20 and GPIO21.
- `usb_serial_chat` — runs a small interactive command console over native USB serial.


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

The script defaults to flashing the `blink` binary:

```sh
./flash.sh
```

Select another binary with the second argument:

```sh
./flash.sh flash usb_serial_chat
```

Build without flashing:

```sh
./flash.sh build usb_serial_chat
```

Inspect the direct-boot entry point and its disassembly:

```sh
./flash.sh inspect blink
```

The serial port can be overridden:

```sh
PORT=/dev/ttyACM1 ./flash.sh flash usb_serial_chat
```

The script builds a release ELF, converts it to a raw binary, verifies the
ESP32-C3 direct-boot header, and writes it directly at flash address `0x0`.

Flashing replaces the firmware stored at the beginning of flash.
Back up any factory firmware first if it needs to be preserved.


## UART0

`uart_echo` tests the ESP32-C3's direct UART0 connection at 115200 baud:

| ESP32-C3 | USB–UART |
|----------|----------|
| GPIO21 TX | RX |
| GPIO20 RX | TX |
| GND | GND |

The board remains powered and flashed through its USB connector.
Do **not** connect the USB–UART adapter's power pins.

Open the adapter with local echo disabled:

```sh
picocom -b 115200 /dev/ttyUSB0
```

It comprises a three-way diagnostic:
```
C3 TX only   → startup banner verifies TX
C3 RX only   → typing any single character toggles the LED
both         → received characters are echoed
```

Note that some ROM output and binary flashing traffic
is expected to show before the example takes control.


## USB serial

The USB serial example uses the ESP32-C3's native USB Serial/JTAG peripheral,
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
and global pointer, initializes `.data` and `.bss`, and hands off the ROM
boot watchdog state for long-running direct-boot code.

devela also provides the matching `esp32_c3_direct_boot.x` linker script.
Its build script makes the linker resource available for the
`riscv32imc-unknown-none-elf` target, and this example selects it from
`.cargo/config.toml`.


## Size

The script reports the raw image size after each build.

An initial release bild of `blink` produced:

```text
image size:         272 bytes
```

Exact sizes may vary with the example, compiler, and toolchain version.
