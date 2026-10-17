# MSP-EXP430G2 LaunchPad examples

Small bare-metal `no_std` programs for the original socketed MSP430 LaunchPad,
using upstream Rust's `msp430-none-elf` target and `msp430-rt`.

The board has a red user LED on P1.0 and a green user LED on P1.6.
The removable MCU determines the memory map and available peripherals.

## Supported MCUs

| `MSP430_CHIP` | Flash | RAM | Status |
| --- | ---: | ---: | --- |
| `g2231` | 2 KiB | 128 B | Build/inspect probe (not tested on silicon here) |
| `g2452` | 8 KiB | 256 B | Supplied DIP-20; build configuration |
| `g2553` | 16 KiB | 512 B | Built and blinked on physical LaunchPad |

Each variant has a separate linker memory layout under `memory/`.
The board's 32.768 kHz crystal is optional and not used by these examples.

## Requirements

Install nightly Rust with its sources:

```sh
rustup +nightly component add rust-src
```

Install [TI's MSP430 GCC Open Source toolchain](https://www.ti.com/tool/MSP430-GCC-OPENSOURCE#downloads)
(the extracted 64-bit Linux archive is sufficient for these Rust examples)
and make `msp430-elf-gcc` available on `PATH`.

```sh
msp430-elf-gcc --version
sudo apt install mspdebug
```

The Rust target is upstream, but uses `msp430-elf-gcc` for the final build;
there is no need to install a prebuilt `msp430-none-elf` `core` artifact.

## Programs

- `minimal` — minimal `msp430-rt` entry point and infinite loop.
- `blink` — toggles the LaunchPad's red LED on P1.0 using direct MMIO.

## Build, inspect and flash

The default chip is the installed `g2553` and the default program is `blink`:

```sh
./example.sh build
./example.sh inspect
./example.sh dump
./example.sh flash
```

To select the other supported chips, set `MSP430_CHIP`:

```sh
MSP430_CHIP=g2231 ./example.sh build minimal
MSP430_CHIP=g2452 ./example.sh inspect blink
MSP430_CHIP=g2553 ./example.sh flash blink
```

Build artifacts are kept separately under `target/g2231`, `target/g2452`
and `target/g2553` to prevent image mixups. Before flashing, the script
connects with MSPDebug and requires the detected device ID to match the
selected chip. **Only flash after checking which MCU is socketed.**

The LaunchPad uses MSPDebug's `rf2500` driver through its USB debugger.
After programming, press the RESET button to start the application if
it remains halted. The red LED jumper at J5 must be installed.

No MCU-specific public devela_micros type is required for these examples.
