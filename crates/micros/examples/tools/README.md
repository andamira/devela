# Example tools

Shared host-side tools used by the examples.

Each example provides a small local `example.sh` wrapper with its concrete
configuration and delegates to the corresponding shared tool in this directory.

For example:

    board/esp32/c3_supermini_oled042/example.sh
        → tools/esp32.sh
    board/nrf/nice_nano/example.sh
        → tools/nrf.sh
    computer/zx/spectrum48/example.sh
        → tools/spectrum48.sh
    computer/atari/atari800xl/example.sh
        → tools/atari8.sh

Shared inspection, dump, and dispatch helpers live in `_common.sh`.

The shell tools target POSIX `sh` and are primarily developed on Linux.

Toolchains are selected through `rustup` or from `PATH` and documented in each
example. Generated artifacts stay in the example's own `target/` directory.

For custom toolchains, separate the compiler from external tools: register
versioned compiler installations with `rustup`, optionally point a short alias
(e.g. `rust-mos` or `rust-z80`) to the validated version, and configure
external SDK/binutils paths separately. The Atari example documents `SDKBIN`
and Fish configuration; ESP32-S3 uses `espup`'s exported linker environment.
A custom compiler need not bundle Cargo: `cargo +<toolchain>`
can use rustup's Cargo fallback.
