#!/bin/sh
#
# Builds, flashes, inspects or dumps an ESP-S3 example binary.

set -e

#* Config *#

TOOLS_DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
. "$TOOLS_DIR/_common.sh"

DIR="${EXAMPLE_DIR:-}"
[ -n "$DIR" ] || {
    echo "error: EXAMPLE_DIR not configured" >&2
    exit 1
}

# Target
TARGET="${TARGET:-xtensa-esp32s3-none-elf}"
TOOLCHAIN="${TOOLCHAIN:-esp}"
TARGET_DIR="$DIR/target"

# Invocation
ACTIONS="build|inspect|dump|flash|run"
ACTION="${1:-flash}"
NAME="${2:-${DEFAULT_NAME:-}}"
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

# espup may provide the GCC/binutils directory via an export file. It can
# also be supplied explicitly, independent of the Rust compiler installation.
ESP_ENV="${ESPUP_EXPORT_FILE:-$HOME/export-esp.sh}"
if [ -r "$ESP_ENV" ]; then
    . "$ESP_ENV"
elif [ -n "${ESPUP_EXPORT_FILE:-}" ]; then
    echo "error: ESPUP_EXPORT_FILE not found: $ESP_ENV" >&2
    exit 1
fi

if [ -n "${XTENSA_GCC_BIN:-}" ]; then
    [ -d "$XTENSA_GCC_BIN" ] || {
        echo "error: XTENSA_GCC_BIN is not a directory: $XTENSA_GCC_BIN" >&2
        exit 1
    }
    PATH="$XTENSA_GCC_BIN:$PATH"
    export PATH
fi

# Use the chip-specific wrapper: generic Xtensa GCC can default to big-endian.
# An explicit Cargo linker override can supply a separately configured driver.
LINKER="${CARGO_TARGET_XTENSA_ESP32S3_NONE_ELF_LINKER:-}"
if [ -z "$LINKER" ]; then
    if command -v xtensa-esp32s3-elf-gcc >/dev/null 2>&1; then
        LINKER=xtensa-esp32s3-elf-gcc
    else
        echo 'error: missing ESP32-S3 GCC wrapper: xtensa-esp32s3-elf-gcc' >&2
        echo 'set XTENSA_GCC_BIN to its bin directory' >&2
        exit 1
    fi
fi
require "$LINKER"
CARGO_TARGET_XTENSA_ESP32S3_NONE_ELF_LINKER="$LINKER"
export CARGO_TARGET_XTENSA_ESP32S3_NONE_ELF_LINKER

set -u


#* Functions *#

build() {
    cd "$DIR"

    cargo "+$TOOLCHAIN" build \
        ${IGNORE_RUST_VERSION:-} \
        --release \
        --bin "$NAME" \
        --target-dir "$TARGET_DIR"

    echo "elf:   $(display_path "$ELF")"

    if command -v "$SIZE" >/dev/null 2>&1; then
        size
    fi
}


# 'run' flashes and starts the firmware on the device.
run_action() {
    flash_action
}

flash_action() {
    require "$FLASHER"

    echo
    echo "flashing: $PORT"

    ESPFLASH_PORT="$PORT" \
        "$FLASHER" flash "$ELF"
}

dispatch
