This project includes code adapted from various permissively licensed sources.

Below is a list of derived works, their origins, and relevant modifications.
See the linked modifications for detailed changes.

## MIT OR Apache-2.0 licenses

- <i id="bytehound-preload"></i>[bytehound-preload] by Jan Bujak,
  as the [`SpinLock`] and [`SpinLockGuard`] structs ([*modifications*][M_bytehound-preload]).
- <i id="cfg-if"></i>[cfg-if] by Alex Crichton,
  as the [`cfg-if!`] macro.
- <i id="const_list"></i>[const_list] by Douglas Dwyer,
  as the [`ConstList`] struct.
- <i id="crossbeam-utils"></i>[crossbeam-utils] by The Crossbeam Project Developers,
  as the [`CacheAlign`] struct ([*modifications*][M_crossbeam-utils]).
- <i id="etcetera"></i>[etcetera] by Luna Razzaghipour,
  as the [`AppEnv`] trait and related structs ([*modifications*][M_etcetera]).
- <i id="fmtor"></i>[fmtor] by Tyler Ruckinger,
  as `ExtOption`'s [`fmt_or`], [`fmt_or_else`] and [`fmt_or_empty`] methods.
- <i id="fxhash"></i>[fxhash] by Christopher Breeden,
  as the [`HasherFx`] struct ([*modifications*][M_fxhash]).
- <i id="mini-alloc"></i>[mini-alloc] by Offchain Labs Inc.,
  as the [`WasmAlloc`] struct ([*modifications*][M_mini-alloc]).
- <i id="no_std_io"></i>[no_std_io]|[core2] by Brendan Molloy,
  as part of the [`sys::io`] module ([*modifications*][M_no_std_io]).
- <i id="numtoa"></i>[numtoa] by Michael Murphy,
  as the [`NumToStr`] trait ([*modifications*][M_numtoa]).
- <i id="pollster"></i>[pollster] by Joshua Barretto,
  as the `ExtFuture::`[`block_on`] method ([*modifications*][M_pollster]).
- <i id="quickdiv"></i>[quickdiv] by Darko Trifunovski,
  as the [`Divisor`] struct ([*modifications*][M_quickdiv]).
- <i id="stack_dst"></i>[stack_dst] by John Hodge,
  as the [`data::dst`] module ([*modifications*][M_stack_dst]).
- <i id="stated-scope-guard"></i>[stated-scope-guard] by EvianZhang,
  as part the [`ScopeGuard`] struct ([*modifications*][M_stated-scope-guard]).
- <i id="static_assertions"></i>[static_assertions] by Nikolai Vazquez,
  as part of the [`const_assert!`] macro and the [`ConstBool`] trait
  ([*modifications*][M_static_assertions]).
- <i id="tailcall-chunk"></i>[tailcall-chunk] by Tushar Mathur,
  as the [`VecChunk`] struct ([*modifications*][M_tailcall-chunk]).
- <i id="tupl"></i>[tupl] by *Dragoteryx*,
  as part of the [`Tuple`] trait ([*modifications*][M_tupl]).

[bytehound-preload]: https://github.com/koute/bytehound/blob/77ea03c7ed90ad4f176c316cd837a77bc09aa6f3/preload/src/spin_lock.rs
  [M_bytehound-preload]: https://github.com/andamira/devela/blob/main/src/_info/vendored/bytehound-preload.md
  [`SpinLock`]: https://docs.rs/devela/latest/devela/work/sync/struct.SpinLock.html
  [`SpinLockGuard`]: https://docs.rs/devela/latest/devela/work/sync/struct.SpinLockGuard.html
[cfg-if]: https://crates.io/crates/cfg-if/1.0.0
  [`cfg-if!`]: https://docs.rs/devela/latest/devela/code/util/macro.cfg_if.html
[const_list]: https://crates.io/crates/const_list/0.1.0
  [`ConstList`]: https://docs.rs/devela/latest/devela/data/list/struct.ConstList.html
[crossbeam-utils]: https://crates.io/crates/crossbeam-utils/0.8.21
  [M_crossbeam-utils]: https://github.com/andamira/devela/blob/main/src/_info/vendored/crossbeam-utils.md
  [`CacheAlign`]: https://docs.rs/devela/latest/devela/sys/mem/struct.CacheAlign.html
[etcetera]: https://crates.io/crates/etcetera/0.8.0
  [M_etcetera]: https://github.com/andamira/devela/blob/main/src/_info/vendored/etcetera.md
  [`AppEnv`]: https://docs.rs/devela/latest/devela/sys/env/trait.AppEnv.html
[fmtor]: https://crates.io/crates/fmtor/0.1.2
  [`fmt_or`]: https://docs.rs/devela/latest/devela/code/util/result/trait.ExtOption.html#tymethod.fmt_or
  [`fmt_or_else`]: https://docs.rs/devela/latest/devela/code/util/result/trait.ExtOption.html#tymethod.fmt_or_else
  [`fmt_or_empty`]: https://docs.rs/devela/latest/devela/code/util/result/trait.ExtOption.html#tymethod.fmt_or_empty
[fxhash]: https://crates.io/crates/fxhash/0.2.1
  [M_fxhash]: https://github.com/andamira/devela/blob/main/src/_info/vendored/fxhash.md
  [`HasherFx`]: https://docs.rs/devela/latest/devela/data/codec/hash/struct.HasherFx.html
[mini-alloc]: https://crates.io/crates/mini-alloc/0.9.0
  [M_mini-alloc]: https://github.com/andamira/devela/blob/main/src/_info/vendored/mini-alloc.md
[no_std_io]: https://crates.io/crates/no_std_io/0.6.0
[core2]: https://crates.io/crates/core2/0.4.0
  [M_no_std_io]: https://github.com/andamira/devela/blob/main/src/_info/vendored/no_std_io.md
  [`sys::io`]: https://docs.rs/devela/latest/devela/sys/io/
[numtoa]: https://crates.io/crates/numtoa/0.2.4
  [M_numtoa]: https://github.com/andamira/devela/blob/main/src/_info/vendored/numtoa.md
  [`NumToStr`]: https://docs.rs/devela/latest/devela/text/fmt/trait.NumToStr.html
[pollster]: https://crates.io/crates/pollster/0.3.0
  [M_pollster]: https://github.com/andamira/devela/blob/main/src/_info/vendored/pollster.md
  [`block_on`]: https://docs.rs/devela/latest/devela/work/future/trait.ExtFuture.html#method.block_on
[quickdiv]: https://crates.io/crates/quickdiv/0.1.1
  [M_quickdiv]: https://github.com/andamira/devela/blob/main/src/_info/vendored/quickdiv.md
  [`Divisor`]: https://docs.rs/devela/latest/devela/num/struct.Divisor.html
[stack_dst]: https://crates.io/crates/stack_dst/0.8.1
  [M_stack_dst]: https://github.com/andamira/devela/blob/main/src/_info/vendored/stack_dst.md
  [`data::dst`]: https://docs.rs/devela/latest/devela/data/dst/index.html
[stated-scope-guard]: https://crates.io/crates/stated-scope-guard/0.1.0
  [M_stated-scope-guard]: https://github.com/andamira/devela/blob/main/src/_info/vendored/stated-scope-guard.md
  [`ScopeGuard`]: https://docs.rs/devela/latest/devela/code/struct.ScopeGuard.html
[static_assertions]: https://crates.io/crates/static_assertions/1.1.0
  [M_static_assertions]: https://github.com/andamira/devela/blob/main/src/_info/vendored/static_assertions.md
  [`const_assert!`]: https://docs.rs/devela/latest/devela/code/util/macro.const_assert.html
  [`ConstBool`]: https://docs.rs/devela/latest/devela/num/logic/trait.ConstBool.html
[tailcall-chunk]: https://crates.io/crates/tailcall-chunk/0.3.1
  [M_tailcall-chunk]: https://github.com/andamira/devela/blob/main/src/_info/vendored/tailcall-chunk.md
  [`VecChunk`]: https://docs.rs/devela/latest/devela/data/list/array/struct.VecChunk.html
[tupl]: https://crates.io/crates/tupl/0.4.0
  [M_tupl]: https://github.com/andamira/devela/blob/main/src/_info/vendored/tupl.md
  [`Tuple`]: https://docs.rs/devela/latest/devela/data/list/tuple/trait.Tuple.html

## MIT licenses
- <i id="const_for"></i>[const_for] by Joachim Enggård Nebel,
  as the [`cfor!`] macro.
- <i id="crunchy"></i>[crunchy] by Eira Fransham,
  as the [`unroll!`] macro ([*modifications*][M_crunchy]).
- <i id="current"></i>[current] by *PistonDevelopers*,
  as the structs [`Current`] and [`CurrentGuard`] ([*modifications*][M_current]).
- <i id="encode"></i>[encode] by Altair Bueno,
  as some items in the [`data::codec`] module ([*modifications*][M_encode]).
- <i id="icy_sixel"></i>[icy_sixel] by Mike Krüger, based on [libsixel] by Hayaki Saito,
  as the [`media::image::sixel`] module ([*modifications*][M_icy_sixel]).
- <i id="musl"></i>[musl] by Rich Felker, et al.,
  as part of the [`Linux`] signal restorer assembly code.
- <i id="object-id"></i>[object-id] by *Altertech*,
  as part of the [`IdPinBox`] and [`IdPin`] structs ([*modifications*][M_object-id]).
- <i id="rawbytes"></i>[rawbytes] by Frank Denis,
  as `Mem`'s [`as_bytes`] and [`as_bytes_mut`] methods.
- <i id="unsized-stack"></i>[unsized-stack] by *storycraft*,
  as the [`FatPtr`] struct.

[const_for]: https://crates.io/crates/const_for/0.1.4
  [`cfor!`]: https://docs.rs/devela/latest/devela/code/util/macro.cfor.html
[crunchy]: https://crates.io/crates/crunchy/0.2.3
  [M_crunchy]: https://github.com/andamira/devela/blob/main/src/_info/vendored/crunchy.md
  [`unroll!`]: https://docs.rs/devela/latest/devela/code/util/macro.unroll.html
[current]: https://crates.io/crates/current/0.1.2
  [M_current]: https://github.com/andamira/devela/blob/main/src/_info/vendored/current.md
  [`Current`]: https://docs.rs/devela/latest/devela/sys/mem/struct.Current.html
  [`CurrentGuard`]: https://docs.rs/devela/latest/devela/sys/mem/struct.CurrentGuard.html
[encode]: https://crates.io/crates/encode/0.1.2
  [M_encode]: https://github.com/andamira/devela/blob/main/src/_info/vendored/encode.md
  [`data::codec`]: https://docs.rs/devela/latest/devela/data/codec/
[icy_sixel]: https://crates.io/crates/icy_sixel/0.1.3
[libsixel]: https://github.com/saitoha/libsixel
  [M_icy_sixel]: https://github.com/andamira/devela/blob/main/src/_info/vendored/icy_sixel.md
  [`media::image::sixel`]: https://docs.rs/devela/latest/devela/media/image/sixel
[musl]: https://git.musl-libc.org/cgit/musl/tag/?h=v1.2.5
  [`Linux`]: https://docs.rs/devela/latest/devela/os/linux/struct.Linux.html
[object-id]: https://crates.io/crates/object-id/0.1.4
  [M_object-id]: https://github.com/andamira/devela/blob/main/src/_info/vendored/object-id.md
  [`IdPin`]: https://docs.rs/devela/latest/devela/data/uid/struct.IdPin.html
  [`IdPinBox`]: https://docs.rs/devela/latest/devela/data/uid/struct.IdPinBox.html
[rawbytes]: https://crates.io/crates/rawbytes/1.0.0
  [`as_bytes`]: https://docs.rs/devela/latest/devela/sys/mem/struct.Mem.html#method.as_bytes
  [`as_bytes_mut`]: https://docs.rs/devela/latest/devela/sys/mem/struct.Mem.html#method.as_bytes_mut
[unsized-stack]: https://crates.io/crates/unsized-stack/0.2.0
  [`FatPtr`]: https://docs.rs/devela/latest/devela/sys/mem/struct.FatPtr.html

## Other licenses
- <i id="8bit_rng"></i>[8bit_rng] by Edward Rosten, (BSD-2),
  as the [`Xyza8a`] and [`Xyza8b`] structs algorithms.
- <i id="apply"></i>[apply] by GeorgeBurton (Unlicense),
  as part of the [`Chain`] and [`Hook`] structs.
- <i id="blit-fonts"></i>[blit-fonts] by Andrew Reece (ISC),
  as the [`FONT_3_5`] and [`FONT_5_6`] consts data<!-- ([*modifications*][M_blit-fonts]) -->.
- <i id="jiff"></i>[jiff] by Andrew Gallant (Unlicense),
  as the [`TimeDelta`] struct ([*modifications*][M_jiff]).
- <i id="pengyhash"></i>[pengyhash] by Alberto Fajardo (BSD-2),
  as the [`HasherPengy`] struct algorithm ([*modifications*][M_pengyhash]).
- <i id="size_of_trait"></i>[size_of_trait] byt Joshua Nelson (BSD-3),
  as the [`size_of_expr!`] macro.
- <i id="Xabc"></i>[Xabc] by *EternityForest* (openly shared),
  as the [`Xabc`] struct algorithm.
- <i id="GraphicGems"></i>[Graphics Gems] (1985–1994) (permissive EULA),
  as various algorithms.

[8bit_rng]: https://github.com/edrosten/8bit_rng
  [`Xyza8a`]: https://docs.rs/devela/latest/devela/num/rand/struct.Xyza8a.html
  [`Xyza8b`]: https://docs.rs/devela/latest/devela/num/rand/struct.Xyza8b.html
[apply]: https://crates.io/crates/apply/0.3.0
  [`Chain`]: https://docs.rs/devela/latest/devela/code/result/trait.Chain.html
  [`Hook`]: https://docs.rs/devela/latest/devela/code/result/trait.Hook.html
[blit-fonts]: https://github.com/azmr/blit-fonts
  <!-- [M_blit-fonts]: https://github.com/andamira/devela/blob/main/src/media/font/bitmap/MODS_BLIT.md -->
  [`FONT_3_5`]: https://docs.rs/devela/latest/devela/media/font/const.FONT_3_5.html
  [`FONT_5_6`]: https://docs.rs/devela/latest/devela/media/font/const.FONT_5_6.html
[jiff]: https://crates.io/crates/jiff/0.2.1
  [M_jiff]: https://github.com/andamira/devela/blob/main/src/_info/vendored/jiff.md
  [`TimeDelta`]: https://docs.rs/devela/latest/devela/phys/time/struct.TimeDelta.html
[pengyhash]: https://github.com/tinypeng/pengyhash/blob/70a23e40a2be2e784a68078213b7675055f21949/pengyhash.c
  [M_pengy]: https://github.com/andamira/devela/blob/main/src/_info/vendored/pengy.md
  [`HasherPengy`]: https://docs.rs/devela/latest/devela/data/codec/hash/struct.HasherPengy.html
[size_of_trait]: https://crates.io/crates/size-of-trait/1.1.3
  [`size_of_expr!`]: https://docs.rs/devela/latest/devela/sys/mem/macro.size_of_expr.html
[Xabc]: https://web.archive.org/web/20140328221846/https://www.electro-tech-online.com/threads/ultra-fast-pseudorandom-number-generator-for-8-bit.124249/
  [`Xabc`]: https://docs.rs/devela/latest/devela/num/rand/struct.Xabc.html
[Graphics Gems]: https://www.realtimerendering.com/resources/GraphicsGems/
