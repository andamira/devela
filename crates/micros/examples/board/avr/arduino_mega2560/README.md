
# Arduino Mega 2560 examples

Small bare-metal `no_std` programs for the Arduino Mega 2560 with an
ATmega2560, using devela's AVR and direct MMIO support.

The examples share one AVR target configuration, build/flash script,
and devela_micros dependency. Each program lives under `src/bin/`.


## Programs

- `blink` — repeatedly drives the board's built-in active-high D13 LED through PB7.
- `usart_chat` — interactive USART0 command/response console at 9600 baud, 8N1.


## Requirements

On Debian, Ubuntu, or Linux Mint:

```sh
sudo apt install gcc-avr avr-libc avrdude picocom
```

The examples use Rust's `avr-none` target with `atmega2560` as the target CPU.

`nightly + rust-src` are selected locally because `-Z build-std` is still required.

The local Cargo configuration selects `atmega2560` explicitly for `avr-none`.
This device selection also reaches AVR GCC at link time, selecting the
ATmega2560 startup code, interrupt-vector layout, and SRAM mapping.


## Build and flash

```sh
./flash.sh build blink
./flash.sh flash blink
```

Replace `blink` with any binary listed above.
`flash` and `blink` are the defaults, so `./flash.sh` builds and flashes `blink`.

The script builds a release ELF, reports its AVR memory usage,
then flashes and verifies it with `avrdude`.

By default it uses:

```text
serial port:  /dev/ttyACM0
upload baud:  115200
```

The serial port can be overridden:

```sh
PORT=/dev/ttyACM1 ./flash.sh flash blink
```

The upload baud rate must match the bootloader; it is independent of any
serial baud rate configured by the firmware itself.


## Inspect and dump

For a concise ELF overview:

```sh
./flash.sh inspect blink
```

This reports section sizes and the largest symbols.

For file and section headers, the complete symbol table, and disassembly:

```sh
./flash.sh dump blink
```

When stdout is interactive and `$EDITOR` is set, the dump is saved beside the
ELF and opened in the editor. Otherwise it is written to stdout:

```sh
./flash.sh dump blink | less
./flash.sh dump blink > /tmp/blink.dump
```


## USART

Flash the USART example:

```sh
./flash.sh flash usart_chat
```
Then open the USB serial port at the 9600 baud rate configured by the firmware:

```sh
picocom -b 9600 /dev/ttyACM0
```

The program sends:

```txt
devela mega2560 ready
commands: ping, led on, led off, status, help
>
```

The upload and application serial rates are separate:

```sh
115200   host ↔ bootloader, while flashing
9600     firmware ↔ host, while the program is running
```


## Size

An initial release build of `blink` produced:

```text
   text    data     bss     dec     hex
    296       0       0     296     128
```

Exact sizes may vary with compiler and toolchain versions.

The firmware does not depend on an AVR HAL or peripheral-access crate.
Rust `core` and devela provide the program-side foundations; AVR GCC,
`avr-libc`, and `avrdude` provide build, startup/linking, and flashing support.
