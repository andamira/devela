# devela

[![Crate](https://img.shields.io/crates/v/devela.svg)](https://crates.io/crates/devela)
[![API](https://docs.rs/devela/badge.svg)](https://docs.rs/devela/)
[![MSRV: 1.71.1](https://flat.badgen.net/badge/MSRV/1.71.1/purple)](https://releases.rs/docs/1.71.1/)

Development extensions for the Rust Standard Library.

See [the documentation](https://docs.rs/devela/) for more information.

## Status

This is currently in an experimental stage of development.

## License
This project is dual licensed under either [MIT](LICENSE-MIT)
or [Apache-2.0](LICENSE-APACHE) at your option.

### Derived Works

This project includes the following derived works:

- A [modified][0] version of [itoa] by David Tolnay, licensed as MIT OR Apache-2.0,
  in [`IntBuf`] and [`IntBufAble`].
- Adaptation of [opt_reduce] by Waffle Lapkin, licensed as MIT,
  as part of the [`OptionExt`] trait.
- Adaptation of [fmtor] by Tyler Ruckinger, licensed as MIT OR Apache-2.0,
  as part of the [`OptionExt`] trait.
- Adaptation of [option-ext] by Simon Ochsenreither, licensed as MPL-2.0,
  as part of the [`OptionExt`] trait.
- Adaptation of [result-ext] by Simon Ochsenreither, licensed as MPL-2.0,
  as part of the [`ResultExt`] trait.
- Adaptation of [apply] by George Burton, licensed as Unlicense,
  as part of the [`Also`] and [`Apply`] traits.

[0]: https://github.com/andamira/devela/blob/main/src/fmt/int_buf/MODIFICATIONS.md
[itoa]: https://crates.io/crates/itoa/1.0.9
[`IntBuf`]: https://docs.rs/devela/latest/devela/fmt/struct.IntBuf.html
[`IntBufAble`]: https://docs.rs/devela/latest/devela/fmt/trait.IntBufAble.html
[opt_reduce]: https://crates.io/crates/opt_reduce/1.0.0
[fmtor]: https://crates.io/crates/fmtor/0.1.2
[option-ext]: https://crates.io/crates/option-ext/0.2.0
[result-ext]: https://crates.io/crates/result-ext/0.2.0
[`OptionExt`]: https://docs.rs/devela/latest/devela/option/trait.OptionExt.html
[`ResultExt`]: https://docs.rs/devela/latest/devela/result/trait.ResultExt.html
[apply]: https://crates.io/crates/apply/0.3.0
[`Also`]: https://docs.rs/devela/latest/devela/ops/trait.Also.html
[`Apply`]: https://docs.rs/devela/latest/devela/ops/trait.Apply.html

## Contributing

Contributions are welcomed to help refine and improve this library over time.
If you notice a bug, have an idea for a new feature, or simply want to suggest
improvements to the existing codebase, please get in touch.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by shall be dual licensed as above,
without any additional terms or conditions.
