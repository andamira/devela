# Example tools

Shared host-side tools used by the examples.

Example directories may expose these tools through local symlinks.

For example:

    board/esp32/s3_lilygo_t_deck/flash.sh
        -> ../../../tools/esp32-s3-flash.sh

The shell tools target POSIX `sh` and are primarily developed on Linux.
