#!/bin/sh
#
# Builds, flashes, inspects or dumps an MSP430G2231 example binary.
#
# TOC
# - configuration
# - require()
# - build()
# - flash()
# - inspect()
# - dump_text()
# - dump()
# - action dispatch

set -eu

DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
TARGET_DIR="$DIR/target"
TARGET="msp430-none-elf"

ACTION="${1:-flash}"
NAME="${2:-minimal}"

ELF="$TARGET_DIR/$TARGET/release/$NAME"

# Host tools:
# ...
CC="${CC:-msp430-elf-gcc}"
SIZE="${SIZE:-msp430-elf-size}"
NM="${NM:-msp430-elf-nm}"
OBJDUMP="${OBJDUMP:-msp430-elf-objdump}"
MSPDEBUG="${MSPDEBUG:-mspdebug}"
MSPDEBUG_DRIVER="${MSPDEBUG_DRIVER:-rf2500}"
INSPECT_SYMBOLS="${INSPECT_SYMBOLS:-12}"

require() {
    command -v "$1" >/dev/null 2>&1 || {
        echo "error: $1 not found" >&2
        exit 1
    }
}

build() {
    require "$CC"

    cd "$DIR"

    cargo +nightly build \
        --release \
        --bin "$NAME" \
        --target-dir "$TARGET_DIR"

    if command -v "$SIZE" >/dev/null 2>&1; then
        echo
        "$SIZE" "$ELF"
    fi

    echo
    echo "elf: $ELF"
}

flash() {
    require "$MSPDEBUG"

    echo
    echo "flashing via MSPDebug/$MSPDEBUG_DRIVER"

    "$MSPDEBUG" "$MSPDEBUG_DRIVER" "prog $ELF"
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
