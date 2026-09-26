#!/bin/sh
#
# Builds or flashes an Arduino Nano example.

set -eu

DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
TARGET_DIR="$DIR/target"
TARGET="avr-none"

ACTION="${1:-flash}"
NAME="${2:-blink}"

PORT="${PORT:-/dev/ttyUSB0}"
UPLOAD_BAUD="${UPLOAD_BAUD:-115200}" # use 57600 for older Nanos

ELF="$TARGET_DIR/$TARGET/release/$NAME.elf"

require() {
    command -v "$1" >/dev/null 2>&1 || {
        echo "error: $1 not found" >&2
        exit 1
    }
}

build() {
    cd "$DIR"

    cargo +nightly build \
        --release \
        --bin "$NAME" \
        --target-dir "$TARGET_DIR"

    if command -v avr-size >/dev/null 2>&1; then
        echo
        avr-size "$ELF"
    fi

    echo
    echo "elf: $ELF"
}

flash() {
    require avrdude

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
    flash)
        build
        flash
        ;;
    *)
        echo "usage: $0 [build|flash] [binary]" >&2
        exit 2
        ;;
esac
