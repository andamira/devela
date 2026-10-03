# ATtiny4 example

Minimal bare-metal `no_std` compilation probe for the ATtiny4.

The example verifies Rust/LLVM support for the reduced AVR (`avrtiny`)
architecture and the ATtiny4 memory layout. Hardware is not required
to build or inspect the resulting ELF.

## Requirements

On Debian, Ubuntu, or Linux Mint:

```sh
sudo apt install gcc-avr avr-libc avrdude
```

The example uses Rust's `avr-none` target with `attiny4` selected as the target CPU.

Nightly and `rust-src` are used because `core` is built locally.

## Build and inspect

The example defaults to the `minimal` binary:

```sh
./example.sh build
./example.sh inspect
./example.sh dump
./example.sh flash
```

## Flashing

Flashing requires a TPI-capable programmer. The shared script defaults to USBasp:

```sh
./example.sh flash minimal
```

The programmer firmware must support TPI for the ATtiny4/5/9/10 family.
