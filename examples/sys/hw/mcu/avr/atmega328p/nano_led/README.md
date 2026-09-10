# ATmega328P · Arduino Nano LED

Minimal bare-metal AVR example using devela's direct MMIO support.

It configures PB5 as an output and drives it high. On the classic Arduino
Nano, PB5 is digital pin 13 and is connected to the built-in LED.

## Requirements

On Debian/Ubuntu:

```sh
sudo apt install gcc-avr avr-libc avrdude
rustup component add rust-src --toolchain nightly
```

A release build of the initial LED-on example is approximately:

```txt
   text    data     bss
    138       0       0
```

Actual values may vary with compiler/toolchain versions.
