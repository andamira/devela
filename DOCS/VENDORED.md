This project includes code adapted from various permissively licensed sources.

Below is a list of derived works, their origins, and relevant modifications.
See linked files for detailed changes.

## Works under dual MIT OR Apache-2.0 licenses
- [cfg_if] by Alex Crichton, as the [`cfg_if`] macro.
- [const_list] by Douglas Dwyer, as the [`ConstList`] struct.
- [crossbeam-utils] by The Crossbeam Project Developers, as the [`CacheAlign`] struct.
- [etcetera] by Luna Razzaghipour, as the [`DirEnv`] trait and related structs ([*mod*][M_dir]).
- [fmtor] by Tyler Ruckinger, as part of the [`ExtOption`] trait.
- [fxhash] by Christopher Breeden, as the [`HasherFx`] struct ([*mod*][M_fxhash]).
- [no_std_io]|[core2] by *Brendan Molloy*, as part of the [`io`] module.
- [numtoa] by Michael Murphy, as the [`NumToStr`] trait.
- [pollster] by Joshua Barretto, as the [`future_block`] fn.
- [quickdiv] by Darko Trifunovski, as the [`Divisor`] struct ([*mod*][M_quickdiv]).
- [stack_dst] by John Hodge, as the [`data::dst`] module ([*mod*][M_stack_dst]).
- [static_assertions] by Nikolai Vazquez, as part of the [`const_assert`] macro
  and the [`ConstBool`] trait ([*mod*][M_sta_ase]).
- [tailcall-chunk] by Tushar Mathur, as the [`VecChunk`] struct ([*mod*][M_tailcall]).
- [tupl] by *Dragoteryx*, as part of the [`Tuple`] trait ([*mod*][M_tupl]).

[cfg_if]: https://crates.io/crates/cfg_if/1.0.0
  [`cfg_if`]: https://docs.rs/devela/latest/devela/code/macro.cfg_if.html
[const_list]: https://crates.io/crates/const_list/0.1.0
  [`ConstList`]: https://docs.rs/devela/latest/devela/data/collections/struct.ConstList.html
[crossbeam-utils]: https://crates.io/crates/crossbeam-utils/0.8.20
  [`CacheAlign`]: https://docs.rs/devela/latest/devela/mem/struct.CacheAlign.html
[etcetera]: https://crates.io/crates/etcetera/0.8.0
  [M_dir]: https://github.com/andamira/devela/blob/main/src/sys/env/dir/MODIFICATIONS.md
  [`DirEnv`]: https://docs.rs/devela/latest/devela/sys/env/trait.DirEnv.html
[fmtor]: https://crates.io/crates/fmtor/0.1.2
  [`ExtOption`]: https://docs.rs/devela/latest/devela/code/trait.ExtOption.html
[fxhash]: https://crates.io/crates/fxhash/0.2.1
  [M_fxhash]: https://github.com/andamira/devela/blob/main/src/data/hash/fx/MODIFICATIONS.md
  [`HasherFx`]: https://docs.rs/devela/latest/devela/data/hash/struct.HasherFx.html
[no_std_io]: https://crates.io/crates/no_std_io/0.6.0
[core2]: https://crates.io/crates/core2/0.4.0
  [`io`]: https://docs.rs/devela/latest/devela/sys/io/
[numtoa]: https://crates.io/crates/numtoa/0.2.4
  [`NumToStr`]: https://docs.rs/devela/latest/devela/text/fmt/trait.NumToStr.html
[stack_dst]: https://crates.io/crates/stack_dst/0.8.1
  [M_stack_dst]: https://github.com/andamira/devela/blob/main/src/data/dst/MODIFICATIONS.md
  [`data::dst`]: https://docs.rs/devela/latest/devela/data/dst/index.html
[pollster]: https://crates.io/crates/pollster/0.3.0
  [`future_block`]: https://docs.rs/devela/latest/devela/exec/fn.future_block.html
[quickdiv]: https://crates.io/crates/quickdiv/0.1.1
  [M_quickdiv]: https://github.com/andamira/devela/blob/main/src/num/int/divisor/MODIFICATIONS.md
  [`Divisor`]: https://docs.rs/devela/latest/devela/num/struct.Divisor.html
[static_assertions]: https://crates.io/crates/static_assertions/1.1.0
  [M_sta_ase]: https://github.com/andamira/devela/blob/main/src/code/asserts/static/MODIFICATIONS.md
  [`const_assert`]: https://docs.rs/devela/latest/devela/code/macro.const_assert.html
  [`ConstBool`]: https://docs.rs/devela/latest/devela/num/logic/trait.ConstBool.html
[tailcall-chunk]: https://crates.io/crates/tailcall-chunk/0.3.1
  [M_tailcall]: https://github.com/andamira/devela/blob/main/src/data/collections/vec/chunk/MODIFICATIONS.md
  [`VecChunk`]: https://docs.rs/devela/latest/devela/data/collections/struct.VecChunk.html
[tupl]: https://crates.io/crates/tupl/0.4.0
  [M_tupl]: https://github.com/andamira/devela/blob/main/build/generate/tuple/MODIFICATIONS.md
  [`Tuple`]: https://docs.rs/devela/latest/devela/data/collections/trait.Tuple.html

## Works under MIT License
- [const_for] by Joachim Enggård Nebel, as the [`cfor`] macro.
- [crunchy] by Eira Fransham, as the [`unroll`] macro ([*mod*][M_unroll]).
- [object-id] by *Altertech*, as part of the [`IdPinBox`] and [`IdPin`] structs ([*mod*][M_objid]).
- [rawbytes] by Frank Denis, as `Mem`'s [`as_bytes`] and [`as_bytes_mut`] methods.
- [unsized-stack] by *storycraft*, as the [`FatPtr`] struct.

[const_for]: https://crates.io/crates/const_for/0.1.4
  [`cfor`]: https://docs.rs/devela/latest/devela/code/macro.cfor.html
[crunchy]: https://crates.io/crates/crunchy/0.2.2
  [`unroll`]: https://docs.rs/devela/latest/devela/code/macro.unroll.html
  [M_unroll]: https://github.com/andamira/devela/blob/main/build/generate/unroll/MODIFICATIONS.md
[object-id]: https://crates.io/crates/object-id/0.1.4
  [M_objid]: https://github.com/andamira/devela/blob/main/src/data/id/pin/MODIFICATIONS.md
  [`IdPin`]: https://docs.rs/devela/latest/devela/data/id/struct.IdPin.html
  [`IdPinBox`]: https://docs.rs/devela/latest/devela/data/id/struct.IdPinBox.html
[rawbytes]: https://crates.io/crates/rawbytes/1.0.0
  [`as_bytes`]: https://docs.rs/devela/latest/devela/mem/struct.Mem.html#method.as_bytes
  [`as_bytes_mut`]: https://docs.rs/devela/latest/devela/mem/struct.Mem.html#method.as_bytes_mut
[unsized-stack]: https://crates.io/crates/unsized-stack/0.2.0
  [`FatPtr`]: https://docs.rs/devela/latest/devela/mem/struct.FatPtr.html

## Other Licenses
- [8bit_rng] by Edward Rosten, (BSD-2) as the [`Xyza8a`] and [`Xyza8b`] structs.
- [apply] by GeorgeBurton (Unlicense) as part of the [`Chain`] and [`Hook`] structs.
- [pengyhash] by Alberto Fajardo (BSD-2), as the [`HasherPengy`] struct.
- [size_of_trait] byt Joshua Nelson (BSD-3) as the [`mem_size_of_expr`] fn.
- [Xabc] by *EternityForest* (openly shared) as the [`Xabc`] struct.
- Graphics Gems (1985–1994) (permissive [EULA]), as various algorithms.

[8bit_rng]: https://github.com/edrosten/8bit_rng
  [`Xyza8a`]: https://docs.rs/devela/latest/devela/num/rand/struct.Xyza8a.html
  [`Xyza8b`]: https://docs.rs/devela/latest/devela/num/rand/struct.Xyza8b.html
[apply]: https://crates.io/crates/apply/0.3.0
  [`Chain`]: https://docs.rs/devela/latest/devela/error/trait.Chain.html
  [`Hook`]: https://docs.rs/devela/latest/devela/error/trait.Hook.html
[pengyhash]: https://github.com/tinypeng/pengyhash/blob/70a23e40a2be2e784a68078213b7675055f21949/pengyhash.c
  [`HasherPengy`]: https://docs.rs/devela/latest/devela/data/hash/struct.HasherPengy.html
[size_of_trait]: https://crates.io/crates/size-of-trait/1.1.3
  [`mem_size_of_expr`]: https://docs.rs/devela/latest/devela/mem/macro.size_of_expr.html
[Xabc]: https://www.electro-tech-online.com/threads/ultra-fast-pseudorandom-number-generator-for-8-bit.124249/
  [`Xabc`]: https://docs.rs/devela/latest/devela/num/rand/struct.Xabc.html

[EULA]: https://github.com/erich666/GraphicsGems/blob/master/LICENSE.md
