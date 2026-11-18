#!/bin/sh
#
# Build Rust-MOS from the pinned, patch-based rust-mos-sync source recipe.
#
# Manual equivalent, after cloning https://github.com/sbechet/rust-mos-sync:
#   scripts/fetch.sh llvm && scripts/apply.sh llvm base-stable
#   scripts/fetch.sh rust && scripts/apply.sh rust base-stable
#   STAGE=1 scripts/build-rust.sh stable
#   # If needed: (cd "$WORK/rust" && ./x build --stage 1 library \
#   #                 --target x86_64-unknown-linux-gnu)
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
# MOS_INSTALL_SUFFIX=<tag> allows another immutable snapshot for the same
# upstream revision (for example, after changing the build configuration).
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

# A target-only bootstrap run can leave host std but omit proc_macro.
# This matters for host-side proc-macro crates even when the final binary is
# no_std. Repair it incrementally, preserving the completed MOS target sysroot.
HOST="$("$STAGE_DIR/bin/rustc" -vV | sed -n 's/^host: //p')"
[ -n "$HOST" ] || { echo 'error: cannot determine Rust-MOS host triple' >&2; exit 1; }
HOST_LIBS="$STAGE_DIR/lib/rustlib/$HOST/lib"
READY_STAGE="$STAGE_DIR"
TEMP_STAGE=
cleanup() { [ -z "$TEMP_STAGE" ] || rm -rf "$TEMP_STAGE"; }
trap cleanup EXIT

if ! ls "$HOST_LIBS"/libproc_macro-*.rlib >/dev/null 2>&1; then
    echo 'host proc_macro missing; rebuilding complete host libraries' >&2
    TEMP_STAGE="$BUILD_DIR/.rust-mos-stage-hostfix.$$"
    [ ! -e "$TEMP_STAGE" ] || {
        echo "error: temporary stage exists: $TEMP_STAGE" >&2
        exit 1
    }
    # Preserve target libraries: bootstrap may reset the stage sysroot.
    cp -a --reflink=auto "$STAGE_DIR" "$TEMP_STAGE"
    (cd "$WORK/rust" && ./x build --stage "$STAGE" library --target "$HOST")
    ls "$HOST_LIBS"/libproc_macro-*.rlib >/dev/null 2>&1 || {
        echo 'error: host proc_macro still missing after the rebuild' >&2
        exit 1
    }
    rm -rf "$TEMP_STAGE/lib/rustlib/$HOST/lib"
    cp -a "$HOST_LIBS" "$TEMP_STAGE/lib/rustlib/$HOST/lib"
    READY_STAGE="$TEMP_STAGE"
fi

require_rlib() {
    LIB_DIR="$1"
    LIB_NAME="$2"
    ls "$LIB_DIR/lib$LIB_NAME-"*.rlib >/dev/null 2>&1 || {
        echo "error: missing $LIB_NAME in $LIB_DIR" >&2
        exit 1
    }
}

require_rlib "$READY_STAGE/lib/rustlib/$HOST/lib" std
require_rlib "$READY_STAGE/lib/rustlib/$HOST/lib" proc_macro
for MOS_TARGET in $MOS_TARGETS; do
    for LIB in core alloc compiler_builtins; do
        require_rlib "$READY_STAGE/lib/rustlib/$MOS_TARGET/lib" "$LIB"
    done
done

# Register immutable snapshots only. Never silently overwrite or reuse a
# previously installed but incomplete compiler.
REV="$(git -C "$SRC" rev-parse --short HEAD)"
INSTALL_BASE="$BUILD_DIR/rust-mos-installs"
SUFFIX="${MOS_INSTALL_SUFFIX:-}"
case "$SUFFIX" in
    *[!a-zA-Z0-9._-]*) echo 'error: invalid MOS_INSTALL_SUFFIX' >&2; exit 2 ;;
esac
INSTALL="$INSTALL_BASE/$VERSION-$REV${SUFFIX:+-$SUFFIX}"
mkdir -p "$INSTALL_BASE"
if [ -e "$INSTALL" ]; then
    require_rlib "$INSTALL/lib/rustlib/$HOST/lib" std
    if ! ls "$INSTALL/lib/rustlib/$HOST/lib"/libproc_macro-*.rlib >/dev/null 2>&1; then
        echo "error: existing snapshot lacks host proc_macro: $INSTALL" >&2
        echo 'set MOS_INSTALL_SUFFIX=hostfix (and select a new rustup toolchain name)' >&2
        exit 1
    fi
    for MOS_TARGET in $MOS_TARGETS; do
        for LIB in core alloc compiler_builtins; do
            require_rlib "$INSTALL/lib/rustlib/$MOS_TARGET/lib" "$LIB"
        done
    done
    echo "using existing validated snapshot: $INSTALL"
else
    TMP="$INSTALL.tmp.$$"
    [ ! -e "$TMP" ] || { echo "error: temporary installation exists: $TMP" >&2; exit 1; }
    cp -a --reflink=auto "$READY_STAGE" "$TMP"
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
