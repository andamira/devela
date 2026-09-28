# ZX Spectrum 48K examples

Small bare-metal `no_std` programs for the Sinclair ZX Spectrum 48K,
using devela's Z80 processor and Spectrum hardware support.

The examples share one Rust-Z80 target configuration, build/run script,
linker script, TAP packager, and `devela_micros` dependency.
Each program lives under `src/bin/`.


## Programs

- `screen` — draws a Spectrum display test pattern and changes the ULA border color.


## Requirements

These examples use the experimental
[`llvm-z80`](https://github.com/llvm-z80/llvm-z80) /
[`rust-z80`](https://github.com/llvm-z80/rust-z80) toolchain rather than
an upstream Rust target.

On Debian, Ubuntu, or Linux Mint:

```sh
sudo apt install \
    build-essential git curl python3 \
    cmake ninja-build pkg-config libssl-dev \
    fuse-emulator-gtk
```

A normal `rustup` installation is also required.

Build Rust-Z80 outside the devela repository. For example:

```sh
mkdir -p ~/src
cd ~/src

git clone https://github.com/llvm-z80/rust-z80.git
cd rust-z80

git submodule update --init --depth 1 src/llvm-project

./x install
rustup toolchain link rust-z80 "$PWD/install"
```

Verify the installation:

```sh
rustc +rust-z80 -Vv
rustc +rust-z80 --print target-list | grep z80
fuse --version
```

The toolchain build compiles its own LLVM-Z80 backend and can require
substantial build time and disk space.


## Build and run

```sh
./build.sh build screen
./build.sh run screen
```

`build` and `screen` are the defaults.

The script:

- builds a release ELF for `z80-unknown-none-elf`;
- verifies that its entry point is `0x8000`;
- converts the ELF into a flat binary;
- packages a small auto-starting BASIC loader and the code into a `.tap`;
- opens the tape in Fuse.

Normally Fuse starts the tape loader automatically.
If automatic loading is disabled, enter:

```text
LOAD ""
```

The BASIC loader reserves memory below `0x8000`,
loads the CODE block at 32768, and
starts it with `RANDOMIZE USR 32768`.

The `screen` program should display a striped eight-color test pattern.


## Artifacts

Generated files are placed under:

```text
target/z80-unknown-none-elf/release/
```

For `screen` the main artifacts are:

```text
screen.elf
screen.bin
screen.tap
```


## Inspect and dump

For the ELF layout and generated artifact sizes:

```sh
./build.sh inspect screen
```

For the complete symbol table and Z80 disassembly:

```sh
./build.sh dump screen
```

When stdout is interactive and `$EDITOR` is set, the dump is saved beside
the ELF and opened in the editor. Otherwise it is written to stdout:

```sh
./build.sh dump screen | less
./build.sh dump screen > /tmp/screen.dump
```

Host `file` and `readelf` may report the experimental Z80 ELF machine value
as an unknown architecture. The dump command uses the LLVM-Z80 `llvm-objdump`
with an explicit Z80 target.


## Toolchain notes

Rust-Z80 and LLVM-Z80 are experimental.

The current linker may warn that Rust `.rlib` metadata members such as
`lib.rmeta` and `lib.rmeta-link` are neither ELF relocatable objects nor LLVM
bitcode. The generated executable is still linked and runs correctly.

Fuse can use the open-source OpenSE ROM when an original `48.rom` is not
installed. The current example accesses display memory and the ULA directly;
the ROM is used only for the normal BASIC/tape-loading path.
