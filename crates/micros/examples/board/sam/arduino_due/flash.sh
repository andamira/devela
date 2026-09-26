#!/bin/sh
#
# Builds or flashes an Arduino Due example.

set -eu

DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
TARGET_DIR="$DIR/target"
TARGET="thumbv7m-none-eabi"

ACTION="${1:-flash}"
NAME="${2:-blink}"

PORT="${PORT:-/dev/ttyACM0}"
BOSSAC_PORT="${PORT#/dev/}"

ELF="$TARGET_DIR/$TARGET/release/$NAME"
BIN="$TARGET_DIR/$TARGET/release/$NAME.bin"

require() {
    command -v "$1" >/dev/null 2>&1 || {
        echo "error: $1 not found" >&2
        exit 1
    }
}

build() {
    cd "$DIR"

    cargo build \
        --release \
        --bin "$NAME" \
        --target-dir "$TARGET_DIR"

    require arm-none-eabi-objcopy
    arm-none-eabi-objcopy -O binary "$ELF" "$BIN"

    if command -v arm-none-eabi-size >/dev/null 2>&1; then
        echo
        arm-none-eabi-size "$ELF"
    fi

    echo
    echo "elf:    $ELF"
    echo "binary: $BIN"
}

enter_samba() {
    echo
    echo "entering SAM-BA: $PORT"

    # Use BOSSA only to trigger the Due Programming Port's 1200-baud
    # ERASE + RESET sequence. Some BOSSA versions reconnect before ROM
    # SAM-BA is ready, so ignore that first connection result and wait.
    bossac \
        --port="$BOSSAC_PORT" \
        --usb-port=0 \
        --arduino-erase \
        >/dev/null 2>&1 || true

    sleep 1.5
}

flash() {
    echo
    echo "flashing: $PORT"

    bossac \
        --port="$BOSSAC_PORT" \
        --usb-port=0 \
        -e -w -v -b \
        "$BIN"
}

reset() {
    echo
    echo "resetting: $PORT"

    # At a non-1200 baud rate, the Due's ATmega16U2 treats a DTR rising
    # edge as RESET-only. This avoids BOSSA -R, which did not reliably
    # leave the tested board running after upload.
    python3 - "$PORT" <<'PY'
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

case "$ACTION" in
    build)
        build
        ;;
    flash)
        build
        require bossac
        require python3
        [ -e "$PORT" ] || {
            echo "error: serial port not found: $PORT" >&2
            exit 1
        }
        enter_samba
        flash
        reset
        ;;
    *)
        echo "usage: $0 [build|flash] [binary]" >&2
        exit 2
        ;;
esac
