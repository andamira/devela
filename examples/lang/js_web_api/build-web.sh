#!/bin/sh
#
## install optional tools
# $ apt install uglifyjs
# $ cargo install wasm-opt

set -ex

CRATE_NAME="js_web_api"
PROFILE="wasm"
WEB_DIR="./web/"
WASM="${CRATE_NAME}.wasm"
TARGET_DIR="target/wasm32-unknown-unknown/${PROFILE}/"
JS_DIR="../../../src/lang/js/"
# JS_URL="https://raw.githubusercontent.com/andamira/devela/refs/heads/main/src/lang/js/web_api.js"

CARGO_TARGET_DIR="./target/" cargo b --profile $PROFILE --target wasm32-unknown-unknown

mkdir -p $WEB_DIR

# wasm binary
cp "${TARGET_DIR}/${WASM}" $WEB_DIR
# wasm-opt -Oz --strip-debug --strip-producers -o "$WEB_DIR/$WASM" "${TARGET_DIR}${WASM}"

# js library
cp "${JS_DIR}web_api.js" $WEB_DIR
# uglifyjs "${JS_DIR}web_api.js" -o "${WEB_DIR}web_api.js"
# curl -s "${JS_URL}" > "${WEB_DIR}web_api.js"

chmod -x "${WEB_DIR}${WASM}"

# cleanup
# rm -r target

echo "start a server in the ./web directory to see the example"
