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
- Adaptation of [rawbytes] by Frank Denis, licensed as MIT,
  into the [`as_bytes`] and [`as_bytes_mut`] functions.
- Integration of [const_for] by Joachim Enggård Nebel, licensed as MIT,
  as the [`const_for`] macro.
- Integration of [size_of_trait] by Joshua Nelson, licensed as BSD-3,
 as the [`mem_size_of_expr`] macro.

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
[rawbytes]: https://crates.io/crates/rawbytes/1.0.0
[`as_bytes`]: https://docs.rs/devela/latest/devela/mem/fn.as_bytes.html
[`as_bytes_mut`]: https://docs.rs/devela/latest/devela/mem/fn.as_bytes_mut.html
[const_for]: https://crates.io/crates/const_for
[`const_for`]: https://docs.rs/devela/latest/devela/codegen/macro.const_for.html
[size_of_trait]: https://crates.io/crates/size-of-trait
[`size_of_expr`]: https://docs.rs/devela/latest/devela/mem/macro.mem_size_of_expr.html
