#!/bin/sh
# devela/examples/sys/hw/mcu/avr/arduino_nano/run.sh
#
# Builds and optionally flashes an Arduino Nano example.

set -eu

DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
TARGET_DIR="$DIR/target"
TARGET="avr-none"

ACTION="${1:-run}"
BIN="${2:-led_on}"

PORT="${PORT:-/dev/ttyUSB0}"
UPLOAD_BAUD="${UPLOAD_BAUD:-115200}" # use 57600 for older nanos

ELF="$TARGET_DIR/$TARGET/release/$BIN.elf"

build() {
    cd "$DIR"

    cargo +nightly build \
        --release \
        --bin "$BIN" \
        --target-dir "$TARGET_DIR"

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
    echo "flashing: $PORT @ $UPLOAD_BAUD baud"

    avrdude \
        -p atmega328p \
        -c arduino \
        -P "$PORT" \
        -b "$UPLOAD_BAUD" \
        -D \
        -U "flash:w:$ELF:e"
}

case "$ACTION" in
    build)
        build
        ;;
    run|flash)
        build
        flash
        ;;
    *)
        echo "usage: $0 [build|run|flash] [binary]" >&2
        exit 2
        ;;
esac
