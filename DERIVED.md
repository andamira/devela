This project includes the following derived works:

- A [modified][0] version of [itoa] by David Tolnay, licensed as MIT OR Apache-2.0,
  in [`IntBuf`] and [`IntBufAble`].
- A [modified][1] version of [stack_dst] by John Hodge, licensed as MIT OR Apache-2.0,
  in the [`data::dst`] module.
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
  into the [`mem_as_bytes`] and [`mem_as_bytes_mut`] functions.
- Integration of [const_for] by Joachim Enggård Nebel, licensed as MIT,
  as the [`cfor`] macro.
- Integration of [no_std_io]|[core2] by Brendan Molloy, licensed as MIT OR Apache-2.0,
  as part of the [`io`] module.
- Integration of [numtoa] by Michael Murphy, licensed as MIT OR Apache-2.0,
  as the [`NumToStr`] trait.
- Integration of [size_of_trait] by Joshua Nelson, licensed as BSD-3,
 as the [`mem_size_of_expr`] macro.

[0]: https://github.com/andamira/devela/blob/main/src/fmt/int_buf/MODIFICATIONS.md
[itoa]: https://crates.io/crates/itoa/1.0.9
[`IntBuf`]: https://docs.rs/devela/latest/devela/text/fmt/struct.IntBuf.html
[`IntBufAble`]: https://docs.rs/devela/latest/devela/text/fmt/trait.IntBufAble.html
[1]: https://github.com/andamira/devela/blob/main/src/data/dst/MODIFICATIONS.md
[stack_dst]: https://crates.io/crates/stack_dst/0.8.1
[`data::dst`]: https://docs.rs/devela/latest/devela/data/dst/index.html
[opt_reduce]: https://crates.io/crates/opt_reduce/1.0.0
[fmtor]: https://crates.io/crates/fmtor/0.1.2
[option-ext]: https://crates.io/crates/option-ext/0.2.0
[result-ext]: https://crates.io/crates/result-ext/0.2.0
[`OptionExt`]: https://docs.rs/devela/latest/devela/error/trait.OptionExt.html
[`ResultExt`]: https://docs.rs/devela/latest/devela/error/trait.ResultExt.html
[apply]: https://crates.io/crates/apply/0.3.0
[`Also`]: https://docs.rs/devela/latest/devela/code/trait.Also.html
[`Apply`]: https://docs.rs/devela/latest/devela/code/trait.Apply.html
[rawbytes]: https://crates.io/crates/rawbytes/1.0.0
[`mem_as_bytes`]: https://docs.rs/devela/latest/devela/mem/fn.mem_as_bytes.html
[`mem_as_bytes_mut`]: https://docs.rs/devela/latest/devela/mem/fn.mem_as_bytes_mut.html
[const_for]: https://crates.io/crates/const_for
[`cfor`]: https://docs.rs/devela/latest/devela/code/macro.cfor.html
[no_std_io]: https://crates.io/crates/no_std_io
[core2]: https://crates.io/crates/core2
[`io`]: https://docs.rs/devela/latest/devela/io/
[numtoa]: https://crates.io/crates/numtoa/0.2.4
[size_of_trait]: https://crates.io/crates/size-of-trait
[`mem_size_of_expr`]: https://docs.rs/devela/latest/devela/mem/size/macro.mem_size_of_expr.html
