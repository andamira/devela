# Atari 800XL examples

Rust `no_std` programs for the Atari 800XL using `devela_micros`, Rust-MOS,
and the LLVM-MOS SDK. The SDK provides Atari DOS startup and links the Rust
static library into a `.xex` executable.


## Toolchain

The compiler is provided by [Rust-MOS-Sync], a patch-based project
that tracks stable Rust. The [LLVM-MOS SDK] supplies Atari startup,
linker drivers, and `.xex` packaging. They are independently versioned.

[Rust-MOS-Sync]: https://github.com/sbechet/rust-mos-sync
[LLVM-MOS SDK]: https://github.com/llvm-mos/llvm-mos-sdk/releases

On x86_64 Linux, install `rustup`, Atari800, and the prerequisite tools:

```sh
sudo apt install build-essential git curl python3 cmake ninja-build \
  pkg-config libssl-dev xz-utils atari800
```

Prefer the upstream **prebuilt toolchain**, if a release matching devela's MSRV
is available. Inspect the installer before running it:

```sh
curl -fsSL \
  https://github.com/sbechet/rust-mos-sync/releases/latest/download/install-stable.sh \
  -o install-mos.sh
less install-mos.sh
bash install-mos.sh stable rust-mos-next
rustc +rust-mos-next -vV
```

The installer verifies and installs the matching SDK too; it prints the SDK
`bin/` path. Set `SDKBIN` to that path (or put it on `PATH`) when running
these examples.

To **build from source** instead, with incremental reuse of source and LLVM
caches, use the repository helper:

```sh
../../../tools/build-mos-rust.sh 1.100.0 "$HOME/tmp/atari"
```

It registers a versioned name (`rust-mos-1.100.0` here), prints the SDK path,
and does not replace an existing `rust-mos` installation. Example:

```sh
TOOLCHAIN=rust-mos-1.100.0 SDKBIN=/path/to/sdk/bin ./example.sh build
```

After verifying the versioned compiler, a permanent `rust-mos` alias can be
linked with `rustup toolchain link` after unregistering any prior `rust-mos`.
The relevant manual source-build commands are at the top of the helper.


# Programs

- [`atari800xl`][s1] ([run][r1]) — writes "Hello from devela".

[s1]: https://github.com/andamira/devela/blob/main/crates/micros/examples/computer/atari/atari800xl/src/lib.rs
[r1]: https://andamira.github.io/play/atari800xl/?demo=atari800xl.xex

The online demo uses freely redistributable Altirra replacement firmware.
See [LICENSE-Altirra.txt](https://github.com/andamira/andamira.github.io/blob/main/play/atari800xl/LICENSE-Altirra.txt).


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

The selected Rust-MOS toolchain must include `core` and `compiler_builtins`
for `mos-unknown-none` (as Rust-MOS-Sync toolchains do). The example no longer
uses `-Zbuild-std` or bypasses Cargo's `rust-version` check. A compiler
reporting Rust 1.100 is not sufficient by itself: verify the example links and
runs successfully. The LLVM-MOS SDK's Atari DOS driver still supplies startup
and final linking.
