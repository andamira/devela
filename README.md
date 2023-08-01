# devela

[![Crate](https://img.shields.io/crates/v/devela.svg)](https://crates.io/crates/devela)
[![API](https://docs.rs/devela/badge.svg)](https://docs.rs/devela/)

A varied collection of mostly low-level helpers & extensions for [Rust].

[Rust]: https://www.rust-lang.org/

## Features

- `NonSpecific*` wrappers over the `NonZero` primitives. E.g.: [`NonSpecificU8`], plus convenient aliases `NonMax*` and `NonMin*`.
- conversion fns: [`vec_into_vec`], [`slice_into_vec`], [`slice_into_array`], [`try_vec_into_vec`], [`try_slice_into_vec`].
- ops fns: [`pmin`], [`pmax`] & [`pclamp`] for `PartialOrd` values.
- slice fns: [`subslice_left`], [`subslice_middle`], [`subslice_right`].
- strings fns: [`counter_string`].
- `no_std` formatting fn: [`format_buf`] and macro: [`format_buf!`].
- path related fns: [`crate_root`], [`crate_root_string`] and macro: [`manifest_dir!`].
- misc. sugar fn: [`bx`] compact `Box` constructor.
- misc. sugar macros:
  - [`cdbg!`] compact `dbg!`.
  - [`iif!`] compact inline `if` and `if let`.
  - [`rfs!`] compact rust format skip.
  - [`S!`] brief `String` constructor.
- free function chaining traits: [`Apply`], [`Also`].
- extension traits:  [`OptionExt`], [`ResultExt`].
- alternative Debug trait [`AltDebug`].

- attribute macros:
  - [`#[compile]`][compile] for boolean conditional compilation.

- reexported external crates:
  - [`az`] casting traits.
  - [`paste!`] macro.

- optional external trait implementations:
  - `bytemuck` traits for `NonSpecific*` types.

See [the documentation](https://docs.rs/devela/) for more information.

[`az`]: https://docs.rs/devela/latest/devela/az/index.html

[`iif!`]: https://docs.rs/devela/latest/devela/macro.iif.html
[`cdbg!`]: https://docs.rs/devela/latest/devela/macro.cdbg.html
[`format_buf!`]: https://docs.rs/devela/latest/devela/macro.format_buf.html
[`iformat!`]: https://docs.rs/devela/latest/devela/macro.iformat.html
[`manifest_dir!`]: https://docs.rs/devela/latest/devela/macro.manifest_dir.html
[`paste!`]: https://docs.rs/devela/latest/devela/macro.paste.html
[`rfs!`]: https://docs.rs/devela/latest/devela/macro.rfs.html
[`S!`]: https://docs.rs/devela/latest/devela/macro.S.html

[`bx`]: https://docs.rs/devela/latest/devela/fn.bx.html
[`crate_root`]: https://docs.rs/devela/latest/devela/fn.crate_root.html
[`crate_root_string`]: https://docs.rs/devela/latest/devela/fn.crate_root_string.html
[`counter_string`]: https://docs.rs/devela/latest/devela/fn.counter_string.html
[`format_buf`]: https://docs.rs/devela/latest/devela/fn.format_buf.html
[`indent`]: https://docs.rs/devela/latest/devela/fn.indent.html
[`pclamp`]: https://docs.rs/devela/latest/devela/fn.pclamp.html
[`pmax`]: https://docs.rs/devela/latest/devela/fn.pmax.html
[`pmin`]: https://docs.rs/devela/latest/devela/fn.pmin.html
[`slice_into_vec`]: https://docs.rs/devela/latest/devela/fn.slice_into_vec.html
[`slice_into_array`]: https://docs.rs/devela/latest/devela/fn.slice_into_array.html
[`subslice_left`]: https://docs.rs/devela/latest/devela/fn.subslice_left.html
[`subslice_middle`]: https://docs.rs/devela/latest/devela/fn.subslice_middle.html
[`subslice_right`]: https://docs.rs/devela/latest/devela/fn.subslice_right.html
[`try_slice_into_vec`]: https://docs.rs/devela/latest/devela/fn.try_slice_into_vec.html
[`try_vec_into_vec`]: https://docs.rs/devela/latest/devela/fn.try_vec_into_vec.html
[`vec_into_vec`]: https://docs.rs/devela/latest/devela/fn.vec_into_vec.html

[`Also`]: https://docs.rs/devela/latest/devela/trait.Also.html
[`AltDebug`]: https://docs.rs/devela/latest/devela/trait.AltDebug.html
[`Apply`]: https://docs.rs/devela/latest/devela/trait.Apply.html
[`OptionExt`]: https://docs.rs/devela/latest/devela/trait.OptionExt.html
[`ResultExt`]: https://docs.rs/devela/latest/devela/trait.ResultExt.html

[`NonSpecificU8`]: https://docs.rs/devela/latest/devela/struct.NonSpecificU8.html

[compile]: https://docs.rs/devela/latest/devela/attr.compile.html

## Status

This is currently in an experimental stage of development.

## Contributing

Contributions are welcomed to help refine and improve this library over time.
If you notice a bug, have an idea for a new feature, or simply want to suggest
improvements to the existing codebase, please get in touch.
