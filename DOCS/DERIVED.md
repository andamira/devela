This project includes the following derived works:

- A [modified][1] version of [stack_dst] by John Hodge, licensed as MIT OR Apache-2.0,
  in the [`data::dst`] module.
- A [modified][2] version of [tupl] by *Dragoteryx*, licensed as MIT or Apache-2.0,
  as part of the [`Tuple`] trait and other items.
- A [modified][3] version of [fxhash] by Christopher Breeden, licensed as MIT OR Apache-2.0,
  as the [`HasherFx`], [`HasherFx32`] and [`HasherFx64`] hashers.
- A [modified][4] version of [object-id] by *Altertech*, licensed as MIT,
  as part of the `IdPinBox` and `IdPin` structs.
- A [modified][5] version of [static_assertions] by Nikolai Vazquez,
  licensed as MIT or Apache-2.0,
  as part of the [`const_assert`] macro, and  [`ConstBool`] trait.
- A modified version of [unsized-stack] by *storycraft*, licensed as MIT,
  as the [`FatPtr`] struct.
- Adaptation of [quickdiv] v0.1.1 by Darko Trifunovski, licensed as Zlib OR MIT OR Apache-2.0,
  as the [`Divisor`] struct.
- Adaptation of [pengyhash] v0.2 by Alberto Fajardo, licensed as BSD-2,
  as the [`hash_pengy`] hasher function.
- Adaptation of [opt_reduce] by Waffle Lapkin, licensed as MIT,
  as part of the [`ExtOption`] trait.
- Adaptation of [fmtor] by Tyler Ruckinger, licensed as MIT OR Apache-2.0,
  as part of the [`ExtOption`] trait.
- Adaptation of [option-ext] by Simon Ochsenreither, licensed as MPL-2.0,
  as part of the [`ExtOption`] trait.
- Adaptation of [result-ext] by Simon Ochsenreither, licensed as MPL-2.0,
  as part of the [`ExtResult`] trait.
- Adaptation of [pollster] by Joshua Barretto, licensed as MIT OR Apache-2.0,
  as the [`future_block`] function.
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
[fxhash]: https://crates.io/crates/fxhash/0.2.1
[3]: https://github.com/andamira/devela/blob/main/src/data/hash/fx/MODIFICATIONS.md
[object-id]: https://crates.io/crates/object-id/0.1.4
[4]: https://github.com/andamira/devela/blob/main/src/data/id/pin/MODIFICATIONS.md
[static_assertions]: https://crates.io/crates/static_assertions/1.1.0
[5]: https://github.com/andamira/devela/blob/main/src/code/asserts/static/MODIFICATIONS.md
[unsized-stack]: https://crates.io/crates/unsized-stack/0.2.0
[quickdiv]: https://crates.io/crates/quickdiv/0.1.1
[`Divisor`]: https://docs.rs/devela/latest/devela/num/struct.Divisor.html
[pengyhash]: https://github.com/tinypeng/pengyhash/blob/70a23e40a2be2e784a68078213b7675055f21949/pengyhash.c
[`hash_pengy`]: https://docs.rs/devela/latest/devela/data/hash/fn.hash_pengy.html
[tupl]: https://crates.io/crates/tupl/0.4.0
[opt_reduce]: https://crates.io/crates/opt_reduce/1.0.0
[fmtor]: https://crates.io/crates/fmtor/0.1.2
[option-ext]: https://crates.io/crates/option-ext/0.2.0
[result-ext]: https://crates.io/crates/result-ext/0.2.0
[`ExtOption`]: https://docs.rs/devela/latest/devela/code/trait.ExtOption.html
[`ExtResult`]: https://docs.rs/devela/latest/devela/code/trait.ExtResult.html
[pollster]: https://crates.io/crates/pollster/0.3.0
[`future_block`]: https://docs.rs/devela/latest/devela/exec/fn.future_block.html
[apply]: https://crates.io/crates/apply/0.3.0
[`Also`]: https://docs.rs/devela/latest/devela/code/trait.Also.html
[`Apply`]: https://docs.rs/devela/latest/devela/code/trait.Apply.html
[rawbytes]: https://crates.io/crates/rawbytes/1.0.0
[`mem_as_bytes`]: https://docs.rs/devela/latest/devela/data/fn.mem_as_bytes.html
[`mem_as_bytes_mut`]: https://docs.rs/devela/latest/devela/data/fn.mem_as_bytes_mut.html
[const_for]: https://crates.io/crates/const_for/0.1.4
[`cfor`]: https://docs.rs/devela/latest/devela/code/macro.cfor.html
[no_std_io]: https://crates.io/crates/no_std_io/0.6.0
[core2]: https://crates.io/crates/core2/0.4.0
[`io`]: https://docs.rs/devela/latest/devela/sys/io/
[numtoa]: https://crates.io/crates/numtoa/0.2.4
[size_of_trait]: https://crates.io/crates/size-of-trait/1.1.3
[`mem_size_of_expr`]: https://docs.rs/devela/latest/devela/data/size/macro.mem_size_of_expr.html
[8bit_rng]: https://github.com/edrosten/8bit_rng
[`Xyza8a`]: https://docs.rs/devela/latest/devela/num/rand/struct.Xyza8a.html
[`Xyza8b`]: https://docs.rs/devela/latest/devela/num/rand/struct.Xyza8b.html
[Xabc]: https://www.electro-tech-online.com/threads/ultra-fast-pseudorandom-number-generator-for-8-bit.124249/
[`Xabc`]: https://docs.rs/devela/latest/devela/num/rand/struct.Xabc.html
