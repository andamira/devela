# ZX Spectrum 48K examples

Rust `no_std` programs for the ZX Spectrum 48K using `devela_micros`
and [Rust-Z80](https://github.com/llvm-z80/rust-z80).


## Toolchain

On Debian-based Linux, install `rustup` and the build dependencies:

```sh
sudo apt install build-essential git curl python3 \
  cmake ninja-build pkg-config libssl-dev fuse-emulator-gtk
```

The helper clones or updates [Rust-Z80](https://github.com/llvm-z80/rust-z80),
checks out its pinned LLVM-Z80 submodule, compiles incrementally and links the
compiler with `rustup`:

```sh
../../../tools/build-z80-rust.sh "$HOME/tmp/z80"
rustc +rust-z80 -vV
```

Existing source and build artifacts are reused. Use `Z80_UPDATE=0` to rebuild
the current checkout without fetching upstream changes, or pass a second
argument to select a different registered name (e.g. `rust-z80-next`).
The corresponding manual commands are at the top of the helper script.

This is an independent Rust compiler fork; its version does **not**
automatically advance when upstream stable Rust does. Inspect `rustc -vV`
and build an example before treating it as compatible with devela's MSRV.
A custom toolchain can be selected as follows:

```sh
TOOLCHAIN=rust-z80-next ./example.sh build
```


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
