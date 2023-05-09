# devela

[![Crate](https://img.shields.io/crates/v/devela.svg)](https://crates.io/crates/devela)
[![API](https://docs.rs/devela/badge.svg)](https://docs.rs/devela/)

rust development helper & extension utilities

## Features

- `#[compile]` attribute macro for conditional compilation.
- `subslice_left`, `subslice_mid`, `subslice_right` functions.
- `pmin`, `pmax` & `pclamp` functions over `PartialOrd` values.
- `format_buf` function and macro for `no_std` formatting.
- `Apply`, `Also` traits for free function chaining.
- `crate_root` & `crate_root_string` functions.
- `counter_string` strings for measuring.
- `rfs!` rust format skip macro.
- `iif!` inline if macro.
- `bx` `Box` constructor.

- reexported external crates:
  - `az` casting traits.
  - `paste` macro.

See [the documentation](https://docs.rs/devela/) for more information.

## Status

This is currently in an experimental stage of development.

## Contributing

Contributions are welcomed to help refine and improve this library over time.
If you notice a bug, have an idea for a new feature, or simply want to suggest
improvements to the existing codebase, please get in touch.
