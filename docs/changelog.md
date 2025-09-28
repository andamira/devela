# devela changelog

[0.25.0-wip] unreleased
=======================

> A place for everything, and everything in its place.

This release changes the library's structure from a single crate to multiple crates,
in order to improve compile times while maintaining most of the gained cohesiveness.
Many feature gates are removed in order to make most features make always available.

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
- re-export `alloc` crate from devela and [base_alloc].
- bump dependencies:
  - `ffmpeg-the-third` to 4.0.
  - `hashbrown` to 0.16.
  - `pyo3` to 0.26.
  - `stringzilla` to 4.0.
  - `ureq` to 3.1.
  - [macros]
    - `proc-macro2` to 1.0.101.
    - `quote` to 1.0.40.
- add optional dependencies to [base]: `memchr`, `simdutf8`.
- remove `_core` and `_dep` re-exports from the public docs.

## features & flags
- new features: `__publish`, `base_safe`, `safe_build`.
- remove features: `_bit*`, `_char*`, `_cmp*`, `_float_*`, `_int_*`, `_num?_all`, `_sort*`, `_str_*`, `_str_nonul`, `_str_u*`, `_text_all`, `ascii`, `cast`, `error`, `fmt`, `join`, `prim`, `split`, `str`.
- remove flags: `bit··`, `char··`, `cmp··`, `_float··`, `_int*··`, `_nums··`, `prim··`, `sort··`, `str··`, `str_u··`.
- add an adittional `nightly_stable_1_??` flag for the 3rd next version.
- rename `_docs` to _`docs_min`, `_docsrs` to `_docs`, `_docsrs_nodep` to `_docs_nodep`.
- add default feature `alloc` to [base_alloc].
- add default feature `std` to [base_std].

## workspace libraries
- declare the `std` external crate.
- remove `_always` structural modules.
- refactor all structural access modules.
- enable `_docsrs` for workspace dependencies.
- support having external optional dependencies.
- new workspace library crates: `devela_base_alloc`, `devela_base_core`, `devela_base_macros`, `devela_base_num`, `devela_base_std`.
- prepare future workspace library crates related to root modules.
- move `core`, `alloc` & `std` re-exports to [base*] libs.
- use a single version, changelog and readme for all workspace libs.
  - move `devela_macros` changelog into `devela` archived changelog history.
  - move `paste` dependency to [base].
- [base]
  - add `_workspace_internal` structural module (replacing `_internal`).
- [base_macros]:
  - move devela_macros macros: `devela_macros`: `cif!`, `compile!`, `compile_attr!`, `ident_total!`, `ident_total_unique!`, `ident_unique!`, `coalesce!`, `field_of!`.
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
- update `tools/check.rs`:
  - bump `devela` to 0.24.0.
  - test all workspace crates.
  - start testing without dependencies.
  - remove `itertools` direct dependency.
  - simplify and homogeinize toolchain selection syntax.
  - configure the exact nightly version to install and use.
- update `config/rustdoc-header.html` to support multiple crates with custom whitelists.
- move `/config/dep_all.rs` to `/build/main/dep_all`.
- update github CI workflows
  - bump `actions/checkout` to v5.
  - add more `no_std` targets, retry downloads and disable fail-fast.


-----------
> *Modules* :
-----------

## code
### error
- update `define_error!` macro.
  - move to `code::error`.
  - update docs, add example.
  - allow accepting multiple tags.
  - make conversion method optional const.
- remove items: `AllError`, `AllResult`, `DataError`, `DataResult`, `ExtError`.

### panic
- move to [base]: `Panic`.

### result
- move to [base]:
  - functions: `serr`, `sok`.
  - macros: `unwrap!`.
  - traits: `Chain`, `ExtOptRes`, `Hook`.
  - types: `Mismatch`, `OptRes`, `Own`.
- move to [base_num]: `ValueQuant`.

### utils
- new macros: `doclink!`, `mod_path!`.
- move to [base]:
  - public macros: `CONST!`, `assert_eq_all!`, `assert_approx_eq_all!`, `capture_first!`, `capture_last!`, `capture_tail_tuple!`, `cfg_if!`, `cfor!`, `const_assert!`, `define_error!`, `deprecate!`, `enumset!`, `ident_const_index!`, `impl_trait!`, `include_from!`, `is!`, `items!`, `maybe!`, `methods_as_fns!`, `mod_from!`, `sf!`, , `structural_mods!`, `type_marker!`.
  - internal macros: `_EMOJI_*`, `_TAG_*`, `_doc!`, `_doc_availability!`, `_doc_miri_warn!`,  `_reexport!`, `_use!`.
- add tags: `_DOC_*`, `_TAG_[CODEC|CODEGEN_BUILD|CONCURRENCY|DATA|EVENT|HASH|ID|PROC_MACRO]`.
- change the emoji for `_TAG_DATA_STRUCTURE`.
- rename `reexport!` internal macro to `_reexport!`.
  - allow accepting multiple tags.
- prefix internal constants `TAG_*` & `EMOJI_*` with `_`
- define `_std_core` separately and privately per crate.
- update `CONST!` macro with new arms: `hidden macro_export`, `inline macro_export`.
- add `location` arms to the `_doc!` macro.
- update `const_assert!` macro:
  - add new arms: `ne_buf`, `ne_str`.
  - add support for comparing slices of primitives and slices of slices of primitives.
- remove temporary value binding functionality from `is!` macro, unnecessary after rust v1.89.
- remove deprecated `iif!` macro.

## data
- move to [base]:
  - macros: `array_init!`, `bitfield!`.
  - types: `ArrayFrom`, `Bitwise`, `Sort`.
- new `SortAlloc` wrapper for `Sort`.
- make `Sort` methods take `&mut self` instead of `self`.
- make `Sort` public `quick_*` methods take `&mut self` as well.
- update `array_init!` to require `ConstDefault` and `Vec` in scope if needed.

### list
- move to [base]:
  - type `ConstList`.
- make all `bitfield!` methods consts.

## lang
- rename `lang::ling` to `lang::hum`.
- rename `lang::ling::grammar` to `lang::hum::gram`.
- move `lang::i18n` to `lang::hum::i18n`.

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

## phys
### time
- remove `TimeError` alias.

## sys
### arch
- new `Arch` methods: `cntvct`, `cycles`, `rdcycle`, `rdtsc`, `rdtscp`.
- new internal macro `ARCH!`.

### env
- vendor `argv` as `IterArgSOsRef` struct and `Env` method `args_os_ref`.

### mem
- move to [base]:
  - macros: `cswap!`, `const_join!`.
  - traits: `MemAligned`.
  - types: `Mem`, `Ptr`, `Slice`.
- rename `join!` macro to `const_join!`.
- new macro `slice!`.
- update `Slice`:
  - rename methods:
    - `trim_leading_bytes` to `trim_leading`
    - `replace_leading_bytes` to `replace_leading`.
  - add new methods:
    - `range_to_inclusive*`.
    - `trim_leading_keep`, `trim_leading_min_len`, `trim_trailing`, `trim_trailing_keep`, `trim_trailing_min_len`, `trim_edges_keep`, `trim_edges_min_len_left`, `trim_edges_min_len_right`.
  - add new `eq` methods for slices of slices of primitives and string slices.

## os
### linux
- new enum `LinuxClock`.
- new `Linux` methods: `clock_getres`, `clock_gettime`.
- new `Linux` syscalls: `sys_clock_getres`, `sys_clock_gettime`.
- fix `Linux`-related warnings & avoid use of `transmute`.
- rename syscalls doc constants, prefix with `_DOC_`.
- new `LinuxError::Other` variant.
- improve `LinuxTimespec`.
  - impl `Display` and `ConstDefault`
  - rename method `with` to `try_with_duration` and make fallible.
  - add corresponding method `try_to_duration`.
  - add saturating methods to convert from/to `Duration`.
  - add corresponding conversions and methods from/to `TimeDelta`.

## text
- move to [base]:
  - traits: `NumToStr`, [`UnicodeScalar`].
  - types: `AsciiDigits`, `ByteSearch`, `GraphemeNonul`, `GraphemeU*`, `Str`, `StringNonul`, `StringU*`, `char7`, `char8`, `char16`.

### ascii
- rename `Ascii` to `AsciiDigits`.
  - rename method: `calc_digit` to `digit_at_power`.
- make `ASCII_TABLE` public.

### char
- new `IterChars` iterator.
- make `text::char` module public.
- new `char7` methods: `to_byte`, `to_str`.
- rename `char*` methods: `to_u32` to `to_scalar`.
- remove previously re-exported `IterChars` (renamed to `IterCharsStd`).
- update `Char`:
  - change `to_ascii_fold` to convert `Æ|Œ` to `E` & `æ|œ` to `e`.
  - remove deprecated methods: `len_to_utf8`, `utf8_?bytes_len`.
  - make it a tuple struct with a single a generic parameter.
  - add methods: `decode_surrogate_pair`, `has_overlong_encoding`, `has_valid_continuation`, `is_surrogate*`, `is_utf8_boundary`, `is_valid_code`, `to_char`, `to_char_lenient`, `to_char_unchecked`, `utf8_len_match`, `utf8_len_match_naive`.
  - rename methods:
    - `is_7bit` to `is_ascii`.
    - `is_valid` to `is_valid_scalar`.
    - `to_code*` to `to_scalar*`.
    - `utf8_len` to `utf8_len_unchecked`.
    - `utf8_len_checked` to `utf8_len`.
  - remove `utf8_bytes_` prefix from `Char<&[u8]>` methods…
  - add private consts: `CONT_MASK` `UTF8_CHAR_LEN`.
  - remove `code_` prefix from `Char<u32>` methods.
  - rename method `byte_len` to `len_bytes`.
  - modify all methods to take `self`.
  - return lenghts as usize.
- update `UnicodeScalar`:
  - rename method `byte_len` to `len_bytes`.

### fmt
- move to [base]:
  - macros: `format_buf!`.
  - types: `FmtWriter`.
- new type: `FmtWriter`.

### str
- remove methods: `to_cstring`, from `String*` & `Grapheme*`.
- make more methods *const* in `GraphemeU8`.
- make `chars` methods *const* when possible.
- update `Str:`
  - add methods for returning substrings in compile time: `range*` `take*`, `*split*`.
  - remove method `from_boxed_utf8_unchecked`.
- update `StringNonul`:
  - new method `new_checked`.
  - make `new` method panic.
  - make **all** methods *const*.
  - impl `Extend` & `FromIterator`.
- update `StringU*`:
  - impl `AsMut<&str>`, `DerefMut`, `Extend`, `FromIterator`.
  - new methods: `from_str`, `from_str_truncate`, `from_str_unchecked`, `new_checked`, `pop_unchecked`, `sanitize`.
  - modify methods:
  - `as_mut_str`: make safe.
  - `push_str`, make const, improve efficiency, update docs & examples.
  - `try_push_str*`, make const, return `Result<usize, usize>`, update docs & examples.
  - make `new` method panic.
  - make **all** methods *const*.
  - impl `Extend` & `FromIterator`.
  - fix `TryFrom<&str>` impl.
  - improve docs.

## ui
### front
#### term
- move to [base]:
  - types: `Ansi`, `AnsiColor3b`, `AnsiColor8b`, `TermSize`.
- change `Ansi::print*` methods to `ansi_print*` functions.
- make all `Ansi` escape-sequence methods *const*.
- fix visibility of `ansi!` macro.


[0.25.0]: https://github.com/andamira/devela/releases/tag/v0.25.0
