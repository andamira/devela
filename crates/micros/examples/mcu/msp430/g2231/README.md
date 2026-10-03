# MSP430G2231 example

Minimal bare-metal `no_std` compilation probe for the TI MSP430G2231.

The MSP430 Rust target is upstream, but LLVM
currently cannot emit MSP430 object files directly.
Rust therefore requires `msp430-elf-gcc` during the build.

- https://www.ti.com/tool/MSP430-GCC-OPENSOURCE#downloads

## Requirements

Install nightly Rust sources:

```sh
rustup +nightly component add rust-src
```

Download TI's MSP430 GCC Open Source toolchain:

<https://www.ti.com/tool/MSP430-GCC-OPENSOURCE#downloads>

On 64-bit Linux, use the Linux installer including the device support files.
After installation, add its `bin` directory to `PATH`.

Verify:

```sh
msp430-elf-gcc --version
msp430-elf-objdump --version
```

For programming through an MSP430 LaunchPad:

```sh
sudo apt install mspdebug
```

## Build and inspect

The example defaults to the `minimal` binary:

```sh
./flash.sh build
./flash.sh inspect
./flash.sh dump
./flash.sh flash
```

## Flashing

The shared script uses MSPDebug and defaults to its `rf2500` driver:

```sh
./flash.sh flash minimal
```

An MSP-EXP430G2 can provide the Spy-Bi-Wire programming/debug connection
for a compatible target MCU.
