#!/bin/sh
#
# Atari 8-bit DOS example runner.

set -eu

TOOLS_DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
. "$TOOLS_DIR/_common.sh"

DIR="${EXAMPLE_DIR:-}"
[ -n "$DIR" ] || { echo "error: EXAMPLE_DIR not configured" >&2; exit 1; }

TARGET="mos-unknown-none"
TARGET_DIR="$DIR/target"
TOOLCHAIN="${TOOLCHAIN:-rust-mos}"

ACTIONS="build|inspect|dump|run"
ACTION="${1:-}"
NAME="${2:-${DEFAULT_NAME:-atari800xl}}"

ARTIFACT_DIR="$TARGET_DIR/$TARGET/release"
RUSTLIB="$ARTIFACT_DIR/libcomputer_atari800xl.a"
XEX="$ARTIFACT_DIR/$NAME.xex"
ELF="$XEX.elf"

RUNNER="${ATARI800:-atari800}"
INSPECT_SYMBOLS="${INSPECT_SYMBOLS:-12}"

require_file() {
    [ -f "$1" ] || { echo "error: file not found: $1" >&2; exit 1; }
}

setup_toolchain() {
    require rustup
    rustup which --toolchain "$TOOLCHAIN" rustc >/dev/null
    rustup which --toolchain "$TOOLCHAIN" cargo >/dev/null

    if [ -z "${SDKBIN:-}" ]; then
        require mos-atari8-dos-clang
        SDKBIN="$(dirname "$(command -v mos-atari8-dos-clang)")"
    fi

    require "$SDKBIN/mos-atari8-dos-clang"

    SIZE="${SIZE:-$SDKBIN/llvm-size}"
    NM="${NM:-$SDKBIN/llvm-nm}"
    OBJDUMP="${OBJDUMP:-$SDKBIN/llvm-objdump}"
}

build() {
    setup_toolchain
    cd "$DIR"

    RUSTC_BOOTSTRAP=1 \
        rustup run "$TOOLCHAIN" cargo build \
            --release --ignore-rust-version --target-dir "$TARGET_DIR"

    require_file "$RUSTLIB"
    "$SDKBIN/mos-atari8-dos-clang" -mcpu=mos6502 -Os \
        "$DIR/driver.c" "$RUSTLIB" -o "$XEX"

    require_file "$XEX"
    require_file "$ELF"
    echo "xex: $(display_path "$XEX")"
    echo "elf: $(display_path "$ELF")"
}

run_action() {
    require "$RUNNER"
    "$RUNNER" -xl -nobasic -run "$XEX"
}

dump_disassembly() {
    "$OBJDUMP" -d --mcpu=mos6502 "$ELF"
}

dispatch
