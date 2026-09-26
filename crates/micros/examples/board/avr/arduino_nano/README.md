
# Arduino Nano examples

Small bare-metal `no_std` programs for the classic Arduino Nano with an
ATmega328P, using devela's AVR and direct MMIO support.

The examples share one AVR target configuration, build/flash script,
and devela_micros dependency. Each program lives under `src/bin/`.


## Programs

- `adc_noise` — weak ADC0/A0 noise harvesting with extraction timing over USART0.
- `blink` — repeatedly drives the board's built-in active-high D13 LED on PB5.
- `timer0_ctc` — Timer0 CTC polling at 1 ms; toggles the LED every 500 ms.
- `timer0_interrupt` — Timer0 compare interrupt; toggles the LED every 250 ms.
- `timer1_capture` — Timer1 input capture; timestamps a rising edge on PB0 / ICP1.
- `timer1_clock` — overflow-extended Timer1 monotonic clock; blinks from `TimeSourceCfg`.
- `timer1_ctc` — 16-bit Timer1 CTC polling; toggles the LED directly every 500 ms.
- `timer1_pwm` — Timer1 fast PWM on PB1 / OC1A (D9); fades an external LED.
- `timer2_ctc` — 8-bit Timer2 CTC polling; toggles the LED directly every 500 ms.
- `usart_chat` — interactive USART0 command/response console at 9600 baud, 8N1.


## Requirements

On Debian, Ubuntu, or Linux Mint:

```sh
sudo apt install gcc-avr avr-libc avrdude picocom
```

The examples use Rust's `avr-none` target with `atmega328p` as the target CPU.

`nightly + rust-src` are selected locally because `-Z build-std` is still required.


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
serial port:  /dev/ttyUSB0
upload baud:  115200
```

The serial port can be overridden:

```sh
PORT=/dev/ttyUSB1 ./flash.sh flash blink
```

Arduino Nano bootloaders use different upload baud rates. The current ATmega328P
bootloader uses 115200 baud, while the older bootloader uses 57600 baud.

For a Nano with the old bootloader:

```sh
UPLOAD_BAUD=57600 ./flash.sh flash blink
```

The upload baud rate must match the bootloader; it is independent of any
serial baud rate configured by the firmware itself.


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
devela nano ready
commands: ping, help, led on, led off, status, help
>
```

On the board tested here, opening `picocom` resets the Nano and the one-shot
message appears automatically. To receive it again, exit and reopen `picocom`.
To exit press `Ctrl+A`, then `Ctrl+X`.

The upload and application serial rates are separate:

```text
115200 or 57600   host ↔ bootloader, while flashing
9600              firmware ↔ host, while the program is running
```


## PWM output

`timer1_pwm` drives Nano D9 / PB1 / OC1A.
Connect D9 through a current-limiting resistor and LED to GND.


## Size

An initial release build of `blink` produced:

```text
   text    data     bss     dec     hex
    168       0       0     168      a8
```

Exact sizes may vary with compiler and toolchain versions.

The firmware does not depend on an AVR HAL or peripheral-access crate.
Rust `core` and devela provide the program-side foundations; AVR GCC,
`avr-libc`, and `avrdude` provide build, startup/linking, and flashing support.
