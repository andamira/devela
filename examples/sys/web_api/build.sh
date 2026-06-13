#!/bin/sh
# devela/examples/sys/web_api/build.sh
#
## install required tools
# $ apt install jq
#
## install alternative tools
# $ apt install esbuild terser
# $ cargo install wasm-opt

set -e

# CONFIG
CRATE_NAME="web_api" # in sync with Cargo.toml, public_html/index.html
PROFILE="release"
WEB_DIR="./public_html/"
#
WASM_NAME="${CRATE_NAME}.wasm"
WASM_MODE="${WASM_MODE:-copy}" # copy|opt
#
JS_ENTRY_NAME="api.js"
JS_MODE="${JS_MODE:-copy}" # copy|min|bundle
JS_LIB_DIR="../../../src/sys/os/browser/web/js/"
JS_OUT_DIR="${WEB_DIR}devela/"

RUSTFLAGS="-C target-feature=+bulk-memory,+simd128"
BUILD_CMD="cargo build --profile $PROFILE --target wasm32-unknown-unknown"

rustup target add wasm32-unknown-unknown

# BUILD
echo "$ export RUSTFLAGS=\"$RUSTFLAGS\""
echo "$ $BUILD_CMD"
#
export RUSTFLAGS=$RUSTFLAGS

BUILD_JSON="$(mktemp)"
trap 'rm -f "$BUILD_JSON"' EXIT
if ! ${BUILD_CMD} --message-format=json-diagnostic-rendered-ansi > "$BUILD_JSON"; then
	jq -r 'select(.reason == "compiler-message") | .message.rendered // empty' "$BUILD_JSON" >&2
	exit 1
fi
WASM_PATH="$(jq -r '
	select(.reason == "compiler-artifact" and .filenames != null)
	| .filenames[]
	| select(endswith(".wasm"))
' "$BUILD_JSON" | tail -n 1)"


# WEB DIR
mkdir -p $WEB_DIR


# WASM
case "$WASM_MODE" in
	copy)
		echo "cp $WASM_PATH $WEB_DIR$WASM_NAME"
		cp "$WASM_PATH" "$WEB_DIR/$WASM_NAME"
		;;
	opt)
		command -v wasm-opt >/dev/null 2>&1 || {
			echo "error: wasm-opt not found. Install it or use WASM_MODE=copy." >&2
			exit 1
		}
		echo "wasm-opt -Oz $WASM_PATH -> $WEB_DIR$WASM_NAME"
		wasm-opt -Oz -all --strip-debug --strip-producers \
			-o "$WEB_DIR/$WASM_NAME" "$WASM_PATH"
		;;
	*)
		echo "error: unknown WASM_MODE '$WASM_MODE'. Use: copy, opt." >&2
		exit 1
		;;
esac


# JS_LIB
rm -rf "$JS_OUT_DIR"
mkdir -p "$JS_OUT_DIR"

case "$JS_MODE" in
	copy)
		echo "cp -r ${JS_LIB_DIR}*.js $JS_OUT_DIR"
		cp "${JS_LIB_DIR}"*.js "$JS_OUT_DIR"
		;;
	min)
		command -v terser >/dev/null 2>&1 || {
			echo "error: terser not found. Install it or use JS_MODE=copy." >&2
			exit 1
		}
		for src in "${JS_LIB_DIR}"*.js; do
			file="$(basename "$src")"
			echo "terser $src -> ${JS_OUT_DIR}${file}"
			terser "$src" -c -o "${JS_OUT_DIR}${file}"
		done
		;;
	bundle)
		command -v esbuild >/dev/null 2>&1 || {
			echo "error: esbuild not found. Install it or use JS_MODE=copy." >&2
			exit 1
		}
		echo "esbuild ${JS_LIB_DIR}${JS_ENTRY_NAME} -> ${JS_OUT_DIR}${JS_ENTRY_NAME}"
		esbuild "${JS_LIB_DIR}${JS_ENTRY_NAME}" \
			--bundle \
			--format=esm \
			--minify \
			--outfile="${JS_OUT_DIR}${JS_ENTRY_NAME}"
		;;
	*)
		echo "error: unknown JS_MODE '$JS_MODE'. Use: copy, min, bundle." >&2
		exit 1
		;;
esac


# CLEAN PERMISSIONS
chmod -x "${WEB_DIR}${WASM_NAME}"


echo "Done."
echo "start a server in the ./public_html directory to see the result. E.g.:"
echo "python3 -m http.server --directory public_html"
