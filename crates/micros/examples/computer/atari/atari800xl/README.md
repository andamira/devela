# Atari 800XL examples

Rust `no_std` programs for the Atari 800XL using `devela_micros`, Rust-MOS,
and the LLVM-MOS SDK. The SDK provides Atari DOS startup and links the Rust
static library into a `.xex` executable.


## Toolchain

On x86_64 Linux, install `rustup`, the Atari800 emulator, and the
[Rust-MOS](https://github.com/kassane/zig-mos-bootstrap/releases/tag/0.1.0)
(1.98.0-dev) and [LLVM-MOS SDK](https://github.com/llvm-mos/llvm-mos-sdk/releases/tag/v23.0.1)
(v23.0.1) archives:

```sh
sudo apt install curl xz-utils atari800

PLATFORMS_DIR="${PLATFORMS_DIR:-$HOME/tmp/}"
ATARI_DIR="$PLATFORMS_DIR/atari"
mkdir -p "$ATARI_DIR/rust" "$ATARI_DIR/sdk"

curl -fL https://github.com/kassane/zig-mos-bootstrap/releases/download/0.1.0/rust-mos-x86_64-linux-glibc-ubuntu.tar.xz \
    -o "$ATARI_DIR/rust-mos.tar.xz"
curl -fL https://github.com/llvm-mos/llvm-mos-sdk/releases/download/v23.0.1/llvm-mos-linux.tar.xz \
    -o "$ATARI_DIR/llvm-mos.tar.xz"

tar -xf "$ATARI_DIR/rust-mos.tar.xz" -C "$ATARI_DIR/rust"
tar -xf "$ATARI_DIR/llvm-mos.tar.xz" -C "$ATARI_DIR/sdk"

rustup toolchain link rust-mos "$ATARI_DIR/rust/rust-mos-x86_64-linux-glibc"
export SDKBIN="$ATARI_DIR/sdk/llvm-mos/bin"

rustc +rust-mos -V
"$SDKBIN/mos-atari8-dos-clang" --version
```

`PLATFORMS_DIR` is an installation example, not a required path. For subsequent
shell sessions, make the LLVM-MOS SDK available through `PATH` or `SDKBIN`.
The runner selects Rust-MOS through `rustup`.


## Build and run

```sh
./example.sh build
./example.sh run
./example.sh inspect
./example.sh dump | less
```

The program writes text using the Atari OS screen editor and changes the
background color. Atari800 needs an XL/XE OS ROM configured.

Artifacts are written to `target/mos-unknown-none/release/`:
`atari800xl.xex` is the runnable image; `atari800xl.xex.elf` contains
symbols for inspection.


## Notes

The experimental Rust-MOS compiler may lag `devela`'s declared MSRV.
The build uses `--ignore-rust-version`; source compatibility is still required.
The LLVM-MOS SDK's Atari DOS driver supplies the startup and linker configuration.
