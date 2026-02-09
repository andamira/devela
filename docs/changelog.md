# devela changelog

[0.25.0-wip] unreleased
=======================

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

<!-- **Legend:** ⚠️ Breaking Change | 🆕 New Addition | ✨ Enhancement | 🐛 Bug Fix -->

-----------
> *Project* :
-----------

## build
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

## cargo
- bump MSRV to 1.93.0.
- add cargo aliases: `L0r`, `w*` (workspace).
- add new cargo env var `CARGO_WORKSPACE_DIR`.
- fix updated syntax for unstable cargo-include in `.cargo/config.toml`.

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
  - [macros]
    - `proc-macro2` to 1.0.101.
    - `quote` to 1.0.40.
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

### [base]
- add `_workspace_internal` structural module (replacing `_internal`).

### [base_macros]
- move devela_macros macros: `devela_macros`: `cif!`, `compile!`, `compile_attr!`, `ident_total!`, `ident_total_unique!`, `ident_unique!`, `coalesce!`, `field_of!`.
- new macro: `repeat!`.
- new compiler predicates: `env`, `env_eq`, `env_ne`, `env_empty`, `env_nonempty`, `nota`.

### [macros]
- use workspace's crate version.
- make it an optional dependency.
- add `devela_base_core` as a dependency.
- enable `doc_cfg` via `nightly_doc` flag.
- remove dependency `hashbrown`.
- remove features: `alloc`, `std`, `nightly`, `nightly_doc`.

### [postbuild]
- add feature `__dbg`.

## manifest
- add workspace hierarchy diagram.
- add *binaries* and *metrics* sections.
- add lint `missing_debug_implementations`.
- make keys parts of the workspace: edition, version, authors, license, documentation.
- simplify debugging info in `dev` profile.
- add `debugging` profile.

## metrics
- rename directory `/benches` to `/metrics`.

## tools & misc. files
- add new binary: `croot`.
- update `tools/check.rs`:
  - bump `devela` to 0.24.0.
  - test all workspace crates.
  - start testing without dependencies.
  - switch rust-script for cargo-script.
  - simplify and homogenize toolchain selection syntax.
  - configure the exact nightly version to install and use.
- move `config/rustdoc-header.html` to `src/_doc/header.html`.
- move `/config/dep_all.rs` to `/build/main/dep_all`.
- update `src/_doc/header.html` to support multiple crates with custom whitelists.
- add flat re-exports of root modules to `zall_` & re-export hidden as `all_`.
- rename all `lib.rs` to `index.rs`.
- rename `_info` to `_doc`
- rename `all` to `zall` & re-export hidden as `all`.
- update github CI workflows
  - bump `actions/checkout` to v5.
  - add more `no_std` targets, retry downloads and disable fail-fast.


-----------
> *Modules* :
-----------

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

### error
- update `define_error!` macro.
  - move to `code::error`.
  - update docs, add example.
  - allow accepting multiple tags.
  - do not automatically derive `Default`.
  - make conversion method optional const.
- remove items: `AllError`, `AllResult`, `DataError`, `DataResult`, `ExtError`.
- update `ArrayFmt` to support the rest of the core formatting traits.

### marker
- make `TypeResource` and `type_marker!` constructors *const*.

### ops
- new macro: `punroll!`.

### panic
- move to [base]: `Panic`.

### result
- move to [base]:
  - functions: `serr`, `sok`.
  - macros: `unwrap!`.
  - traits: `Morph`, `Hook`, `OptionExt`, `OptResExt`, `ResultExt`
  - types: `Mismatch`, `OptRes`, `OptionFmt`, `OptionFmtOr`, `OptionFmtOrElse`, `Own`.
- new macros: `hook!`, `morph!`.
- rename:
  - `Chain` to `Morph`.
  - `ExtOption` to `OptionExt`.
  - `ExtOptRes` to `OptResExt`.
  - `ExtResult` to `ResultExt`.
- update `unwrap!`:
  - add arms: `err?`, `ok_err_map`, `ok_if*`, `ok_map*`, `ok_some_map`, `some_if*`, `some_map*`, `some_ok_map*`.
  - make *const* the arms that call `unreachable`.
  - rename arms:
    - `ok_map_err`? to `ok_err_map?`.
    - `ok_if_map_err`? to `ok_if_err_map?`.

### utils
- new macros: `compile_warn!`, `doc_location!`, `doclink!`, `fn_name!`, `lets!`, `mod_path!`, `repeat!`, `type_count!`, `whilst!`, `write_at!`.
- move to [base]:
  - public macros: `CONST!`, `assert_eq_all!`, `assert_approx_eq_all!`, `capture_first!`, `capture_last!`, `capture_tail_tuple!`, `cfg_if!`, `const_assert!`, `define_error!`, `deprecate!`, `enumset!`, `ident_const_index!`, `impl_trait!`, `include_from!`, `is!`, `items!`, `maybe!`, `methods_as_fns!`, `mod_from!`, `sf!`, , `structural_mods!`, `type_marker!`.
  - internal macros: `__crate_name!`, `__dbg!`, `__std!`, `_EMOJI_*`, `_TAG_*`, `_doc!`, `_doc_availability!`, `_doc_miri_warn!`,  `_reexport!`, `_tags!`, `_use!`.
- add tags: `_DOC_*`, `_TAG_[ALG|APPLE|ASSERT|AUDIO|BIT|CODE|CODEC|CODEGEN_BUILD|CONSTRUCTION|COLOR|CONCURRENCY|DATA|DEBUG|EVENT|EXAMPLE|FS|GEOM_DIR|GUARD|HASH|ID|IMAGE|INIT|INTERACTION|INTERNAL|IO|LAYOUT|LIFETIME|LIN|LINUX|LIST|LOGIC|MAYBE|MEM|PLATFORM|PROC_MACRO|RUNTIME|SYMB|TERM|UNIX|VALUE|WAVE|WINDOWS|WIP]`.
- change the emoji for `_TAG_DATA_STRUCTURE`.
- new re-exports: `select_unpredictable`.
- new functions: `cold_path`, `likely`, `unlikely`.
- rename `reexport!` internal macro to `_reexport!`.
  - mark as `doc(no_inline)`.
  - allow accepting multiple tags.
  - fix rendering of std path links.
- prefix internal constants `TAG_*` & `EMOJI_*` with `_`
- define `_std_core` separately and privately per crate.
- update `CONST!` macro with new arms: `hidden macro_export`, `inline macro_export`.
- update `impl_traits!` macro:
  - add new arms for: `Display+Error`, `FromStr`, `PartialEq`.
  - change syntax from a single <gen> group to a double [decl][args] group, to support const generics.
- update `const_assert!` macro:
  - add new arms: `ne_buf`, `ne_str`.
  - add support for comparing slices of primitives and slices of slices of primitives.
- remove temporary value binding functionality from `is!` macro, unnecessary after rust v1.89.
- remove vendored macro `cfor!`, replace with `whilst!`.
- remove deprecated `iif!` macro.

---
## data
- move to [base]:
  - macros: `bitfield!`, `init_array!`.
  - traits: `BitOps`.
  - types: `ArrayFrom`, `Bitwise`, `Oneof`, `Sort`.
- new macro: `define_handle!`.
- new `SortAlloc` wrapper for `Sort`.
- make `Sort` methods take `&mut self` instead of `self`.
- make `Sort` public `quick_*` methods take `&mut self` as well.

### bit
- move `BitOps` & `Bitwise` to `num::bit`.
- make the module private.

### codec
#### hash
  - move to [base]: `HasherFx`, `HasherBuildFx`.
  - new module `data::codec::hash::check`.
  - new type `Adler32`.

#### radix
- move `Base` to [base].
- remove methods that allocate.

### error
- recreate the type `MismatchedCapacity`.
- remove type: `DataOverflow`.
- update `MismatchedBounds`.

### iter
- new traits: `IteratorLending`, `IteratorLendingDoubleEnded`, `IteratorLendingExactSize`, `IteratorLendingPeek`.

### key
- update `define_static_map!`:
  - support custom attributes and visibility.
  - add example items: `ExampleStaticMapConstU8`, `ExampleStaticMapTypeId`, `ExampleStaticMapU16`.
  - improve docs.

### list
- move to [base]:
  - traits: `ArrayExt`.
  - types: `ArrayFmt`, `ConstList`.
- new macros: `define_bufline!`.
- new example type: `BufLineExample`.
- rename:
  - `ExtArray` to `ArrayExt`.
  - `ExtVec` to `VecExt`.
- make all `bitfield!` methods consts.
- update `init_array!`:
  - rename `array_init!` to `init_array!`.
  - require `ConstInit` and `Vec` in scope if needed.
  - rename `const_init` arm related to traits to `INIT in`.
  - rename `const_init*` arms unrelated to traits to `const_fn*`.
- update `Oneof`
  - new methods: `copy_*`.
  - remove methods: `variant_name`, `is_variant_name`, `first_non_unit`.
  - make methods const: `as_mut_*`, `as_ref_*`, `as_tuple_ref_options`.

### uid
- move `IdPin` to [base].
- new type `IdRegistry`.

---
## geom
- make `num::geom` a new `geom` root module.
- new modules: `affine`, `rel`, `space`.
- rename `shape` to `fig`, as well as the related feature.

### dir
- new module.
- move here `Orientation`, `Angle`, `AngleDirection` & `AngleKind`.
- new types: `Boundary1d`, `Boundary2d`, `Boundary3d`.

### metric
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

---
## lang
- rename `lang::ling` to `lang::hum`.
- rename `lang::ling::grammar` to `lang::hum::gram`.
- move `lang::i18n` to `lang::hum::i18n`.
- new modules: `disc`, `gram`, `prog`, `repr`, `sem`.
- move `ffi` to `prog::ffi`.

### prog
- new module: `dsl`.

#### ffi
##### c
- new type aliases: `c_mode_t`, `c_off_t`.

##### js
- update `JsInstant`:
  - make method const: `delta_since`.
  - remove methods: `const_delta_since`.

---
## media
### audio
- new types: `AudioChannel`, `AudioChannels`.

### color
- move to [base]:
  - types: `Gamma`, `Lum`, `Rgb`, `Rgba`.
  - aliases: `Lightness`, `LinearLightness`, `Luma`, `Luminance`, `Rgb8`, `Rgba8`, `RgbaPre8`, `Rgb16`, `Rgba16`, `RgbaPre16`, `RgbF32`, `RgbaF32`, `RgbaPreF32`, `RgbF64`, `RgbaF64`, `RgbaPreF64`, `RgbLinF32`, `RgbaLinF32`, `RgbaLinPreF32`, `RgbLinF64`, `RgbaLinF64`, `RgbaLinPreF64`.
- new type `GammaConst`.

### font
- new type: `FontArt`.
- new const: `FONT_ART_3_4`.
- rename:
  - `BitmapFont` to `FontBitmap`.
  - `FONT_3_3` to `FONT_BIT_3_3`.
  - `FONT_3_5` to `FONT_BIT_3_5`.
  - `FONT_5_6` to `FONT_BIT_5_6`.

### image
- add a new sixel implementation:
  - new types: `SixelChar`, `SixelColor`, `SixelEncoder`, `SixelPalette`, `SixelPaletteIter`.
- remove legacy vendored sixel implementation.

---
## num
- move to [base]:
  - all data, numeric, text & time error types.
  - types: `Cast`, `Int`, `True`.
  - traits: `NumConst`.
- make `Num*` traits depend on the `num` feature.
- move `num::unit` to `phys::unit`.
- make `num::error` public.

### dom
- new module `num::dom`.
- move here `num::float`, `num::frac`, `num::int`, `num::traits`.

#### real
##### float
- rename: `ExtFloat` to `FloatExt`.
- new types: `f32bits`, `f32bits_niche`, `f64bits`, `f64bits_niche`.
- update `Float`
  - new methods: `poly_eval_const`, `sin_minimax`, `cos_minimax`, `sin_cos_minimax`.
  - remove deprecated methods: `const_signum`, `const_copysign`, `const_clamp`, `const_max`, `const_min`.
  - make std methods *const*: `fract`, `normalize`, `set_normalized`, `split`, `trunc`.
  - split out std-enabled implementation as internal `FloatStd`.
- Change `ExtFloat` to use `*_minimax` methods by default.
- move to [base]
  - aliases: `fsize`.
  - traits: `FloatConst`.
  - types: `Float`.
  - float shared docs prefixed with `_FLOAT_`.

#### int
- move to [base]: `Int`, `[iu]size_*`.
- prefix int shared docs with `_INT_`.
- new macros: `define_divisor!`.
- new types: `DivisorExample`, `IntAlloc`, `IntError`, `IntResult`.
- remove type: `Divisor`.
- make all `Int` methods *const*.

### fin
- new module `num::fin`.
- move here `num::bit`, `num::logic` and `num::ord`.

#### bit
- new module `num::bit`.
- update `BitOps` & `Bitwise`.
  - rearrange methods in thematic impl blocks.
  - new methods: `[is_][un]set[_checked][_range]`, `[un]set_all`, `flip[_checked]`, `flip[_checked]_range_if`, `[is_][un]set_mask`.
- separate documentations for `BitOps` and `Bitwise` as individual constants.

#### logic
- move to [base]: `ConstBool`, `False`, `True`, `const_bool!`, `[iu]size_*`.

#### ord
- rename `Compare` to `Cmp`.
- new macro `cmp!`.
- update `Cmp`:
  - move to [base].
  - new impl for `Ordering`.
  - new methods: `minmax`, `pminmax`, `total_cmp`.
  - un-gate impls and many dependent const methods.

### grain
- new module `num::grain`.
- move inside: `num::cast`, `num::niche`, `num::wide`.

#### cast
- fix `Cast` wrapping methods performance, and correctness for negative integers.

#### niche
- new macros: `niche_prim!`, `nv!`.
- new types: `MaybeNiche`, `NonNiche`, `NicheValueError`.
- move to [base]: `NonExtreme*`, `NonValue*`, `ne!`, `nz!`.
- update macros: `ne`, `nv`, `nz`, adding lossy constructors.

#### wide
- new module `num::grain::wide`.
- new macro: `define_lane!`.
- new internal helper macros.
- new example type `ExampleLane4_i32`.
- support `nightly_simd` & `dep_wide` in [base_core].
- re-export some of `core::simd` types and traits.

### lin
- new module: `num::lin`.
- move here `geom::linear::{matrix, vector}`.

### prob
- new module `num::prob`.
- move here `num::rand`.

#### rand
- move `num::prob::rand` to [base].
- rename `Lgc16` to `Lcg16`.
- new macro: `define_pcg!`.
- new traits: `Rand`, `RandAlloc`, `RandStd`.
- new type: `Pcg32`.

### quant
- move to [base]: `Cycle`, `CycleCount`, `Interval`,  `Sign`.
- update `Interval`:
  - use individual `IncompatibleBounds` error.
  - add methods: `bound`, `bound_mut`, `bound_as_ref`.
- update `Sign`
  - make part of `quant`.
  - rename variant `None` to `Zero`.
  - add methods: `eq`, `is_negative`, `is_positive`, `is_zero`, `is_nonzero`, `invert`, `same_direction`, `combine`, `pow`, `abs`, `neg_abs`, `fold`, `fold_slice`.
- move `ValueQuant` from `code::result` to `num::quant`.

### symb
- new modules `num::symb`.

---
## org
- new `org` module.

---
## phys
### astro
- new `phys::astro` module.

### time
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
- update `TimeSplitMilliNanoNorm`
  - add method `from_duration`.
  - rename `from_duration` method to `from_duration_subsec`.
- update `WeekDay`: make all methods const.
- rename `UnixTimeI64` to `TimeUnixI64`, `UnixTimeU32` to `TimeUnixU32`.
  - make their `new` method const.

---
## run
- new `run` root module.

### frame
- new items: `RunTick`.

### setup
- move capabilities from `ui::back::cap` to `run::setup::cap`.
  - rename `UiCap*` to `RunCap*`.

---
## sys
### arch
- new `Arch` methods: `cntvct`, `cycles`, `rdcycle`, `rdtsc`, `rdtscp`.
- new internal macro `ARCH!`.

### device
- new module: `sys::device`.
- move `media::midi` to `sys::device::midi`.
- move `sys::sound` to `sys::device::audio`.

#### display
- new module: `sys::device::display::x11`.
- new types: `XDisplay`, `XError`, `XEvent`, `XWindow`.

### env
- vendor `argv` as `IterArgSOsRef` struct and `Env` method `args_os_ref`.

### fs
- rename `ExtPath` to `PathExt`.

### hw
- new module `sys::hw`.

### io
- new `IoDuplex` trait.
- refactor the `sys::io` module.
- update `Io`: add `pipe` method.
- re-export `IoPipeReader`, `IoPipeWriter`.

### log
- new type `LoggerStatic`.
- new macro `slog!`.
- rename `ExtLogger` to `LoggerExt`.

### mem
- new alias: `MaybeByte`.
- new types: `ExampleArena`, `ExampleArenaHandle`.
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
- update `Mem`
  - rename method `bytes_from_bits` to `bytes_from_bits_saturating`.
  - new method: `bytes_from_bits` with a faster impl.

#### cell
- rename `ExtCellOption` to `CellOptionExt`.

#### size
- move `ByteSized` and `size_of_expr!` to [base].

#### slice
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
    - `trim_leading_bytes` to `trim_leading`
    - `replace_leading_bytes` to `replace_leading`.
  - add new methods:
    - `range_to_inclusive*`.
    - `chunk[_mut]`, `chunks_exact[_mut]`.
    - `clone`, `copy_into`, `copy_str_into`, `copy_utf8_into`.
    - `trim_leading_keep`, `trim_leading_min_len`, `trim_trailing`, `trim_trailing_keep`, `trim_trailing_min_len`, `trim_edges_keep`, `trim_edges_min_len_left`, `trim_edges_min_len_right`.
  - add new `eq` methods for slices of slices of primitives and string slices.

### os
- repurpose module to include operating supervisors.
- new `Libc` namespace.
- new modules: `sys::os::browser`, `sys::os::fd`.

#### browser
- move `lang::ffi::js::web` to `sys::os::browser::web`.
- move `examples::lang::js_web*` to `examples::sys::web*`.

#### linux
- new types `LinuxClock`, `LinuxTime`, `LinuxInstant`.
- new `Linux` methods: `clock_getres`, `clock_gettime`, `exit`.
- new `Linux` syscalls: `sys_clock_getres`, `sys_clock_gettime`.
- fix `Linux`-related warnings & avoid use of `transmute`.
- rename syscalls doc constants, prefix with `_DOC_`.
- update `LinuxError`:
  - new `Other` variant.
  - impl `Error`.
- improve `LinuxTimespec`.
  - impl `Display` and `ConstInit`
  - rename method `with` to `try_with_duration` and make fallible.
  - add corresponding method `try_to_duration`.
  - add saturating methods to convert from/to `Duration`.
  - add corresponding conversions and methods from/to `TimeDelta`.

---
## text
- new struct: `TextLut`.
- move to [base]:
  - traits: `NumToStr`.
  - types: `ByteSearch`, `Digits`, `GraphemeNonul`, `GraphemeU*`, `Str`, `StringNonul`, `StringU*`, `char7`, `char8`, `char16`.

### char
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
  - remove `utf8_bytes_` prefix from `Char<&[u8]>` methods…
  - add private consts: `CONT_MASK` `UTF8_CHAR_LEN`.
  - remove `code_` prefix from `Char<u32>` methods.
  - rename method `byte_len` to `len_bytes`.
  - modify all methods to take `self`.
  - return lengths as usize.
- update `UnicodeScalar`:
  - new methods: `as_ascii_translit`, `is_combining`, `is_combining_common`, `is_control`, `is_control_common`, `is_fullwidth`, `is_fullwidth_common`, `to_char`, `to_scalar`.
  - rename method `byte_len` to `len_bytes`.
  - reorder methods.

#### ascii
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

### error
- re-export std's `FromUtf8Error`.

### fmt
- new macro: `fmtcat`.
- new trait: `DebugExt`.
- new types: `FmtNum`, `FmtNumConf`, `FmtNumShape`, `FmtNumSign`, `FmtWriter`.
- re-export `FromFn` as `FmtFromFn`.
- move to [base]:
  - macros: `format_buf!`.
- remove vendored `numtoa` crate, `NumToStr` trait replaced with `Digits` struct.
- add method `Fmt::from_fn`.

### grapheme
- new types: `GraphemeKind`, `GraphemeScanner`.
- feature-bound all grapheme-related items.
- vendor `grapheme_machine` as items: `GraphemeBoundary`, `GraphemeMachine`, `GraphemePropCb`, `GraphemePropInCb`, `GraphemeProps`.
- impl `Grapheme` for scalar types.
- update the `Grapheme` trait:
  - add new methods: `grapheme_chars`, `grapheme_is_kind`, `grapheme_kind`, `grapheme_len_bytes`, `grapheme_len_chars`, `grapheme_len_utf8`.
- update `Grapheme[Nonul|U*]`:
  - remove methods: `to_cstring`.
  - make `new` method panic
  - add new methods: `eq`, `[as|into]_string_[nonul|u8]`, `from_charu[_unchecked]`, `from_str`, `new_checked`.
  - make methods unsafe: `as_bytes_mut`, `as_str_mut`.
  - implement `PartialEq` and `Hash` manually.
  - implement `PartialEq` between string and grapheme types.
  - make all methods *const*.

### layout
- new module `text::layout`.
- new types: `TextCohesion`, `TextCursor`, `TextFit`,  `TextIndex`, `TextLayout`, `TextLayoutStep`, `TextSpan`, `TextSymbol`, `TextUnit`.

### str
- remove methods: `to_cstring`, from `String*`.
- make `chars` methods *const* when possible.
- add more impls of `PartialEq` against string slices.
- update `Str:`
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

---
## ui
### event
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
- update `EventWindow`
  - fix `Moved` variant to use `i32`.
  - new methods `is_[geometry|resize|move|focus|close|redraw|visibility|pointer_crossing|text_input|stream_signal]`, `[resize|move]_coords`.
  - new variants: `Entered`, `Left`, `Minimized`, `Maximized`, `Restored`, `FullscreenEntered`, `FullscreenExited`.

#### key
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

### front
#### term
- feature-gate with `term`.
- rename `AnsiColor3b` to `AnsiColor3` and `AnsiColor8b` `AnsiColor8`.
- move to [base]:
  - types: `Ansi`, `AnsiColor3`, `AnsiColor8`, `TermSize`.
- change `Ansi::print*` methods to `ansi_print*` functions.
- new type: `AnsiColor`.
- update `Ansi:`
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

---
## vita
- new `vita` module.

---
## work
### future
- rename:
  - `ExtFuture` to `FutureExt`.
  - `ExtProcess` to `ProcessExt`.
    - `ProcessExt::id` to `self_pid`.

### process
- new macro: `cmd!`.
- new trait: `OutputExt`.
- new types: `CommandFlow`, `ExitStatusError`.
- rename re-exported types back by removing the `Process` prefix, except for `Child*`→`Process*` renames and `ProcessTermination`.

### sync
- move `portable-atomic-util` dependent re-exports to [base_alloc]: `Arc`, `ArkWeak`.

---
## yard
- new internal root module `yard`.
- new workspace-internal macro `_devela_policy!`.
- move here most private macros from `code::util`.


[0.25.0]: https://github.com/andamira/devela/releases/tag/v0.25.0
