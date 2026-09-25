
# Arduino Due examples

Small bare-metal `no_std` programs for the Arduino Due,
using devela's SAM3X8E and direct MMIO support.

The examples share one SAM3X8E target configuration, build/flash runner,
and `devela_micros` dependency. Each program lives under `src/bin/`.


## Programs

- `blink` — repeatedly drives the built-in D13 / `L` LED through PIOB / PB27
  using an uncalibrated software delay.


## Requirements

On Debian, Ubuntu, or Linux Mint:

```sh
sudo apt install binutils-arm-none-eabi bossa-cli python3
```

The examples use Rust's `thumbv7m-none-eabi` target
for the SAM3X8E's Arm Cortex-M3 (Armv7-M) core.

The runner has been tested with BOSSA 1.9.1.
BOSSA command-line behavior varies between versions;
the numeric `--usb-port=0` / `--usb-port=1` forms used here are intentional.


## Build and flash

Use the **Programming Port**, the USB connector nearest the DC power jack.
```sh
./flash.sh build blink
./flash.sh run blink
```

Replace `blink` with any binary listed above. `run` and `blink` are the defaults,
so `./flash.sh` builds, flashes, and resets `blink`.

By default the runner uses:

```text
serial port: /dev/ttyACM0
```

Override it when needed:

```sh
PORT=/dev/ttyACM1 ./flash.sh run blink
```

The runner performs three board-specific steps:

1. asks BOSSA `--arduino-erase` to trigger the Programming Port's 1200-baud
   ERASE + RESET sequence, then waits for ROM SAM-BA;
2. writes, verifies, and selects Flash boot with `bossac ... -e -w -v -b`;
3. resets the SAM3X8E with a reset-only DTR pulse at 115200 baud.

It deliberately does **not** use `bossac -R`: with the tested BOSSA 1.9.1
setup, `-R` did not reliably leave the board running the newly flashed image.
The Python snippet uses only the standard library and exists solely to produce
the final reset-only DTR pulse.


## USB ports and recovery

The Due has two different USB paths:

- **Programming Port**, nearest the DC jack: normally appears as USB
  `2341:003d`, goes through the ATmega16U2, and uses `--usb-port=0`.
  This is the normal runner path.
- **Native USB**, nearest RESET: connects directly to the SAM3X8E.
  ROM SAM-BA appears as USB `03eb:6124` and uses `--usb-port=1`.
  The current bare-metal examples do not initialize native USB, so this port
  normally disappears once the application boots.

To force ROM SAM-BA manually, power the board, hold **ERASE** for about
two seconds, release it, then briefly press **RESET**. This erases Flash and
clears Flash boot selection. `bossac ... -i` should then report
`Boot Flash: false`; flashing with `-b` selects Flash boot again.

For example, through the Programming Port:

```sh
bossac --port=ttyACM0 --usb-port=0 -i
```

If a different BOSSA version rejects these options or behaves differently,
check `bossac --help` before changing the board-side code.


## Size

The runner reports the current ELF size after each build. Exact sizes vary
with compiler, optimization, and example changes.

Rust `core`, devela, and `devela_micros` provide the firmware-side startup,
linking, and MMIO foundations. GNU Arm binutils provide `objcopy` / `size`,
BOSSA performs SAM-BA flashing, and Python's standard library provides the
final Programming-Port DTR reset.
