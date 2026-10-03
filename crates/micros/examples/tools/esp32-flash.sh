#!/bin/sh
#
# Shared ESP32 direct build, flash, inspect, and dump tool.
# Example-local wrappers provide target-specific defaults.

set -eu

#* Config *#

TOOLS_DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
. "$TOOLS_DIR/_flash-common.sh"

DIR="${EXAMPLE_DIR:-}"
TARGET="${TARGET:-}"
TARGET_DIR="$DIR/target"
CHIP="${CHIP:-ESP32}"

[ -n "$DIR" ] || {
    echo "error: EXAMPLE_DIR not configured" >&2
    exit 1
}
[ -n "$TARGET" ] || {
    echo "error: TARGET not configured" >&2
    exit 1
}

# Invocation
ACTION="${1:-}"
NAME="${2:-${DEFAULT_NAME:-}}"

# Artifacts
ELF="$TARGET_DIR/$TARGET/release/$NAME"
IMAGE="$TARGET_DIR/$TARGET/release/$NAME.bin"

# Inspection
INSPECT_SYMBOLS="${INSPECT_SYMBOLS:-12}"

# Flashing
PORT="${PORT:-/dev/ttyACM0}"

# Host tools
SIZE="${SIZE:-rust-size}"
NM="${NM:-rust-nm}"
OBJDUMP="${OBJDUMP:-rust-objdump}"
FLASHER="${FLASHER:-espflash}"


#* Functions *#

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
        echo "error: invalid $CHIP direct-boot header: $HEADER" >&2
        exit 1
    fi

    echo "direct-boot header: $HEADER"
    echo "elf:                $(display_path "$ELF")"
    echo "image:              $(display_path "$IMAGE")"
    echo "image size:         $(wc -c < "$IMAGE") bytes"

    if command -v "$SIZE" >/dev/null 2>&1; then
        size
    fi
}

flash() {
    require "$FLASHER"

    echo
    echo "flashing direct-boot image: $PORT"

    ESPFLASH_PORT="$PORT" \
        "$FLASHER" write-bin 0x0 "$IMAGE"
}

dispatch
