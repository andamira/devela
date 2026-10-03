#!/bin/sh
#
# Builds, flashes, inspects or dumps an ESP32-C3 example binary.

set -eu

#* Config *#

# Invoking directory; remains the example directory through a local symlink.
DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"

# Resolve the actual tool directory to find shared helpers.
SELF="$0"
while [ -L "$SELF" ]; do
    BASE="$(CDPATH= cd -- "$(dirname -- "$SELF")" && pwd)"
    LINK="$(readlink "$SELF")"

    case "$LINK" in
        /*) SELF="$LINK" ;;
        *)  SELF="$BASE/$LINK" ;;
    esac
done
TOOLS_DIR="$(CDPATH= cd -- "$(dirname -- "$SELF")" && pwd)"
. "$TOOLS_DIR/_flash-common.sh"

# Target
TARGET="riscv32imc-unknown-none-elf"
TARGET_DIR="$DIR/target"

# Invocation
ACTION="${1:-}"
NAME="${2:-blink}"

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
    require "$FLASHER"

    echo
    echo "flashing direct-boot image: $PORT"

    ESPFLASH_PORT="$PORT" \
        "$FLASHER" write-bin 0x0 "$IMAGE"
}

dispatch
