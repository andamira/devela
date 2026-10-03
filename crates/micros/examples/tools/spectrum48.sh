#!/bin/sh
#
# Builds, packages, runs, inspects or dumps ZX Spectrum 48K examples.

set -eu

#* Config *#

TOOLS_DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
. "$TOOLS_DIR/_common.sh"

DIR="${EXAMPLE_DIR:-}"
[ -n "$DIR" ] || {
    echo "error: EXAMPLE_DIR not configured" >&2
    exit 1
}

# Target
TARGET="z80-unknown-none-elf"
TARGET_DIR="$DIR/target"
TOOLCHAIN="${TOOLCHAIN:-rust-z80}"

# Invocation
ACTIONS="build|inspect|dump|run"
ACTION="${1:-}"
NAME="${2:-${DEFAULT_NAME:-}}"

# Artifacts
ELF="$TARGET_DIR/$TARGET/release/$NAME.elf"
IMAGE="$TARGET_DIR/$TARGET/release/$NAME.bin"
TAP="$TARGET_DIR/$TARGET/release/$NAME.tap"

# Inspection
INSPECT_SYMBOLS="${INSPECT_SYMBOLS:-12}"

# Image
ORIGIN_HEX="0x8000"

# Host tools
PYTHON="${PYTHON:-python3}"
RUNNER="${RUNNER:-fuse}"

# These are resolved from the Rust-Z80 installation by setup_toolchain().
SIZE="${SIZE:-}"
NM="${NM:-}"
OBJDUMP="${OBJDUMP:-}"
OBJCOPY="${OBJCOPY:-}"
LD_LLD="${LD_LLD:-}"

TAP_TOOL="$DIR/mk-tap.py"


#* Functions *#

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

setup_toolchain() {
    require rustc

    SYSROOT="$(rustc "+$TOOLCHAIN" --print sysroot)"
    HOST="$(rustc "+$TOOLCHAIN" -vV | sed -n 's/^host: //p')"

    # With `rustup toolchain link rust-z80 rust-z80/install`,
    # the checkout is normally the parent of the sysroot.
    RUST_Z80_ROOT="${RUST_Z80_ROOT:-$(dirname "$SYSROOT")}"

    # llvm-z80 currently builds its tools here.
    LLVM_Z80_BIN="${LLVM_Z80_BIN:-$RUST_Z80_ROOT/build/$HOST/llvm/bin}"

    LD_LLD="${LD_LLD:-$SYSROOT/lib/rustlib/$HOST/bin/gcc-ld/ld.lld}"
    OBJCOPY="${OBJCOPY:-$LLVM_Z80_BIN/llvm-objcopy}"
    OBJDUMP="${OBJDUMP:-$LLVM_Z80_BIN/llvm-objdump}"
    SIZE="${SIZE:-$LLVM_Z80_BIN/llvm-size}"
    NM="${NM:-$LLVM_Z80_BIN/llvm-nm}"

    require_executable "$LD_LLD"
    require_executable "$OBJCOPY"
    require_executable "$OBJDUMP"
    require_executable "$SIZE"
    require_executable "$NM"
}

build() {
    require cargo
    require readelf
    require "$PYTHON"
    require_file "$TAP_TOOL"

    setup_toolchain

    cd "$DIR"

    CARGO_TARGET_Z80_UNKNOWN_NONE_ELF_LINKER="$LD_LLD" \
        cargo "+$TOOLCHAIN" build \
            --release \
            --bin "$NAME" \
            --target-dir "$TARGET_DIR"

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
        "$IMAGE"

    "$PYTHON" "$TAP_TOOL" \
        "$IMAGE" \
        "$TAP" \
        "$ORIGIN_HEX" \
        >/dev/null

    echo "entry:      $ENTRY"
    echo "elf:        $(display_path "$ELF")"
    echo "image:      $(display_path "$IMAGE")"
    echo "image size: $(wc -c < "$IMAGE") bytes"
    echo "tap:        $(display_path "$TAP")"
    echo "tap size:   $(wc -c < "$TAP") bytes"

    size
}

run_action() {
    require "$RUNNER"

    echo "running: $(display_path "$TAP")"

    "$RUNNER" \
        -m 48 \
        -t "$TAP"
}

dump_disassembly() {
    "$OBJDUMP" \
        -d \
        --triple=z80 \
        "$ELF"
}

dispatch
