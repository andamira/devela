#!/bin/sh
#
# Builds, flashes, inspects or dumps a CH32V003 example binary.
#
# TOC
# - configuration
# - require()
# - build()
# - flash()
# - inspect()
# - dump_text()
# - dump()
# - action dispatch

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
TARGET="riscv32e-unknown-none-elf"
TARGET_DIR="$DIR/target"

# Invocation
ACTION="${1:-flash}"
NAME="${2:-minimal}"

# Artifacts
ELF="$TARGET_DIR/$TARGET/release/$NAME"
BIN="$TARGET_DIR/$TARGET/release/$NAME.bin"

# Inspection
INSPECT_SYMBOLS="${INSPECT_SYMBOLS:-12}"

# Flashing
PART="${PART:-t4}"
PROGRAMMER="${PROGRAMMER:-usbasp}"

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
    "$OBJCOPY" -O binary "$ELF" "$BIN"

    echo
    echo "elf:    $ELF"
    echo "binary: $BIN"
    echo "size:   $(wc -c < "$BIN") bytes"

    if command -v "$SIZE" >/dev/null 2>&1; then
        echo
        "$SIZE" "$ELF"
    fi
}

flash() {
    require "$FLASHER"

    echo
    echo "flashing CH32V003"

    "$FLASHER" flash "$BIN"
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
