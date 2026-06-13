#!/bin/sh
# devela_ffi/build.sh

set -e

# CONFIG
CRATE_NAME="devela_ffi"
PROFILE="release"
LIB_DIR="./libs"
BUILD_CMD="cargo build --profile $PROFILE"

# GENERATE BINDINGS
echo "$ cargo run --quiet --bin ffi_bindgen"
cargo run --quiet --bin ffi_bindgen

# BUILD
echo "$ $BUILD_CMD"

BUILD_JSON="$(mktemp)"
trap 'rm -f "$BUILD_JSON"' EXIT

if ! ${BUILD_CMD} --message-format=json-diagnostic-rendered-ansi > "$BUILD_JSON"; then
    if command -v jq >/dev/null 2>&1; then
        jq -r 'select(.reason == "compiler-message") | .message.rendered // empty' "$BUILD_JSON" >&2
    else
        cat "$BUILD_JSON" >&2
    fi
    exit 1
fi

mkdir -p "$LIB_DIR"

if ! command -v jq >/dev/null 2>&1; then
    echo "error: jq is required to discover generated library paths" >&2
    echo "install: sudo apt install jq" >&2
    exit 1
fi

# Cargo tells us the real paths, so this works with custom CARGO_TARGET_DIR.
LIB_PATHS="$(jq -r '
    select(.reason == "compiler-artifact" and .target.name == "'"$CRATE_NAME"'")
    | .filenames[]
    | select(
        endswith(".so")
        or endswith(".dylib")
        or endswith(".dll")
        or endswith(".a")
        or endswith(".lib")
    )
' "$BUILD_JSON" | sort -u)"

if [ -z "$LIB_PATHS" ]; then
    echo "error: no generated system libraries found for $CRATE_NAME" >&2
    exit 1
fi

echo "$LIB_PATHS" | while IFS= read -r path; do
    [ -n "$path" ] || continue

    name="$(basename "$path")"
    echo "cp $path $LIB_DIR/$name"
    cp "$path" "$LIB_DIR/$name"

    case "$name" in
        *.so|*.dylib)
            if command -v strip >/dev/null 2>&1; then
                strip "$LIB_DIR/$name" 2>/dev/null || true
            fi
            ;;
    esac

    # macOS dynamic libraries remember their install name.
    case "$name" in
        *.dylib)
            if command -v install_name_tool >/dev/null 2>&1; then
                install_name_tool -id "@rpath/$name" "$LIB_DIR/$name" 2>/dev/null || true
            fi
            ;;
    esac
done

echo
echo "Done. Copied libraries:"
find "$LIB_DIR" -maxdepth 1 -type f -print | sort
