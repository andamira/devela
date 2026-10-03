#!/bin/sh
#
# Builds, flashes, inspects or dumps an ESP32-C3 example binary.
#
# TOC
# - configuration
# - require()
# - find_objcopy()
# - build()
# - flash()
# - inspect()
# - dump_text()
# - dump()
# - action dispatch

set -eu

DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
TARGET_DIR="$DIR/target"
TARGET="riscv32imc-unknown-none-elf"

ACTION="${1:-flash}"
NAME="${2:-blink}"

PORT="${PORT:-/dev/ttyACM0}"

ELF="$TARGET_DIR/$TARGET/release/$NAME"
IMAGE="$TARGET_DIR/$TARGET/release/$NAME.bin"

# Host tools:
# - rust-{objcopy,size,nm,objdump}: cargo-binutils / Rust LLVM tools
# - llvm-objcopy: optional objcopy fallback
# - espflash: flashing
SIZE="${SIZE:-rust-size}"
NM="${NM:-rust-nm}"
OBJDUMP="${OBJDUMP:-rust-objdump}"
INSPECT_SYMBOLS="${INSPECT_SYMBOLS:-12}"

require() {
    command -v "$1" >/dev/null 2>&1 || {
        echo "error: $1 not found" >&2
        exit 1
    }
}

find_objcopy() {
    for tool in rust-objcopy llvm-objcopy; do
        if command -v "$tool" >/dev/null 2>&1; then
            printf '%s\n' "$tool"
            return
        fi
    done

    echo "error: rust-objcopy or llvm-objcopy is required" >&2
    exit 1
}

build() {
    cd "$DIR"

    cargo build \
        --release \
        --bin "$NAME" \
        --target-dir "$TARGET_DIR"

    OBJCOPY="$(find_objcopy)"
    "$OBJCOPY" -O binary "$ELF" "$IMAGE"

    HEADER="$(
        od -An -tx1 -N8 "$IMAGE" |
        tr -d '[:space:]'
    )"

    if [ "$HEADER" != "1d04dbae1d04dbae" ]; then
        echo "error: invalid ESP32-C3 direct-boot header: $HEADER" >&2
        exit 1
    fi

    echo
    echo "direct-boot header: $HEADER"
    echo "elf:                $ELF"
    echo "image:              $IMAGE"
    echo "image size:         $(wc -c < "$IMAGE") bytes"

    if command -v "$SIZE" >/dev/null 2>&1; then
        echo
        "$SIZE" "$ELF"
    fi
}

flash() {
    require espflash

    echo
    echo "flashing direct-boot image: $PORT"

    ESPFLASH_PORT="$PORT" \
        espflash write-bin 0x0 "$IMAGE"
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
