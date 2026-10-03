#!/bin/sh
#
# Builds, flashes, inspects or dumps an MSP430G2231 example binary.

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
TARGET="${TARGET:-msp430-none-elf}"
TARGET_DIR="$DIR/target"

# Invocation
ACTIONS="build|inspect|dump|flash"
ACTION="${1:-}"
NAME="${2:-${DEFAULT_NAME:-}}"

# Artifacts
ELF="$TARGET_DIR/$TARGET/release/$NAME"

# Inspection
INSPECT_SYMBOLS="${INSPECT_SYMBOLS:-12}"

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

    echo "elf:   $(display_path "$ELF")"

    if command -v "$SIZE" >/dev/null 2>&1; then
        size
    fi
}

flash_action() {
    require "$FLASHER"

    echo
    echo "flashing via MSPDebug/$FLASHER_DRIVER"

    "$FLASHER" "$FLASHER_DRIVER" "prog $ELF"
}

dispatch
