# Changelog from 2025

[0.25.0] 2026-03-11
===================

> A place for everything, and everything in its place.

```
This release changes the library's structure from a single crate to multiple crates,
in order to improve compile times while maintaining most of the gained cohesiveness.
Many feature gates are removed in order to make most features make always available.
```

## Key changes:
- core extraction: separate foundational types, traits, and macros into base crates.
- feature consolidation: removed feature gates, make functionality always available.
- text system overhaul: major improvements to scalar, string, and grapheme handling.
- const evolution: many methods made const across numeric, text, and system modules.
- build system enhancements: improved build configuration and post-build processing
- memory & system improvements: enhanced slice operations and system arches support.
- error system refinement: updated error macros and type organization.
- msrv bump: minimum supported rust version increased to 1.93.0.

 ------------------------------------------------------------------------------

# Project

## infra
### build
- define `CRATE_NAME` constant.
- move `/meta/build` to `/build/main`.
- move build fn utils as `Build` methods.
- new `Build` namespace in `devela_base_std`.
- make `devela_base_std` optional for builds.
- add rerun instructions for changed env vars.
- new internal env vars `__DEVELA_MEMBER`, `__DEVELA_MEMBER_NAME`.
- make sure `CARGO_TARGET_DIR` and `CARGO_WORKSPACE_DIR` are always defined.
- add new `devela_postbuild` crate to `build/post`.
- add build config flag aliases: `any_target_arch_linux`, `any_target_arch_riscv`.
- add [base] symlinks to `devela/main/[alias|environment|features].rs`.
- move `/config/dep_all.rs` to `/build/main/dep_all`.

### cargo
- bump MSRV to 1.93.0.
- add new cargo env var `CARGO_WORKSPACE_DIR`.
- fix updated syntax for unstable cargo-include in `.cargo/config.toml`.

#### config
- remove `./cargo/nightly.toml`.
- add cargo aliases: `L0r`, `w*` (workspace).

#### manifest
- add workspace hierarchy diagram.
- add *binaries* and *metrics* sections.
- add lints: `missing_debug_implementations`, `unused_features`.
- make keys parts of the workspace: edition, version, authors, license, documentation.
- simplify debugging info in `dev` profile.
- add `debugging` profile.

### tools
- new `x` workspace command wrapper.
- new files in `tools`: `x.fish`, `x.sh`, `x-env-common.sh`, `x-env-native.sh`, `x-env-nightly.sh`.
- remove `tools/cargo-native`.
- update `tools/check.rs`:
  - bump `devela` to 0.24.0.
  - test all workspace crates.
  - start testing without dependencies.
  - switch rust-script for cargo-script.
  - simplify and homogenize toolchain selection syntax.
  - configure the exact nightly version to install and use.

### CI
- bump `actions/checkout` to v5.
- add more `no_std` targets, retry downloads and disable fail-fast.

## workspace
- new `/crates/` directory.
- add `devela_sentinel` crate.
- remove the `game` root module.
- declare the `std` external crate.
- add `_reexports` structural modules.
- remove `_always` structural modules.
- add new root modules: `org`, `vita`.
- refactor all structural access modules.
- enable `_docsrs` for workspace dependencies.
- support having external optional dependencies.
- new workspace library crates: `devela_base_alloc`, `devela_base_core`, `devela_base_macros`, `devela_base_std`.
- prepare future workspace library crates related to root modules.
- use a single version, changelog and readme for all workspace libs.
  - move `devela_macros` changelog into `devela` archived changelog history.
  - replace `paste` dependency with `pastey` and move to [base].
- add flat re-exports of root modules to `zall_` & re-export hidden as `all_`.
- rename `all` to `zall` & re-export hidden as `all`.
- rename all `lib.rs` to `index.rs`.
- rename `_info` to `_doc`.
  - move `config/rustdoc-header.html` to `src/_doc/header.html`.
  - update `src/_doc/header.html` to support multiple crates with custom whitelists.

### [base]
- add `_workspace_internal` structural module (replacing `_internal`).

#### `devela_base_macros`
- move devela_macros macros: `devela_macros`: `cif!`, `compile!`, `compile_attr!`, `ident_total!`, `ident_total_unique!`, `ident_unique!`, `coalesce!`, `field_of!`.
- new macro: `repeat!`.
- new compiler predicates: `env`, `env_eq`, `env_ne`, `env_empty`, `env_nonempty`, `nota`.

### `devela_macros`
- use workspace's crate version.
- make it an optional dependency.
- add `devela_base_core` as a dependency.
- enable `doc_cfg` via `nightly_doc` flag.
- remove dependency `hashbrown`.
- remove features: `alloc`, `std`, `nightly`, `nightly_doc`.

## dependencies
- make all optional external optional dependencies part of the workspace.
- move `core`, `alloc` & `std` re-exports to `src/yard/_dep`.
- remove `_core` and `_dep` re-exports from public docs.
- move `_dep` to `yard/_dep` & re-export from the root.
- re-export hidden workspace dependencies from `_dep`.
- re-export `alloc` from devela and [base_alloc].
- bump dependencies:
  - `hashbrown` to 0.16.
  - `memchr` to 2.8.
  - `rand_core` to 0.10.
  - [macros]:
    - `proc-macro2` to 1.0.101.
    - `quote` to 1.0.45.
- remove dependencies:
  - `const-str`, and related `str!` macro.
  - `crossterm` and `miniquad`; move to revela.
  - `libm` and related `Float` and `ExtFloat` functionality.
  - itertools and related re-exports.
  - remove: `allocator-api2`, `bumpalo`, `fontdue`, `ffmpeg-the-third`, `flume`, `fontdue`,  `gilrs`, `image`, `kira`, `midir`, `rayon`, `regex-lite`, `rodio`, `sdl2`, `sdl3`, `stringzilla`, `symphonia`, `sysinfo`, `toml_edit`, `tokio`, `unicode-segmentation`, `unicode-width`, `ureq`, `winnow`.
- add optional dependencies to [base]: `memchr`, `rand_core`, `simdutf8`.

## features & flags
- new features: `__publish`, `__std`, `__docs_internal`, `_docs_examples`, `_docs_max`, `event`, `grapheme`, `int`, `org`, `safe_build`, `safe_org`, `safe_vita`, `translit`, `vita`, `x11`.
- remove features: `_bit*`, `_char*`, `_cmp*`, `_float_*`, `_int_*`, `_num?_all`, `_sort*`, `_str_*`, `_str_nonul`, `_str_u*`, `_text_all`, `ascii`, `cast`, `desk`, `error`, `fmt`, `join`, `metric`, `nightly_bigint`, `prim`, `safe_layout`, `split`, `str`.
- remove flags: `bit··`, `char··`, `cmp··`, `_float··`, `_int*··`, `_nums··`, `prim··`, `sort··`, `str··`, `str_u··`.
- add an additional `nightly_stable_1_??` flag for the 3rd next version.
- rename:
  - `_docs` to _`docs_min`.
  - `_docsrs` to `_docs`.
  - `_docsrs_nodep` to `_docs_nodep`.
  - `__no_test` to `__exclude_test`.
  - `linear` to `lin` and move from `geom` to `num`.
- add default feature `alloc` to [base_alloc].
- add default feature `std` to [base_std].

## metrics
- rename directory `/benches` to `/metrics`.

---

# Modules

## code
- rename `ExtAny` to `AnyExt`.
- new trait: `ConstInitCore`.
- new types: `CodeLocation`, `CodeSpan`.
  - implement for `NonZero*` and many other types.
- move to [base]:
  - all `ConstInit*` impls.
  - `impl_cdef!` workspace-internal macro.
    - modify it to receive the trait as an argument.
- move `ScopeGuard` to [base_core].

### code::error
- update `define_error!` macro.
  - move to `code::error`.
  - update docs, add example.
  - allow accepting multiple tags.
  - do not automatically derive `Default`.
  - make conversion method optional const.
- remove items: `AllError`, `AllResult`, `DataError`, `DataResult`, `ExtError`.
- update `ArrayFmt` to support the rest of the core formatting traits.

### code::marker
- make `TypeResource` and `type_marker!` constructors *const*.
- new traits: `Prim`, `PrimFitPtr`, `PrimIndex`.

### code::ops
- new macro: `punroll!`.
- new types: `CallSemantics`, `CallBindTime`, `CallContext`, `CallDispatch`, `CallOpenness`, `CallStorage`.

### code::panic
- move to [base]: `Panic`.

### code::result
- move to [base]:
  - functions: `serr`, `sok`.
  - macros: `unwrap!`.
  - traits: `Morph`, `Hook`, `OptionExt`, `OptResExt`, `ResultExt`.
  - types: `Mismatch`, `OptRes`, `OptionFmt`, `OptionFmtOr`, `OptionFmtOrElse`, `Own`.
- new macros: `hook!`, `morph!`.
- rename:
  - `Chain` to `Morph`.
  - `ExtOption` to `OptionExt`.
  - `ExtOptRes` to `OptResExt`.
  - `ExtResult` to `ResultExt`.
- update `unwrap!`:
  - add arms: `err?`, `err_map*`, `ok_err_map`, `ok_if*`, `ok_map*`, `ok_some_map`, `some_if*`, `some_map*`, `some_ok_map*`.
  - receive expect `$msg` args as `expr` instead of `literal`, to be compatible with `concat!`.
  - make *const* the arms that call `unreachable`.
  - rename arms:
    - `ok_map_err`? to `ok_err_map?`.
    - `ok_if_map_err`? to `ok_if_err_map?`.

### code::utils
- new macros: `compile_warn!`, `doc_location!`, `doclink!`, `fn_name!`, `lets!`, `mod_path!`, `repeat!`, `type_count!`, `whilst!`, `write_at!`.
- move to [base]:
  - public macros: `CONST!`, `assert_eq_all!`, `assert_approx_eq_all!`, `capture_first!`, `capture_last!`, `capture_tail_tuple!`, `cfg_if!`, `const_assert!`, `define_error!`, `deprecate!`, `enumset!`, `ident_const_index!`, `impl_trait!`, `include_from!`, `is!`, `items!`, `maybe!`, `methods_as_fns!`, `mod_from!`, `sf!`, , `structural_mods!`, `type_marker!`.
  - internal macros: `__crate_name!`, `__dbg!`, `__std!`, `_EMOJI_*`, `_TAG_*`, `_doc!`, `_doc_availability!`, `_doc_miri_warn!`,  `_reexport!`, `_tags!`, `_use!`.
- add tags: `_DOC_*`, `_TAG_[ALG|APPLE|ASSERT|AUDIO|BIT|CODE|CODEC|CODEGEN_BUILD|CONSTRUCTION|COLOR|CONCURRENCY|DATA|DEBUG|EVENT|EXAMPLE|EXEC|FS|GEOM_DIR|GUARD|HASH|ID|IMAGE|INIT|INTERACTION|INTERNAL|INTROSPECT|IO|LAYOUT|LIFETIME|LIN|LINUX|LIST|LOGIC|MAYBE|MEM|PLATFORM|PROC_MACRO|RUNTIME|SYMB|TERM|UNIX|VALUE|WAVE|WINDOWS|WIP]`.
- change the emoji for `_TAG_DATA_STRUCTURE`.
- new re-exports: `select_unpredictable`.
- new functions: `cold_path`, `likely`, `unlikely`.
- rename `reexport!` internal macro to `_reexport!`.
  - mark as `doc(no_inline)`.
  - allow accepting multiple tags.
  - fix rendering of std path links.
- prefix internal constants `TAG_*` & `EMOJI_*` with `_`.
- define `_std_core` separately and privately per crate.
- update `CONST!` macro with new arms: `hidden macro_export`, `inline macro_export`.
- update `impl_traits!` macro:
  - add new arms for: `Display+Error`, `FromStr`, `PartialEq`.
  - change syntax from a single <gen> group to a double [decl][args] group, to support const generics.
- update `const_assert!` macro:
  - add new arms: `ne_buf`, `ne_str`.
  - add support for comparing slices of primitives and slices of slices of primitives.
- update `is!` macro.
  - change expr separators from `;` to `,`, to reduce potential ambiguity.
  - remove temporary value binding functionality unnecessary after rust v1.89.
- remove vendored macro `cfor!`, replace with `whilst!`.
- remove deprecated `iif!` macro.

## data
- move to [base]:
  - macros: `bitfield!`, `init_array!`.
  - traits: `BitOps`.
  - types: `ArrayFrom`, `Bitwise`, `Oneof`, `Sort`.
- remove module `data::list`.
- new macro: `define_handle!`.
- new type: `HandleExample`.
- new `SortAlloc` wrapper for `Sort`.
- make `Sort` methods take `&mut self` instead of `self`.
- make `Sort` public `quick_*` methods take `&mut self` as well.

### data::access
- new module `data::access`.
- move here `address` & `iter`.

#### data::access::iter
- new traits: `IteratorLending`, `IteratorLendingDoubleEnded`, `IteratorLendingExactSize`, `IteratorLendingPeek`, `IteratorLendingPeekDoubleEnded`.
- new macro: `iter_strided!`.
- new types: `StridedIter`, `StridedIterMut`.

### data::codec
- move here: `bit/`.
- rename `serde/` to `deser/`.

#### data::codec::bit
- move `BitOps` & `Bitwise` to `num::bit`.
- make all `bitfield!` methods consts.
- make the module private.

#### data::codec::hash
  - move to [base]: `HasherFx`, `HasherBuildFx`.
  - new module `data::codec::hash::check`.
  - new type `Adler32`.

#### data::codec::radix
- move `Base` to [base].
- remove methods that allocate.

### data::error
- recreate the type `MismatchedCapacity`.
- remove type: `DataOverflow`.
- update `MismatchedBounds`.

### data::id
- new module `data::id`.
- move here `key/` and `uid/`.

#### data::id::key
- update `define_static_map!`:
  - support custom attributes and visibility.
  - add example items: `StaticMapConstU8Example`, `StaticMapTypeIdExample`, `StaticMapU16Example`.
  - improve docs.

#### data::id::uid
- move `IdPin` to [base].
- new type `IdRegistry`.

### data::layout
- new module `data::layout`.
- move here: `DataCollection`, `dst/`, `pool/`, `sort/`, `table/`.
- move here: `list/` submodules: `array/`, `buf/`→`buffer/`, `queue/`, `stack/`.

#### data::layout::array
- move to [base]:
  - traits: `ArrayExt`.
  - types: `ArrayFmt`, `ConstList`.
- rename:
  - `ExtArray` to `ArrayExt`.
  - `ExtVec` to `VecExt`.
- update `init_array!`:
  - rename `array_init!` to `init_array!`.
  - require `ConstInit` and `Vec` in scope if needed.
  - rename `const_init` arm related to traits to `INIT in`.
  - rename `const_init*` arms unrelated to traits to `const_fn*`.

#### data::layout::buffer
- new macros: `buffer_linear!`.
- new example types: `BufferExample`, `BuffeAllocExample`, `BufferViewExample`.

### data::topol
- new module `data::topol`.
- move here: `listlink/`→`linked/`.

### data::value
- move here: `NoData`, `of/`, `tuple/`.
- update `Oneof`:
  - new methods: `copy_*`.
  - remove methods: `variant_name`, `is_variant_name`, `first_non_unit`.
  - make methods const: `as_mut_*`, `as_ref_*`, `as_tuple_ref_options`.

## geom
- make `num::geom` a new `geom` root module.
- new modules: `affine/`, `rel/`, `space/`.
- rename `shape` to `fig`, as well as the related feature.

### geom::dir
- new module.
- move here `Orientation`, `Angle`, `AngleDirection` & `AngleKind`.
- new types: `Boundary1d`, `Boundary2d`, `Boundary3d`.

### geom::metric
- move to [base]:
  - internal macro `_impl_metric!`.
  - types: `Distance`, `Extent`, `Orientation`, `Position`, `Region`, `RegionStrided`, `Stride`.
- remove `metric` feature gate for `Orientation`, `Region` & `Stride`.
- update `_impl_metric!` helper macro.
  - implement 2d|3d common methods.
  - implement `ConstInit*`.
- update `Extent`:
  - add methods for 2|3d: `length[_ref|_mut]`, `width[_ref|_mut]`, `height[_ref|_mut]`, `depth[_ref|_mut]`, `breadth[_ref|_mut]`.
- remove `c_` prefix from int methods.

## lang
- rename `lang::ling` to `lang::hum`.
- rename `lang::ling::grammar` to `lang::hum::gram`.
- move `lang::i18n` to `lang::hum::i18n`.
- new modules: `disc`, `gram`, `prog`, `repr`, `sem`.
- move `ffi` to `prog::ffi`.

### lang::prog
- new module: `dsl`.

#### lang::prog::ffi
##### lang::prg::ffi::c
- new type aliases: `c_mode_t`, `c_off_t`.

##### lang::prg::ffi::js
- update `JsInstant`:
  - make method const: `delta_since`.
  - remove methods: `const_delta_since`.

## media
- new modules: `compo`, `doc`, `visual`.

### media::audio
- new types: `AudioChannel`, `AudioChannels`.

### media::font
- new type: `FontArt`.
- new const: `FONT_ART_3_4`.
- rename:
  - `BitmapFont` to `FontBitmap`.
  - `FONT_3_3` to `FONT_BIT_3_3`.
  - `FONT_3_5` to `FONT_BIT_3_5`.
  - `FONT_5_6` to `FONT_BIT_5_6`.

### media::visual
#### media::visual::color
- move to [base]:
  - types: `Gamma`, `Lum`, `Rgb`, `Rgba`.
  - aliases: `Lightness`, `LinearLightness`, `Luma`, `Luminance`, `Rgb8`, `Rgba8`, `RgbaPre8`, `Rgb16`, `Rgba16`, `RgbaPre16`, `RgbF32`, `RgbaF32`, `RgbaPreF32`, `RgbF64`, `RgbaF64`, `RgbaPreF64`, `RgbLinF32`, `RgbaLinF32`, `RgbaLinPreF32`, `RgbLinF64`, `RgbaLinF64`, `RgbaLinPreF64`.
- new type `GammaConst`.

#### media::visual::image
- add a new sixel implementation:
  - new types: `SixelChar`, `SixelColor`, `SixelEncoder`, `SixelPalette`, `SixelPaletteIter`.
- remove legacy vendored sixel implementation.

## num
- move to [base]:
  - all data, numeric, text & time error types.
  - types: `Cast`, `Int`, `True`.
  - traits: `NumConst`.
- make `Num*` traits depend on the `num` feature.
- move `num::unit` to `phys::unit`.
- make `num::error` public.

### num::dom
- new module `num::dom`.
- move here `num::float`, `num::frac`, `num::int`, `num::traits`.

#### num::dom::real
##### num::dom::real::float
- rename: `ExtFloat` to `FloatExt`.
- new types: `f32bits`, `f32bits_niche`, `f64bits`, `f64bits_niche`.
- update `Float`:
  - new methods: `poly_eval_const`, `sin_minimax`, `cos_minimax`, `sin_cos_minimax`.
  - remove deprecated methods: `const_signum`, `const_copysign`, `const_clamp`, `const_max`, `const_min`.
  - make std methods *const*: `fract`, `normalize`, `set_normalized`, `split`, `trunc`.
  - split out std-enabled implementation as internal `FloatStd`.
- Change `ExtFloat` to use `*_minimax` methods by default.
- move to [base]:
  - aliases: `fsize`.
  - traits: `FloatConst`.
  - types: `Float`.
  - float shared docs prefixed with `_FLOAT_`.

#### num::dom::int
- move to [base]: `Int`, `[iu]size_*`.
- prefix int shared docs with `_INT_`.
- new macros: `define_divisor!`.
- new types: `DivisorExample`, `IntAlloc`, `IntError`, `IntResult`.
- remove type: `Divisor`.
- make all `Int` methods *const*.

### num::fin
- new module `num::fin`.
- move here `num::bit`, `num::logic` and `num::ord`.

#### num::fin::bit
- new module `num::bit`.
- update `BitOps` & `Bitwise`.
  - rearrange methods in thematic impl blocks.
  - new methods: `[is_][un]set[_checked][_range]`, `[un]set_all`, `flip[_checked]`, `flip[_checked]_range_if`, `[is_][un]set_mask`.
- separate documentations for `BitOps` and `Bitwise` as individual constants.

#### num::fin::logic
- move to [base]: `ConstBool`, `False`, `True`, `const_bool!`, `[iu]size_*`.

#### num::fin::ord
- rename `Compare` to `Cmp`.
- new macro `cmp!`.
- update `Cmp`:
  - move to [base].
  - new impl for `Ordering`.
  - new methods: `minmax`, `pminmax`, `total_cmp`.
  - un-gate impls and many dependent const methods.

### num::grain
- new module `num::grain`.
- new macro `cast!`.
- new traits `PrimScalar`, `PrimInt`, `PrimSint`, `PrimUint`, `PrimFloat`.
- move inside: `num::cast`, `num::niche`, `num::wide`.
- fix `Cast` wrapping methods performance, and correctness for negative integers.

#### num::grain::niche
- new macros: `niche_prim!`, `nv!`.
- new types: `MaybeNiche`, `NonNiche`, `NicheValueError`.
- move to [base]: `NonExtreme*`, `NonValue*`, `ne!`, `nz!`.
- update macros: `ne`, `nv`, `nz`, adding lossy constructors.

#### num::grain::wide
- new module `num::grain::wide`.
- new macro: `define_lane!`.
- new internal helper macros.
- new example type `Lane4_i32Example`.
- support `nightly_simd` & `dep_wide` in [base_core].
- re-export some of `core::simd` types and traits.

### num::lin
- new module: `num::lin`.
- move here `geom::linear::{matrix, vector}`.

### num::prob
- new module `num::prob`.
- move here `num::rand`.

#### num::prob::rand
- move `num::prob::rand` to [base].
- rename `Lgc16` to `Lcg16`.
- rename `xorshift_custom!` to `define_xorshift!`.
- new macro: `define_pcg!`.
- new traits: `Rand`, `RandAlloc`, `RandStd`.
- new type: `Pcg32`.

### num::quant
- move to [base]: `Cycle`, `CycleCount`, `Interval`,  `Sign`.
- update `Interval`:
  - use individual `IncompatibleBounds` error.
  - add methods: `bound`, `bound_mut`, `bound_as_ref`.
- update `Sign`:
  - make part of `quant`.
  - rename variant `None` to `Zero`.
  - add methods: `eq`, `is_negative`, `is_positive`, `is_zero`, `is_nonzero`, `invert`, `same_direction`, `combine`, `pow`, `abs`, `neg_abs`, `fold`, `fold_slice`.
- move `ValueQuant` from `code::result` to `num::quant`.

### num::symb
- new module `num::symb`.

## org
- new `org` module.

## phys
- new modules: `phys::astro`

### phys::time
- new public module: `phys::time::source`.
- new types: `TimeFake`, `TimeFakeRef`.
- remove `TimeError` alias.
- remove `time` feature gate for `NoTime`, `Timecode`, `TimeDelta`, `TimeSplit`.
- rename `TimeGranularity` to `TimeScale`.
  - add `Ratio` variant.
  - add new variant aliases: `Mins`, `Secs`.
  - add new methods: `convert[_simulated]`, `is_fixed`, `new_ratio`, `some_ratio`, `to_ratio[_simulated]`.
- update `TimeSource`:
  - rename `granularity` method to `time_source` and return `TimeSource`.
  - change `MONOTONIC` from a const generic to the `time_is_monotonic` method.
  - new method: `time_is_absolute`.
  - rename methods: `now_*` to `time_now*`.
  - remove methods: `epoch_*`.
  - fix impl for `SystemInstant`.
- update `TimeDelta`:
  - make method const: `from_js`, `[div|mul]_f[32|64]`, `[try_]from_[millis|secs]_f[32|64]`.
  - remove methods: `const_from_js`, const_try_from_millis_f64.
- update `TimeSplitMilliNanoNorm`:
  - add method `from_duration`.
  - rename `from_duration` method to `from_duration_subsec`.
- update `WeekDay`: make all methods const.
- rename `UnixTimeI64` to `TimeUnixI64`, `UnixTimeU32` to `TimeUnixU32`.
  - make their `new` method const.

## run
- new `run` root module.

### run::regime
- renme `UiService` to `RunService` and move here.
- move capabilities from `ui::back::cap` to `run::setup::cap`.
  - rename `UiCap*` to `RunCap*`.

### run::time
- new items: `RunTick`.

## sys
### sys::arch
- new `Arch` methods: `cntvct`, `cycles`, `rdcycle`, `rdtsc`, `rdtscp`.
- new internal macro `ARCH!`.

### sys::device
- new module: `sys::device`.
- move `media::midi` to `sys::device::midi`.
- move `sys::sound` to `sys::device::audio`.

#### sys::device::display
- new module: `sys::device::display::x11`.
- new types: `XDisplay`, `XError`, `XEvent`, `XWindow`.

### sys::env
- vendor `argv` as `IterArgSOsRef` struct and `Env` method `args_os_ref`.

### sys::fs
- rename `ExtPath` to `PathExt`.

### sys::hw
- new module `sys::hw`.

### sys::io
- new `IoDuplex` trait.
- refactor the `sys::io` module.
- update `Io`: add `pipe` method.
- re-export `IoPipeReader`, `IoPipeWriter`.

### sys::log
- new type `LoggerStatic`.
- new macro `slog!`.
- rename `ExtLogger` to `LoggerExt`.

### sys::mem
- new alias: `MaybeByte`.
- new types: `ArenaExample`, `ArenaHandleExample`, `ArenaMarkExample`.
- new macros: `define_arena`.
- new submodules: `alloc`, `bound`, `layout`, `view`.
- move previous submodules:
  - `align`, `pin`, `ptr` inside `bound`.
  - `alloc`, `arena`, `storage` inside `alloc`.
  - `borrow`, `slice` inside `view`.
- move to [base]:
  - macros: `cswap!`.
  - traits: `MemAligned`.
  - types: `CacheAlign`, `Mem`, `Ptr`.
- rename `ExtMem` to `MemExt`.
- update `Mem`:
  - rename method `bytes_from_bits` to `bytes_from_bits_saturating`.
  - new method: `bytes_from_bits` with a faster impl.

#### sys::mem::cell
- rename `ExtCellOption` to `CellOptionExt`.

#### sys::mem::size
- move `ByteSized` and `size_of_expr!` to [base].

#### sys::mem::slice
- new macro `slice!`.
- new types: `SliceIter`, `SliceIterMut`.
- move to [base]:
  - macros: `const_join!`.
  - types: `Slice`.
- rename:
  - `join!` to `const_join!`.
  - `ExtSlice` to `SliceExt`.
- update `Slice`:
  - rename methods:
    - `copy_from_slice` to `copy`.
    - `trim_leading_bytes` to `trim_leading`.
    - `replace_leading_bytes` to `replace_leading`.
  - add new methods:
    - `range_to_inclusive*`.
    - `chunk[_mut]`, `chunks_exact[_mut]`.
    - `clone`, `copy_into`, `copy_str_into`, `copy_utf8_into`.
    - `trim_leading_keep`, `trim_leading_min_len`, `trim_trailing`, `trim_trailing_keep`, `trim_trailing_min_len`, `trim_edges_keep`, `trim_edges_min_len_left`, `trim_edges_min_len_right`.
  - add new `eq` methods for slices of slices of primitives and string slices.

### sys::os
- repurpose module to include operating supervisors.
- new `Libc` namespace.
- new modules: `sys::os::browser`, `sys::os::fd`.

#### sys::os::browser
- move `lang::ffi::js::web` to `sys::os::browser::web`.
- move `examples::lang::js_web*` to `examples::sys::web*`.

#### sys::os::linux
- new types `LinuxClock`, `LinuxTime`, `LinuxInstant`.
- new `Linux` methods: `clock_getres`, `clock_gettime`, `exit`.
- new `Linux` syscalls: `sys_clock_getres`, `sys_clock_gettime`.
- fix `Linux`-related warnings & avoid use of `transmute`.
- rename syscalls doc constants, prefix with `_DOC_`.
- update `LinuxError`:
  - new `Other` variant.
  - impl `Error`.
- improve `LinuxTimespec`.
  - impl `Display` and `ConstInit`.
  - rename method `with` to `try_with_duration` and make fallible.
  - add corresponding method `try_to_duration`.
  - add saturating methods to convert from/to `Duration`.
  - add corresponding conversions and methods from/to `TimeDelta`.

#### sys::os::term
- new module `sys::os::term`.
- feature-gate with `term`.
- rename `AnsiColor3b` to `AnsiColor3` and `AnsiColor8b` `AnsiColor8`.
- move to [base]:
  - types: `Ansi`, `AnsiColor3`, `AnsiColor8`, `TermSize`.
- change `Ansi::print*` methods to `ansi_print*` functions.
- new type: `AnsiColor`.
- update `Ansi`:
  - reverse the order of arguments in `CURSOR_MOVE*` to be columns first.
  - add methods: `COLOR_[BG|FG]_BRIGHT`, `CURSOR_MOVE`, `DEFAULT_[BG|FG]`, `MOUSE_X10_[ENABLE|DISABLE]`, `MOUSE_[NORMAL|TRACKING|UTF8]`, `MOUSE_SGR[_PIXELS]`, `strip_codes`.
  - rename current associated const items with a `_B` suffix.
  - add duplicated items with the old name returning a string slice or a `StringNonul`.
  - update digits formatting methods to use `Digits::write_digits10`.
  - modify `CURSOR_*` methods taking `u32` to take `u16`.
  - make all escape-sequence methods *const*.
  - fix codes related to alternate screen.
- update `ansi!`:
  - add new arms `p!` and `@p!` that auto-unwrap.
  - fix macro visibility.

## text
- new struct: `TextLut`.
- move to [base]:
  - traits: `NumToStr`.
  - types: `ByteSearch`, `Digits`, `GraphemeNonul`, `GraphemeU*`, `Str`, `StringNonul`, `StringU*`, `char7`, `char8`, `char16`.

### text::char
- new macro: `ch!`.
- new fn: `scalar_as_ascii_translit`.
- new types: `CharIter`, `charu`, `charu_niche`.
- new `char7` methods: `to_byte`, `to_str`, `try_from_u8_array`, `from_u8_array_unchecked`, `read_u8_slice`, `read_u8_slice_trunc`, `write_u8_slice`, `write_str`, `to_ascii_lowercase_array`, `eq_ignore_ascii_case`.
- new `char[7|8|16]` methods: `to_charu`, `try_from_charu`.
- new `TextLut` consts: `ASCII_BASE36_OFFSET`, `DIGITS_BASE36`, `DECIMAL_PAIRS`, `POWERS10`.
- mark `char[7|8|16]` as must_use.
- impl `ConstInit` for `char*`.
- rename `AsciiChar` to `CharAscii`.
- rename `char*` methods:
  - `to_u32` to `to_scalar`.
  - `*ascii_char*` to `*char_ascii*`.
- make `text::char` module public.
- move `text::ascii` to `text::char::ascii`.
- remove previously re-exported `IterChars`.
- update `Char`:
  - change `to_ascii_fold` to convert `Æ|Œ` to `E` & `æ|œ` to `e`.
  - remove deprecated methods: `len_to_utf8`, `utf8_?bytes_len`.
  - make it a tuple struct with a single a generic parameter.
  - add methods: `decode_surrogate_pair`, `has_overlong_encoding`, `has_valid_continuation`, `is_combining`, `is_combining_common`, `is_control`, `is_control_common`, `is_fullwidth`, `is_fullwidth_common`, `is_surrogate*`, `is_utf8_boundary`, `is_valid_code`, `to_char`, `to_char_lenient`, `to_char_unchecked`, `len_utf8_match`, `len_utf8_match_naive`.
  - rename methods:
    - `is_7bit` to `is_ascii`.
    - `is_valid` to `is_valid_scalar`.
    - `to_ascii_str` to `as_ascii`.
    - `to_ascii_str_unchecked` to `as_ascii_unchecked`.
    - `to_code*` to `to_scalar*`.
    - `utf8_len` to `len_utf8_unchecked`.
    - `utf8_len_checked` to `len_utf8`.
  - remove `utf8_bytes_` prefix from `Char<&[u8]>` methods.
  - add private consts: `CONT_MASK` `UTF8_CHAR_LEN`.
  - remove `code_` prefix from `Char<u32>` methods.
  - rename method `byte_len` to `len_bytes`.
  - modify all methods to take `self`.
  - return lengths as usize.
- update `UnicodeScalar`:
  - new methods: `as_ascii_translit`, `is_combining`, `is_combining_common`, `is_control`, `is_control_common`, `is_fullwidth`, `is_fullwidth_common`, `to_char`, `to_scalar`.
  - rename method `byte_len` to `len_bytes`.
  - reorder methods.

#### text::char::ascii
- rename `ASCII_TABLE` to `LUT_ASCII_CHARS` and make it a public *const*.
- rename `Ascii` to `Digits`.
- update `Digits`:
  - new const: `MAX_DIGITS_16`.
  - new methods: `count_digits16`, `digit_at_index10[_checked]`, `digit_at_index16[_checked]`, `digit_value_at_index10[_checked]`, `digit_value_at_index16[_checked]`, `digits16`, `write_digits10`, `write_digits10_fast`, `write_digits10_omit0`, `write_digits16`, `write_digits16_omit0`.
  - new private method: `digit_at_power16`.
  - rename const: `MAX_DIGITS` to `MAX_DIGITS_10` and make them of type `u8`.
  - rename methods:
    - `calc_digit` to `digit_at_power10` and make private.
    - `count_digits` to `count_digits10`.
    - `digits_*` to `digits10_*`.
    - `digits` to `digits10`.

### text::error
- re-export std's `FromUtf8Error`.

### text::fmt
- new macro: `fmtcat`.
- new trait: `DebugExt`.
- new types: `FmtNum`, `FmtNumConf`, `FmtNumGroup`, `FmtNumShape`, `FmtNumSign`, `FmtWriter`.
- re-export `FromFn` as `FmtFromFn`.
- move to [base]:
  - macros: `format_buf!`.
- remove vendored `numtoa` crate, `NumToStr` trait replaced with `Digits` struct.
- add method `Fmt::from_fn`.

### text::grapheme
- new types: `GraphemeKind`, `GraphemeScanner`.
- feature-bound all grapheme-related items.
- vendor `grapheme_machine` as items: `GraphemeBoundary`, `GraphemeMachine`, `GraphemePropCb`, `GraphemePropInCb`, `GraphemeProps`.
- impl `Grapheme` for scalar types.
- update the `Grapheme` trait:
  - add new methods: `grapheme_chars`, `grapheme_is_kind`, `grapheme_kind`, `grapheme_len_bytes`, `grapheme_len_chars`, `grapheme_len_utf8`.
- update `Grapheme[Nonul|U*]`:
  - remove methods: `to_cstring`.
  - make `new` method panic.
  - add new methods: `eq`, `[as|into]_string_[nonul|u8]`, `from_charu[_unchecked]`, `from_str`, `new_checked`.
  - make methods unsafe: `as_bytes_mut`, `as_str_mut`.
  - implement `PartialEq` and `Hash` manually.
  - implement `PartialEq` between string and grapheme types.
  - make all methods *const*.

### text::layout
- new module `text::layout`.
- new types: `TextCohesion`, `TextCursor`, `TextFit`,  `TextIndex`, `TextLayout`, `TextLayoutStep`, `TextSpan`, `TextSymbol`, `TextUnit`.

### text::str
- remove methods: `to_cstring`, from `String*`.
- make `chars` methods *const* when possible.
- add more impls of `PartialEq` against string slices.
- update `Str`:
  - add methods for returning substrings in compile time: `range*` `take*`, `*split*`.
  - remove method `from_boxed_utf8_unchecked`.
- common updates for `StringNonul` and `StringU`:
  - new methods: `eq`, `from_charu`, `from_charu_unchecked`, `from_str`, `from_str_truncate`, `from_str_unchecked`, `new_checked`.
  - impl `AsRef<&str>`, `Deref`, `Extend`, `FmtWrite`, `FromIterator`.
  - make methods unsafe: `as_bytes_mut`, `as_str_mut`.
  - make **all** methods *const*.
  - make `new` method panic.
  - rename methods `from_bytes*` to `from_array*`.
- update `StringNonul`:
  - new methods: `chars`, `char_count`.
  - rename methods `from_byte_array*` to `from_array*`.
  - impl `PartialEq`.
- update `StringU*`:
  - new methods: `pop_unchecked`, `push_charu`, `sanitize`.
  - modify methods:
  - `as_mut_str`: make safe.
  - `push_str`, make const, improve efficiency, update docs & examples.
  - `try_push_str*`, make const, return `Result<usize, usize>`, update docs & examples.
  - remove `AsMut` & `DerefMut` impls.
  - fix `TryFrom<&str>` impl.
  - impl `PartialEq`.
  - improve docs.
- add new method `repeat_into_slice` to `Str` & `StrExt`.
- improve method `repeat_into` for `Str` & `StrExt`.
- improve method `new_counter` for `Str`, `StrExt` & `StringExt`.

## ui
- remove modules: `ui::back`, `ui::front`.
- new modules: `ui::intent`, `ui::view`.

### ui::event
- new types: `DeviceId`, `Event`, `EventKind`, `EventQueue`, `EventTag`, `EventTarget`, `EventTimestampMode`, `EventWindow`, `KeyDead`, `WindowId`.
- change `EventPointer.pressure` field to be `f32bits_niche`.
- rename `time_stamp` fields to `timestamp`.
- derive `Eq` & `Hash` for all event types.
- implement `ConstInit` for all types.
- update Event:
  - new fields `count`, `processed`, `target`.
  - new methods: `mark_processed`, `mark_count`, `clear_count`, `finalize`, `count`, `tag`.
- update `Key` & `KeyFfi`:
  - rename `Period` variant to `Dot`.
  - add new `Dead` variant.
- update `KeyState`: add variant `Repeat`.
- update `EventTimestamp`:
  - manually impl `Debug` and `DebugExt`.
  - remove all inner unsafe.
  - make it support timestamps of 0 ms.
  - change inner representation to `f32bits_niche`.
  - add methods: `as_millis_f32_to_u32`, `as_millis_u32`, `from_millis_u32_as_f32`.
  - remove methods: `try_from_js`, `try_from_millis_f32`,  `try_from_millis_u32`, `try_from_secs_f32`.
- change `EventKeyFfi.timestamp` field to be `f32bits`.
- update `EventWindow`:
  - fix `Moved` variant to use `i32`.
  - new methods `is_[geometry|resize|move|focus|close|redraw|visibility|pointer_crossing|text_input|stream_signal]`, `[resize|move]_coords`.
  - new variants: `Entered`, `Left`, `Minimized`, `Maximized`, `Restored`, `FullscreenEntered`, `FullscreenExited`.

#### ui::event::key
- update `KeyPad`, add variant `Comma`.
- update `KeyMod`:
  - rename variant `IsoLevel3Shift` to `AltGr`.
  - remove variants: `LeftMeta`, `RightMeta`, `LeftHyper`, `RightHyper`.
- update `KeyMods`:
  - rename `ctrl` in methods to  `control`.
  - rename method `has_meta` to `has_super`.
  - add bits and methods for `IsoLevel5Shift`.
  - add getter and setter methods: `set_*`, `unset_*`.
  - remove old obsolete variants.
- remove `KeyAlpha`.
- update `Key` & `KeyFfi`:
  - rename `F(u8)` variant to `Fn(u8)`.
  - integrate `KeyAlpha`'s variants.
  - add new punctuation variants.
  - add new `Scancode(u16)` variant.

## vita
- new `vita` module.

## work
### work::future
- rename:
  - `ExtFuture` to `FutureExt`.
  - `ExtProcess` to `ProcessExt`.
    - `ProcessExt::id` to `self_pid`.

### work::process
- new macro: `cmd!`.
- new trait: `OutputExt`.
- new types: `CommandFlow`, `ExitStatusError`.
- rename re-exported types back by removing the `Process` prefix, except for `Child*`→`Process*` renames and `ProcessTermination`.

### work::sync
- move `portable-atomic-util` dependent re-exports to [base_alloc]: `Arc`, `ArkWeak`.

## yard
- new internal root module `yard`.
- new workspace-internal macro `_devela_policy!`.
- move here most private macros from `code::util`.

[0.25.0]: https://github.com/andamira/devela/releases/tag/v0.25.0

[0.24.0] 2025-08-15
===================

-----------
**Project**

## crate
- bump MSRV to 1.89.0.
- bump dependencies:
  - `allocator-api2` → 0.3.
  - `bumpalo` → 3.18.
  - `bytemuck` → 1.23.
  - `crossterm` → 0.29.
  - `libm` → 2.15.
  - `pyo3` → 0.25.1.
  - `rodio` → 0.21.1.
  - `sdl2` → 0.38.
  - `sysinfo` → 0.36.
  - `toml_edit` → 0.23.
  - `tokio` → 1.47.
- disable dependencies: `sdl3`.
- remove `std` requirement for optional dependencies: `crossterm`, `pyo3`, `regex_lite`, `sysinfo`.

## documentation
- change documentation URL to repository's custom build to avoid docs.rs surprise bugs.
- improve rustdoc header loading, make loading more reliable.
- rename `DOCS/` → `docs/` and make its contents lowercase.
- add the `no-deps` key to docs.rs package metadata.
- fix multiple katex warnings.
- new doc tag: `TAG_NICHE`.

## examples
- new `js_web_worker` example.
- delete the `niche` example.

## flags
- rename flags: `nightly_stable_next*` → `nightly_stable_1_**` to indicate the exact versions.
- add flags:
  - `nightly_become` enabling `explicit_tail_calls` feature (commented out).
  - `nightly_unstable` to group the non-`nightly_stable` features.
- disable `nightly_autodiff` flag.

## libs
- move `devela_macros` code to `devela/libs/macros`.

## meta
- rename `/build/` → `/meta/`.
- rename `/build/generate/` → `/meta/codegen/`.

## tools
- rename `utils` → `tools`.
- change `check.rs`:
  - add new args: `-A` | `--install-arches`, `-N` | `--install-nightly`
  - behavior change: no longer installs components automatically.
  - bump dependencies; fix warnings; refactor.

-----------
**Modules**

## code
- new trait `Introspect`.
- new doc tag: `TAG_ALLOCATOR`.
- rename `set_panic_handler!`'s macro `web_api` arm → `web`.

### result
- add re-exports: `core::option::*`, `core::result::*`.

## data
- add re-exports: `IterFromCoroutine`.
- add modules: `data::list::of`.
- relocate and rename `code::result::Enum` → `data::list::of::Oneof`.
- rename `Oneof` methods → ordinals: `A` → `_0`, `B` → `_1`, ….

## game
- add:
  - module: `game`.
  - features: `game`, `game_safe`.
  - reflection flags: `game··`.

## lang
### ffi
- new types: `Js`, `JsConsole`, `JsValue`, `WebDocument`, `WebWindow`, `WebWindowState`.
- move and rename the `js_str*` fns as public `Js` `read_str*` methods.
- new `Web` methods:
  - `console_count[_reset]`.
- rename types:
  - `Js` → `Web`.
  - `JsEvent*` → `WebEvent*`.
  - `JsPermission*` → `WebPermission*`.
  - `JsWorker*` → `WebWorker*`.

## media
- delete: `[Audio|Color|Draw|Font|Media|Midi][Result|Error]`.

### color
- add types: `Gamma`, `Lum`, `Rgb`, `Rgba`.
  - implement for `u8`, `u16`, `f32`, `f64`.
- add aliases for different `Rgb` color types.
- add `Lum` sub-type aliases: `Lightness`, `LinearLightness`, `Luma`, `Luminance`.
- add module: `media::color::rgb`.
- remove the `Color` namespace.
  - move all its functionality to `Gamma`.
- rename the `ColorBase` trait → `Color`.
- update the `Color` trait:
  - make type `Component` bound on `NumConst`.
  - add constants: `COLOR_[BITS|COUNT|HAS_ALPHA]`, `COLOR_IS_[INT|LINEAR|PREMUL]`.
  - add methods: `color_[bits|has_alpha]`, `color_[red|green|blue|alpha]`, `color_is_[int|linear|premul]`.

## num
- update the `NumConst` trait.
- require the trait bound `PartialEq<Self::Num>`.
- make all its associated constant values be `Option`al.
- add consts: `NUM_[MAX|MIN][_NORM]`, `NUM_IS_[BIG|INT|FLOAT|FIXED|SIGNED|NICHE]`.
- add auto-implemented methods over `&self`, to query the associated constant values.

### float
- add new `Float` methods: `classify`, `next_down`, `next_up`.
- delete the `alg` feature.

### geom
#### metric
- rename `Extent` field `size` → `dim` for consitency.
- add missing attributes `must_use` and `repr(transparent)`.
- remove type aliases: `Extent2d`, `Extent3d`, `Region2d`, `Region3d`.
- remove `metric` feature-gate for `Distance`, `Extent` and `Position`.
- impl `From` arrays and tuples for `Distance`, `Extent`, `Orientation`, `Position` and `Stride`.

#### shape
- update `Point:` make mut accesors *const*.
- add new alias: `Points2d`.
- add new method: `Points::new`.

### quant
- new macro: `interval!`.

### niche
- add new:
  - macros: `ne!`, `nz!`.
  - types: `Non[Extreme|Value][I|U]size`.
  - methods to `NonValue*`: `new_lossy`.
- make `impl_non_value!` private.
- pre-generate all `NonValue*` types.
- improve the efficiency of `NonValue*<MAX>`.

## sys

### arch
- new `Wasm` methods: `heap_base`, `remaining_memory`.
- change `Wasm::memory_grow` to have the same signature as `core::arch::wasm32::memory_grow`.
- add support for new:
  - architectures: `amdgpu`.
  - OS targets: `amdhsa`, `cygwin`, `psx`.
  - vendor targets: `amd`, `mti`, `openwrt`, `vex`.

### mem
- new `Mem` methods: `align_down`, `align_up`, `is_aligned`, `is_aligned_to`.
- new `Ptr` methods `nn_*` to construct `PtrNonNull`.
- remove macros: `addr_of!`, `addr_of_mut!`.

#### alloc
- new types: `BumpAlloc`, `WasmAlloc`.
- vendor `mini-alloc` as `WasmAlloc`.

#### borrow
- new types: `Backing`, `MaybeOwned`.
- new trait: `Ownership`.

### os
#### linux
- add methods:
  - `Linux`: `print[ln]_unchecked[_fast]`, `eprint_bytes`.

## ui
### layout
- delete:
  - types: `LayoutError`, `LayoutResult`.

## work
### process
- new `ExtProcess` method `command`.


[0.23.0] 2025-03-02
===================

### Added
- new features:
  - lang: `glsl`, `js`.
  - num: `linear`, `metric`, `shape`.
  - ui: `desk`, `term`, `web`.
  - capability: `_maxest`, `_value_all`, `_value[8|16|32|64|128|256|512|1024]`.
  - safety: `unsafe_ffi`.
- new flags: `ffi··`, `geom··`.
- new traits:
  - data:
    - codec: `Decodable`, `Encodable`, `EncodabeLen`.
    - table: `DataValue[Copy]`, `DataType[Copy]`, `DataRaw[Copy]`.
  - num: `NumConst`.
  - sys: `AppEnv`, `ExtLog`.
  - ui: `MiniquadEventHandlerExt`, `UiService`.
  - work: `ExtProcess`.
- new consts:
  - media::font: `FONT_3_3`, `FONT_3_5`, `FONT_5_6`.
  - sys::os::linux (namespaced): `LiNUX_EXIT`, `LINUX_O_FLAGS`, `LINUX_S_IFMT`, `LINUX_SEEK`, `LINUX_F_CMD`.
  - `Float`|`FloatConst`: `EXPONENT_BIAS`, `EXPONENT_BITS`, `SIGNIFICANT_BITS`.
  - `AngleDirection::{CounterClockwise, CCW, RightHandRule, RHR, Clockwise, CW, LeftHandRule, LHR}`.
- new types:
  - code: `Enum`, `ScopeGuard`, `TimeError`, `Timeout`.
  - data: `ArrayFrom`, `NoData`.
    - codec: `CodecBe`, `CodecLe`, `CodecIf`, `CodecJoin`, `CodecFlags`, `CodecLen`, `CodecLenValue`.
      - radix `Base`, `Crockford`, `Rfc4648`, `Rfc4648Hex`.
    - key: `StaticMapEntry`.
    - table: `DataValue*`, `DataType*`, `DataRaw*`.
  - lang: `g_*`, `js_*`, `JsEventKind`, `JsEventMouse`, `JsEventPointer`, `JsInstant`, `JsKeyLocation`, `JsPermission`, `JsPermissionState`, `JsTextMetrics`, `JsTextMetricsFull`, `JsTimeout`, `JsWorker`, `JsWorkerError`, `JsWorkerJob`.
  - media: `BitmapFont`, `Sixel`, `Dither`, `PixelFormat`, `SixelError`, `SixelMean`, `SixelQuality`, `SixelSplit`.
  - num:
    - geom:
      - linear: `Matrix`.
      - metric: `Distance`, `Orientation`, `Position`, `Region`, `Stride`, `RegionStrided`.
    - quant: `Cycle`, `CycleCount`, `Ratio`.
  - phys: `TimeDelta`, `TimeGranularity`, `TimeSource`, `TimeSourceFake`.
  - sys:
    - env::app: `AppApple`, `AppConfig`, `AppWindows`, `AppUnix`, `AppXdg`,
    - io: `IoEmpty`, `IoRepeat`.
    - log: `LogConfig`.
    - mem: `Current`, `CurrentGuard`, `SpinLock`, `SpinLockGuard`.
    - os::linux: `LinuxError`, `LinuxResult`, `LinuxSiginfo`, `LinuxStat`.
  - ui: `UiCap`, `UiCapImage`, `UiCapInput`, `UiCapSound`, `UiCapSystem`, `UiCapWindow`.
    - event: `EventButton`, `EventButtonState`, `EventMouse`, `EventPointer`, `EventPointerType`, `EventTimestamp`, `EventKey`,
      - key: `KeyAlpha`, `KeyMedia`, `KeyMod`, `KeyMods`, `KeyPad`, `KeyState`.
    - back: `CrosstermService`, `MiniquadPixels`, `MiniquadService`.
    - front::term: `TermSize`.
  - work: `SleepSpin`.
  - namespaces: `Ansi`, `Fmt`, `Fs`, `FsPath`, `Io`, `Iter`, `Js`, `Linux`, `Log`, `Panic`.
- new macros:
  - `define_static_map!`, `is`, `methods_as_fns!`.
  - `join!`, `js_reexport`, `maybe!`, `miniquad!`, `xorshift_custom!`.
  - `ansi!`, `os_print`, `os_println`, `os_eprint`, `os_eprintln`.
  - `linux_entry`, `set_panic_handler!`.
- new modules:
  - data: `{codec::{self, crypto, radix}, list, key, table, uid}`.
  - lang: `{dsl, ffi::{self, c, glsl}, i18n, ling::{art, nat}}`.
  - media: `{image::sixel, video}`.
  - num: `{geom::metric, ord, quant}`.
  - phys: `{bio, chem, elec, mech, unit}`.
  - sys: `{log, net, fs}`.
  - ui: `{back::{self, crossterm, miniquad}, front}`.
  - work: `sync::mpsc`.
- new macro arms:
  - `format_buf!`: `?`.
  - `str!`: `ip_addr`.
  - `unwrap!`: `ok_err`, `*_guaranteed_or_ub`.
- new methods:
  - `Char`: `len_utf8`, `code_len_utf8[_unchecked]`,`code_to_utf8_bytes[_unchecked]`, `[code_]to_ascii_str[_unchecked]`, `is_valid`, `to_ascii_fold[_unchecked]`, `utf8_bytes_to_code[_unchecked]`, `utf8_len[_checked]`.
  - `Env::*`.
  - `ExtAny`: `type_hash`, `type_hash_with`.
  - `ExtFloat`: `sqrt_nr`.
  - `ExtFuture`: `pending`, `poll_fn`, `ready`.
  - `ExtThread`: `sleep_ms`, `sleep_us`, `sleep_ns`.
  - `Float`: `midpoint`, `recip`, `sqrt_hybrid`, `to_degrees`, `to_radians`.
  - `HasherFx`: `hash_bytes_with_seed`.
  - `IoError`: `other`.
  - `Linux` syscalls: `sys_[open|close|lseek|dup|dup2|fcntl|stat|fstat|getdents|pipe|pipe2]`.
  - `Slice`: `copy_from_slice`, `<u8>::copy_array`, `<u8>::copy_array_at`, `<u8>::copied_array_at`, `<u8>::to_array`.
  - `Str`: `__utf8_bytes_to_str`.
  - prngs: `from_state`, `inner_state`.
- new variants:
  - `IoErrorKind:` `OutOfMemory`, `FilesystemLoop`, `FilesystemQuotaExceeded`, `CrossesDevices`, `InvalidFilename`, `InProgress`.
- new re-exports:
  - core: `any::type_name` as `any_type_name`,
  - lang: `c_void`.
  - std:
    - `alloc::System` as `SystemAlloc`.
    - env: `*`.
    - io: `IoEmpty`, `IoIntoInnerError`, `IoRepeat`, `Std[err|in|out][Lock]`.
    - panic: `PanicHookInfo`.
    - process: `*`.
    - sync: `LazyLock`, `mpsc::*`.
  - log: `*`.
- new optional dependencies: `ffmpeg-the-third`, `fltk`, `flume`, `fontdue`, `gilrs`, `image`, `itertools`, `orion`, `ring`, `sdl2`, `sdl3`, `simdutf8`, `toml_edit`, `ureq`.
- new profile: `wasm`.
- new example: `js_web_api`.
- add musl architectures to `check.rs` script.
- add docs for monitored nightly features and for disabled dependencies.
- add more doc tags: `TAG_[FAKE|FFI|FMT|GEOM|NO|NUM|QUANT|RAND|TEXT|TIME]`.
- add more compile targets for docs.rs.

### Removed
- remove standalone re-exported fns from `std::{fmt, iter}`.
- remove items:
  - sys::os::linux: `LinuxTerminal`, `LinuxTerminalSize`.
- remove standalone fns:
  - io: `io_*`.
  - panic: `panic_*`.
  - sys::os::linux: `linux_*`.
  - text: `crate_root`, `crate_root_string`.
  - work: `future_block`, `future_pending`, `future_ready`.
- remove methods:
  - `Float:` `const_round_ties_odd`.
- remove private variant `IoErrorKind::Uncategorized`.
- remove features: `linux_deps`, `unsafe_async`.
- remove modules:
  - `data::collections`.
  - `num::alg`.
- disable optional dependencies: `fltk`, `js-sys`, `nc`, `ring`, `rustix`, `rkyv`, `tinyaudio`, `tracing`, `wasm-bindgen`, `web-sys`.
- deprecate:
  - `iif!`.
  - `Char::len_to_utf8`.
  - `Float::{const_[clamp|max|min|signum|copysign]}`.

### Changed
- bump MSRV to 1.85.0.
- rename features:
  - `_docs_max` to `_max`, `_docs_min` to `_docs`, `_string_*` to `_str_*`.
- change features into cfg flags:
  - `nightly, [nightly_[allocator|autodiff|bigint|coro|doc|float|simd|stable|stable_[next1|next2|later]`.
- rename flags:
  - `prim···` flag to `prim··`, `_str*` to `_str*`.
- rename/move modules:
  - `code::result::{error, panic}` inside `code`.
  - `data::collections::{array, destaque, list, stack, vec}` inside `data::list`.
  - `data::{bit, hash, serde}` inside `data::codec`.
  - `data::error` inside `code::error`, make private.
  - `num::cmp` to `num::ord`.
  - `num::alg::linear` to `num::geom::linear`.
  - `num::geom::shape::{angle, extent}` inside `num::geom::metric`.
  - `num::wave` to `phys::wave`.
  - `sys::path` inside `sys::fs`, make private.
  - `work::async` to `work::future`.
- rename/move items:
  - `ExtFloatConst` to `FloatConst`.
  - `LoggerConfig` to `LogConfig`.
  - `TextWrite` trait to `FmtWrite`.
  - `Coro` to `CoroWorker`, `CoroYield` to `CoroWork`, `CoroRun` to `CoroManager`.
  - re-exports: `Layout` to `MemLayout`, `LayoutError` to `MemLayoutError`.
  - `impl_error!` to `define_error!`.
- rename examples:
  - `coro_run` to `coro_manager`.
- rename variants:
  - `AngleDirection`: `CounterClockwise` to `Positive`, `Clockwise` to `Negative`.
- rename constants:
  - `LINUX_SIGACTION`: remove `SA_` prefix.
  - `LINUX_SIGNAL`: remove `SIG` prefix.
- rename/move fns/methods:
  - from prngs: `next_state` method to `peek_next_state`.
  - `fmt_write`, `fmt_format` and `format_buf_args` to `Fmt::{write, format, format_buf`, respectively.
  - `bytes_from_bits` fn to `Mem::bytes_from_bits`.
  - `ExtThread::parallelism` to `available_parallelism`.
- deprecate fns/methods:
  - `Char`: `utf8_2bytes_len`, `utf8_3bytes_len`, `utf8_4bytes_len`.
- modify methods:
  - `LinuxSigaction`: `new` method has an additional `restorer` argument.
- remove feature-gates:
  - `hash` for: `FxHasher`.
  - `io` for: `IoError`, `IoErrorKind`, `IoRead`, `IoWrite`, `IoBytes`, `IoChain`, `IoTake`.
  - `rand` for: `Xorshift128p`.
  - `str` for: `Str`.
- auto-enable features:
  - `str`: when enabling `_str_u*`.
- make customizable: `XorShift[16|32|64]`.
- make const methods:
  - `Float`: `clamp`, `copysign`, `div_euclid`, `max`, `min`, `min_total`, `round_ties_odd`, `signum`.
  - `IoBufReader`: `buffer`, `capacity`, `get_ref`, `get_mut`, `new`.
  - `IoChain`: `get_ref`, `get_mut`, `new`.
  - `IoCursor`: `get_ref`, `get_mut`, `new`, `position`, `set_position`.
  - `Mem`: `swap`.
  - `Ptr`: `copy`, `copy_nonoverlapping`, `replace`, `write`, `write_bytes`, `write_unaligned`.
- change attributes:
  - mark `Float` and `Sign` as `must_use`.
- derive Copy for `Lgc16`.
- update `str!` macro docs, tests and syntax: remove `:` suffix.
- make modules public:
  - `data::error`.
  - `num::geom::shape`.
  - `sys::env`.
  - `work::{future, process, sync::atomic}`.
- change `msvc` windows target for `gnu`.
- update scripts:
  - `utils/check.rs`:
    - add target components for the nightly toolchain.
    - add new fn `run_cargo_with_env`, sharing most of the logic with `run_cargo`.
    - enable `__dbg` feature & `-Ctarget-cpu=native`.
  - `build/features.rs`: support cfg flags auto-enabling other flags. Improve docs.
- improve the `.rustfmt.toml` config file.
- improve the docs for vendored items.

### Fixed
- improve build script debug output.
- make `array_init` use absolute paths internally.
- enable nightly features depending on `alloc` and `std`.
- fix and improve `Float` methods: `[cos|sin|tan]_series`.
- feature-gate:
  - `ExtFuture::block`.
  - namespaced re-exported unsafe methods with `unsafe··`.
- refactor `rustdoc-header.html` to be modular, more efficient and versatile.
- compile in docs.rs with `cpu-native` flag.

[0.22.1] 2025-01-13
===================

- fix docs compilation.


[0.22.0] 2025-01-13
===================

### Added

#### New features & flags
- new features for:
  - code: `code`, `error`, `_unroll`, `_unroll_128`, `_unroll_256`, `_unroll_512`, `_unroll_1024`, `_unroll_2048`.
  - data: `hash`.
  - doc: `_docsrs[_stable]_nodep`.
  - num: `alg`, `geom`, `prim`, `cast`, `join`, `split`, `unit`, `wave`, `_cmp_f16`, `_cmp_f128`, `_float_f16`, `_float_f128`.
  - media `media`, `safe_media`, `audio`, `color`, `draw`, `font`, `image`, `midi`.
  - sys: `time`, `linux`, `dep_linux`, `unsafe_syscall`.
  - text: `ascii`, `fmt`, `str`, `_char7`, `_char8`, `_char16`.
  - nightly: `nightly_alloc`, `nightly_autodiff`, `nightly_bigint`, `nightly_float`, `nightly_stable_next1`, `nightly_stable_next2`, `nightly_stable_later`.
  - safety: `safe_audio`, `safe_color`, `safe_draw`, `safe_ffi`, `safe_font`, `safe_image`, `safe_io`, `safe_layout`, `safe_ui`, `safest`, `unsafe_async`.
  - ui: `ui`, `layout`.
  - other: `alloc_deps`, `lang`, `windows`, `__force_miri_dst`.
- new cfg flags: `cargo_primary_package`, `*·`, `_*··`.

#### New items
- new structs:
  - `Interval`, `Pnm`.
  - new namespaces: `Alloc`, `Arch`, `ByteSearch`, `Char`, `Env`, `Mem`, `Ptr`, `Str`.
  - new standalone error types: `FailedErrorConversion`, `DataNotEnough`, `NotImplemented`, `NotSupported`, `ElementNotFound`, `InvalidAxisLength`, `KeyAlreadyExists`, `MismatchedCapacity`, `MismatchedDimensions`, `MismatchedIndices`, `NodeEmpty`, `NodeLinkNotSet`, `NodeLinkNotUnique`, `NotEnoughElements`, `NotEnoughSpace`, `IndexOutOfBounds`, `DataOverflow`, `PartiallyAdded`, `InvalidChar`, `InvalidUtf8`, `SystemTimeError`.
  - new composite error types: `NotAvailable`, `DataNotEnough`, `MismatchedBounds`, `PartialSpace`.
  - `False`, `True`, `UnitBi`, `UnitSi`.
  - `HasherPengy`.
  - `Lgc16`.
  - `TimeSplit`.
  - `TypeResource`.
  - `WaveletHaar`, `WaveletUnitVec`.
- new aliases:
  - `AllocMapFx`, `AllocSetFx`, `FmtResult`, `NoTime`.
  - `MediaResult`, `AudioResult`, `ColorResult`, `DrawResult`,`FontResult`, `ImageResult`.
  - `UiResult`, `LayoutResult`.
  - re-exported primitives: `char`.
  - `TimeSplitYearNano`, `TimeSplitYearDay`, `TimeSplitYearSec`, `TimeSplitHourSec`, `TimeSplitHourNano`, `TimeSplitMilliNano`.
- new enums:
  - `CompressionMode`, `EncodingMode`, `WaveletUnitRole`.
  - `AllError`, `AllErrorKind`, `MediaError`, `ColorError`, `AudioError`, `DrawError`, `FontError`, `ImageError`, `UiError`, `LayoutError`.
- new enum variants:
  - `DataError::ElementNotFound`.
- new traits:
  - `ColorBase`, `ExtCellOption`, `ExtError`, `ExtOptRes`, `ExtPath`, `ExtThread`, `MemPod`, `TypeResourced`, `Unit`.
  - `WaveletCompressionVec`, `WaveletTransformVec`.
- new associated methods and constants for:
  - `Array`: `from_fn`, `contains_[from|to|between]`, `as_bare_mut_slice`, `get[_mut]`.
  - `Array`, when storing `Option<T>`.
  - `Array2d`: `cap_col`, `cap_row`, `cap_major`, `cap_minor`, `num_major`, `num_minor`.
  - `BareBox`.
  - `ExtAny` method `type_id`.
  - `Float` and `ExtFloat`:
    - `FRAC_1_SQRT_2PI`.
    - `eval_poly` to evaluate polynomials.
    - for calculus: `derivative`, `integrate`, `partial_derivative_[x|y]`.
  - `Float` and `ExtFloatConst` consts: `LOW_MARGIN`, `MEDIUM_MARGIN`, `HIGH_MARGIN`.
  - `Graph` methods: `edge_exists_unchecked`, `edge_remove`.
  - `NonValue*`: `is_max`, `is_min`, `[checked|strict|saturating|wrapping]_[add|sub]`.
  - `Slice`: `eq`, `from_mut`, `from_ref`, `from_raw_parts`, `from_raw_parts_mut`, `range[_mut][_checked]`, `range_from[_mut][_checked]`, `range_to[_mut][_checked]`, `take_first[_mut][_checked]`, `take_last[_mut][_checked]`, `take_omit_last[_mut][_checked]`.
  - `UnicodeScalar` & `char*` types: `MIN`.
- new macros:
  - `capture_first!`, `capture_last!`, `capture_tail_tuple!`, `const_bool!`, `field_of!`, `id_seq!`, `impl_non_value!`, `impl_trait!`, `include_from!`, `mod_from!`, `str!`, `type_marker!`, `type_resource!`, `unroll`.
  - private: `doc_availability!`, `impl_error`, `EMOJI_*`, `TAG_*`.
- new vendored items
  - structs: `CacheAlign`, `ConstList`, `FatPtr`, `IdPinBox`, `IdPin`, `VecChunk`.
  - macros: `cfg_if!`, `const_assert!`.
  - traits: `ConstBool`.
- new optional dependencies:
  - `allocator-api2`, `bumpalo`, `crossterm`, `jiff`, `js-sys`, `kira`, `midir`, `nc`, `pyo3`, `rayon`, `regex-lite`, `rkyv`, `rodio`, `rustix`, `safe_arch`, `serde`, `stringzilla`, `symphonia`, `sysinfo`, `tinyaudio`, `tokio`, `tracing`, `wasm-bindgen`, `web-sys`, `winnow`.
- new re-exported:
  - items from: `alloc::alloc`,`core::{cell, num, ops, result}`, `std::{backtrace, fmt, path}`.
  - fns: `array_from_fn`, `array_from_mut`, `array_from_ref`.
  - macros:
    - `assert_unchecked!`, `autodiff`, `compile_error!`, `concat`, `enumint!`, `format!`, `format_args!`, `option_env!`, `stringify`, `thread_local!`, `write!`, `writeln!`.
    - wrapped: `env!`, as `env_!`, `vec!` as `vec_!`.
  - structs:
    - `NonZero`, `Saturating`, `Wrapping`, `OsStr`, `OsString`.
    - `HashMapEntry` and `BTreeMapEntry`.
    - `HashMap` and `BTreeMap` from `std` if `hashbrown` is disabled.
    - `FromStr`, `IterChars`.
  - crate items from multiple related modules, like errors and strings.
- new modules: `num::alg`, `sys::sound`, `media::{audio, color, draw, font, image, layout}`, `phys`, `ui`.
- new `sys::os::linux` module and example `linux`.
- new `NonValue*` constants `MAX`, `MIN`.
- new lints.

#### Examples, utilities, manifest, files
- new examples:
  - `id_pin`.
  - `id_seq` and type `ExampleIdSeqUsize`.
  - `enumint` and type `ExampleEnumIntU8`.
- manifest:
  - add `patches` section.
  - add table of contents.
  - add profiles: `dev-lto`, `release-lto`.
- new `.clippy.toml` configuration file.
- new github workflows: `get_errno.yml`, `get_syscall.yml`.
- new scripts in `utils/`: `cargo-native`, `manifest.sh`, `release_dates.rs`, `get_errno.sh`, `get_syscall.sh`, `docs_coverage.sh`, `docs_items.rs`.
- new convenience fn: `manifest_dir` in `build::utils`.
- hide `no_inline` items re-exports.
- new file `DOCS/VENDORED_rustdoc.md`
- show docs for the build scripts (private).
- add system of internal structural modules.
- rustdoc html header:
  - load katexextensions `mchem` and `copy-tex`.
  - trust `\href` commands.
  - move to `config/`.
- new directory `config/`.
  - put here a copy of `Cargo.toml::dep_all`.

### Removed
- remove custom no_std `Error` definition.
- remove items: `NeverOk`, `NeverErr`, `HasherFx32`, `HasherFx64`.
- remove types: `char24`, `char32`, `InRange*`, `NonRange*`, `HourMilliSplit`, `SecNanoSplit`, `YearSecSplit`.
- remove aliases of text-related types.
- remove features: `_default`, `_max`, `_non_value_*`, `_in_range`, `num_geom`, `unsafe_const`.
- remove standalone `char_*` fns (namespaced in `Char`).
- remove standalone fn `hash_pengy` (made part of `HasherPengy`).
- remove most re-exported fns from `std::mem` (namespaced in `Mem`).
- remove re-exported fns from `std::ptr` (namespaced in `Ptr`).
- remove convenience fn: `out_dir` from the build script.
- disable `Graph*`, `Node*`, and `NodeIndex*` types.
- remove `Float::const_abs`.

### Changed

#### Misc.
- bump rust version to 1.83.0.
- start using `core::error::Error`.

#### Features & flags
- rename features:
  - `unsafe_dyn` to `unsafe_layout`.
  - `nightly_stabilized` to `nightly_stable`.
  - `_[max|min]_docs` to `_docs_[max|min]`, `_docsrs_max` to `_docsrs`.
  - `dep_linux` to `linux_deps`, `dep_text` to `text_deps`, `dep_work` to `work_deps`.
- rename compilation flags:
  - `_some_*` to `_*··`.
  - `_int_i_·` to `_int_i··`, `_int_u_·` to `_int_u··`, `_string_u_·` to `_string_u··`.
- modify how features `_non_value_u8`, `_non_value_u16` are enabled for `Char*` types.
- feature gate methods returning `DataResult` in: `Array`, `Array2d`, `ArrayUninit`, `Bitwise`, `BitOps`.
- show build *env* variables if `__dbg` feature is enabled.

#### Items
- structs:
  - make `data::dst` types use `MemPod` instead of `bytemuck::Pod`.
  - rename:
    - `Also` to `Tap`, `Apply` to `Chain`.
    - `COLOR` to `Color`.
    - `Color` to `ColorBase`.
    - `GcdExt` to `GcdResult`.
    - `AllocMap` to `HashMap`, `AllocSet` to `HashSet`.
    - `AllocOrdMap` to `BTreeMap`, `AllocOrdSet` to `BTreeSet`.
    - `AllocPrioQueue` to `BinaryHeap`.
    - `AllocLinkedList` to `LinkedList`.
    - `Dst*` types const-generic `N` to `CAP`.
    - `sys::io` items by prefixing them with `Io`.
    - `Egc` to `Grapheme`, `EgcString` to `GraphemeString`.
    - `EgcNonul` to `GraphemeNonul`, `EgcU8` to `GraphemeU8`.
    - `UninitArray` to `ArrayUninit`.
- `Compare`
  - make `Compare<usize>` always compiled.
  - implement for `f16` and `128`.
- functions and constant:
  - `Graph::edge_exists` no loger panics.
  - rename:
    - `Array::len` to `capacity`.
    - `mem_*` prefixed fns as `Mem` methods.
    - `ptr_*` prefixed fns as `Ptr` methods.
  - make *const* most of the `Angle` methods.
  - make *const* all the `Float` methods that were previously feature-gated, and add:
    `eval_poly`, `factorial`, `mul_add_fallback`, `scale`, `lerp`, `ln*_series`, `log[10|2]_series`.
  - make *const* versions of the following `Float` methods:
    `clamp_nan`, `fisr`, `hypot_fisr`, `max_nan`, `min_nan`, `cbrt_nr`, `sqrt_nr`, `hypot_nr`, `rem_euclid`, `*_series`, `*_series_terms*`.
  - add additional *const* methods: `const_floor`, `const_ceil`, `const_round`, `const_round_ties_away`, `const_round_ties_even`, `const_round_ties_odd`, `const_trunc`, `const_fract`, `const_split`, `const_signum`, `const_copysign`, `const_clamp`, `const_min`, `const_max`, `const_powi`.
  - improve precision of `ExtFloatConst` constants from 35 to 80 decimals.
  - remove all `inline` attributes for most functions
- macros:
  - update `cdbg!` to support a single `@`.
  - update `reexport!` to support receiving an optional tag argument.
  - rename `mem_size_of_expr!` to `size_of_expr!`.
  - rename re-wrapped macros to avoid prelude collision when glob importing:
    - `env`→`env_`, `panic`→`panic_`, `vec`→`vec_`.
- modules:
  - make modules public: `data::serde`, `sys::arch`, `text::fmt`.
  - rename:
    - `num::geom::algebra` to `num::algebra::linear`.
    - `exec` to `work`, and related features.
    - `mem` to `sys::mem`.
    - `sys::ffi` to `ffi.
    - `time` to `phys::time`.
    - `_alloc` & `_std` inside `_dep`.
    - `_dep::_core` to `_core`.
    - `_deps` to `_dep`.
    - `_lib*` by removing the `lib` prefix.
- traits:
  - impl `Num` for niche types.
- dependencies:
  - make `bytemuck` an optional dependency.

#### Examples, utilities, manifest, files
- rename `DOCS/DERIVED.md` to `VENDORED.md`.
- rename the `tools` directory to `utils`.
- simplify aliases for `cargo-asm`.
- refactor the build script.

### Fixed
- reduce noise from required features on methods from `Divisor`, `Int`, `Float`, `Frac`.
- make `utils/check.rs` not compile with `all_dep` when cross-compiling certain arches.
- hide public macros from the crate root when `cfg(cargo_primary_package)`.
- fix build script utility call paths, add missing `_tuple*` features.
- fix `bitfield` and `enumset` being able to be called from the root.
- several fixes for linux syscalls on multiple architectures.
- simplify system of documentable & testable examples.
- fix reexported fns: `fmt_format`, `fmt_write`.
- fix `f64::NR_TOLERANCE` from 1e-14 to 1e-12.
- fix a few tests and examples.
- fix `RcWeak` re-export.
- fix `HashSetFx` alias.


[macros/0.12.1] - 2025-01-07
============================

- new macro `field_of`.
- make `std` a default feature.
- remove `hashbrown` from default features.
- remove inline attributes.

[0.24.0]: https://github.com/andamira/devela/releases/tag/v0.24.0
[0.23.0]: https://github.com/andamira/devela/releases/tag/v0.23.0
[0.22.1]: https://github.com/andamira/devela/releases/tag/v0.22.1
[0.22.0]: https://github.com/andamira/devela/releases/tag/v0.22.0

[macros/0.12.1]: https://github.com/andamira/devela/releases/tag/macros/v0.12.1
