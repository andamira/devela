This project includes code adapted from various permissively licensed sources.

Below is a list of derived works, their origins, and relevant modifications.
See the linked modifications for detailed changes.

## MIT OR Apache-2.0 licenses

- <i id="argv"></i>[argv] by David Tolnay,
  as the [`IterArgsOsRef`] struct and the [`Env::args_os_ref`] method ([*modifications*][M_argv]).
- <i id="bytehound-preload"></i>[bytehound-preload] by Jan Bujak,
  as the [`SpinLock`] and [`SpinLockGuard`] structs ([*modifications*][M_bytehound-preload]).
- <i id="cfg-if"></i>[cfg-if] by Alex Crichton,
  as the [`cfg-if!`] macro ([*modifications*][M_cfg-if]).
- <i id="const_list"></i>[const_list] by Douglas Dwyer,
  as the [`ConstList`] struct.
- <i id="crossbeam-utils"></i>[crossbeam-utils] by The Crossbeam Project Developers,
  as the [`CacheAlign`] struct ([*modifications*][M_crossbeam-utils]).
- <i id="etcetera"></i>[etcetera] by Luna Razzaghipour,
  as the [`AppEnv`] trait and related structs ([*modifications*][M_etcetera]).
- <i id="fmtor"></i>[fmtor] by Tyler Ruckinger,
  as `OptionExt`'s [`fmt_or`], [`fmt_or_else`] and [`fmt_or_empty`] methods.
- <i id="fxhash"></i>[fxhash] by Christopher Breeden,
  as the [`HasherFx`] struct ([*modifications*][M_fxhash]).
- <i id="mini-alloc"></i>[mini-alloc] by Offchain Labs Inc.,
  as the [`WasmAlloc`] struct ([*modifications*][M_mini-alloc]).
- <i id="no_std_io"></i>[no_std_io]|[core2] by Brendan Molloy,
  as part of the [`sys::io`] module ([*modifications*][M_no_std_io]).
- <i id="pollster"></i>[pollster] by Joshua Barretto,
  as the `FutureExt::`[`block_on`] method ([*modifications*][M_pollster]).
- <i id="quickdiv"></i>[quickdiv] by Darko Trifunovski,
  as the [`define_divisor!`] macro ([*modifications*][M_quickdiv]).
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

[argv]: https://crates.io/crates/argv/0.1.13
  [M_argv]: https://github.com/andamira/devela/blob/main/src/_doc/vendored/argv.md
  [`IterArgsOsRef`]: https://andamira.github.io/devela/latest/devela/sys/env/struct.IterArgsOsRef.html
  [`Env::args_os_ref`]: https://andamira.github.io/devela/latest/devela/sys/env/struct.Env.html#method.args_os_ref
[bytehound-preload]: https://github.com/koute/bytehound/blob/77ea03c7ed90ad4f176c316cd837a77bc09aa6f3/preload/src/spin_lock.rs
  [M_bytehound-preload]: https://github.com/andamira/devela/blob/main/src/_doc/vendored/bytehound-preload.md
  [`SpinLock`]: https://andamira.github.io/devela/latest/devela/work/sync/struct.SpinLock.html
  [`SpinLockGuard`]: https://andamira.github.io/devela/latest/devela/work/sync/struct.SpinLockGuard.html
[cfg-if]: https://crates.io/crates/cfg-if/1.0.1
  [M_cfg-if]: https://github.com/andamira/devela/blob/main/src/_doc/vendored/cfg-if.md
  [`cfg-if!`]: https://andamira.github.io/devela/latest/devela/code/util/macro.cfg_if.html
[const_list]: https://crates.io/crates/const_list/0.1.0
  [`ConstList`]: https://andamira.github.io/devela/latest/devela/data/list/struct.ConstList.html
[crossbeam-utils]: https://crates.io/crates/crossbeam-utils/0.8.21
  [M_crossbeam-utils]: https://github.com/andamira/devela/blob/main/src/_doc/vendored/crossbeam-utils.md
  [`CacheAlign`]: https://andamira.github.io/devela/latest/devela/sys/mem/struct.CacheAlign.html
[etcetera]: https://crates.io/crates/etcetera/0.8.0
  [M_etcetera]: https://github.com/andamira/devela/blob/main/src/_doc/vendored/etcetera.md
  [`AppEnv`]: https://andamira.github.io/devela/latest/devela/sys/env/trait.AppEnv.html
[fmtor]: https://crates.io/crates/fmtor/0.1.2
  [`fmt_or`]: https://andamira.github.io/devela/latest/devela/code/util/result/trait.OptionExt.html#tymethod.fmt_or
  [`fmt_or_else`]: https://andamira.github.io/devela/latest/devela/code/util/result/trait.OptionExt.html#tymethod.fmt_or_else
  [`fmt_or_empty`]: https://andamira.github.io/devela/latest/devela/code/util/result/trait.OptionExt.html#tymethod.fmt_or_empty
[fxhash]: https://crates.io/crates/fxhash/0.2.1
  [M_fxhash]: https://github.com/andamira/devela/blob/main/src/_doc/vendored/fxhash.md
  [`HasherFx`]: https://andamira.github.io/devela/latest/devela/data/codec/hash/struct.HasherFx.html
[mini-alloc]: https://crates.io/crates/mini-alloc/0.9.0
  [M_mini-alloc]: https://github.com/andamira/devela/blob/main/src/_doc/vendored/mini-alloc.md
[no_std_io]: https://crates.io/crates/no_std_io/0.6.0
[core2]: https://crates.io/crates/core2/0.4.0
  [M_no_std_io]: https://github.com/andamira/devela/blob/main/src/_doc/vendored/no_std_io.md
  [`sys::io`]: https://andamira.github.io/devela/latest/devela/sys/io/
[pollster]: https://crates.io/crates/pollster/0.3.0
  [M_pollster]: https://github.com/andamira/devela/blob/main/src/_doc/vendored/pollster.md
  [`block_on`]: https://andamira.github.io/devela/latest/devela/work/future/trait.FutureExt.html#method.block_on
[quickdiv]: https://crates.io/crates/quickdiv/0.1.1
  [M_quickdiv]: https://github.com/andamira/devela/blob/main/src/_doc/vendored/quickdiv.md
  [`define_divisor!`]: https://andamira.github.io/devela/latest/devela/num/macro.define_divisor.html
[stack_dst]: https://crates.io/crates/stack_dst/0.8.1
  [M_stack_dst]: https://github.com/andamira/devela/blob/main/src/_doc/vendored/stack_dst.md
  [`data::dst`]: https://andamira.github.io/devela/latest/devela/data/dst/index.html
[stated-scope-guard]: https://crates.io/crates/stated-scope-guard/0.1.0
  [M_stated-scope-guard]: https://github.com/andamira/devela/blob/main/src/_doc/vendored/stated-scope-guard.md
  [`ScopeGuard`]: https://andamira.github.io/devela/latest/devela/code/struct.ScopeGuard.html
[static_assertions]: https://crates.io/crates/static_assertions/1.1.0
  [M_static_assertions]: https://github.com/andamira/devela/blob/main/src/_doc/vendored/static_assertions.md
  [`const_assert!`]: https://andamira.github.io/devela/latest/devela/code/util/macro.const_assert.html
  [`ConstBool`]: https://andamira.github.io/devela/latest/devela/num/fin/logic/trait.ConstBool.html
[tailcall-chunk]: https://crates.io/crates/tailcall-chunk/0.3.1
  [M_tailcall-chunk]: https://github.com/andamira/devela/blob/main/src/_doc/vendored/tailcall-chunk.md
  [`VecChunk`]: https://andamira.github.io/devela/latest/devela/data/list/array/struct.VecChunk.html
[tupl]: https://crates.io/crates/tupl/0.4.0
  [M_tupl]: https://github.com/andamira/devela/blob/main/src/_doc/vendored/tupl.md
  [`Tuple`]: https://andamira.github.io/devela/latest/devela/data/list/tuple/trait.Tuple.html

## MIT licenses
- <i id="crunchy"></i>[crunchy] by Eira Fransham,
  as the [`unroll!`] macro ([*modifications*][M_crunchy]).
- <i id="current"></i>[current] by *PistonDevelopers*,
  as the structs [`Current`] and [`CurrentGuard`] ([*modifications*][M_current]).
- <i id="encode"></i>[encode] by Altair Bueno,
  as some items in the [`data::codec`] module ([*modifications*][M_encode]).
- <i id="grapheme_machine"></i>[grapheme_machine] by Martin Atkins,
  as [`GraphemeMachine`] and related items in [`text::grapheme`] ([*modifications*][M_grapheme_machine]).
- <i id="musl"></i>[musl] by Rich Felker, et al.,
  as part of the [`Linux`] signal restorer assembly code.
- <i id="object-id"></i>[object-id] by *Altertech*,
  as part of the [`IdPinBox`] and [`IdPin`] structs ([*modifications*][M_object-id]).
- <i id="rawbytes"></i>[rawbytes] by Frank Denis,
  as `Mem`'s [`as_bytes`] and [`as_bytes_mut`] methods.
- <i id="stdext"></i>[stdext] by Igor Aleksanov,
  as [`compile_warn!`] and [`fn_name!`] macros ([*modifications*][M_stdext]).
- <i id="transliteration"></i>[transliteration] by *yf-hk*,
  as the [`scalar_as_ascii_translit`] fn.
- <i id="unsized-stack"></i>[unsized-stack] by *storycraft*,
  as the [`FatPtr`] struct.

[crunchy]: https://crates.io/crates/crunchy/0.2.3
  [M_crunchy]: https://github.com/andamira/devela/blob/main/src/_doc/vendored/crunchy.md
  [`unroll!`]: https://andamira.github.io/devela/latest/devela/code/util/macro.unroll.html
[current]: https://crates.io/crates/current/0.1.2
  [M_current]: https://github.com/andamira/devela/blob/main/src/_doc/vendored/current.md
  [`Current`]: https://andamira.github.io/devela/latest/devela/sys/mem/struct.Current.html
  [`CurrentGuard`]: https://andamira.github.io/devela/latest/devela/sys/mem/struct.CurrentGuard.html
[encode]: https://crates.io/crates/encode/0.1.2
  [M_encode]: https://github.com/andamira/devela/blob/main/src/_doc/vendored/encode.md
  [`data::codec`]: https://andamira.github.io/devela/latest/devela/data/codec/
[grapheme_machine]: https://crates.io/crates/grapheme_machine/0.2.0
  [M_grapheme_machine]: https://github.com/andamira/devela/blob/main/src/_doc/vendored/grapheme_machine.md
  [`GraphemeMachine`]: https://andamira.github.io/devela/latest/devela/text/grapheme/struct.GraphemeMachine.html
  [`text::grapheme`]: https://andamira.github.io/devela/latest/devela/text/grapheme/
[musl]: https://git.musl-libc.org/cgit/musl/tag/?h=v1.2.5
  [`Linux`]: https://andamira.github.io/devela/latest/devela/os/linux/struct.Linux.html
[object-id]: https://crates.io/crates/object-id/0.1.4
  [M_object-id]: https://github.com/andamira/devela/blob/main/src/_doc/vendored/object-id.md
  [`IdPin`]: https://andamira.github.io/devela/latest/devela/data/uid/struct.IdPin.html
  [`IdPinBox`]: https://andamira.github.io/devela/latest/devela/data/uid/struct.IdPinBox.html
[rawbytes]: https://crates.io/crates/rawbytes/1.0.0
  [`as_bytes`]: https://andamira.github.io/devela/latest/devela/sys/mem/struct.Mem.html#method.as_bytes
  [`as_bytes_mut`]: https://andamira.github.io/devela/latest/devela/sys/mem/struct.Mem.html#method.as_bytes_mut
[stdext]: https://crates.io/crates/stdext/0.3.3
  [M_stdext]: https://github.com/andamira/devela/blob/main/src/_doc/vendored/stdext.md
  [`compile_warn!`]: https://andamira.github.io/devela/latest/devela/code/util/macro.compile_warn.html
  [`fn_name!`]: https://andamira.github.io/devela/latest/devela/code/util/macro.fn_name.html
[transliteration]: https://github.com/yf-hk/transliteration
  [`scalar_as_ascii_translit`]: https://andamira.github.io/devela/latest/devela/text/char/fn.scalar_as_ascii_translit.html
[unsized-stack]: https://crates.io/crates/unsized-stack/0.2.0
  [`FatPtr`]: https://andamira.github.io/devela/latest/devela/sys/mem/struct.FatPtr.html

## Other compatible licenses
- <i id="8bit_rng"></i>[8bit_rng] by Edward Rosten, (BSD-2),
  as the [`Xyza8a`] and [`Xyza8b`] structs algorithms.
- <i id="apply"></i>[apply] by GeorgeBurton (Unlicense),
  as part of the [`Hook`] and [`Morph`] structs ([*modifications*][M_apply]).
- <i id="blit-fonts"></i>[blit-fonts] by Andrew Reece (ISC),
  as the [`FONT_BIT_3_5`] and [`FONT_BIT_5_6`] consts data<!-- ([*modifications*][M_blit-fonts]) -->.
- <i id="jiff"></i>[jiff] by Andrew Gallant (Unlicense),
  as the [`TimeDelta`] struct ([*modifications*][M_jiff]).
- <i id="pengyhash"></i>[pengyhash] by Alberto Fajardo (BSD-2),
  as the [`HasherPengy`] struct algorithm ([*modifications*][M_pengyhash]).
- <i id="size_of_trait"></i>[size_of_trait] by Joshua Nelson (BSD-3),
  as the [`size_of_expr!`] macro.
- <i id="Xabc"></i>[Xabc] by *EternityForest* (openly shared),
  as the [`Xabc`] struct algorithm.
- <i id="GraphicGems"></i>[Graphics Gems] (1985–1994) (permissive EULA),
  as various algorithms.

[8bit_rng]: https://github.com/edrosten/8bit_rng
  [`Xyza8a`]: https://andamira.github.io/devela/latest/devela/num/rand/struct.Xyza8a.html
  [`Xyza8b`]: https://andamira.github.io/devela/latest/devela/num/rand/struct.Xyza8b.html
[agg]: https://crates.io/crates/agg/0.1.0
[apply]: https://crates.io/crates/apply/0.3.0
  [M_apply]: https://github.com/andamira/devela/blob/main/src/_doc/vendored/apply.md
  [`Hook`]: https://andamira.github.io/devela/latest/devela/code/result/trait.Hook.html
  [`Morph`]: https://andamira.github.io/devela/latest/devela/code/result/trait.Morph.html
[blit-fonts]: https://github.com/azmr/blit-fonts
  <!-- [M_blit-fonts]: https://github.com/andamira/devela/blob/main/src/media/font/bitmap/MODS_BLIT.md -->
  [`FONT_BIT_3_5`]: https://andamira.github.io/devela/latest/devela/media/font/const.FONT_BIT_3_5.html
  [`FONT_BIT_5_6`]: https://andamira.github.io/devela/latest/devela/media/font/const.FONT_BIT_5_6.html
[jiff]: https://crates.io/crates/jiff/0.2.1
  [M_jiff]: https://github.com/andamira/devela/blob/main/src/_doc/vendored/jiff.md
  [`TimeDelta`]: https://andamira.github.io/devela/latest/devela/phys/time/struct.TimeDelta.html
[pengyhash]: https://github.com/tinypeng/pengyhash/blob/70a23e40a2be2e784a68078213b7675055f21949/pengyhash.c
  [M_pengyhash]: https://github.com/andamira/devela/blob/main/src/_doc/vendored/pengy.md
  [`HasherPengy`]: https://andamira.github.io/devela/latest/devela/data/codec/hash/struct.HasherPengy.html
[size_of_trait]: https://crates.io/crates/size-of-trait/1.1.3
  [`size_of_expr!`]: https://andamira.github.io/devela/latest/devela/sys/mem/macro.size_of_expr.html
[Xabc]: https://web.archive.org/web/20140328221846/https://www.electro-tech-online.com/threads/ultra-fast-pseudorandom-number-generator-for-8-bit.124249/
  [`Xabc`]: https://andamira.github.io/devela/latest/devela/num/rand/struct.Xabc.html
[Graphics Gems]: https://www.realtimerendering.com/resources/GraphicsGems/

<!-- WIPZONE -->
<!-- - <i id="agg"></i>[agg] by Brian Savage, (BSD-2), (TODO WIP) -->
<!--   as the [`Pixels`], [`Raster`] and [`Render`] structs. -->
<!-- - [bdf] by *meh* (WTFPL) as part of the [`Bdf`] struct. -->
