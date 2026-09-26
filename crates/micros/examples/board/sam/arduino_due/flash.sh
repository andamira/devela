#!/bin/sh
#
# Builds, flashes, inspects or dumps an Arduino Due example binary.
#
# TOC
# - configuration
# - require()
# - build()
# - enter_samba()
# - flash()
# - reset()
# - inspect()
# - dump_text()
# - dump()
# - action dispatch

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

# Host tools:
# - arm-none-eabi-{objcopy,size,nm,objdump}: GNU Arm binutils
# - bossac: SAM-BA entry and flashing
# - python3: final Programming-Port reset
OBJCOPY="${OBJCOPY:-arm-none-eabi-objcopy}"
SIZE="${SIZE:-arm-none-eabi-size}"
NM="${NM:-arm-none-eabi-nm}"
OBJDUMP="${OBJDUMP:-arm-none-eabi-objdump}"
INSPECT_SYMBOLS="${INSPECT_SYMBOLS:-12}"

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

    require "$OBJCOPY"
    "$OBJCOPY" -O binary "$ELF" "$BIN"

    if command -v "$SIZE" >/dev/null 2>&1; then
        echo
        "$SIZE" "$ELF"
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

inspect() {
    require "$SIZE"
    require "$NM"

    echo
    echo "elf: $ELF"

    echo
    echo "sections:"
    "$SIZE" -A "$ELF"

    echo
    echo "largest symbols:"
    "$NM" -S --size-sort "$ELF" | tail -n "$INSPECT_SYMBOLS"
}

dump_text() {
    require "$SIZE"
    require "$NM"
    require "$OBJDUMP"

    echo "ELF: $ELF"

    echo
    echo "===== SIZE ====="
    "$SIZE" -A "$ELF"

    echo
    echo "===== FILE ====="
    "$OBJDUMP" -f "$ELF"

    echo
    echo "===== SECTIONS ====="
    "$OBJDUMP" -h "$ELF"

    echo
    echo "===== SYMBOLS ====="
    "$NM" -n -S "$ELF"

    echo
    echo "===== DISASSEMBLY ====="
    "$OBJDUMP" -d "$ELF"
}

dump() {
    if [ -t 1 ] && [ -n "${EDITOR:-}" ]; then
        DUMP="$TARGET_DIR/$TARGET/release/$NAME.dump.txt"
        dump_text > "$DUMP"

        echo "dump: $DUMP"
        "$EDITOR" "$DUMP"
    else
        dump_text
    fi
}

case "$ACTION" in
    build)
        build
        ;;
    flash)
        require bossac
        require python3
        [ -e "$PORT" ] || {
            echo "error: serial port not found: $PORT" >&2
            exit 1
        }
        build
        enter_samba
        flash
        reset
        ;;
    inspect)
        build
        inspect
        ;;
    dump)
        build
        dump
        ;;
    *)
        echo "usage: $0 [build|flash|inspect|dump] [binary]" >&2
        exit 2
        ;;
esac
