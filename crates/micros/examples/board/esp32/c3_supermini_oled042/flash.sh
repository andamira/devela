#!/bin/sh
#
# Builds, flashes, or inspects an ESP32-C3 direct-boot example.

set -eu

DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
TARGET_DIR="$DIR/target"
TARGET="riscv32imc-unknown-none-elf"

ACTION="${1:-flash}"
NAME="${2:-led_on}"

PORT="${PORT:-/dev/ttyACM0}"

ELF="$TARGET_DIR/$TARGET/release/$NAME"
IMAGE="$TARGET_DIR/$TARGET/release/$NAME.bin"

require() {
    command -v "$1" >/dev/null 2>&1 || {
        echo "error: $1 not found" >&2
        exit 1
    }
}

find_objcopy() {
    for tool in rust-objcopy llvm-objcopy; do
        if command -v "$tool" >/dev/null 2>&1; then
            printf '%s\n' "$tool"
            return
        fi
    done

    echo "error: rust-objcopy or llvm-objcopy is required" >&2
    exit 1
}

build() {
    cd "$DIR"

    cargo build \
        --release \
        --bin "$NAME" \
        --target-dir "$TARGET_DIR"

    OBJCOPY="$(find_objcopy)"
    "$OBJCOPY" -O binary "$ELF" "$IMAGE"

    HEADER="$(
        od -An -tx1 -N8 "$IMAGE" |
        tr -d '[:space:]'
    )"

    if [ "$HEADER" != "1d04dbae1d04dbae" ]; then
        echo "error: invalid ESP32-C3 direct-boot header: $HEADER" >&2
        exit 1
    fi

    echo
    echo "direct-boot header: $HEADER"
    echo "elf:                $ELF"
    echo "image:              $IMAGE"
    echo "image size:         $(wc -c < "$IMAGE") bytes"

    if command -v llvm-size >/dev/null 2>&1; then
        echo
        llvm-size "$ELF"
    fi
}

flash() {
    require espflash

    echo
    echo "flashing direct-boot image: $PORT"

    ESPFLASH_PORT="$PORT" \
        espflash write-bin 0x0 "$IMAGE"
}

inspect() {
    require rust-nm
    require rust-objdump

    rust-nm -n "$ELF" | grep ' _start$'
    rust-objdump -d --disassemble-symbols=_start "$ELF"
}

case "$ACTION" in
    build)
        build
        ;;
    flash)
        build
        flash
        ;;
    inspect)
        build
        inspect
        ;;
    *)
        echo "usage: $0 [build|flash|inspect] [binary]" >&2
        exit 2
        ;;
esac
