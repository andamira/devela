# devela changelog

[0.25.0-wip] unreleased
=======================

-----------
> *Project* :
-----------

## build
- move `/meta/build` back to `/build`.

## cargo
- add new cargo doc workspace aliases: `dw*`.

## dependencies
- new workspace library dependency: `devela_base`.
- bump dependencies:
  - `ureq` → 3.1.
  - [macros]
    - `proc-macro2` → 1.0.101.
    - `quote` → 1.0.40.

## features & flags
- new `_lib··` flag
- new features = `lib_all``, `lib_base`.

## libs
- use a single version, changelog and readme for all workspace libs.
  - move `devela_macros` changelog into `devela` archived changelog history.
- [base]:
  - move devela macros: `CONST!`, `cdbg`, `include_from!`, `is!`, `items!`, `mod_from!`, `sf!`.
  - move internal (for the workspace) devela macros: `EMOJI_*`, `TAG_*`.
  - move `paste` dependency.
  - add build script.
    - add `cargo_primary_package` flag.

## manifest
- add *binaries* and *metrics* sections.

## metrics
- rename directory `/benches` → `/metrics`.

## tools & misc. files
- new file `config/lib_all`.
- update `tools/check.rs`:
  - configure the exact nightly version to install and use.


-----------
> *Modules* :
-----------

## code
- remove deprecated `iif!` macro.
- remove temporary value binding functionality from `is!` macro, unnecessary after rust v1.89.
- add doc tags: `TAG_DEVELA_[BASE|DATA|MACROS|NUM]`.


[0.25.0]: https://github.com/andamira/devela/releases/tag/v0.25.0
