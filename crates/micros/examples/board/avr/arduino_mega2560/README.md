
# Arduino Mega2560 examples

Small bare-metal `no_std` programs for the Arduino Mega 2560 with an
ATmega2560, using devela's AVR and direct MMIO support.

The examples share one AVR target configuration, build/flash runner,
and devela_micros dependency. Each program lives under `src/bin/`.


## Programs

- `led_on` — GPIO output through PORTB / PB7; turns on the built-in D13 LED.
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
./run.sh build led_on
./run.sh run led_on
```

Replace `led_on` with any binary listed above.
`run` and `led_on` are the defaults, so `./run.sh` builds and flashes `led_on`.

The runner builds a release binary, reports its AVR memory usage,
then flashes and verifies it with `avrdude`.

By default it uses:

```text
serial port:  /dev/ttyACM0
upload baud:  115200
```

The serial port can be overridden:

```sh
PORT=/dev/ttyACM1 ./run.sh run led_on
```

The upload baud rate must match the bootloader; it is independent of any
serial baud rate configured by the firmware itself.


## USART

Flash the USART example:

```sh
./run.sh run usart_chat
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

An initial release build of `led_on` produced:

```text
   text    data     bss     dec     hex
    138       0       0     138      8a
```

Exact sizes may vary with compiler and toolchain versions.

The firmware does not depend on an AVR HAL or peripheral-access crate.
Rust `core` and devela provide the program-side foundations; AVR GCC,
`avr-libc`, and `avrdude` provide build, startup/linking, and flashing support.
