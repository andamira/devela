#!/bin/sh
# devela::examples::lang::js_web::build.sh
#
## install required tools
# $ apt install jq
#
## install alternative tools
# $ apt install uglifyjs
# $ cargo install wasm-opt

set -e

# CONFIG
CRATE_NAME="js_web"
WASM_NAME="${CRATE_NAME}.wasm"
JS_LIB_DIR="../../../src/lang/ffi/js/web/"
JS_LIB_NAME="web_api.js"
# JS_LIB_URL="https://raw.githubusercontent.com/andamira/devela/refs/heads/main/src/lang/ffi/js/web/${JS_LIB_NAME}"
PROFILE="release"
WEB_DIR="./public_html/"
# WAIT:https://blog.rust-lang.org/2025/04/04/c-abi-changes-for-wasm32-unknown-unknown/
# RUSTFLAGS="-C target-feature=+bulk-memory,+simd128" # WAIT:stable-wasm-c-abi
RUSTFLAGS="-C target-feature=+bulk-memory,+simd128 -Zwasm-c-abi=spec" # TEMP
# BUILD_CMD="cargo build --profile $PROFILE --target wasm32-unknown-unknown" WAIT:stable-wasm-c-abi
BUILD_CMD="cargo +nightly build --profile $PROFILE --target wasm32-unknown-unknown" # TEMP


# BUILD
echo "$ export RUSTFLAGS=\"$RUSTFLAGS\""
echo "$ $BUILD_CMD"
#
export RUSTFLAGS=$RUSTFLAGS
WASM_PATH=$(${BUILD_CMD} --message-format=json \
	| jq -r 'select(.filenames != null) | .filenames[] | select(endswith(".wasm"))' )


# WEB DIR
mkdir -p $WEB_DIR


# WASM
echo "cp $WASM_PATH $WEB_DIR$WASM_NAME"
cp "$WASM_PATH" "$WEB_DIR/$WASM_NAME"
#
# alternative:
# wasm-opt -Oz --strip-debug --strip-producers -o "$WEB_DIR/$WASM_NAME" "${WASM_PATH}"


# JS_LIB
echo " to $WEB_DIR$JS_LIB_NAME"
cp "${JS_LIB_DIR}${JS_LIB_NAME}" "${WEB_DIR}"
#
# alternative:
# uglifyjs "${JS_LIB_DIR}${JS_LIB_NAME}" -o "${WEB_DIR}${JS_LIB_NAME}"
#
# alternative:
# curl -s "${JS_LIB_URL}" > "${WEB_DIR}${JS_LIB_NAME}"


# CLEAN PERMISSIONS
chmod -x "${WEB_DIR}${WASM_NAME}"


echo "Done."
echo "start a server in the ./public_html directory to see the result. E.g.:"
echo "python3 -m http.server --directory public_html"
