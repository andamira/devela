#!/bin/sh
#
# Shared AVR build, flash, inspect, and dump tool.
# Example-local wrappers provide device and programmer defaults.

set -eu

#* Config *#

# Invoking directory; remains the example directory through a local symlink,
# or may be provided explicitly by a wrapper.
INVOKING_DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
DIR="${EXAMPLE_DIR:-$INVOKING_DIR}"

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
TARGET="avr-none"
TARGET_DIR="$DIR/target"

# Invocation
ACTION="${1:-}"
NAME="${2:-${DEFAULT_NAME:-blink}}"

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

    if command -v "$SIZE" >/dev/null 2>&1; then
        echo
        "$SIZE" "$ELF"
    fi

    echo
    echo "elf: $ELF"
}

flash() {
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
