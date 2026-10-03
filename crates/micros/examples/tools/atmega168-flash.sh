#!/bin/sh
#
# Builds, flashes, inspects or dumps an ATmega168 example binary.

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
TARGET="avr-none"
TARGET_DIR="$DIR/target"

# Invocation
ACTION="${1:-flash}"
NAME="${2:-blink}"

# Artifacts
ELF="$TARGET_DIR/$TARGET/release/$NAME.elf"

# Inspection
INSPECT_SYMBOLS="${INSPECT_SYMBOLS:-12}"

# Flashing
PORT="${PORT:-/dev/ttyUSB0}"
UPLOAD_BAUD="${UPLOAD_BAUD:-19200}"

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

    echo
    echo "flashing: $PORT @ $UPLOAD_BAUD baud"

    "$FLASHER" \
        -p atmega168 \
        -c arduino \
        -P "$PORT" \
        -b "$UPLOAD_BAUD" \
        -D \
        -U "flash:w:$ELF:e"
}

dispatch
