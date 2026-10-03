#!/bin/sh
#
# Builds, flashes, inspects or dumps a SAM example binary.

set -eu

#* Config *#

TOOLS_DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
. "$TOOLS_DIR/_flash-common.sh"

DIR="${EXAMPLE_DIR:-}"
[ -n "$DIR" ] || {
    echo "error: EXAMPLE_DIR not configured" >&2
    exit 1
}

# Target
TARGET="${TARGET:-}"
TARGET_DIR="$DIR/target"
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
BOSSAC_PORT="${PORT#/dev/}"

# Host tools
OBJCOPY="${OBJCOPY:-arm-none-eabi-objcopy}"
SIZE="${SIZE:-arm-none-eabi-size}"
NM="${NM:-arm-none-eabi-nm}"
OBJDUMP="${OBJDUMP:-arm-none-eabi-objdump}"
FLASHER="${FLASHER:-bossac}"
PYTHON="${PYTHON:-python3}"


#* Functions *#

build() {
    cd "$DIR"

    cargo build \
        --release \
        --bin "$NAME" \
        --target-dir "$TARGET_DIR"

    require "$OBJCOPY"
    "$OBJCOPY" -O binary "$ELF" "$IMAGE"

    echo "elf:                $(display_path "$ELF")"
    echo "image:              $(display_path "$IMAGE")"
    echo "image size:         $(wc -c < "$IMAGE") bytes"

    if command -v "$SIZE" >/dev/null 2>&1; then
        size
    fi
}

enter_samba() {
    echo
    echo "entering SAM-BA: $PORT"

    # Use BOSSA only to trigger the Due Programming Port's 1200-baud
    # ERASE + RESET sequence. Some BOSSA versions reconnect before ROM
    # SAM-BA is ready, so ignore that first connection result and wait.
    "$FLASHER" \
        --port="$BOSSAC_PORT" \
        --usb-port=0 \
        --arduino-erase \
        >/dev/null 2>&1 || true

    sleep 1.5
}

flash() {
    echo
    echo "flashing: $PORT"

    "$FLASHER" \
        --port="$BOSSAC_PORT" \
        --usb-port=0 \
        -e -w -v -b \
        "$IMAGE"
}

reset() {
    echo
    echo "resetting: $PORT"

    # At a non-1200 baud rate, the Due's ATmega16U2 treats a DTR rising
    # edge as RESET-only. This avoids BOSSA -R, which did not reliably
    # leave the tested board running after upload.
    "$PYTHON" - "$PORT" <<'PY'
import fcntl
import os
import struct
import sys
import termios
import time

fd = os.open(sys.argv[1], os.O_RDWR | os.O_NOCTTY)

attrs = termios.tcgetattr(fd)
attrs[4] = termios.B115200
attrs[5] = termios.B115200
termios.tcsetattr(fd, termios.TCSANOW, attrs)

dtr = struct.pack("I", termios.TIOCM_DTR)
fcntl.ioctl(fd, termios.TIOCMBIC, dtr)
time.sleep(0.05)
fcntl.ioctl(fd, termios.TIOCMBIS, dtr)
time.sleep(0.10)

os.close(fd)
PY

    sleep 0.5
}

flash_action() {
    require "$FLASHER"
    require "$PYTHON"

    [ -e "$PORT" ] || {
        echo "error: serial port not found: $PORT" >&2
        exit 1
    }

    build
    enter_samba
    flash
    reset
}

dispatch
