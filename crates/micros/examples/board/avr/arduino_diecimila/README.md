# Arduino Diecimila examples

Small bare-metal `no_std` programs for the Arduino Diecimila with an
ATmega168, using devela's AVR and direct MMIO support.

The examples share one AVR target configuration, build/flash script,
and devela_micros dependency. Each program lives under `src/bin/`.


## Programs

- `blink` — repeatedly drives the board's built-in active-high D13 LED on PB5.
- `usart_chat` — interactive USART0 command/response console at 9600 baud, 8N1.


## Requirements

On Debian, Ubuntu, or Linux Mint:

```sh
sudo apt install gcc-avr avr-libc avrdude picocom
```

The examples use Rust's `avr-none` target with `atmega168` as the target CPU.

`nightly + rust-src` are selected locally because `-Z build-std` is still required.


## Build and flash

The example defaults to the `blink` binary when an action is given.
Running the script without an action prints its usage.

```sh
./flash.sh
./flash.sh build
./flash.sh flash
```

Select another binary explicitly when needed:

```sh
./flash.sh flash usb_serial_chat
```

The script builds a release ELF, reports its AVR memory usage,
then flashes and verifies it with `avrdude`.

By default it uses:

```text
serial port:  /dev/ttyUSB0
upload baud:  19200
```

The serial port can be overridden:

```sh
PORT=/dev/ttyUSB1 ./flash.sh flash blink
```

The 19200 baud default matches Arduino's maintained ATmega168 Diecimila
board definition. Some individual boards may have a different bootloader;
`UPLOAD_BAUD` can be overridden when needed.

The upload baud rate is independent of any serial baud rate configured by
the firmware itself.


## USART

Flash the USART example:

```sh
./flash.sh flash usart_chat
```

Then open the serial port at the 9600 baud rate configured by the firmware:

```sh
picocom -b 9600 /dev/ttyUSB0
```

The program sends:

```text
devela diecimila ready
commands: ping, help, led on, led off, status, help
>
```

To exit `picocom`, press `Ctrl+A`, then `Ctrl+X`.

The upload and application serial rates are separate:

```text
19200   host ↔ bootloader, while flashing
9600    firmware ↔ host, while the program is running
```


## Size

A minimal release build of `blink` currently produces:

```text
   text    data     bss     dec     hex
    168       0       0     168      a8
```

Exact sizes may vary with compiler and toolchain versions.

The firmware does not depend on an AVR HAL or peripheral-access crate.
Rust `core` and devela provide the program-side foundations; AVR GCC,
`avr-libc`, and `avrdude` provide build, startup/linking, and flashing support.
