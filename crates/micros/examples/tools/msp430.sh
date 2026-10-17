#!/bin/sh
#
# Builds, flashes, inspects or dumps an MSP430 example binary.

set -eu

#* Config *#

TOOLS_DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
. "$TOOLS_DIR/_common.sh"

DIR="${EXAMPLE_DIR:-}"
[ -n "$DIR" ] || {
    echo "error: EXAMPLE_DIR not configured" >&2
    exit 1
}

# Target
TARGET="${TARGET:-msp430-none-elf}"
TARGET_DIR="${TARGET_DIR:-$DIR/target}"

# Invocation
ACTIONS="build|inspect|dump|flash"
ACTION="${1:-}"
NAME="${2:-${DEFAULT_NAME:-}}"

# Artifacts
ELF="$TARGET_DIR/$TARGET/release/$NAME"

# Inspection
INSPECT_SYMBOLS="${INSPECT_SYMBOLS:-12}"

# Host tools
CC="${CC:-msp430-elf-gcc}"
SIZE="${SIZE:-msp430-elf-size}"
NM="${NM:-msp430-elf-nm}"
OBJDUMP="${OBJDUMP:-msp430-elf-objdump}"
FLASHER="${FLASHER:-mspdebug}"
FLASHER_DRIVER="${FLASHER_DRIVER:-rf2500}"


#* Functions *#

build() {
    require "$CC"

    cd "$DIR"

    cargo +nightly build \
        --release \
        --bin "$NAME" \
        --target-dir "$TARGET_DIR"

    echo "elf:   $(display_path "$ELF")"

    if command -v "$SIZE" >/dev/null 2>&1; then
        size
    fi
}

flash_action() {
    require "$FLASHER"

    case "${MSP430_CHIP:-}" in
        g2231) expected=2231 ;;
        g2452) expected=2452 ;;
        g2553) expected=2553 ;;
        *) echo "error: MSP430_CHIP required for safe flashing" >&2; exit 2 ;;
    esac

	# Check the MCU unless fast flashing was explicitly requested.
	if [ "${MSP430_FAST_FLASH:-0}" != 1 ]; then

		# Probe before erasing Flash; avoid loading an image for another MCU.
		if ! probe=$("$FLASHER" -n "$FLASHER_DRIVER" regs 2>&1); then
			printf '%s\n' "$probe" >&2
			echo "error: unable to probe target chip" >&2
			exit 1
		fi
		actual=$(printf '%s\n' "$probe" |
			sed -n 's/.*Device ID: 0x\([[:xdigit:]]*\).*/\1/p' |
			head -n 1 | tr '[:upper:]' '[:lower:]')
		if [ "$actual" != "$expected" ]; then
			echo "error: expected MSP430G$expected but detected device ID 0x${actual:-unknown}" >&2
			echo "check the socketed MCU and MSP430_CHIP before flashing" >&2
			exit 1
		fi

	else
		echo "warning: skipping MCU identity check" >&2
	fi

	echo
	echo "flashing MSP430G$expected via MSPDebug/$FLASHER_DRIVER"
	"$FLASHER" -n "$FLASHER_DRIVER" "prog $ELF"
}

dispatch
