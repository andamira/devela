<!-- devela/examples/sys/hw/mcu/avr/arduino_nano/README.md -->

# Arduino Nano examples

Small bare-metal `no_std` programs for the classic Arduino Nano with an
ATmega328P, using devela's AVR and direct MMIO support.

The examples share one AVR target configuration, build/flash runner, and
devela dependency. Each program lives under `src/bin/`.

## Programs

## Programs

- `led_on` — GPIO output through PORTB / PB5; turns on the built-in D13 LED.
- `timer0_ctc` — Timer0 CTC polling at 1 ms; toggles the LED every 500 ms.
- `timer0_interrupt` — Timer0 compare interrupt; toggles the LED every 250 ms.
- `timer1_ctc` — 16-bit Timer1 CTC polling; toggles the LED directly every 500 ms.
- `usart_tx` — USART0 transmission at 9600 baud, 8N1; sends `hello from devela`.


## Requirements

On Debian, Ubuntu, or Linux Mint:

```sh
sudo apt install gcc-avr avr-libc avrdude picocom
rustup component add rust-src --toolchain nightly
```

The examples use Rust's `avr-none` target with `atmega328p` as the target CPU.

## Build and flash

```sh
./run.sh build led_on
./run.sh run led_on
```

Replace `led_on` with any binary listed above.
`run` and `led_on` are the defaults, so `./run.sh` builds and flashes `led_on`.

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
To exit press `Ctrl+A`, then `Ctrl+X`.

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
