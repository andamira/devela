#!/bin/sh
#
# Builds, flashes, inspects or dumps a LILYGO T-Display-S3 example binary.
#
# TOC
# - configuration
# - ESP environment
# - require()
# - build()
# - flash()
# - inspect()
# - dump_text()
# - dump()
# - action dispatch

set -e

DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
TARGET_DIR="$DIR/target"
TARGET="xtensa-esp32s3-none-elf"

ACTION="${1:-flash}"
NAME="${2:-blink}"

PORT="${PORT:-/dev/ttyACM0}"

ELF="$TARGET_DIR/$TARGET/release/$NAME"

# espup generates this file after installing the Xtensa Rust environment.
ESP_ENV="${ESPUP_EXPORT_FILE:-$HOME/export-esp.sh}"

# Host tools supplied by the Espressif toolchain.
SIZE="${SIZE:-xtensa-esp-elf-size}"
NM="${NM:-xtensa-esp-elf-nm}"
OBJDUMP="${OBJDUMP:-xtensa-esp-elf-objdump}"
INSPECT_SYMBOLS="${INSPECT_SYMBOLS:-12}"


# ESP environment
# ------------------------------------------------------------------------------

if [ ! -r "$ESP_ENV" ]; then
    echo "error: ESP environment file not found: $ESP_ENV" >&2
    echo "install the ESP32-S3 Rust toolchain with espup first" >&2
    exit 1
fi

# This modifies only this /bin/sh process and its children.
# It never replaces or modifies the user's interactive shell.
. "$ESP_ENV"

set -u


require() {
    command -v "$1" >/dev/null 2>&1 || {
        echo "error: $1 not found" >&2
        exit 1
    }
}


build() {
    cd "$DIR"

    cargo +esp build \
        --release \
        --bin "$NAME" \
        --target-dir "$TARGET_DIR"

    echo
    echo "elf: $ELF"

    if command -v "$SIZE" >/dev/null 2>&1; then
        echo
        "$SIZE" "$ELF"
    fi
}


flash() {
    require espflash

    echo
    echo "flashing: $PORT"

    ESPFLASH_PORT="$PORT" \
        espflash flash "$ELF"
}


inspect() {
    require "$SIZE"
    require "$NM"

    echo
    echo "elf: $ELF"

    echo
    echo "sections:"
    "$SIZE" -A "$ELF"

    echo
    echo "largest symbols:"
    "$NM" -S --size-sort "$ELF" | tail -n "$INSPECT_SYMBOLS"
}


dump_text() {
    require "$SIZE"
    require "$NM"
    require "$OBJDUMP"

    echo "ELF: $ELF"

    echo
    echo "===== SIZE ====="
    "$SIZE" -A "$ELF"

    echo
    echo "===== FILE ====="
    "$OBJDUMP" -f "$ELF"

    echo
    echo "===== SECTIONS ====="
    "$OBJDUMP" -h "$ELF"

    echo
    echo "===== SYMBOLS ====="
    "$NM" -n -S "$ELF"

    echo
    echo "===== DISASSEMBLY ====="
    "$OBJDUMP" -d "$ELF"
}


dump() {
    if [ -t 1 ] && [ -n "${EDITOR:-}" ]; then
        DUMP="$TARGET_DIR/$TARGET/release/$NAME.dump.txt"
        dump_text > "$DUMP"

        echo "dump: $DUMP"
        "$EDITOR" "$DUMP"
    else
        dump_text
    fi
}


case "$ACTION" in
    build)
        build
        ;;
    flash)
        build
        flash
        ;;
    inspect)
        build
        inspect
        ;;
    dump)
        build
        dump
        ;;
    *)
        echo "usage: $0 [build|flash|inspect|dump] [binary]" >&2
        exit 2
        ;;
esac
