#!/bin/sh
# devela/examples/sys/hw/mcu/esp32/c3_supermini_oled042/run.sh
#
# Builds and optionally flashes the ESP32-C3 direct-boot LED example.

set -eu

DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
TARGET_DIR="$DIR/target"
TARGET="riscv32imc-unknown-none-elf"
BIN="${2:-led}"

ELF="$TARGET_DIR/$TARGET/release/$BIN"
IMAGE="$TARGET_DIR/$TARGET/release/$BIN.bin"

PORT="${PORT:-/dev/ttyACM0}"

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

    cargo build --release --bin "$BIN" --target-dir "$TARGET_DIR"

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
    echo "binary:             $ELF"
    echo "image:              $IMAGE"
    echo "image size:         $(wc -c < "$IMAGE") bytes"

    if command -v llvm-size >/dev/null 2>&1; then
        echo
        llvm-size "$ELF"
    fi
}
flash() {
    command -v espflash >/dev/null 2>&1 || {
        echo "error: espflash not found" >&2
        exit 1
    }

    echo
    echo "flashing direct-boot image: $PORT"

    ESPFLASH_PORT="$PORT" \
        espflash write-bin 0x0 "$IMAGE"
}
inspect() {
    rust-nm -n "$ELF" | grep ' _start$'
    rust-objdump -d --disassemble-symbols=_start "$ELF"
}

case "${1:-run}" in
    build)
        build
        ;;
    run|flash)
        build
        flash
        ;;
    inspect)
        build
        inspect
        ;;
    *)
        echo "usage: $0 [build|run|flash|inspect] [binary]" >&2
        exit 2
        ;;
esac
