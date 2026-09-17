#!/bin/sh

set -e

cd "$(dirname "$0")"

# CONFIG
CRATE_NAME="devela_bridge"
PROFILE="bridge"
LIB_DIR="./libs"

BUILD_CMD="cargo rustc --lib --profile $PROFILE -F std --crate-type staticlib,cdylib"


# GENERATE BINDINGS

echo "$ cargo run --quiet --bin bridge_bindgen -F __bindgen"
cargo run --quiet --bin bridge_bindgen -F __bindgen

# BUILD

echo "$ $BUILD_CMD"

if ! command -v jq >/dev/null 2>&1; then
    echo "error: jq is required to discover generated library paths" >&2
    echo "install: sudo apt install jq" >&2
    exit 1
fi

BUILD_JSON="$(mktemp)"
trap 'rm -f "$BUILD_JSON"' EXIT

if ! ${BUILD_CMD} --message-format=json-diagnostic-rendered-ansi > "$BUILD_JSON"; then
    jq -r '
        select(.reason == "compiler-message")
        | .message.rendered // empty
    ' "$BUILD_JSON" >&2
    exit 1
fi

mkdir -p "$LIB_DIR"

# Cargo tells us the actual artifact paths.
LIB_PATHS="$(jq -r '
    select(
        .reason == "compiler-artifact"
        and .target.name == "'"$CRATE_NAME"'"
    )
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
                install_name_tool \
                    -id "@rpath/$name" \
                    "$LIB_DIR/$name" 2>/dev/null || true
            fi
            ;;
    esac
done

echo
echo "Done. Copied libraries:"
find "$LIB_DIR" -maxdepth 1 -type f -print | sort
