# devela

[![Crate](https://img.shields.io/crates/v/devela.svg)](https://crates.io/crates/devela)
[![API](https://docs.rs/devela/badge.svg)](https://docs.rs/devela/)

rust development helper & extension utilities

## Features


- conversion fns: `vec_into_vec`, `slice_into_vec`, `slice_into_array`, `try_vec_into_vec`, `try_slice_into_vec`.
- ops fns: `pmin`, `pmax` & `pclamp` useful for `PartialOrd` values.
- slice fns: `subslice_left`, `subslice_mid`, `subslice_right`.
- strings fns: `counter_string`.
- `no_std` formatting fn: `format_buf` and macro: `format_buf!`.
- path related fns: `crate_root`, `crate_root_string` and macro: `manifest_dir!.
- misc. sugar fn: `bx` compact `Box` constructor.
- misc. sugar macros:
  - `cdbg!` compact `dbg!`.
  - `iif!` compact inline if.
  - `rfs!` compact rust format skip.
- free function chaning traits: `Apply`, `Also`.

- attribute macros:
  - `#[compile]` for boolean conditional compilation.

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
