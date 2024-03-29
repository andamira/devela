This project includes the following derived works:

- A [modified][1] version of [stack_dst] by John Hodge, licensed as MIT OR Apache-2.0,
  in the [`data::dst`] module.
- A [modified][2] version of [tupl] by *Dragoteryx*, licensed as MIT or Apache-2.0,
  as part of the [`ExtTuple`] trait.
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
- Integration of [8bit_rng] algorithms by Edward Rosten, licensed as BSD-2,
  as the [`Xyza8a`] and [`Xyza8b`] RNGs.
- Integration of [Xabc] algorithm openly published by *EternityForest*,
  as the [`Xabc`] RNG.

[1]: https://github.com/andamira/devela/blob/main/src/data/dst/MODIFICATIONS.md
[stack_dst]: https://crates.io/crates/stack_dst/0.8.1
[`data::dst`]: https://docs.rs/devela/latest/devela/data/dst/index.html
[2]: https://github.com/andamira/devela/blob/main/src/data/collections/tuple/MODIFICATIONS.md
[tupl]: https://crates.io/crates/tupl/0.4.0
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
[const_for]: https://crates.io/crates/const_for/0.1.4
[`cfor`]: https://docs.rs/devela/latest/devela/code/macro.cfor.html
[no_std_io]: https://crates.io/crates/no_std_io/0.6.0
[core2]: https://crates.io/crates/core2/0.4.0
[`io`]: https://docs.rs/devela/latest/devela/io/
[numtoa]: https://crates.io/crates/numtoa/0.2.4
[size_of_trait]: https://crates.io/crates/size-of-trait/1.1.3
[`mem_size_of_expr`]: https://docs.rs/devela/latest/devela/mem/size/macro.mem_size_of_expr.html
[8bit_rng]: https://github.com/edrosten/8bit_rng
[`Xyza8a`]: https://docs.rs/devela/latest/devela/num/rand/struct.Xyza8a.html
[`Xyza8b`]: https://docs.rs/devela/latest/devela/num/rand/struct.Xyza8b.html
[Xabc]: https://www.electro-tech-online.com/threads/ultra-fast-pseudorandom-number-generator-for-8-bit.124249/
[`Xabc`]: https://docs.rs/devela/latest/devela/num/rand/struct.Xabc.html
