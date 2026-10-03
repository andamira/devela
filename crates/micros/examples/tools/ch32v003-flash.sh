#!/bin/sh
#
# Builds, flashes, inspects or dumps a CH32V003 example binary.
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
TARGET="riscv32e-unknown-none-elf"

ACTION="${1:-flash}"
NAME="${2:-minimal}"

ELF="$TARGET_DIR/$TARGET/release/$NAME"
BIN="$TARGET_DIR/$TARGET/release/$NAME.bin"

# Host tools:
# - rust-{objcopy,size,nm,objdump}: cargo-binutils / Rust LLVM tools
# - llvm-objcopy: optional objcopy fallback
# - wlink: flashing
OBJCOPY="${OBJCOPY:-rust-objcopy}"
SIZE="${SIZE:-rust-size}"
NM="${NM:-rust-nm}"
OBJDUMP="${OBJDUMP:-rust-objdump}"
READOBJ="${READOBJ:-rust-readobj}"
INSPECT_SYMBOLS="${INSPECT_SYMBOLS:-12}"

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

    require "$OBJCOPY"
    "$OBJCOPY" -O binary "$ELF" "$BIN"

    echo
    echo "elf:    $ELF"
    echo "binary: $BIN"
    echo "size:   $(wc -c < "$BIN") bytes"

    if command -v "$SIZE" >/dev/null 2>&1; then
        echo
        "$SIZE" "$ELF"
    fi
}

flash() {
    require wlink

    echo
    echo "flashing CH32V003"

    wlink flash "$BIN"
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
	echo "===== RISC-V ATTRIBUTES ====="
	"$READOBJ" --arch-specific "$ELF"

    echo
    echo "===== DISASSEMBLY ====="
    "$OBJDUMP" -d "$ELF"

	echo
	echo "===== DISASSEMBLY, NO ALIASES ====="
	"$OBJDUMP" -d -M no-aliases "$ELF"
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
