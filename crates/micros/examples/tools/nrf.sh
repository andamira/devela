#!/bin/sh
#
# Shared nRF52840 UF2 build, package, inspect, dump, and flash tool.
# Example-local wrappers select the board's layout and defaults.

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
TARGET="${TARGET:-thumbv7em-none-eabihf}"
TARGET_DIR="${TARGET_DIR:-$DIR/target}"

# Invocation
ACTIONS="build|inspect|dump|uf2|flash"
ACTION="${1:-}"
NAME="${2:-${DEFAULT_NAME:-}}"

# Artifacts
ELF="$TARGET_DIR/$TARGET/release/$NAME"
IMAGE="$ELF.bin"
UF2="$ELF.uf2"

# Inspection
INSPECT_SYMBOLS="${INSPECT_SYMBOLS:-12}"

# UF2 application layout (Adafruit bootloader + S140 v6)
FLASH_BASE="${FLASH_BASE:-0x26000}"
FLASH_END="${FLASH_END:-0xEC000}"

# Host tools
if command -v llvm-objcopy >/dev/null 2>&1; then
    DEFAULT_OBJCOPY=llvm-objcopy
else
    DEFAULT_OBJCOPY=arm-none-eabi-objcopy
fi
OBJCOPY="${OBJCOPY:-$DEFAULT_OBJCOPY}"
SIZE="${SIZE:-arm-none-eabi-size}"
NM="${NM:-arm-none-eabi-nm}"
OBJDUMP="${OBJDUMP:-arm-none-eabi-objdump}"
PYTHON="${PYTHON:-python3}"

#* Functions *#

build() {
    cd "$DIR"

    cargo build \
        --release \
        --bin "$NAME" \
        --target-dir "$TARGET_DIR"

    [ -f "$ELF" ] || {
        echo "error: expected ELF not found: $ELF" >&2
        exit 1
    }
    echo "elf:   $(display_path "$ELF")"

    if command -v "$SIZE" >/dev/null 2>&1; then
        size
    fi
}

uf2_action() {
    require "$OBJCOPY"
    require "$PYTHON"

    "$OBJCOPY" -O binary "$ELF" "$IMAGE"
    "$PYTHON" "$TOOLS_DIR/uf2.py" "$IMAGE" "$UF2" \
        --base "$FLASH_BASE" --end "$FLASH_END"

    echo "image: $(display_path "$IMAGE")"
    echo "uf2:   $(display_path "$UF2")"
}

flash_action() {
    [ -n "${MOUNT:-}" ] || {
        echo "error: MOUNT must point to the mounted NICENANO UF2 volume" >&2
        exit 2
    }
    require mountpoint
    mountpoint -q "$MOUNT" || {
        echo "error: not a mountpoint: $MOUNT" >&2
        exit 2
    }
    [ -f "$MOUNT/INFO_UF2.TXT" ] || {
        echo "error: INFO_UF2.TXT not found: $MOUNT" >&2
        exit 2
    }
    grep -qi 'nRF52840-nicenano' "$MOUNT/INFO_UF2.TXT" || {
        echo "error: unexpected UF2 board ID" >&2
        exit 2
    }
    [ "${CONFIRM_LAYOUT:-}" = s140_v6 ] || {
        echo "error: inspect bootloader/SoftDevice and set CONFIRM_LAYOUT=s140_v6" >&2
        exit 2
    }

    uf2_action

    # Avoid silently writing to an ordinary directory if USB disconnected.
    mountpoint -q "$MOUNT" || {
        echo "error: UF2 volume disconnected before upload" >&2
        exit 2
    }
    echo "flashing UF2: $(display_path "$UF2") -> $MOUNT/"
    cp "$UF2" "$MOUNT/"
    sync
}

case "$ACTION" in
    uf2) build; uf2_action ;;
    *) dispatch ;;
esac
