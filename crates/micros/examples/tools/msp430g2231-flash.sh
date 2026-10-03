#!/bin/sh
#
# Builds, flashes, inspects or dumps an MSP430G2231 example binary.

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
TARGET="msp430-none-elf"
TARGET_DIR="$DIR/target"

# Invocation
ACTION="${1:-}"
NAME="${2:-minimal}"

# Artifacts
ELF="$TARGET_DIR/$TARGET/release/$NAME"

# Inspection
INSPECT_SYMBOLS="${INSPECT_SYMBOLS:-12}"

# Flashing
PORT="${PORT:-/dev/ttyACM0}"
UPLOAD_BAUD="${UPLOAD_BAUD:-115200}"

# Host tools
CC="${CC:-msp430-elf-gcc}"
SIZE="${SIZE:-msp430-elf-size}"
NM="${NM:-msp430-elf-nm}"
OBJDUMP="${OBJDUMP:-msp430-elf-objdump}"
FLASHER="${FLASHER:-mspdebug}"
FLASHER_DRIVER="${FLASHER_DRIVER:-rf2500}"


#* Functions *#

build() {
    require "$CC"

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
    echo "flashing via MSPDebug/$FLASHER_DRIVER"

    "$FLASHER" "$FLASHER_DRIVER" "prog $ELF"
}

dispatch
