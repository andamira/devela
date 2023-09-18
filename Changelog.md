# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to
[Semantic Versioning].

## [Unreleased]

### Added
- new structs `LINUX_SYS_X86_64`, `LINUX_SYS_X86`, `LINUX_SYS_ARM`, `LINUX_SYS_AARCH64`, `LINUX_SYS_RISCV`, `LINUX_SIGACTION`, `LinuxSigaction`.
- new functions: `linux_sys_getrand`, `linux_sys_rt_sigaction`, `linux_random_bytes`, `linux_random_u8`, `linux_random_u16`, `linux_random_u32`, `linux_random_u64`, `linux_random_i128`.
- add `linux_sys_nanosleep` for `riscv`.
- add missing `LINUX_ERRNO` values.
- new type alias `LINUX_SYS`.
- new `check_miri.sh` script.
- new dependency: `const-str`.
- reexport renamed const-str macros from `ascii` and `str` modules.
- new macros: `os_print`, `os_println`, `os_eprint`, `os_eprintln`.
- add scripts to get linux syscalls and error numbers as rust constants.
- add CI actions to get `syscall.h` and `errno.h` values for each target.
- new `atomic` dependency.
- reexport `atomic::Atomic` type.
- new `sync` module and `atomic` submodule.
- reexport `core::sync::atomic::{fence, Ordering}` as `atomic_fence` and `AtomicOrdering`, respectively.
- reexport `core::num::NonZero*` types from `num` module.

### Removed
- remove the already deprecated macros `cdbg`, `rfs`.
- remove the already deprecated feature `no-std`.

### Changed
- rename `SysTimeSpec` to `LinuxTimespec`, `SysTermios` to `LinuxTermios`.
- rename `ERRNO` to `LINUX_ERRNO`, `IOCTL` to `LINUX_IOCTL`.
- rename `_features` module to `_doc`.
- move check scripts to `/tools/`.

### Fixed
- fix `ioctl` and `nanosleep` syscalls for `aarch64`.
- fix `nanosleep` syscall for `arm`.
- fix `ioctl` syscall for `riscv`.
- refactor modules: `ascii`, `codegen`, `option`, `result`, `str`.
- ensure dependencies doesn't include default features.
- update .gitattributes to show all languages.
- move lengthy `target_arch` rustdoc tags to the doc-comments.
- update documentation on reexported types.

## [0.10.0] - 2023-09-13

### Added
- new features: `unsafe_mem`, `unsafe_str`.
- new modules : `ascii`, `os::terminal`, `str`, `_features`.
- new fns in `ascii:`: `ascii_d[1-4]`, `u[BITS]_to_ascii`.
- new fns in `mem:` `as_bytes`, `as_bytes_mut`, `as_bytes_sized`.
- new fns in `os::linux`: `sys_ioctl`.
- new fns in `os::terminal`: `is_terminal`, `enable_raw_terminal`, `disable_raw_terminal`, `eprint`, `eprintln` `get_char`, `get_dirty_char`, `get_line`, `get_utf8_bytes`, `get_str`, `pause_until_char`, `println`, `prompt`.
- new structs in `os::linux`: `ERRNO`, `FILENO`, `IOCTL`, `SysTermios`.
- new struct in `os::terminal`: `Ansi`, `AnsiColor`.
- new methods for `SysTimeSpecs`: `as_ptr` and `as_mut_ptr`.
- new trait `StrExt`.
- new macro: `sfb`.

### Changed
- rename and deprecate `rfs` macro for `sf`.
- change `SysTimeSpec` to accept arguments in `new` fn, and derive `Default`.
- move `os::linux` file descriptor constnts to `FILENO` struct.
- make `bytemuck` non-optional.

### Fixed
- use `isize` instead of `i32` for `os::linux` `errno` constants.
- use `c_int` and `c_ulong` instead of `i32` and `u64` for `fd` constants and syscall args.
- remove cfg check for `os::linux` to be `no_std` compatible.
- move documentation on features to `_features`.
- move derived works info to a separate file.
- avoid loading `sys_nanosleep` in risc-v.
- fix `az` and `bytemuck` reexports.
- fix arm assembly.

## [0.9.0] - 2023-09-08

### Added
- new features: `full`, `unsafe_os`.
- add `full` to `nightly_docs` feature.
- new `os` module, with `linux` submodule.
- new functions: `sys_exit`, `sys_read`, `sys_write`, `sys_nanosleep`, `get_byte`, `print`, `print_bytes`.
- new struct `SysTimeSpec`.
- add `bytemuck`'s feature `extern_crate_std` to `std`.

### Removed
- remove the `Copy` trait from `IntBuf`.
- remove the `boxed` module and `bx` fn.
- remove the `string` module and `S` macro.
- remove `unsafe_num` from the `bytemuck` feature.
- remove `bytemuck` from the `unsafe` feature.

### Changed
- deprecate and rename `cdbg` macro to `bdbg`.

### Fixed
- fix the `iif` module.
- improve the `num` docs.
- improve docs related to features.
- refactor manifest and update comments.
- separate lengthy module level docs into markdown files.

## [0.8.0] - 2023-08-29

### Added
- add features: `safest` and `unsafe`.
- new `deprecate_feature` macro.
- new `sleep4` macro.

### Removed
- remove `num::*` module reexport from the prelude.
- remove fns `counter_string`, `indent`, macro `iformat` (move to `textos`).

### Changed
- deprecate feature `no-std` for `no_std`.
- bump MSRV to `1.72.0`.

### Fixed
- fix cargo aliases.
- fix external reexports.


## [0.7.1] - 2023-08-23

### Changed
- unhide the macros from the root.
- reexport `paste` with wrapped documentation.

## [0.7.0] - 2023-08-23

### Added
- new number types: `NonRange*`, `Range*`.
- new `SliceExtMut` trait.
- new `OptionExt` methods: `fmt_or`, `fmt_or_else`, `fmt_or_empty`.
- new structs: `OptionFmtOr`, `OptionFmtOrElse`, `OptionFmtOrEmpty`.
- new `codegen` module; reexport the `compile` and `paste` macros from there.
- add categories: `development-tools`, `no-std::no-alloc`.

### Removed
- avoid reexporting external macros from the root and hide the root documentation for the rest.

### Changed
- implement `InBufAble` for `NonRange*` and `Range*`.
- rename feature `unsafe_non_specific` to `unsafe_num`
- move mutable methods from `SliceExt` to the new `SliceExtMut` trait.
- implement the slice extension traits for slice references, arrays and vecs.
- reexport used bytemuck types in `all`.
- include `bytemuck` in `unsafe`.
- bump MSRV to `1.71.1`.
- rename `unsafe_*` features: `unsafe_int_buf` to `unsafe_fmt`, `unsafe_cmp_float` to `unsafe_cmp`, `unsafe_uninit_array` to `unsafe_convert`.
- remove `std` requirement for `cdbg` macro.
- bump `devela_macros` to `0.5.0`.
  - reexport new macros: `compile_attr` and `cif`.

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
- add cargo categories.

### Changed
- enable cargo publish.
- update docs.

## [0.1.0] - 2022-12-17

### Added
- add functions `pclamp`, `pmax`, `pmin`, `project_root_path`,
  `project_root_path_string`.
- add macro `iif`.


[unreleased]: https://github.com/andamira/devela/compare/v0.10.0...HEAD
[0.10.0]: https://github.com/andamira/devela/releases/tag/v0.9.0
[0.9.0]: https://github.com/andamira/devela/releases/tag/v0.9.0
[0.8.0]: https://github.com/andamira/devela/releases/tag/v0.8.0
[0.7.1]: https://github.com/andamira/devela/releases/tag/v0.7.1
[0.7.0]: https://github.com/andamira/devela/releases/tag/v0.7.0
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
