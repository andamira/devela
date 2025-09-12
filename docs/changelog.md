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
- add new cargo doc workspace aliases `w*`.
- add new cargo env var `CARGO_WORKSPACE_DIR`.

## dependencies
- re-export `alloc` crate from devela and [base_alloc].
- bump dependencies:
  - `hashbrown` to 0.16.
  - `pyo3` to 0.26.
  - `ureq` to 3.1.
  - [macros]
    - `proc-macro2` to 1.0.101.
    - `quote` to 1.0.40.
- add optional dependencies to [base]: `memchr`, `simdutf8`.

## features & flags
- new features: `__build`, `__force_test_no_mangle`, `__publish`, `base_safe`, `safe_build`.
- remove features: `_bit*`, `_char*`, `_cmp*`, `_float_*`, `_int_*`, `_num?_all`, `_sort*`, `_str_*`, `_str_nonul`, `_str_u*`, `_text_all`, `ascii`, `cast`, `error`, `join`, `prim`, `split`.
- remove flags: `bit··`, `char··`, `cmp··`, `_float··`, `_int*··`, `_nums··`, `prim··`, `sort··`, `str··`, `str_u··`.
- rename `_docs` to _`docs_min`, `_docsrs` to `_docs`, `_docsrs_nodep` to `_docs_nodep`.
- add default feature `alloc` to [base_alloc].
- add default feature `std` to [base_std].

## workspace libraries
- declare the `std` external crate.
- refactor all structural access modules.
- enable `_docsrs` for workspace dependencies.
- support having external optional dependencies.
- new workspace library crates: `devela_base_alloc`, `devela_base_core`, `devela_base_macros`, `devela_base_num`, `devela_base_std`, `devela_code`, `devela_data`, `devela_media` `devela_num`, `devela_text`.
- move `core`, `alloc` & `std` re-exports to [base*] libs.
- use a single version, changelog and readme for all workspace libs.
  - move `devela_macros` changelog into `devela` archived changelog history.
  - move `paste` dependency to [base].
- [base]
  - add `_workspace_internal` structural module (replacing `_internal`).
  - remove `_always` structural modules.
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
  - configure the exact nightly version to install and use.
- update `config/rustdoc-header.html` to support multiple crates with custom whitelists.
- move `/config/dep_all.rs` to `/build/main/dep_all`.


-----------
> *Modules* :
-----------

## code
### error
- update `define_error!` macro.
  - move to `code::error`.
  - update docs, add example.
  - make conversion method optional const.
- remove items: `AllError`, `AllResult`, `DataError`, `DataResult`, `ExtError`.

### panic
- move to [base]: `Panic`.

### result
- move to [base]:
  - functions: `serr`, `sok`.
  - macros: `unwrap!`.
  - traits: `Chain`, `ExtOptRes`, `Hook`.
  - types: `Mismatch`, `OptRes`, `Own`, `ValueQuant`.

### utils
- new macros: `doclink!`, `mod_path!`.
- move to [base]:
  - public macros: `CONST!`, `assert_eq_all!`, `assert_approx_eq_all!`, `capture_first!`, `capture_last!`, `capture_tail_tuple!`, `cfg_if!`, `cfor!`, `const_assert!`, `define_error!`, `deprecate!`, `enumset!`, `ident_const_index!`, `impl_trait!`, `include_from!`, `is!`, `items!`, `maybe!`, `methods_as_fns!`, `mod_from!`, `sf!`, , `structural_mods!`, `type_marker!`.
  - internal macros: `EMOJI_*`, `TAG_*`, `_doc!`, `_doc_availability!`, `_doc_miri_warn!`,  `_reexport!`, `_use!`.
- add tags: `_DOC_*`, `TAG_[CODEC|CODEGEN_BUILD|CONCURRENCY|DATA|EVENT|HASH|ID|PROC_MACRO]`.
- change the emoji for `TAG_DATA_STRUCTURE`.
- rename `reexport!` internal macro to `_reexport!`.
- define `_std_core` separately and privately per crate.
- update `CONST!` macro with new arms: `hidden macro_export`, `inline macro_export`.
- update `const_assert!` macro
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

## num
- move to [base]:
  - aliases: `fsize` and `[iu]size_*`.
  - all data, numeric, text & time error types.
  - macros: `const_bool!`, `ne!`, `nz!`.
  - types: `Cast`, `Compare`, `Cycle`, `CycleCount`, `False`, `Int`, `Interval`,  `NonExtreme*`, `NonValue*`, `Sign`, `True`.
  - traits: `ConstBool`.
- new types: `IntAlloc`, `IntError`, `IntResult`, `NicheValueError`.
- move float shared docs to `devela_base_num` prefixed with `_FLOAT_`.
- move int shared docs to `devela_base_num` prefixed with `_INT_`.
- update `Interval` to use individual `IncompatibleBounds` error.
- un-gate `Compare` impls and many dependent const methods.
- make all `Int` methods *const*.
- make `Sign` part of `quant`.

## phys
### time
- remove `TimeError` alias.

## sys
### env
- vendor `argv` as `IterArgSOsRef` struct and `Env` method `args_os_ref`.

### mem
- move to [base]:
  - macros: `cswap!`, `const_join!`.
  - traits: `MemAligned`.
  - types: `Mem`, `Ptr`, `Slice`.
- rename `join!` macro to `const_join!`.
- update `Slice` with new `eq` methods for slices of slices of primitives and string slices.

## text
- move to [base]:
  - traits: `NumToStr`, [`UnicodeScalar`].
  - types: `Ascii`, `ByteSearch`, `GraphemeNonul`, `GraphemeU*`, `Str`, `StringNonul`, `StringU*`, `char7`, `char8`, `char16`.
- remove methods: `to_cstring`, from `String*` & `Grapheme*`.
- remove `Str::from_boxed_utf8_unchecked` method.
- new `StringU*` method: `sanitize`.
- make more methods *const* in `GraphemeU8` and `StringU8`.

## ui
### front
#### term
- move to [base]:
  - types: `Ansi`, `AnsiColor3b`, `AnsiColor8b`, `TermSize`.
- change `Ansi::print*` methods to `ansi_print*` functions.
- fix visibility of `ansi!` macro.


[0.25.0]: https://github.com/andamira/devela/releases/tag/v0.25.0
