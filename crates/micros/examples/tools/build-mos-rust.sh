#!/bin/sh
#
# Build Rust-MOS from the pinned, patch-based rust-mos-sync source recipe.
#
# Manual equivalent, after cloning https://github.com/sbechet/rust-mos-sync:
#   scripts/fetch.sh llvm && scripts/apply.sh llvm base-stable
#   scripts/fetch.sh rust && scripts/apply.sh rust base-stable
#   STAGE=1 scripts/build-rust.sh stable
#   # link the printed stage directory with rustup toolchain link
#
# Usage:
#   build-mos-rust.sh <rust-version> <build-dir> [toolchain-name]
#   build-mos-rust.sh 1.100.0 "$HOME/tmp/atari" rust-mos-1.100.0
#
# Uses the project's own cache, pinned SDK, patches and bootstrap. Existing
# work and LLVM caches are reused. The requested Rust version must match the
# version currently pinned by rust-mos-sync; do not silently build another.
#
# Defaults: MOS_UPDATE=1 (git pull --ff-only), STAGE=1, and
# MOS_TARGETS='mos-unknown-none mos-sim-none'. Set MOS_TARGETS to more target
# triples to build their core/alloc libraries too. Build only, no flashing.
#
# A versioned sysroot copy avoids later incremental builds overwriting the
# installed toolchain. A new rustup name leaves any existing 'rust-mos' intact.
# Register 'rust-mos' as an alias after verifying the new toolchain.

set -eu

VERSION="${1:-}"
BUILD_DIR="${2:-}"
TOOLCHAIN="${3:-rust-mos-$VERSION}"
UPDATE="${MOS_UPDATE:-1}"

if [ -z "$VERSION" ] || [ -z "$BUILD_DIR" ] || [ "$#" -gt 3 ]; then
    echo "usage: ${0##*/} <rust-version> <build-dir> [toolchain-name]" >&2
    exit 2
fi

case "$VERSION" in
    *[!0-9.]*|'') echo 'error: expected a Rust version such as 1.100.0' >&2; exit 2 ;;
esac

if [ "$(uname -s)-$(uname -m)" != Linux-x86_64 ]; then
    echo 'error: this helper currently supports Linux x86_64 only' >&2
    exit 1
fi

for cmd in git rustup bash python3 cmake ninja curl tar sha256sum; do
    command -v "$cmd" >/dev/null 2>&1 || {
        echo "error: missing command: $cmd" >&2
        exit 1
    }
done

mkdir -p "$BUILD_DIR"
BUILD_DIR="$(CDPATH= cd -- "$BUILD_DIR" && pwd)"
SRC="$BUILD_DIR/rust-mos-sync"

if [ ! -e "$SRC" ]; then
    git clone --depth 1 https://github.com/sbechet/rust-mos-sync.git "$SRC"
elif [ ! -d "$SRC/.git" ]; then
    echo "error: $SRC exists, but is not a Git checkout" >&2
    exit 1
fi

if [ "$UPDATE" = 1 ]; then
    [ "$(git -C "$SRC" branch --show-current)" = main ] || {
        echo 'error: expected main; set MOS_UPDATE=0 for a pinned checkout' >&2
        exit 1
    }
    git -C "$SRC" diff --quiet && git -C "$SRC" diff --cached --quiet || {
        echo 'error: local source modifications; refusing to update' >&2
        exit 1
    }
    git -C "$SRC" pull --ff-only origin main
elif [ "$UPDATE" != 0 ]; then
    echo 'error: MOS_UPDATE must be 0 or 1' >&2
    exit 2
fi

PINNED_VERSION="$(python3 - "$SRC/versions.toml" <<'PY'
import sys, tomllib
with open(sys.argv[1], 'rb') as f:
    print(tomllib.load(f)['rust']['stable']['tag'])
PY
)"
[ "$VERSION" = "$PINNED_VERSION" ] || {
    echo "error: requested Rust $VERSION; upstream pins $PINNED_VERSION" >&2
    echo 'use a matching rust-mos-sync source revision or another toolchain version' >&2
    exit 1
}

# Share cache and work across upgrades, never store large build trees inside
# the cloned recipe repository itself.
export WORK="${WORK:-$BUILD_DIR/rust-mos-work}"
export CACHE="${CACHE:-$BUILD_DIR/rust-mos-cache}"
export STAGE="${STAGE:-1}"
export MOS_TARGETS="${MOS_TARGETS:-mos-unknown-none mos-sim-none}"

(
    cd "$SRC"
    bash scripts/fetch.sh llvm stable
    bash scripts/apply.sh llvm base-stable
    bash scripts/fetch.sh rust stable
    bash scripts/apply.sh rust base-stable
)

STAGE_DIR="$(cd "$SRC" && bash scripts/build-rust.sh stable)"
[ -x "$STAGE_DIR/bin/rustc" ] || {
    echo "error: expected compiler at $STAGE_DIR/bin/rustc" >&2
    exit 1
}
case "$("$STAGE_DIR/bin/rustc" --version)" in
    "rustc $VERSION"|"rustc $VERSION "*) ;;
    *) echo "error: built compiler does not report Rust $VERSION" >&2; exit 1 ;;
esac
"$STAGE_DIR/bin/rustc" --print target-list | grep -qx mos-unknown-none || {
    echo 'error: installed compiler does not advertise mos-unknown-none' >&2
    exit 1
}
[ -d "$STAGE_DIR/lib/rustlib/mos-unknown-none/lib" ] || {
    echo 'error: missing mos-unknown-none target libraries' >&2
    exit 1
}

# Install an immutable snapshot: the stage directory is mutable and will be
# reused by the next build, so it must never serve as a versioned rustup path.
REV="$(git -C "$SRC" rev-parse --short HEAD)"
INSTALL_BASE="$BUILD_DIR/rust-mos-installs"
INSTALL="$INSTALL_BASE/$VERSION-$REV"
mkdir -p "$INSTALL_BASE"
if [ ! -e "$INSTALL" ]; then
    TMP="$INSTALL.tmp.$$"
    [ ! -e "$TMP" ] || { echo "error: temporary installation exists: $TMP" >&2; exit 1; }
    cp -a --reflink=auto "$STAGE_DIR" "$TMP"
    mv "$TMP" "$INSTALL"
fi

if rustup toolchain list | awk '{print $1}' | grep -Fxq "$TOOLCHAIN"; then
    CURRENT="$(rustup which --toolchain "$TOOLCHAIN" rustc)"
    if [ "$(realpath "$CURRENT")" != "$(realpath "$INSTALL/bin/rustc")" ]; then
        echo "error: toolchain $TOOLCHAIN already points elsewhere: $CURRENT" >&2
        exit 1
    fi
else
    rustup toolchain link "$TOOLCHAIN" "$INSTALL"
fi

SDKBIN="$(cd "$SRC" && bash scripts/build-sdk.sh)/bin"

printf '\nRegistered: %s\n' "$TOOLCHAIN"
rustc "+$TOOLCHAIN" -vV
printf 'SDK tools: %s\n' "$SDKBIN"
printf 'Atari example: TOOLCHAIN=%s SDKBIN=%s ./example.sh build\n' "$TOOLCHAIN" "$SDKBIN"
