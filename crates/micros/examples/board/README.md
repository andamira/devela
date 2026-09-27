# Board examples

Bare-metal `no_std` examples for microcontroller boards supported by this crate.

Each board directory is a standalone Cargo workspace with the target
configuration and host-side tooling needed to build and flash it.

- `avr/arduino_mega2560` — Arduino Mega 2560 with ATmega2560.
- `avr/arduino_nano` — classic Arduino Nano with ATmega328P.
- `esp32/c3_supermini_oled_042` — ESP32-C3 SuperMini with 0.42″ OLED.
- `esp32/c6_waveshare_touch_lcd147` — Waveshare ESP32-C6-Touch-LCD-1.47.
- `sam/arduino_due` — Arduino Due with SAM3X8E.

See each directory's README for its programs, requirements, and usage.

## Host support

Board helper scripts (`flash.sh`) are developed and tested on Linux.
They use POSIX shell and standard Unix command-line tools. macOS should
generally work with the corresponding host tools and an appropriate PORT.
On Windows, the underlying Rust and flashing tools are available natively,
but these shell helpers require a Unix-compatible environment such as WSL
or MSYS2; alternatively, their underlying commands can be run directly.
