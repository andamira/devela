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
- new features: `__publish`, `__std`, `base_safe`, `grapheme`, `safe_build`.
- remove features: `_bit*`, `_char*`, `_cmp*`, `_float_*`, `_int_*`, `_num?_all`, `_sort*`, `_str_*`, `_str_nonul`, `_str_u*`, `_text_all`, `ascii`, `cast`, `error`, `fmt`, `join`, `prim`, `split`, `str`.
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
- add flat re-exports of root modules to `_info`.
- update github CI workflows
  - bump `actions/checkout` to v5.
  - add more `no_std` targets, retry downloads and disable fail-fast.


-----------
> *Modules* :
-----------

## code
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
  - traits: `Chain`, `ExtOption`, `ExtOptRes`, `ExtResult`, `Hook`.
  - types: `Mismatch`, `OptRes`, `OptionFmt`, `OptionFmtOr`, `OptionFmtOrElse`, `Own`.
- move to [base_num]: `ValueQuant`.
- update `unwrap!`: add arms: `some_if*`, `ok_map_err`, `ok_if*`.

### utils
- new struct: `Lut`.
- new macros: `doclink!`, `lets!`, `mod_path!`, `repeat!`, `whilst!`, `write_at!`.
- move to [base]:
  - public macros: `CONST!`, `assert_eq_all!`, `assert_approx_eq_all!`, `capture_first!`, `capture_last!`, `capture_tail_tuple!`, `cfg_if!`, `const_assert!`, `define_error!`, `deprecate!`, `enumset!`, `ident_const_index!`, `impl_trait!`, `include_from!`, `is!`, `items!`, `maybe!`, `methods_as_fns!`, `mod_from!`, `sf!`, , `structural_mods!`, `type_marker!`.
  - internal macros: `__dbg!`, `__std!`, `_EMOJI_*`, `_TAG_*`, `_doc!`, `_doc_availability!`, `_doc_miri_warn!`,  `_reexport!`, `_use!`.
- add tags: `_DOC_*`, `_TAG_[CODEC|CODEGEN_BUILD|CONCURRENCY|DATA|DEBUG|EVENT|EXAMPLE|HASH|ID|PROC_MACRO]`.
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

## data
- move to [base]:
  - macros: `array_init!`, `bitfield!`.
  - types: `ArrayFrom`, `Bitwise`, `Oneof`, `Sort`.
- new `SortAlloc` wrapper for `Sort`.
- make `Sort` methods take `&mut self` instead of `self`.
- make `Sort` public `quick_*` methods take `&mut self` as well.
- update `array_init!` to require `ConstDefault` and `Vec` in scope if needed.

### key
- update `define_static_map!`:
  - support custom attributes and visibility.
  - add example items: `ExampleStaticMapConstU8`, `ExampleStaticMapTypeId`, `ExampleStaticMapU16`.
  - improve docs.

### list
- move to [base]:
  - type `ConstList`.
- make all `bitfield!` methods consts.

## lang
- rename `lang::ling` to `lang::hum`.
- rename `lang::ling::grammar` to `lang::hum::gram`.
- move `lang::i18n` to `lang::hum::i18n`.

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

## num
- move to [base]:
  - aliases: `fsize` and `[iu]size_*`.
  - all data, numeric, text & time error types.
  - macros: `const_bool!`, `ne!`, `nz!`.
  - types: `Cast`, `Cmp`, `Cycle`, `CycleCount`, `False`, `Int`, `Interval`,  `NonExtreme*`, `NonValue*`, `Sign`, `True`.
  - traits: `ConstBool`.
- new types: `IntAlloc`, `IntError`, `IntResult`, `NicheValueError`.
- move float shared docs to `devela_base_num` prefixed with `_FLOAT_`.
- move int shared docs to `devela_base_num` prefixed with `_INT_`.
- un-gate `Compare` impls and many dependent const methods.
- update `Interval` to use individual `IncompatibleBounds` error.
- fix `Cast` wrapping methods performance, and correctness for negative integers.
- move `ValueQuant` from `code::result` to `num::quant`.
- make std `Float` methods `trunc`, `fract` & `split` *const*.
- make all `Int` methods *const*.
- make `Sign` part of `quant`.
- rename `Compare` to `Cmp`.

### geom

#### metric
- impl 2d|3d common methods for all `num::geom::metric` types.
  - update `_impl_metric!` helper macro.
- update `Extent`:
  - add methods for 2|3d`length[_ref|_mut]`, `width[_ref|_mut]`, `height[_ref|_mut]`, `depth[_ref|_mut]`, `breadth[_ref|_mut]`.
- remove `c_` prefix from int methods.

### niche
- new macro: `nv!`.
- new type: `NonNiche`.

## phys
### time
- remove `TimeError` alias.

## sys
### arch
- new `Arch` methods: `cntvct`, `cycles`, `rdcycle`, `rdtsc`, `rdtscp`.
- new internal macro `ARCH!`.

### env
- vendor `argv` as `IterArgSOsRef` struct and `Env` method `args_os_ref`.

### log
- new type `LoggerStatic`.
- new macro `slog!`.

### mem
- new alias: `MaybeByte`.
- new types: `ArenaHandle`, `ExampleArena`.
- new macros: `define_arena`, `slice!`.
- move to [base]:
  - macros: `cswap!`, `const_join!`.
  - traits: `MemAligned`.
  - types: `CacheAlign`, `Mem`, `Ptr`, `Slice`.
- rename `join!` macro to `const_join!`.
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

## os
### linux
- new enum `LinuxClock`.
- new `Linux` methods: `clock_getres`, `clock_gettime`, `exit`.
- new `Linux` syscalls: `sys_clock_getres`, `sys_clock_gettime`.
- fix `Linux`-related warnings & avoid use of `transmute`.
- rename syscalls doc constants, prefix with `_DOC_`.
- update `LinuxError`:
  - new `Other` variant.
  - impl `Error`.
- improve `LinuxTimespec`.
  - impl `Display` and `ConstDefault`
  - rename method `with` to `try_with_duration` and make fallible.
  - add corresponding method `try_to_duration`.
  - add saturating methods to convert from/to `Duration`.
  - add corresponding conversions and methods from/to `TimeDelta`.

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
- impl `ConstDefault` for `char*`.
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

## ui
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


[0.25.0]: https://github.com/andamira/devela/releases/tag/v0.25.0
