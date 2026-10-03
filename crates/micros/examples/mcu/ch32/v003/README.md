# CH32V003 example

Minimal bare-metal `no_std` compilation probes for the WCH CH32V003,
a 32-bit RV32EC microcontroller with 16 KiB of flash and 2 KiB of SRAM.

The examples intentionally use upstream Rust
rather than a WCH-specific Rust toolchain.

Rust's `riscv32e-unknown-none-elf` target provides RV32E/ILP32E
code generation, while `-C target-feature=+c` enables
the compressed instruction extension required for RV32EC.

Because this Tier-3 target has no prebuilt `core`,
`core` is compiled locally with `-Z build-std`.

## Requirements

Install the Rust source and LLVM tools:

```sh
rustup +nightly component add rust-src llvm-tools-preview
```

Install the WCH-Link command-line tool:

```sh
sudo apt install libudev-dev libusb-1.0-0-dev
cargo install --git https://github.com/ch32-rs/wlink
```

No external RISC-V compiler or linker is required.

## Programs

- `minimal` — minimal startup and infinite loop.
- `abi` — exercises the RV32E calling convention and compressed instructions.

## Build and inspect

The example defaults to the `minimal` binary:

```sh
./flash.sh build
./flash.sh inspect
./flash.sh dump
./flash.sh flash
```

The dump includes the ELF RISC-V attributes and a no-alias disassembly
so RV32E+C code generation can be inspected directly.

## Flashing

Hardware flashing requires a WCH-LinkE or another programmer
supporting the CH32V003 one-wire debug interface.

```sh
./flash.sh flash minimal
```
