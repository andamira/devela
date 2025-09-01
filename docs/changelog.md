# devela changelog

[0.25.0-wip] unreleased
=======================

> Refactor the project as a workspace.

-----------
> *Project* :
-----------

## build
- move `/meta/build` to `/build/main`.
- add new `devela_postbuild` crate to `build/post`.
- new `Build` namespace in `devela_base_std`.
- move build fn utils as `Build` methods.
- make `devela_base_std` optional for builds.
- add rerun instructions for changed env vars.
- make sure `CARGO_TARGET_DIR` is always defined.

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

## features & flags
- new features: `__build`, `__force_test_no_mangle`, `__publish`, `base_safe`, `safe_build`.
- remove features: `_bit*`, `_cmp*`, `_sort*`, `cast`, `error`, `join`, `prim`, `split`.
- remove flags: `bit··`, `cmp··`, `prim··`, `sort··`.
- rename `_docs` to _`docs_min`, `_docsrs` to `_docs`, `_docsrs_nodep` to `_docs_nodep`.
- add default feature `alloc` to [base_alloc].
- add default feature `std` to [base_std].

## workspace libraries
- declare the `std` external crate.
- refactor all structural access modules.
- enable `_docsrs` for workspace dependencies.
- new workspace library crates: `devela_base`, `devela_base_macros`, `devela_base_std`, `devela_base_alloc`, `devela_code`, `devela_data`, `devela_media` `devela_num`, `devela_text`.
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
  - add `devela_base` as a dependency.
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
- new file `config/lib_all`.
- update `tools/check.rs`:
  - bump `devela` to 0.24.0.
  - remove `itertools` direct dependency.
  - configure the exact nightly version to install and use.


-----------
> *Modules* :
-----------

## code
### error
- move to [base]: `Mismatch`.
- update `define_error!` macro.
  - move to `code::error`.
  - update docs, add example.
  - make conversion method optional const.
- remove items: `AllError`, `AllResult`, `DataError`, `DataResult`, `ExtError`.

### utils
- new macros: `mod_path!`.
- new internal macros: `_doclink`.
- move to [base]:
  - public macros: `CONST!`, `assert_eq_all!`, `assert_approx_eq_all!`, `cfg_if!`, `cfor!`, `define_error!`, `deprecate!`, `enumset!`, `ident_const_index!`, `impl_trait!`, `include_from!`, `is!`, `items!`, `maybe!`, `methods_as_fns!`, `mod_from!`, `sf!`, , `structural_mods!`, `type_marker!`.
  - internal macros: `EMOJI_*`, `TAG_*`, `_doc!`, `_doc_availability!`, `_doc_miri_warn!`,  `_reexport!`.
- add tags: `_DOC_*`, `TAG_DEVELA_[BASE[_MACROS|ALLOC|STD]|DATA|MACROS|NUM]`, `TAG_[CODEC|CONCURRENCY|DATA|EVENT|HASH|ID]`.
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
  - `Bitwise`, `bitfield!`, `Sort`.
- new `SortAlloc` wrapper for `Sort`.
- make `Sort` methods take `&mut self` instead of `self`.
- make `Sort` public `quick_*` methods take `&mut self` as well.

### list
- move to [base]:
  - type `ConstList`.

## num
- move to [base]:
  - aliases: `fsize` and `[iu]size_*`.
  - all data, numeric, text & time error types.
  - types: `Cast`, `Compare`, `Cycle`, `CycleCount`, `False`, `Interval`, `Sign`, `True`.
  - traits: `ConstBool`.
  - macros: `const_bool!`.
- update `Interval` to use individual `IncompatibleBounds` error.
- un-gate `Compare` impls and many dependent const methods.
- make `Sign` part of `quant`.

## phys
### time
- remove `TimeError` alias.

## sys
### env
- vendor `argv` as `IterArgSOsRef` struct and `Env` method `args_os_ref`.

### mem
- move to [base]:
  - macro `cswap!`.
- update `Slice` with new `eq` methods for slices of slices of primitives and string slices.

[0.25.0]: https://github.com/andamira/devela/releases/tag/v0.25.0
