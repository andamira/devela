# nice!nano nRF52840 examples

Minimal bare-metal application with no RTOS/HAL, using the board's existing
Adafruit nRF52 UF2 bootloader **with Nordic S140 v6 (6.1.1)**.

## Before flashing

Double-tap RESET (or short RST to GND twice), connect USB, and inspect
`INFO_UF2.TXT` on the virtual `NICENANO` drive. Confirm that the bootloader
is really the nice!nano Adafruit bootloader **and uses S140 v6**. This is
**not** a profile for S140 v7 or bare-metal address-zero firmware.

The existing partitioning is preserved:
- `0x00000..0x26000`: MBR + S140 v6 (do not overwrite).
- `0x26000..0xEC000`: application Flash (used by this firmware).
- `0xEC000..0xF4000`: application settings/data (reserved).
- `0xF4000..0x100000`: bootloader (do not overwrite).
- RAM below `0x20004000` reserved for compatibility.

The bootloader's virtual disk is **not** a conventional 32 MiB Flash disk.
Copying its visible files is not a reliable full-chip backup. Re-enter UF2
mode by double-tapping RESET after flashing. USB serial (`/dev/ttyACM0`)
exposed in *bootloader mode* is not a serial interface provided by `blink`.

## Build and flash

Requires Rust `thumbv7em-none-eabihf`, Python 3, and `llvm-objcopy` (or
`arm-none-eabi-objcopy`). `inspect` and `dump` additionally use ARM binutils.
The shared runner forces `--target-dir` to the example's own `target/`,
independently of the workspace or `CARGO_TARGET_DIR`. From this directory:

```sh
./example.sh build             # ELF only
./example.sh inspect           # Sections and large symbols
./example.sh dump | less       # Detailed ELF inspection
./example.sh uf2               # ELF -> BIN -> UF2 (no hardware access)
# Check INFO_UF2.TXT, verify S140 version and find the mounted UF2 volume.
MOUNT=/media/$USER/NICENANO CONFIRM_LAYOUT=s140_v6 ./example.sh flash
```

The upload helper refuses a non-mounted directory, a missing/mismatched
`INFO_UF2.TXT` board ID, or flashing without explicit layout confirmation. Set `MOUNT` to the real location (it may be under
`/run/media/$USER/` or `/media/$USER/` depending on the desktop).

`blink` toggles the blue status LED on **P0.15, active-high**. The
USB/BLE application stacks are not enabled. Pro Micro nRF52840 variants
share the chip but may have a different LED, power wiring and bootloader;
do not flash this exact profile onto a clone without checking those facts.

See https://nicekeyboards.com/docs/nice-nano/ and
https://github.com/adafruit/Adafruit_nRF52_Bootloader .
