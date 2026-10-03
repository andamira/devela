#!/bin/sh
#
# Shared AVR build, flash, inspect, and dump tool.
# Example-local wrappers provide device and programmer defaults.

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
TARGET="${TARGET:-avr-none}"
TARGET_DIR="$DIR/target"

# Invocation
ACTIONS="build|inspect|dump|flash"
ACTION="${1:-}"
NAME="${2:-${DEFAULT_NAME:-}}"

# Artifacts
ELF="$TARGET_DIR/$TARGET/release/$NAME.elf"

# Inspection
INSPECT_SYMBOLS="${INSPECT_SYMBOLS:-12}"

# Flashing
PART="${PART:-}"
PROGRAMMER="${PROGRAMMER:-}"
PORT="${PORT:-}"
UPLOAD_BAUD="${UPLOAD_BAUD:-}"
NO_ERASE="${NO_ERASE:-0}"

# Host tools
SIZE="${SIZE:-avr-size}"
NM="${NM:-avr-nm}"
OBJDUMP="${OBJDUMP:-avr-objdump}"
FLASHER="${FLASHER:-avrdude}"


#* Functions *#

build() {
    cd "$DIR"

    cargo +nightly build \
        --release \
        --bin "$NAME" \
        --target-dir "$TARGET_DIR"

    echo "elf:   $(display_path "$ELF")"

    if command -v "$SIZE" >/dev/null 2>&1; then
        size
    fi
}

flash_action() {
    require "$FLASHER"

    [ -n "$PART" ] || {
        echo "error: AVR part not configured (PART)" >&2
        exit 1
    }
    [ -n "$PROGRAMMER" ] || {
        echo "error: AVR programmer not configured (PROGRAMMER)" >&2
        exit 1
    }

    set -- \
        -p "$PART" \
        -c "$PROGRAMMER"

    if [ -n "$PORT" ]; then
        set -- "$@" -P "$PORT"
    fi

    if [ -n "$UPLOAD_BAUD" ]; then
        set -- "$@" -b "$UPLOAD_BAUD"
    fi

    if [ "$NO_ERASE" = 1 ]; then
        set -- "$@" -D
    fi

    echo
    if [ -n "$PORT" ]; then
        echo "flashing: $PART via $PROGRAMMER on $PORT"
    else
        echo "flashing: $PART via $PROGRAMMER"
    fi

    "$FLASHER" "$@" \
        -U "flash:w:$ELF:e"
}

dispatch
