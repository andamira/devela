# devela_ffi

Experimental C ABI bridge for selected [`devela`](https://github.com/andamira/devela) functionality.

This crate builds Rust code as system libraries that can be used from C, Odin,
and other languages that can call a C ABI.

It currently provides a small smoke-test ABI for checking:

- scalar arguments and return values
- pointer + length arguments
- out pointers
- returned C strings
- link-time imports
- runtime symbol loading

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
libdevela_ffi.so
libdevela_ffi.a
```

On macOS or Windows the dynamic/static library suffixes differ.

## Run examples

```sh
./run_all.sh
```

Or run individual examples from `examples/`.

## Features

The FFI crate mirrors selected `devela` features.

Example:

```toml
[features]
linux = ["devela/linux"]
term = ["devela/term"]
```

The ABI surface is intentionally explicit. Enabling a Rust feature does not
automatically imply every possible foreign symbol.

## Status

Experimental. The ABI is not stable.
