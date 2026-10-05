# Target registry for sentry probes.
# Sourced by ../inspect.sh.

add_target() {
    case " $TARGETS " in
        *" $1 "*) ;;
        *) TARGETS="${TARGETS}${TARGETS:+ }$1" ;;
    esac
}

select_target() {
    case "$1" in
        host)
            add_target host
            ;;

        thumbv7m|armv7m|sam3x8e)
            add_target thumbv7m
            ;;
        esp32c3|c3)
            add_target esp32c3
            ;;
        esp32c6|c6)
            add_target esp32c6
            ;;
        atmega328p|avr)
            add_target atmega328p
            ;;
        wasm32|web)
            add_target wasm32
            ;;
        msp430|msp430g2231)
            add_target msp430
            ;;
        esp32s3|s3)
            add_target esp32s3
            ;;

        desktop|desktop_all)
            select_target host
            ;;
        mcu|mcu_all)
            select_target atmega328p
            select_target msp430
            select_target thumbv7m
            select_target esp32c3
            select_target esp32c6
            select_target esp32s3
            ;;
        esp32|esp32_all)
            select_target esp32c3
            select_target esp32c6
            select_target esp32s3
            ;;
        arm)
            select_target thumbv7m
            ;;
        riscv)
            select_target esp32c3
            select_target esp32c6
            ;;

        cpu8|8_bit)
            select_target atmega328p
            ;;
        cpu16|16_bit)
            select_target msp430
            ;;
        cpu32|32_bit)
            select_target thumbv7m
            select_target esp32c3
            select_target esp32c6
            select_target esp32s3
            ;;
        ptr16)
            select_target atmega328p
            select_target msp430
            ;;
        ptr32)
            select_target thumbv7m
            select_target esp32c3
            select_target esp32c6
            select_target esp32s3
            select_target wasm32
            ;;

        all)
            select_target desktop
            select_target mcu
            select_target web
            ;;

        *)
            echo "error: unknown target or group: $1" >&2
            exit 2
            ;;
    esac
}

target_config() {
    TARGET_TOOLCHAIN=""
    TARGET_TRIPLE=""
    TARGET_RUSTFLAGS=""
    TARGET_BUILD_STD=0
    TARGET_SETUP=""
    TARGET_OUTPUT=""

    case "$1" in
        host)
            TARGET_OUTPUT="$(rustc -vV | sed -n 's/^host: //p')"
            ;;

        thumbv7m)
            TARGET_TRIPLE="thumbv7m-none-eabi"
            TARGET_OUTPUT="$TARGET_TRIPLE"
            ;;

        esp32c3)
            TARGET_TRIPLE="riscv32imc-unknown-none-elf"
            TARGET_OUTPUT="$TARGET_TRIPLE"
            ;;

        esp32c6)
            TARGET_TRIPLE="riscv32imac-unknown-none-elf"
            TARGET_OUTPUT="$TARGET_TRIPLE"
            ;;

        atmega328p)
            TARGET_TOOLCHAIN="nightly"
            TARGET_TRIPLE="avr-none"
            TARGET_RUSTFLAGS="-C target-cpu=atmega328p"
            TARGET_BUILD_STD=1
            TARGET_OUTPUT="avr-none-atmega328p"
            ;;

        wasm32)
            TARGET_TOOLCHAIN="nightly"
            TARGET_TRIPLE="wasm32-unknown-unknown"
            TARGET_RUSTFLAGS="--cfg nightly"
            TARGET_OUTPUT="$TARGET_TRIPLE"
            ;;

        msp430)
            TARGET_TOOLCHAIN="nightly"
            TARGET_TRIPLE="msp430-none-elf"
            TARGET_BUILD_STD=1
            TARGET_OUTPUT="$TARGET_TRIPLE"
            ;;

        esp32s3)
            TARGET_TOOLCHAIN="${TOOLCHAIN:-esp}"
            TARGET_TRIPLE="xtensa-esp32s3-none-elf"
            TARGET_BUILD_STD=1
            TARGET_SETUP="esp"
            TARGET_OUTPUT="$TARGET_TRIPLE"
            ;;

        *)
            echo "error: no configuration for target: $1" >&2
            exit 2
            ;;
    esac
}

target_usage() {
    cat <<'EOF'
targets:
  host
  thumbv7m, armv7m, sam3x8e
  esp32c3, c3
  esp32c6, c6
  atmega328p, avr
  wasm32, web
  msp430, msp430g2231
  esp32s3, s3

groups:
  desktop, desktop_all
  mcu, mcu_all
  esp32, esp32_all
  arm
  riscv
  cpu8, 8_bit
  cpu16, 16_bit
  cpu32, 32_bit
  ptr16
  ptr32
  all
EOF
}
