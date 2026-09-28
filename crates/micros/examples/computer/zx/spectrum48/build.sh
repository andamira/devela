#!/bin/sh
#
# Builds, packages, runs, inspects or dumps a Rust/Z80 ZX Spectrum program.
#
# Usage:
#   ./build.sh build [binary]
#   ./build.sh run [binary]
#   ./build.sh inspect [binary]
#   ./build.sh dump [binary]
#
# Environment overrides:
#   TOOLCHAIN=rust-z80
#   RUST_Z80_ROOT=/path/to/rust-z80
#   LLVM_Z80_BIN=/path/to/custom/llvm/bin
#   TARGET_DIR=/path/to/target
#   OBJCOPY=/path/to/llvm-objcopy
#   OBJDUMP=/path/to/llvm-objdump
#   LD_LLD=/path/to/ld.lld
#

set -eu

DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"

TARGET="z80-unknown-none-elf"
TOOLCHAIN="${TOOLCHAIN:-rust-z80}"

ACTION="${1:-build}"
NAME="${2:-screen}"

# Keep this in sync with link.x and mk-tap.py.
ORIGIN_HEX="0x8000"
ORIGIN_DEC="32768"

# Deliberately ignore a global CARGO_TARGET_DIR.
TARGET_DIR="${TARGET_DIR:-$DIR/target}"

ELF="$TARGET_DIR/$TARGET/release/$NAME.elf"
BIN="$TARGET_DIR/$TARGET/release/$NAME.bin"
TAP="$TARGET_DIR/$TARGET/release/$NAME.tap"
DUMP="$TARGET_DIR/$TARGET/release/$NAME.dump.txt"

require() {
    command -v "$1" >/dev/null 2>&1 || {
        echo "error: $1 not found" >&2
        exit 1
    }
}

require_executable() {
    [ -x "$1" ] || {
        echo "error: executable not found: $1" >&2
        exit 1
    }
}

require_file() {
    [ -f "$1" ] || {
        echo "error: file not found: $1" >&2
        exit 1
    }
}

# Resolve the installed rust-z80 sysroot first.
require rustc

SYSROOT="$(rustc "+$TOOLCHAIN" --print sysroot)"
HOST="$(rustc "+$TOOLCHAIN" -vV | sed -n 's/^host: //p')"

# With `rustup toolchain link rust-z80 rust-z80/install`,
# the checkout is normally the parent of the sysroot.
RUST_Z80_ROOT="${RUST_Z80_ROOT:-$(dirname "$SYSROOT")}"

# llvm-z80 currently builds these here.
LLVM_Z80_BIN="${LLVM_Z80_BIN:-$RUST_Z80_ROOT/build/$HOST/llvm/bin}"

LD_LLD="${LD_LLD:-$SYSROOT/lib/rustlib/$HOST/bin/gcc-ld/ld.lld}"
OBJCOPY="${OBJCOPY:-$LLVM_Z80_BIN/llvm-objcopy}"
OBJDUMP="${OBJDUMP:-$LLVM_Z80_BIN/llvm-objdump}"

TAP_TOOL="$DIR/mk-tap.py"

build() {
    require cargo
    require python3
    require readelf

    require_executable "$LD_LLD"
    require_executable "$OBJCOPY"
    require_file "$TAP_TOOL"

    echo "toolchain:  $TOOLCHAIN"
    echo "target:     $TARGET"
    echo "sysroot:    $SYSROOT"
    echo "linker:     $LD_LLD"
    echo "llvm tools: $LLVM_Z80_BIN"
    echo

    export CARGO_TARGET_DIR="$TARGET_DIR"
    export CARGO_TARGET_Z80_UNKNOWN_NONE_ELF_LINKER="$LD_LLD"

	cd "$DIR"

    cargo "+$TOOLCHAIN" build \
        --release \
        --bin "$NAME"

    require_file "$ELF"

    ENTRY="$(
        readelf -h "$ELF" |
            awk '/Entry point address:/ { print $4; exit }'
    )"

    if [ "$ENTRY" != "$ORIGIN_HEX" ]; then
        echo "error: unexpected ELF entry point: $ENTRY" >&2
        echo "expected: $ORIGIN_HEX" >&2
        exit 1
    fi

    "$OBJCOPY" \
        -O binary \
        "$ELF" \
        "$BIN"

    python3 "$TAP_TOOL" \
        "$BIN" \
        "$TAP" \
        "$ORIGIN_HEX"

    echo
    echo "built:"
    echo "  elf: $ELF ($(wc -c < "$ELF" | tr -d ' ') bytes)"
    echo "  bin: $BIN ($(wc -c < "$BIN" | tr -d ' ') bytes)"
    echo "  tap: $TAP ($(wc -c < "$TAP" | tr -d ' ') bytes)"
    echo "  entry: $ENTRY"
}

run() {
    require fuse

    echo
    echo "Starting Fuse with:"
    echo "  $TAP"
    echo
    echo "At the Spectrum BASIC prompt:"
    echo
    echo "  CLEAR $((ORIGIN_DEC - 1))"
    echo "  LOAD \"\" CODE"
    echo "  RANDOMIZE USR $ORIGIN_DEC"
    echo

    fuse \
        -m 48 \
        -t "$TAP"
}

inspect() {
    require file
    require readelf

    echo
    echo "===== FILE ====="
    file "$ELF"

    echo
    echo "===== ELF HEADER ====="
    readelf -h "$ELF"

    echo
    echo "===== SECTIONS ====="
    readelf -S "$ELF"

    echo
    echo "===== ENTRY SYMBOL ====="
    readelf -s "$ELF" | grep '_start' || true

    echo
    echo "===== ARTIFACTS ====="
    echo "elf: $(wc -c < "$ELF" | tr -d ' ') bytes"
    echo "bin: $(wc -c < "$BIN" | tr -d ' ') bytes"
    echo "tap: $(wc -c < "$TAP" | tr -d ' ') bytes"
}

dump_text() {
    require_executable "$OBJDUMP"

    echo "ELF: $ELF"

    echo
    echo "===== ELF HEADER ====="
    readelf -h "$ELF"

    echo
    echo "===== SECTIONS ====="
    readelf -S "$ELF"

    echo
    echo "===== SYMBOLS ====="
    readelf -s "$ELF"

    echo
    echo "===== Z80 DISASSEMBLY ====="
    "$OBJDUMP" \
        -d \
        --triple=z80 \
        "$ELF"
}

dump() {
    if [ -t 1 ] && [ -n "${EDITOR:-}" ]; then
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
    run)
        build
        run
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
        echo "usage: $0 [build|run|inspect|dump] [binary]" >&2
        exit 2
        ;;
esac
