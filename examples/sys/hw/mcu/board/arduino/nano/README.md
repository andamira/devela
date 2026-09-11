<!-- devela/examples/sys/hw/mcu/board/arduino/nano/README.md -->

# Arduino Nano examples

Small bare-metal `no_std` programs for the classic Arduino Nano with an
ATmega328P, using devela's AVR and direct MMIO support.

The examples share one AVR target configuration, build/flash runner, and
devela dependency. Each program lives under `src/bin/`.

## Programs

| Binary     | Demonstrates                          | Result                                |
| ---------- | ------------------------------------- | ------------------------------------- |
| `led_on`   | GPIO output through PORTB / PB5       | Turns on the built-in D13 LED         |
| `usart_tx` | USART0 transmission at 9600 baud, 8N1 | Sends `hello from devela` over serial |

## Requirements

On Debian, Ubuntu, or Linux Mint:

```sh
sudo apt install gcc-avr avr-libc avrdude picocom
rustup component add rust-src --toolchain nightly
```

The examples use Rust's `avr-none` target with `atmega328p` as the target CPU.

## Build and flash

Build an example without flashing it:

```sh
./run.sh build led_on
./run.sh build usart_tx
```

Build and flash it:

```sh
./run.sh run led_on
./run.sh run usart_tx
```

`run` is the default action, and `led_on` is the default binary,
so this is equivalent to `./run.sh run led_on`:

```sh
./run.sh
```

The runner builds a release binary, reports its AVR memory usage,
then flashes and verifies it with `avrdude`.

By default it uses:

```text
serial port:  /dev/ttyUSB0
upload baud:  115200
```

The serial port can be overridden:

```sh
PORT=/dev/ttyUSB1 ./run.sh run led_on
```

Arduino Nano bootloaders use different upload baud rates. The current ATmega328P
bootloader uses 115200 baud, while the older bootloader uses 57600 baud.

For a Nano with the old bootloader:

```sh
UPLOAD_BAUD=57600 ./run.sh run led_on
```

The upload baud rate must match the bootloader; it is independent of any
serial baud rate configured by the firmware itself.

## USART output

Flash the USART example:

```sh
./run.sh run usart_tx
```

Then open the serial port at the 9600 baud rate configured by the firmware:

```sh
picocom -b 9600 /dev/ttyUSB0
```

The program sends:

```text
hello from devela
```

On the board tested here, opening `picocom` resets the Nano and the one-shot
message appears automatically. To receive it again, exit and reopen `picocom`.

To exit `picocom`, press `Ctrl+A`, then `Ctrl+X`.

The upload and application serial rates are separate:

```text
115200 or 57600   host ↔ bootloader, while flashing
9600              firmware ↔ host, while the program is running
```

## Size

An initial release build of `led_on` produced:

```text
   text    data     bss     dec     hex
    138       0       0     138      8a
```

Exact sizes may vary with compiler and toolchain versions.

The firmware does not depend on an AVR HAL or peripheral-access crate.
Rust `core` and devela provide the program-side foundations; AVR GCC,
`avr-libc`, and `avrdude` provide build, startup/linking, and flashing support.
