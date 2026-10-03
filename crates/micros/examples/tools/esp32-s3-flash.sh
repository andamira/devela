#!/bin/sh
#
# Builds, flashes, inspects or dumps an ESP-S3 example binary.

set -e

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
TARGET="xtensa-esp32s3-none-elf"
TARGET_DIR="$DIR/target"

# Invocation
ACTION="${1:-}"
NAME="${2:-blink}"
IGNORE_RUST_VERSION=
if [ "${3:-}" = "--ignore-rust-version" ]; then
    IGNORE_RUST_VERSION=--ignore-rust-version
fi

# Artifacts
ELF="$TARGET_DIR/$TARGET/release/$NAME"

# Inspection
INSPECT_SYMBOLS="${INSPECT_SYMBOLS:-12}"

# Flashing
PORT="${PORT:-/dev/ttyACM0}"

# Host tools supplied by the Espressif toolchain.
SIZE="${SIZE:-xtensa-esp-elf-size}"
NM="${NM:-xtensa-esp-elf-nm}"
OBJDUMP="${OBJDUMP:-xtensa-esp-elf-objdump}"
FLASHER="${FLASHER:-espflash}"


#* ESP environment *#

# espup generates this file after installing the Xtensa Rust environment.
ESP_ENV="${ESPUP_EXPORT_FILE:-$HOME/export-esp.sh}"

if [ ! -r "$ESP_ENV" ]; then
    echo "error: ESP environment file not found: $ESP_ENV" >&2
    echo "install the ESP32-S3 Rust toolchain with espup first" >&2
    exit 1
fi

# This modifies only this /bin/sh process and its children.
# It never replaces or modifies the user's interactive shell.
. "$ESP_ENV"

set -u


#* Functions *#

build() {
    cd "$DIR"

    cargo +esp build \
        ${IGNORE_RUST_VERSION:-} \
        --release \
        --bin "$NAME" \
        --target-dir "$TARGET_DIR"

    echo
    echo "elf: $ELF"

    if command -v "$SIZE" >/dev/null 2>&1; then
        echo
        "$SIZE" "$ELF"
    fi
}


flash() {
    require "$FLASHER"

    echo
    echo "flashing: $PORT"

    ESPFLASH_PORT="$PORT" \
        "$FLASHER" flash "$ELF"
}

dispatch
