#!/bin/sh
# devela/examples/sys/env/minimal/run.sh

set -eu

DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
ROOT=$(CDPATH= cd -- "$DIR/../../../.." && pwd)
TARGET_DIR="${CARGO_TARGET_DIR:-$ROOT/target/examples}"
TARGET_DIR="${TARGET_DIR%/}"

MODE="${1:-}"

show_size() {
	binary="$1"
	bytes=$(wc -c < "$binary" | tr -d '[:space:]')
	kib=$(awk -v bytes="$bytes" 'BEGIN { printf "%.2f", bytes / 1024 }')
	printf '\nbinary: %s\n' "$binary"
	printf 'size:   %s bytes (%s KiB)\n' "$bytes" "$kib"
}

case "$MODE" in
	clib)
		TARGET=$(rustc +nightly -vV | sed -n 's/^host: //p')
		PACKAGE="minimal-clib"

		cargo +nightly \
			--quiet \
			-Zscript \
			run \
			--manifest-path "$DIR/clib.rs" \
			--target-dir "$TARGET_DIR" \
			--target "$TARGET" \
			--release
		;;

	linux)
		TARGET="x86_64-unknown-linux-none"
		PACKAGE="minimal-linux"

		cargo +nightly \
			--quiet \
			--config "target.$TARGET.rustflags = ['-C', 'relocation-model=static']" \
			-Zbuild-std=core,compiler_builtins \
			-Zbuild-std-features=compiler-builtins-mem \
			-Zscript \
			run \
			--manifest-path "$DIR/linux.rs" \
			--target-dir "$TARGET_DIR" \
			--target "$TARGET" \
			--release
		;;

	none|none-compact)
		TARGET="x86_64-unknown-none"
		PACKAGE="minimal-linux"

		TARGET_RUSTFLAGS="['-C', 'relocation-model=static']"
		if [ "$MODE" = "none-compact" ]; then
			TARGET_RUSTFLAGS="['-C', 'relocation-model=static', '-C', 'link-arg=-N']"
		fi

		cargo +nightly \
			--quiet \
			--config "target.$TARGET.rustflags = $TARGET_RUSTFLAGS" \
			-Zscript \
			run \
			--manifest-path "$DIR/linux.rs" \
			--target-dir "$TARGET_DIR" \
			--target "$TARGET" \
			--release
		;;

	*)
		echo "usage: $0 {clib|linux|none|none-compact}" >&2
		exit 2
		;;
esac

show_size "$TARGET_DIR/$TARGET/release/$PACKAGE"
