#!/bin/sh
set -eu

DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
TARGET_DIR="$DIR/target"
OUT_DIR="$DIR/out"

. "$DIR/_shared/targets.sh"

# Probe -----------------------------------------------------------------------

PROBE_DIR="$(pwd -P)"
MANIFEST="$PROBE_DIR/Cargo.toml"
PROBE="${PROBE_DIR##*/}"
PACKAGE="sentry-$PROBE"

[ -f "$MANIFEST" ] || {
    echo "error: no Cargo.toml in current directory:" >&2
    echo "  $PROBE_DIR" >&2
    echo >&2
    echo "run this script from a probe package, e.g.:" >&2
    echo "  cd $DIR/num" >&2
    echo "  ../inspect.sh avr" >&2
    exit 2
}

# Defaults --------------------------------------------------------------------

PROFILE="${PROFILE:-inspect}"
DEFAULT_TARGETS="${DEFAULT_TARGETS:-host}"
DEFAULT_KIND="${DEFAULT_KIND:-lib}"
DEFAULT_BIN="${DEFAULT_BIN:-}"

# Optional per-probe configuration.
[ -f "$PROBE_DIR/inspect.conf" ] && . "$PROBE_DIR/inspect.conf"

KIND="$DEFAULT_KIND"
BIN="$DEFAULT_BIN"
SELECTORS=""
FEATURES=""
ALL_FEATURES=0
NO_DEFAULT_FEATURES=0

usage() {
    cat <<EOF
usage:
  ../inspect.sh [OPTIONS] [TARGET|GROUP]...

Run from a probe package directory.

options:
  --lib                     inspect the library target (default)
  --bin NAME                inspect a binary target
  -F, --features FEATURES   enable Cargo features
  --all-features            enable all Cargo features
  --no-default-features     disable default Cargo features
  -h, --help                show this help

EOF
    target_usage
    cat <<EOF

examples:
  ../inspect.sh
  ../inspect.sh avr
  ../inspect.sh host avr
  ../inspect.sh cpu8
  ../inspect.sh mcu
  ../inspect.sh all
  ../inspect.sh -F scale avr
  ../inspect.sh --bin bitten cpu8

generated:
  $OUT_DIR/<probe>/<unit>/<target>.s

environment:
  PROFILE=<name>       Cargo profile (default: inspect)
  TOOLCHAIN=<name>     ESP32-S3 Rust toolchain (default: esp)
  ESPUP_EXPORT_FILE=…  Espressif environment (default: ~/export-esp.sh)

The script does not install Rust targets, rust-src, or toolchains.
EOF
}

display_path() {
    case "$1" in
        "$PROBE_DIR"/*)
            printf '%s\n' "${1#"$PROBE_DIR"/}"
            ;;
        "$DIR"/*)
            printf '../%s\n' "${1#"$DIR"/}"
            ;;
        *)
            printf '%s\n' "$1"
            ;;
    esac
}

# Arguments -------------------------------------------------------------------

while [ "$#" -gt 0 ]; do
    case "$1" in
        --lib)
            KIND=lib
            BIN=""
            ;;
        --bin)
            [ "$#" -ge 2 ] || {
                echo "error: --bin requires a name" >&2
                exit 2
            }
            KIND=bin
            BIN=$2
            shift
            ;;
        -F|--features)
            [ "$#" -ge 2 ] || {
                echo "error: $1 requires a feature list" >&2
                exit 2
            }
            FEATURES="${FEATURES}${FEATURES:+,}$2"
            shift
            ;;
        --all-features)
            ALL_FEATURES=1
            ;;
        --no-default-features)
            NO_DEFAULT_FEATURES=1
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        -*)
            echo "error: unknown option: $1" >&2
            echo >&2
            usage >&2
            exit 2
            ;;
        *)
            SELECTORS="${SELECTORS}${SELECTORS:+ }$1"
            ;;
    esac
    shift
done

case "$KIND" in
    lib)
        UNIT=lib
        ;;
    bin)
        [ -n "$BIN" ] || {
            echo "error: binary target requires a name" >&2
            exit 2
        }
        UNIT=$BIN
        ;;
    *)
        echo "error: invalid probe kind: $KIND" >&2
        exit 2
        ;;
esac

[ -n "$SELECTORS" ] || SELECTORS="$DEFAULT_TARGETS"

# Target selection ------------------------------------------------------------

TARGETS=""
for selector in $SELECTORS; do
    select_target "$selector"
done

# Output ----------------------------------------------------------------------

OUT="$OUT_DIR/$PROBE/$UNIT"
mkdir -p "$OUT"

# Cargo -----------------------------------------------------------------------

cargo_emit_asm() {
    ASM=$1

    set -- rustc

    if [ "$TARGET_BUILD_STD" -eq 1 ]; then
        set -- "$@" -Zbuild-std=core
    fi

    set -- "$@" \
        --manifest-path "$MANIFEST" \
        --target-dir "$TARGET_DIR" \
        --profile "$PROFILE"

    case "$KIND" in
        lib) set -- "$@" --lib ;;
        bin) set -- "$@" --bin "$BIN" ;;
    esac

    [ "$NO_DEFAULT_FEATURES" -eq 0 ] || set -- "$@" --no-default-features
    [ "$ALL_FEATURES" -eq 0 ] || set -- "$@" --all-features
    [ -z "$FEATURES" ] || set -- "$@" --features "$FEATURES"
    [ -z "$TARGET_TRIPLE" ] || set -- "$@" --target "$TARGET_TRIPLE"

    set -- "$@" -- --emit=asm="$ASM"

    (
        if [ "$TARGET_SETUP" = "esp" ]; then
            ESP_ENV="${ESPUP_EXPORT_FILE:-$HOME/export-esp.sh}"
            [ -r "$ESP_ENV" ] || {
                echo "error: ESP environment file not found: $ESP_ENV" >&2
                exit 1
            }
            . "$ESP_ENV"
        fi

        if [ -n "$TARGET_RUSTFLAGS" ]; then
            CURRENT_RUSTFLAGS="${RUSTFLAGS-}"
            export RUSTFLAGS="${CURRENT_RUSTFLAGS}${CURRENT_RUSTFLAGS:+ }$TARGET_RUSTFLAGS"
        fi

        if [ -n "$TARGET_TOOLCHAIN" ]; then
            cargo "+$TARGET_TOOLCHAIN" "$@"
        else
            cargo "$@"
        fi
    )
}

inspect_target() {
    target_config "$1"

    ASM="$OUT/$TARGET_OUTPUT.s"

    cargo_emit_asm "$ASM"

    # Cargo does not track our explicitly emitted assembly as one of its
    # ordinary artifacts. If it was deleted while Cargo's build remained
    # fresh, clean only this probe package and regenerate it.
    if [ ! -f "$ASM" ]; then
        cargo clean \
            --manifest-path "$MANIFEST" \
            --target-dir "$TARGET_DIR" \
            -p "$PACKAGE"

        cargo_emit_asm "$ASM"
    fi

    [ -f "$ASM" ] || {
        echo "error: assembly was not emitted: $(display_path "$ASM")" >&2
        exit 1
    }

    echo "  -> $(display_path "$ASM")"

    if [ -n "${INSPECT_OUTPUT_LIST:-}" ]; then
        printf '%s\n' "$ASM" >> "$INSPECT_OUTPUT_LIST"
    fi
}

# Run -------------------------------------------------------------------------

for target in $TARGETS; do
    echo
    echo "== $PROBE/$UNIT :: $target =="
    inspect_target "$target"
done
