# Example tools

Shared host-side tools used by the examples.

Each example provides a small local `example.sh` wrapper with its concrete
configuration and delegates to the corresponding shared tool in this directory.

For example:

    board/esp32/c3_supermini_oled042/example.sh
        → tools/esp32.sh

Shared inspection, dump, and dispatch helpers live in `_common.sh`.

The shell tools target POSIX `sh` and are primarily developed on Linux.
