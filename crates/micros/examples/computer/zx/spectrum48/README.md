# ZX Spectrum 48K examples

Rust `no_std` programs for the ZX Spectrum 48K using `devela_micros` and
[Rust-Z80](https://github.com/llvm-z80/rust-z80).


## Toolchain

On Debian-based Linux, install `rustup` and the build dependencies:

```sh
sudo apt install build-essential git curl python3 \
  cmake ninja-build pkg-config libssl-dev fuse-emulator-gtk

PLATFORMS_DIR="${PLATFORMS_DIR:-$HOME/tmp}"
mkdir -p "$PLATFORMS_DIR/z80"
cd "$PLATFORMS_DIR/z80"

git clone https://github.com/llvm-z80/rust-z80.git
cd rust-z80
git submodule update --init --depth 1 src/llvm-project
./x install
rustup toolchain link rust-z80 "$PWD/install"

rustc +rust-z80 -V
```

The path is only an example: `rustup` registers the toolchain independently
of its location. Building Rust-Z80 also builds the LLVM-Z80 backend.


# Programs

- [`screen`][s1] ([run][r1]) — display test pattern.
- [`paint`][s2] ([run][r2]) — interactive paint application.

[s1]: https://github.com/andamira/devela/blob/main/crates/micros/examples/computer/zx/spectrum48/src/bin/screen.rs
[r1]: https://andamira.github.io/play/spectrum48/?demo=screen.tap
[s2]: https://github.com/andamira/devela/blob/main/crates/micros/examples/computer/zx/spectrum48/src/bin/paint.rs
[r2]: https://andamira.github.io/play/spectrum48/?demo=paint.tap


## Build and run

```sh
./example.sh build
./example.sh run

./example.sh run paint
./example.sh inspect paint
./example.sh dump paint | less
```

The runner builds the Z80 ELF, converts it to a binary, and packages a `.tap`
with an auto-starting BASIC loader. The generated `.elf`, `.bin`, and `.tap`
files are under `target/z80-unknown-none-elf/release/`.

If Fuse does not autoload the tape, enter `LOAD ""` in Spectrum BASIC.
