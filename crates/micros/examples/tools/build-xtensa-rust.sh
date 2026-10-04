#!/bin/sh
#
# Builds and installs an Espressif Xtensa Rust toolchain for x86_64 Linux.
#
# usage:
#   build-xtensa-rust.sh <version> <build-dir> [toolchain-name]
#
# examples:
#   build-xtensa-rust.sh 1.99.0.0 /tmp/rust-build
#   build-xtensa-rust.sh 1.99.0.0 . esp-local
#
# <version> is the exact esp-rs extended version, including its fourth component.
#
# <build-dir> must already exist. Building Rust and LLVM can
# consume tens of gigabytes, so no default build location is chosen.
#
# The default rustup toolchain name is `esp-<version>`.
#
# espup is still required separately: its Xtensa GCC/binutils environment
# is reused for linking binaries produced by this Rust toolchain.

set -eu

VERSION="${1:-}"
BUILD_DIR="${2:-}"
TOOLCHAIN="${3:-}"

if [ -z "$VERSION" ] || [ -z "$BUILD_DIR" ]; then
    echo "usage: ${0##*/} <version> <build-dir> [toolchain-name]" >&2
    echo "example: ${0##*/} 1.99.0.0 /tmp/rust-build" >&2
    exit 2
fi

if ! printf '%s\n' "$VERSION" |
    grep -Eq '^[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+$'
then
    echo "error: expected an esp-rs version such as 1.99.0.0" >&2
    exit 2
fi

if [ ! -d "$BUILD_DIR" ]; then
    echo "error: build directory does not exist: $BUILD_DIR" >&2
    exit 2
fi

BUILD_DIR="$(CDPATH= cd -- "$BUILD_DIR" && pwd)"
TOOLCHAIN="${TOOLCHAIN:-esp-$VERSION}"

HOST="x86_64-unknown-linux-gnu"
SRC="$BUILD_DIR/rust-esp-$VERSION"

RUSTUP_HOME="${RUSTUP_HOME:-$HOME/.rustup}"
DEST="$RUSTUP_HOME/toolchains/$TOOLCHAIN"

if [ "$(uname -s)" != "Linux" ] || [ "$(uname -m)" != "x86_64" ]; then
    echo "error: this script currently supports x86_64 Linux only" >&2
    exit 1
fi

if [ -e "$SRC" ]; then
    echo "error: source directory already exists: $SRC" >&2
    exit 1
fi

if [ -e "$DEST" ]; then
    echo "error: rustup toolchain already exists: $DEST" >&2
    exit 1
fi

mkdir -p "$BUILD_DIR"

echo "version:    $VERSION"
echo "build dir:  $SRC"
echo "toolchain:  $DEST"
echo

git clone --recursive --depth 1 --shallow-submodules \
    -b "esp-$VERSION" \
    https://github.com/esp-rs/rust.git \
    "$SRC"

cd "$SRC"

python3 src/bootstrap/configure.py \
    --experimental-targets=Xtensa \
    --release-channel=nightly \
    --release-description="$VERSION" \
    --enable-extended \
    --enable-cargo-native-static \
    --tools=rustdoc,clippy,cargo,rustfmt,rust-analyzer-proc-macro-srv,src \
    --dist-compression-formats=xz \
    --enable-lld \
    --enable-profiler \
    --host "$HOST"

# Build the complete host toolchain.
python3 x.py dist --stage 2

# Xtensa targets use build-std, so install the matching Rust sources too.
python3 x.py dist rust-src

DIST="$SRC/build/dist"

cd "$DIST"

tar -xf "rust-nightly-$HOST.tar.xz"

bash "rust-nightly-$HOST/install.sh" \
    --destdir="$DEST" \
    --prefix='' \
    --without=rust-docs-json-preview,rust-docs \
    --disable-ldconfig

tar -xf rust-src-nightly.tar.xz

bash rust-src-nightly/install.sh \
    --destdir="$DEST" \
    --prefix='' \
    --disable-ldconfig

echo
echo "installed Rust toolchain: $TOOLCHAIN"
echo "build tree: $SRC"
echo
rustc "+$TOOLCHAIN" --version
cargo "+$TOOLCHAIN" --version
rustc "+$TOOLCHAIN" --print target-list |
    grep -x xtensa-esp32s3-none-elf
