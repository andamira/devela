#!/bin/sh
set -eu

PROBE_DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
PROBES_DIR="$(CDPATH= cd -- "$PROBE_DIR/.." && pwd)"
TARGET_DIR="$PROBES_DIR/target"

. "$PROBES_DIR/_shared/common.sh"

export CARGO_TARGET_DIR="$TARGET_DIR"
cd "$PROBE_DIR"

usage() {
    echo "usage: $0 {host|android|avr}"
}

case "${1:-}" in
    host)
        cargo run --bin stderr -F std
        ;;

    android)
        TARGET="${TARGET:-arm64-v8a}"
        API="${API:-21}"

        cargo ndk \
            -t "$TARGET" \
            -P "$API" \
            run --bin android -F android
        ;;

    avr)
        PROFILE="release"
        PORT="${PORT:-/dev/ttyUSB0}"
        UPLOAD_BAUD="${UPLOAD_BAUD:-115200}"

        AVR_RUSTFLAGS="${RUSTFLAGS-}"
        AVR_RUSTFLAGS="${AVR_RUSTFLAGS}${AVR_RUSTFLAGS:+ }-C target-cpu=atmega328p"

        RUSTFLAGS="$AVR_RUSTFLAGS" \
            cargo +nightly build \
                -Zbuild-std=core \
                --profile "$PROFILE" \
                --target avr-none \
                --bin avr \
                -F avr

        SIZE="${SIZE:-avr-size}"
        ELF="$TARGET_DIR/avr-none/$PROFILE/avr.elf"

        echo "elf:    $(display_path "$ELF")"

        if command -v "$SIZE" >/dev/null 2>&1; then
            size
        fi

        avrdude \
            -p atmega328p \
            -c arduino \
            -P "$PORT" \
            -b "$UPLOAD_BAUD" \
            -D \
            -U "flash:w:$ELF:e"

        echo
        echo "serial: picocom -b 9600 $PORT"
        ;;

    ""|-h|--help)
        usage
        ;;

    *)
        echo "error: unknown environment: $1" >&2
        usage >&2
        exit 2
        ;;
esac
