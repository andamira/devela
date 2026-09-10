<!-- devela/examples/sys/env/minimal/README.md -->

# Minimal environments

Small `no_std` executables comparing different runtime and Rust target environments.

## Running

```sh
./run.sh clib
./run.sh linux
./run.sh none
./run.sh none-compact
```

The runner builds and executes each mode, then reports the resulting binary size.

Nightly and `rust-src` are required:

```sh
rustup +nightly component add rust-src
```

The `none` modes also require:

```sh
rustup +nightly target add x86_64-unknown-none
```

## Modes

| Mode           | Rust target                 | Runtime                  | `target_os` |
| -------------- | --------------------------- | ------------------------ | ----------- |
| `clib`         | host                        | C runtime + libc         | host OS     |
| `linux`        | `x86_64-unknown-linux-none` | devela + Linux syscalls  | `linux`     |
| `none`         | `x86_64-unknown-none`       | devela + Linux syscalls  | `none`      |
| `none-compact` | `x86_64-unknown-none`       | same, compact ELF layout | `none`      |

`clib.rs` uses the hosted C runtime and libc.

`linux.rs` provides its own entry point through devela and uses Linux syscalls
directly. The same source is used by the `linux` and `none` modes, showing the
difference between targeting Linux explicitly and using the Linux syscall ABI
from a freestanding Rust target.

`none-compact` additionally passes `-N` to the linker. It is kept as an explicit
size experiment because it changes the normal ELF segment layout and protections.

Exact binary sizes depend on the toolchain.
