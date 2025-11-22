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
- workspace restructuring: split into multiple crates (devela_base_*).
- feature consolidation: removed feature gates, make functionality always available.
- text system overhaul: major improvements to scalar, string, and grapheme handling.
- const evolution: many methods made const across numeric, text, and system modules.
- build system enhancements: improved build configuration and post-build processing
- memory & system improvements: enhanced slice operations and system arches support.
- error system refinement: updated error macros and type organization.
- msrv bump: minimum supported rust version increased to 1.90.0.

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
- make sure `CARGO_TARGET_DIR` and `CARGO_WORKSPACE_DIR` are always defined.
- add new `devela_postbuild` crate to `build/post`.
- add build config flag aliases: `any_target_arch_linux`, `any_target_arch_riscv`, `base_safe*`.
- add [base] symlinks to `devela/main/[alias|environment|features].rs`.

## cargo
- bump MSRV to 1.90.0.
- add new cargo doc workspace aliases `w*`.
- add new cargo env var `CARGO_WORKSPACE_DIR`.

## dependencies
- make all optional external optional dependencies part of the workspace.
- re-export `alloc` crate from devela and [base_alloc].
- bump dependencies:
  - `hashbrown` to 0.16.
  - [macros]
    - `proc-macro2` to 1.0.101.
    - `quote` to 1.0.40.
- add optional dependencies to [base]: `memchr`, `rand_core`, `simdutf8`.
- remove `_core` and `_dep` re-exports from the public docs.
- remove dependencies:
  - `const-str`, and related `str!` macro.
  - `libm` and related `Float` and `ExtFloat` functionality.
  - itertoos and related re-exports.
  - remove: `allocator-api2`, `bumpalo`, `fontdue`, `ffmpeg-the-third`, `flume`, `fontdue`,  `gilrs`, `image`, `kira`, `midir`, `rayon`, `regex-lite`, `rodio`, `sdl2`, `sdl3`, `stringzilla`, `symphonia`, `sysinfo`, `toml_edit`, `tokio`, `unicode-segmentation`, `unicode-width`, `ureq`, `winnow`.


## features & flags
- new features: `__publish`, `__std`, `base_safe`, `grapheme`, `safe_build`, `x11`.
- remove features: `_bit*`, `_char*`, `_cmp*`, `_float_*`, `_int_*`, `_num?_all`, `_sort*`, `_str_*`, `_str_nonul`, `_str_u*`, `_text_all`, `ascii`, `cast`, `error`, `fmt`, `join`, `metric`, `prim`, `split`, `str`.
- remove flags: `bit··`, `char··`, `cmp··`, `_float··`, `_int*··`, `_nums··`, `prim··`, `sort··`, `str··`, `str_u··`.
- add an adittional `nightly_stable_1_??` flag for the 3rd next version.
- rename:
  - `_docs` to _`docs_min`.
  - `_docsrs` to `_docs`.
  - `_docsrs_nodep` to `_docs_nodep`.
  - `__no_test` to `__exclude_test`.
- add default feature `alloc` to [base_alloc].
- add default feature `std` to [base_std].

## workspace libraries
- declare the `std` external crate.
- remove `_always` structural modules.
- refactor all structural access modules.
- enable `_docsrs` for workspace dependencies.
- support having external optional dependencies.
- new workspace library crates: `devela_base_alloc`, `devela_base_core`, `devela_base_data`, `devela_base_macros`, `devela_base_num`, `devela_base_std`, `devela_base_text`.
- prepare future workspace library crates related to root modules.
- move `core`, `alloc` & `std` re-exports to [base*] libs.
- use a single version, changelog and readme for all workspace libs.
  - move `devela_macros` changelog into `devela` archived changelog history.
  - replace `paste` dependency with `pastey` and move to [base].
- [base]
  - add `_workspace_internal` structural module (replacing `_internal`).
- [base_macros]:
  - move devela_macros macros: `devela_macros`: `cif!`, `compile!`, `compile_attr!`, `ident_total!`, `ident_total_unique!`, `ident_unique!`, `coalesce!`, `field_of!`.
  - new macro: `repeat!`.
- [macros]:
  - use workspace's crate version.
  - make it an optional dependency.
  - add `devela_base_core` as a dependency.
  - enable `doc_cfg` via `nightly_doc` flag.
  - remove dependency `hashbrown`.
  - remove features: `alloc`, `std`, `nightly`, `nightly_doc`.
- [postbuild]
  - add feature `__dbg`.

## manifest
- add lint `missing_debug_implementations`.
- make keys parts of the workspace: edition, version, authors, license, documentation.
- add *binaries* and *metrics* sections.
- add workspace hiearchy diagram.

## metrics
- rename directory `/benches` to `/metrics`.

## tools & misc. files
- add new binary: `croot`.
- update `tools/check.rs`:
  - bump `devela` to 0.24.0.
  - test all workspace crates.
  - start testing without dependencies.
  - remove `itertools` direct dependency.
  - simplify and homogeinize toolchain selection syntax.
  - configure the exact nightly version to install and use.
- update `config/rustdoc-header.html` to support multiple crates with custom whitelists.
- move `/config/dep_all.rs` to `/build/main/dep_all`.
- add flat re-exports of root modules to `zall_` & re-export hidden as `all_`.
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
- new trait `ConstInitCore`.
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
  - make conversion method optional const.
- remove items: `AllError`, `AllResult`, `DataError`, `DataResult`, `ExtError`.
- update `ArrayFmt` to support the rest of the core formatting traits.

### marker
- make `TypeResource` and `type_marker!` constructors *const*.

### panic
- move to [base]: `Panic`.

### result
- move to [base]:
  - functions: `serr`, `sok`.
  - macros: `unwrap!`.
  - traits: `Chain`, `Hook`, `OptionExt`, `OptResExt`, `ResultExt`
  - types: `Mismatch`, `OptRes`, `OptionFmt`, `OptionFmtOr`, `OptionFmtOrElse`, `Own`.
- move to [base_num]: `ValueQuant`.
- rename:
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
- new struct: `Lut`.
- new macros: `compile_warn!`, `doclink!`, `fn_name!`, `lets!`, `mod_path!`, `repeat!`, `type_count!`, `whilst!`, `write_at!`.
- move to [base]:
  - public macros: `CONST!`, `assert_eq_all!`, `assert_approx_eq_all!`, `capture_first!`, `capture_last!`, `capture_tail_tuple!`, `cfg_if!`, `const_assert!`, `define_error!`, `deprecate!`, `enumset!`, `ident_const_index!`, `impl_trait!`, `include_from!`, `is!`, `items!`, `maybe!`, `methods_as_fns!`, `mod_from!`, `sf!`, , `structural_mods!`, `type_marker!`.
  - internal macros: `__dbg!`, `__std!`, `_EMOJI_*`, `_TAG_*`, `_doc!`, `_doc_availability!`, `_doc_miri_warn!`,  `_reexport!`, `_use!`.
- add tags: `_DOC_*`, `_TAG_[CODEC|CODEGEN_BUILD|CONCURRENCY|DATA|DEBUG|EVENT|EXAMPLE|GEOM_DIR|HASH|ID|PROC_MACRO]`.
- change the emoji for `_TAG_DATA_STRUCTURE`.
- rename `reexport!` internal macro to `_reexport!`.
  - allow accepting multiple tags.
- prefix internal constants `TAG_*` & `EMOJI_*` with `_`
- define `_std_core` separately and privately per crate.
- update `CONST!` macro with new arms: `hidden macro_export`, `inline macro_export`.
- add `location` arms to the `_doc!` macro.
- update `impl_traits!` macro:
  - add new arms for: `PartialEq`.
- update `const_assert!` macro:
  - add new arms: `ne_buf`, `ne_str`.
  - add support for comparing slices of primitives and slices of slices of primitives.
- remove temporary value binding functionality from `is!` macro, unnecessary after rust v1.89.
- remove vendored macro `cfor!`, replace with `whilst!`.
- remove deprecated `iif!` macro.

---
## data
- move to [base]:
  - macros: `array_init!`, `bitfield!`.
  - types: `ArrayFrom`, `Bitwise`, `Oneof`, `Sort`.
- new macro: `define_handle!`.
- new `SortAlloc` wrapper for `Sort`.
- make `Sort` methods take `&mut self` instead of `self`.
- make `Sort` public `quick_*` methods take `&mut self` as well.
- update `array_init!`:
  - require `ConstInit` and `Vec` in scope if needed.
  - rename `const_init` arm related to traits to `INIT in`.
  - rename `const_init*` arms unrelated to traits to `const_fn*`.

### key
- update `define_static_map!`:
  - support custom attributes and visibility.
  - add example items: `ExampleStaticMapConstU8`, `ExampleStaticMapTypeId`, `ExampleStaticMapU16`.
  - improve docs.

### list
- move to [base]:
  - traits: `ArrayExt`.
  - types: `ArrayFmt`, `ConstList`.
- rename:
  - `ExtArray` to `ArrayExt`.
  - `ExtVec` to `VecExt`.
- make all `bitfield!` methods consts.
- update `Oneof`
  - new methods: `copy_*`.
  - remove methods: `variant_name`, `is_variant_name`, `first_non_unit`.
  - make methods const: `as_mut_*`, `as_ref_*`, `as_tuple_ref_options`.

---
## lang
- rename `lang::ling` to `lang::hum`.
- rename `lang::ling::grammar` to `lang::hum::gram`.
- move `lang::i18n` to `lang::hum::i18n`.

### ffi
#### c
- new type aliases: `c_mode_t`, `c_off_t`.

---
## media
### font

- new type: `FontArt`.
- new const: `FONT_ART_3_4`.
- rename:
  - `BitmapFont` to `FontBitmap`.
  - `FONT_3_3` to `FONT_BIT_3_3`.
  - `FONT_3_5` to `FONT_BIT_3_5`.
  - `FONT_5_6` to `FONT_BIT_5_6`.

### image
- rename vendored sixel types with the `Legacy*` prefix.
- add a new sixel implementation:
  - new types: `SixelChar`, `SixelColor`, `SixelEncoder`, `SixelPalette`, `SixelPaletteIter`.

---
## num
- move to [base]:
  - all data, numeric, text & time error types.
  - macros: `const_bool!`.
  - types: `Cast`, `Cycle`, `CycleCount`, `False`, `Interval`,  `Sign`, `True`.
  - traits: `ConstBool`.
- update `Interval` to use individual `IncompatibleBounds` error.
- fix `Cast` wrapping methods performance, and correctness for negative integers.
- move `ValueQuant` from `code::result` to `num::quant`.
- update `Sign`
  - make part of `quant`.
  - rename variant `None` to `Zero`.
  - add methods: `eq`, `is_negative`, `is_positive`, `is_zero`, `is_nonzero`, `invert`, `same_direction`, `combine`, `pow`, `abs`, `neg_abs`, `fold`, `fold_slice`.

### float
- new types: `f32bits`, `f32bits_niche`, `f64bits`, `f64bits_niche`.
- move aliases: `fsize`, to [base].
- move float shared docs to `devela_base_num` prefixed with `_FLOAT_`.
- make std `Float` methods *const*: `fract`, `normalize`, `set_normalized`, `split`, `trunc`.
- rename: `ExtFloat` to `FloatExt`.

### geom
#### dir
- new module.
- move here `Orientation`, `Angle`, `AngleDirection` & `AngleKind`.

#### metric
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

### int
- move to [base]: `Int`, `[iu]size_*`.
- move int shared docs to `devela_base_num` prefixed with `_INT_`.
- new types: `IntAlloc`, `IntError`, `IntResult`.
- make all `Int` methods *const*.

### logic
- move to [base]: `Int`, `[iu]size_*`.

### niche
- new macro: `nv!`.
- new types: `MaybeNiche`, `NonNiche`, `NicheValueError`.
- move to [base]: `NonExtreme*`, `NonValue*`, `ne!`, `nz!`.
- update macros: `ne`, `nv`, `nz`, adding lossy constructors.

### ord
- move `Cmp` to [base].
- rename: `Compare` to `Cmp`.
- un-gate `Cmp` impls and many dependent const methods.

---
## phys
### time
- remove `TimeError` alias.

---
## sys
### arch
- new `Arch` methods: `cntvct`, `cycles`, `rdcycle`, `rdtsc`, `rdtscp`.
- new internal macro `ARCH!`.

### display
- new module `sys::display::x11`.
- new types: `XDisplay`, `XError`, `XEvent`, `XWindow`.

### env
- vendor `argv` as `IterArgSOsRef` struct and `Env` method `args_os_ref`.

### fs
- rename `ExtPath` to `PathExt`.

### hw
- new module `sys::hw`.
- move `sys::sound` to `sys::hw::audio`.

### log
- new type `LoggerStatic`.
- new macro `slog!`.
- rename `ExtLogger` to `LoggerExt`.

### mem
- new alias: `MaybeByte`.
- new types: `ExampleArena`, `ExampleArenaHandle`.
- new macros: `define_arena`.
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

#### slice
- new macro `slice!`.
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
    - `copy_into`, `copy_str_into`, `copy_utf8_into`.
    - `trim_leading_keep`, `trim_leading_min_len`, `trim_trailing`, `trim_trailing_keep`, `trim_trailing_min_len`, `trim_edges_keep`, `trim_edges_min_len_left`, `trim_edges_min_len_right`.
  - add new `eq` methods for slices of slices of primitives and string slices.

### os
- repurpose module to include operating supervisors.
- new `Libc` namespace.

#### linux
- new enum `LinuxClock`.
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
- move to [base]:
  - traits: `NumToStr`.
  - types: `ByteSearch`, `Digits`, `GraphemeNonul`, `GraphemeU*`, `Str`, `StringNonul`, `StringU*`, `char7`, `char8`, `char16`.

### char
- new macro: `ch!`.
- new fn: `scalar_as_ascii_translit`.
- new types: `CharIter`, `charu`, `charu_niche`.
- new `char7` methods: `to_byte`, `to_str`.
- new `char[7|8|16]` methods: `to_charu`, `try_from_charu`.
- new `Lut` consts: `ASCII_BASE36_OFFSET`, `DIGITS_BASE36`, `DECIMAL_PAIRS`, `POWERS10`.
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
  - return lenghts as usize.
- update `UnicodeScalar`:
  - new methods: `as_ascii_translit`, `is_combining`, `is_combining_common`, `is_control`, `is_control_common`, `is_fullwidth`, `is_fullwidth_common`, `to_char`, `to_scalar`.
  - rename method `byte_len` to `len_bytes`.
  - reorder methods.

#### ascii
- rename `ASCII_TABLE` to `LUT_ASCII_CHARS` and make it a public *const*.
- rename `Ascii` to `Digits`.
- update `Digits`:
  - new type: `AnsiColor`.
  - new const: `MAX_DIGITS_16`.
  - new methods: `count_digits16`, `digit_at_index10[_checked]`, `digit_at_index16[_checked]`, `digit_value_at_index10[_checked]`, `digit_value_at_index16[_checked]`, `digits16`, `write_digits10`, `write_digits10_fast`, `write_digits10_omit0`, `write_digits16`, `write_digits16_omit0`.
  - new private method: `digit_at_power16`.
  - rename const: `MAX_DIGITS` to `MAX_DIGITS_10` and make them of type `u8`.
  - rename methods:
    - `calc_digit` to `digit_at_power10` and make private.
    - `count_digits` to `count_digits10`.
    - `digits_*` to `digits10_*`.
    - `digits` to `digits10`.
- update derives for `AnsiColor*`.

### fmt
- new macro: `fmtcat`.
- new types: `FmtNum`, `FmtWriter`.
- move to [base]:
  - macros: `format_buf!`.
- remove vendored `numtoa` crate, `NumToStr` trait replaced with `Digits` struct.

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

---
## ui
### events
- new types: `Event`, `EventKind`, `EventWindow`.
- change `EventPointer.pressure` field to be `f32bits_niche`.
- rename `time_stamp` fields to `timestamp`.
- derive `Eq` & `Hash` for all event types.
- implement `ConstInit` for all types.
- update `EventTimestamp`
  - remove all inner unsafe.
  - make it support timestamps of 0 ms.
  - change inner representation to `f32bits_niche`.
  - add methods: `as_millis_f32_to_u32`, `as_millis_u32`, `from_millis_u32_as_f32`.
  - remove methods: `try_from_js`, `try_from_millis_f32`,  `try_from_millis_u32`, `try_from_secs_f32`.
- change `EventKeyFfi.timestamp` field to be `f32bits`.

#### key
- update `KeyPad`, add variant `Comma`.
- update `KeyMod`:
  - rename variant `IsoLevel3Shift` to `AltGr`.
  - remove variants: `LeftMeta`, `RightMeta`, `LeftHyper`, `RightHyper`.
- update `KeyMods`, rename method `has_meta` to `has_super`.
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
- update `Ansi:`
  - reverse the order of arguments in `CURSOR_MOVE*` to be columns first.
  - add methods: `COLOR_FG_BRIGHT`, `COLOR_BG_BRIGHT`, `strip_codes`.
  - rename current associated const items with a `_B` suffix.
  - add duplicated items with the old name returning a string slice or a `StringU8`.
  - modify `CURSOR_MOVE_N` method to use `Digits::digit_at_index10`.
  - modify `CURSOR_*` methods taking `u32` to take `u16`.
  - make all escape-sequence methods *const*.
  - fix codes related to alternate screen.
- update `ansi!`:
  - add new arms `p!` and `@p!` that auto-unwrap.
  - refactor to accomodate `Ansi` changes. TODO
  - fix macro visibility.

---
## work
### future
- rename:
  - `ExtFuture` to `FutureExt`.
  - `ExtProcess` to `ProcessExt`.

[0.25.0]: https://github.com/andamira/devela/releases/tag/v0.25.0
