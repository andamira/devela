#!/bin/sh
#
# Builds, flashes, inspects or dumps a CH32V003 example binary.

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
TARGET="${TARGET:-riscv32e-unknown-none-elf}"
TARGET_DIR="$DIR/target"

# Invocation
ACTIONS="build|inspect|dump|flash"
ACTION="${1:-}"
NAME="${2:-${DEFAULT_NAME:-}}"

# Artifacts
ELF="$TARGET_DIR/$TARGET/release/$NAME"
IMAGE="$TARGET_DIR/$TARGET/release/$NAME.bin"

# Inspection
INSPECT_SYMBOLS="${INSPECT_SYMBOLS:-12}"

# Host tools
OBJCOPY="${OBJCOPY:-rust-objcopy}"
SIZE="${SIZE:-rust-size}"
NM="${NM:-rust-nm}"
OBJDUMP="${OBJDUMP:-rust-objdump}"
READOBJ="${READOBJ:-rust-readobj}"
FLASHER="${FLASHER:-wlink}"


#* Functions *#

build() {
    cd "$DIR"

    cargo +nightly build \
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

flash_action() {
    require "$FLASHER"

    echo
    echo "flashing CH32V003"

    "$FLASHER" flash "$IMAGE"
}

dump_extra_before() {
    require "$READOBJ"

    echo
    echo "===== RISC-V ATTRIBUTES ====="
    "$READOBJ" --arch-specific "$ELF"
}

dump_extra_after() {
    echo
    echo "===== DISASSEMBLY, NO ALIASES ====="
    "$OBJDUMP" -d -M no-aliases "$ELF"
}

dispatch
