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

## cargo
- add new cargo doc workspace aliases `w*`.

## dependencies
- new workspace library dependency: `devela_base`.
- bump dependencies:
  - `ureq` → 3.1.
  - [macros]
    - `proc-macro2` → 1.0.101.
    - `quote` → 1.0.40.

## features & flags
- new `_lib··` flag
- new features: `lib_all``, `lib_macros`.
- new debug feature: `__force_test_no_mangle`.

## libs
- enable `_docsrs` for workspace dependencies.
- new workspace library crates: `devela_base`, `devela_base_macros`, `devela_base_std`.
- use a single version, changelog and readme for all workspace libs.
  - move `devela_macros` changelog into `devela` archived changelog history.
  - move `paste` dependency to [base].
- [base]
  - add `_workspace_private` structural module (replacing `_internal`).
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
- make keys parts of the workspace: edition, version, authors, license, documentation.
- add *binaries* and *metrics* sections.
- add workspace hiearchy diagram.

## metrics
- rename directory `/benches` → `/metrics`.

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
- remove items: `AllError`, `AllResult`, `DataError`, `DataResult`, `ExtError`.

### utils
- move to [base]:
  - public macros `CONST!`, `cdbg`, `define_error!`, `include_from!`, `is!`, `items!`, `mod_from!`, `sf!`.
  - hidden macros: `EMOJI_*`, `TAG_*`.
- add doc tags: `TAG_DEVELA_[BASE[_MACROS|ALLOC|STD]|DATA|MACROS|NUM]`.
- update `CONST!` macro with new arms: `hidden macro_export`, `inline macro_export`.
- remove temporary value binding functionality from `is!` macro, unnecessary after rust v1.89.
- remove deprecated `iif!` macro.

## sys
### env
- vendor `argv` as `IterArgSOsRef` struct and `Env` method `args_os_ref`.

[0.25.0]: https://github.com/andamira/devela/releases/tag/v0.25.0
