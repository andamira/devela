# devela_bridge

Foreign-language bindings and ABI bridges for devela.

This crate exposes selected devela functionality through explicit,
stable-layout foreign interfaces, currently targeting C-compatible ABIs
and generated bindings for C and Odin.

It currently provides a small smoke-test ABI.


## Build

```sh
./build.sh
```

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


## Documentation

- [API documentation][api]: browse the library by module.
- [WIP documentation][wip]: current development API.

[api]: https://docs.rs/devela_bridge/latest/devela_bridge/
[wip]: https://andamira.github.io/devela_bridge/wip/devela_bridge/
