# devela_bridge

Experimental C ABI bridge for selected [`devela`](https://github.com/andamira/devela) functionality.

This crate builds Rust code as system libraries that can be used from
C, Odin, and other languages that can call a C ABI.

It currently provides a small smoke-test ABI.


## Build

```sh
./build.sh
````

The script builds the Rust library and copies generated artifacts into:

```text
libs/
```

Typical outputs:

```text
libdevela_bridge.so
libdevela_bridge.a
```

On macOS or Windows the dynamic/static library suffixes differ.


## Run examples

```sh
./examples/run_all.sh
```

Or run individual examples from `examples/`.


## Features

The bridge crate mirrors selected `devela` features.

Example:

```toml
[features]
linux = ["devela/linux"]
term = ["devela/term"]
```

The ABI surface is intentionally explicit. Enabling a Rust feature does not
automatically imply every possible foreign symbol.


## Design

`devela_bridge` keeps its ordinary Rust library portable and `no_std`-capable.

Normal workspace builds produce an `rlib`. Native foreign-language artifacts such
as `staticlib` and `cdylib` libraries are built explicitly by `build.sh`, together
with the generated bindings they expose.

Unsafe capabilities are enabled only by bridged functionality that actually
requires them, rather than by the bridge crate as a whole.


## Status

Experimental. The ABI is not stable.
