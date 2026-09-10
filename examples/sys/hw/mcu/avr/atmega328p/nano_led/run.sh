#!/bin/sh
# devela/examples/hw/mcu/avr/atmega328p/nano_led/run.sh
#
# Builds and optionally flashes the Nano LED example.

set -eu

DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
TARGET_DIR="$DIR/target"
BIN="nano-led"
ELF="$TARGET_DIR/avr-none/release/$BIN.elf"

PORT="${PORT:-/dev/ttyUSB0}"
BAUD="${BAUD:-57600}"

build() {
    cd "$DIR"

    cargo +nightly build --release --target-dir "$TARGET_DIR"

    if command -v avr-size >/dev/null 2>&1; then
        echo
        avr-size "$ELF"
    fi

    echo
    echo "binary: $ELF"
}

flash() {
    command -v avrdude >/dev/null 2>&1 || {
        echo "error: avrdude not found" >&2
        exit 1
    }

    echo
    echo "flashing: $PORT @ $BAUD baud"

    avrdude \
        -p atmega328p \
        -c arduino \
        -P "$PORT" \
        -b "$BAUD" \
        -D \
        -U "flash:w:$ELF:e"
}

case "${1:-run}" in
    build)
        build
        ;;
    run|flash)
        build
        flash
        ;;
    *)
        echo "usage: $0 [build|run|flash]" >&2
        exit 2
        ;;
esac
