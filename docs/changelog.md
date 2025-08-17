# devela changelog

[0.25.0-wip] unreleased
=======================

-----------
> *Project* :
-----------

## features & flags
- new `_lib··` flag
- new features = `lib_all``, `lib_base`.

## dependencies
- new workspace library dependency: `devela_base`.
- bump dependencies:
  - `ureq` → 3.1.
  - [macros] `proc-macro2` → 1.0.101.

## libs
- [base]:
  - move devela macros: `CONST!`, `is!`, `items!`, `sf!`.
  - add build script.
    - add `cargo_primary_package` flag.

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


[0.25.0]: https://github.com/andamira/devela/releases/tag/v0.25.0
