# Board examples

Bare-metal `no_std` examples for microcontroller boards supported by this crate.

Each board directory is a standalone Cargo workspace with the target configuration
and host-side tooling needed to build and flash it.

- `avr/arduino_nano` — classic Arduino Nano with ATmega328P.
- `esp32/c3_supermini_oled_042` — ESP32-C3 SuperMini with 0.42″ OLED.
- `sam/arduino_due` — Arduino Due with SAM3X8E.

See each directory's README for its programs, requirements, and usage.
