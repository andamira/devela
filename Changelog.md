# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to
[Semantic Versioning].

## [Unreleased]

### Added
- new number types: `NonRange*`, `Range*`.
- new `SliceExtMut` trait.
- new `OptionExt` methods: `fmt_or`, `fmt_or_else`, `fmt_or_empty`.
- new structs: `OptionFmtOr`, `OptionFmtOrElse`, `OptionFmtOrEmpty`.

### Changed
- implement `InBufAble` for `NonRange*` and `Range*`.
- rename feature `unsafe_non_specific` to `unsafe_num`
- move mutable methods from `SliceExt` to the new `SliceExtMut` trait.
- implement the slice extension traits for slice references, arrays and vecs.
- reexport used bytemuck types in `all`.
- include `bytemuck` in `unsafe`.
- bump MSRV to `1.71.1`.
- rename `unsafe_*` features: `unsafe_int_buf` to `unsafe_fmt`, `unsafe_cmp_float` to `unsafe_cmp`, `unsafe_uninit_array` to `unsafe_convert`.

### Fixed
- improve `Debug` impl for `NonSpecific*` and `NonRange*`.
- fix unsafe features safeguarding.
- improve `num` tests.

## [0.6.1] - 2024-08-08

### Changed
- implement `IntBufable` for `NonZero*` and `NonSpecific*`.
- make traits sealed: `OptionExt`, `ResultExt`, `SliceExt`.
- update the `prelude` module to re-export `core::num::NonZero*` and `devela::num::*`.
- update the `all` module to inline every item except foreign ones.
- update information about licensing and derived works.

## [0.6.0] - 2023-08-06

### Added
- new features: `nightly_docs`, `unsafe_cmp_float`, `unsafe_int_buf`,
  `unsafe_uninit_array`, `unsafe_non_specific`.
- add `bytemuck` as an optional dependency.
- new const functions for comparing primitives: `total_cmp_*`, `max_*`, `min_*`,
  `clamp_*`.
  - includes const unsafe and non-const safe versions of functions for comparing
    floating-point primitives.
- add `IntBuf` struct.
- new traits: `FromPrimitives`, `IntoPrimitives`, `SliceExt`,  `IntBufAble`.
- add additional targets for testing `x86_64-unknown-linux-gnu`,
  `x86_64-pc-windows-msvc`, `x86_64-apple-darwin`, `x86_64-unknown-none`,
  `i686-unknown-linux-gnu`, `aarch64-unknown-none`.

### Removed
- delete `safest` feature.
- remove `safe` from the default features.

### Changed

- bump MSRV to `v1.71.0`.
- rename modules and reorganize items in a similar fashion to the rust standard library.
- constify and rename `subslice_left` to `slice_lsplit`, `subslice_right` to
  `slice_rsplit` and `subslice_middle` to `slice_msplit_right`.
- new `slice_msplit_left` function.
- improve documentation, specially regarding features and safety.

## [0.5.3] - 2023-07-24

### Changed
- implement `Default` for `NonMax*`.

## [0.5.2] - 2023-07-24

### Added
- new `safest` feature.

### Changed
- improve `NonSpecific` types:
  - implement `From` and `TryFrom` traits.
  - add `NonMax*` and `NonMin*` aliases.

## [0.5.1] - 2023-06-22

### Added
- new `NonSpecific*` wrappers over the `NonZero*` primitives.
- new `unsafe` feature.


## [0.5.0] - 2023-06-09

### Added
- new traits `AltDebug`, `OptionExt`, `ResultExt`. 
- new macros `S`, `iformat`.
- new `indent` function.

### Changed
- improve `iif` macro adding suport to `if let` expressions and empty else clauses.

## [0.4.1] - 2023-05-30

### Fixed
- fix the `cdbg` macro.

## [0.4.0] - 2023-05-23

### Added
- new functions: `slice_into_vec`, `try_slice_into_vec`, `try_vec_into_vec`,
  `vec_into_vec`, `slice_into_array`.
- new macros: `manifest_dir`, `cdbg`.

### Changed
- update MSRV to `1.63.0`.

## [0.3.0] - 2023-05-09

### Added
- reexport the `az` crate and the `paste` macro.

### Changed
- improve the `compile` attribute macro to support the `not()` option. 

## [0.2.0] - 2023-05-07

### Added
- new dependecy `devela_macros`.
- new `compile` attribute macro.

## [0.1.10] - 2023-03-29

### Fixes
- fix `alloc` compilation.

## [0.1.9] - 2023-03-29

### Added
- add `format_buf` function and macro.

## [0.1.8] - 2023-03-17

### Added
- add `alloc` and `no-std` features.

### Changed
- bump MSRV to `1.60.0`.

## [0.1.7] - 2023-03-11

### Added
- add `Also` & `Apply` traits.

## [0.1.6] - 2023-03-09

### Added
- new functions `subslice_left`, `subslice_mid`, `subslice_right`.

## [0.1.5] - 2023-03-03

### Added
- new `rfs` macro that allows skipping rust formatting.

## [0.1.4] - 2023-02-18

### Added
- add `nighly` feature

### Changed
- update the `iif` macro to support the absence of a false branch.

## [0.1.3] - 2023-02-17

### Fixed
- fix `no_std` mode.

## [0.1.2] - 2023-01-10

### Added
- add `safe` feature.

### Fixes
- minor fixes and updates.

## [0.1.1] - 2023-01-10

### Added
- add `bx` function.

### Changed
- enable cargo publish.
- add cargo categories.
- update docs.

## [0.1.0] - 2022-12-17

### Added
- add functions `pclamp`, `pmax`, `pmin`, `project_root_path`,
  `project_root_path_string`.
- add macro `iif`.

[unreleased]: https://github.com/andamira/devela/compare/v0.6.1...HEAD
[0.6.1]: https://github.com/andamira/devela/releases/tag/v0.6.1
[0.6.0]: https://github.com/andamira/devela/releases/tag/v0.6.0
[0.5.3]: https://github.com/andamira/devela/releases/tag/v0.5.3
[0.5.2]: https://github.com/andamira/devela/releases/tag/v0.5.2
[0.5.1]: https://github.com/andamira/devela/releases/tag/v0.5.1
[0.5.0]: https://github.com/andamira/devela/releases/tag/v0.5.0
[0.4.1]: https://github.com/andamira/devela/releases/tag/v0.4.1
[0.4.0]: https://github.com/andamira/devela/releases/tag/v0.4.0
[0.3.0]: https://github.com/andamira/devela/releases/tag/v0.3.0
[0.2.0]: https://github.com/andamira/devela/releases/tag/v0.2.0
[0.1.10]: https://github.com/andamira/devela/releases/tag/v0.1.10
[0.1.9]: https://github.com/andamira/devela/releases/tag/v0.1.9
[0.1.8]: https://github.com/andamira/devela/releases/tag/v0.1.8
[0.1.7]: https://github.com/andamira/devela/releases/tag/v0.1.7
[0.1.6]: https://github.com/andamira/devela/releases/tag/v0.1.6
[0.1.5]: https://github.com/andamira/devela/releases/tag/v0.1.5
[0.1.4]: https://github.com/andamira/devela/releases/tag/v0.1.4
[0.1.3]: https://github.com/andamira/devela/releases/tag/v0.1.3
[0.1.2]: https://github.com/andamira/devela/releases/tag/v0.1.2
[0.1.1]: https://github.com/andamira/devela/releases/tag/v0.1.1
[0.1.0]: https://github.com/andamira/devela/releases/tag/v0.1.0

[Keep a Changelog]: https://keepachangelog.com/en/1.0.0/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html
