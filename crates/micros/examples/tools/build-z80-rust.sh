#!/bin/sh
#
# Build or update the Rust-Z80 compiler and its pinned LLVM-Z80 backend.
#
# Manual equivalent (first installation):
#   git clone https://github.com/llvm-z80/rust-z80.git rust-z80
#   cd rust-z80
#   git submodule update --init --depth 1 src/llvm-project
#   ./x install
#   rustup toolchain link rust-z80 "$PWD/install"
#
# Usage:
#   build-z80-rust.sh <build-dir> [toolchain-name]
#   build-z80-rust.sh "$HOME/tmp/z80" rust-z80
#
# Existing source/build trees are reused. By default the parent Git repository
# is fast-forwarded; its pinned LLVM submodule is then checked out.
# Set Z80_UPDATE=0 to build the currently checked-out sources without fetching.
# Z80_EXPECT_RUST=1.100.0 rejects a compiler that reports a different version.
#
# The toolchain is linked only after successful compilation and verification.
# If the name is already registered to a different installation, it is left
# untouched: pick a new name or explicitly unregister the old one yourself.

set -eu

BUILD_DIR="${1:-}"
TOOLCHAIN="${2:-rust-z80}"
SRC_URL="https://github.com/llvm-z80/rust-z80.git"
UPDATE="${Z80_UPDATE:-1}"

if [ -z "$BUILD_DIR" ] || [ "$#" -gt 2 ]; then
    echo "usage: ${0##*/} <build-dir> [toolchain-name]" >&2
    exit 2
fi

for cmd in git rustup python3 cmake ninja; do
    command -v "$cmd" >/dev/null 2>&1 || {
        echo "error: missing command: $cmd" >&2
        exit 1
    }
done

mkdir -p "$BUILD_DIR"
BUILD_DIR="$(CDPATH= cd -- "$BUILD_DIR" && pwd)"
SRC="$BUILD_DIR/rust-z80"

if [ ! -e "$SRC" ]; then
    git clone "$SRC_URL" "$SRC"
elif [ ! -d "$SRC/.git" ]; then
    echo "error: $SRC exists, but is not a Git checkout" >&2
    exit 1
fi

if [ "$UPDATE" = 1 ]; then
    [ "$(git -C "$SRC" branch --show-current)" = master ] || {
        echo 'error: expected the master branch; set Z80_UPDATE=0 for a pinned checkout' >&2
        exit 1
    }
    git -C "$SRC" diff --quiet && git -C "$SRC" diff --cached --quiet || {
        echo 'error: local source modifications; refusing to update' >&2
        exit 1
    }
    git -C "$SRC" pull --ff-only origin master
elif [ "$UPDATE" != 0 ]; then
    echo 'error: Z80_UPDATE must be 0 or 1' >&2
    exit 2
fi

# The parent repository owns the LLVM revision. Do not git-pull the submodule.
git -C "$SRC" submodule update --init --depth 1 src/llvm-project

printf 'Rust-Z80 revision: '
git -C "$SRC" rev-parse --short HEAD
printf 'LLVM-Z80 revision: '
git -C "$SRC/src/llvm-project" rev-parse --short HEAD

(cd "$SRC" && ./x install)

INSTALL="$SRC/install"
[ -x "$INSTALL/bin/rustc" ] || {
    echo "error: expected compiler at $INSTALL/bin/rustc" >&2
    exit 1
}
"$INSTALL/bin/rustc" --print target-list | grep -qx z80-unknown-none-elf || {
    echo 'error: installed compiler does not advertise z80-unknown-none-elf' >&2
    exit 1
}
if [ -n "${Z80_EXPECT_RUST:-}" ]; then
    case "$("$INSTALL/bin/rustc" --version)" in
        "rustc $Z80_EXPECT_RUST"|"rustc $Z80_EXPECT_RUST "*) ;;
        *) echo "error: Rust-Z80 compiler is not Rust $Z80_EXPECT_RUST" >&2; exit 1 ;;
    esac
fi

# If already linked to this same installation, there is nothing to re-link.
if rustup toolchain list | awk '{print $1}' | grep -Fxq "$TOOLCHAIN"; then
    CURRENT="$(rustup which --toolchain "$TOOLCHAIN" rustc)"
    if [ "$(realpath "$CURRENT")" != "$(realpath "$INSTALL/bin/rustc")" ]; then
        echo "error: toolchain $TOOLCHAIN already points elsewhere: $CURRENT" >&2
        echo 'choose another name, or unregister the old name explicitly' >&2
        exit 1
    fi
else
    rustup toolchain link "$TOOLCHAIN" "$INSTALL"
fi

printf '\nRegistered: %s\n' "$TOOLCHAIN"
rustc "+$TOOLCHAIN" -vV
