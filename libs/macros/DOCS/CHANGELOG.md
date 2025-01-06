# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to
[Semantic Versioning].

## [0.13.0] - (TBD)

## [0.12.1] - (TBD)

- new macro `field_of`.
- remove inline attributes.

## [0.12.0] - 2024-12-01

- add `hashbrown` optional dependency.
- replace `std` with `dep_hashbrown` in `default` feature.
- make `ident_unique` and `ident_total_unique` compile without `std`.
- remove support from 32 and 64 bit `enumint` representations.
- improve parsing resiliency of `enumint`.
- remove feature `__lints`.

## [0.11.0] - 2024-10-14

### Changed
- rename feature `docsrs` to `_docsrs`.
- make `compile_doc` hidden.

### Added
- new macro `enumint`.
- new features: `nightly_doc`, `__lints`.
- new dependencies: `quote`, `proc_macro2`.
- add `.rustfmt.toml`.

### Fixed
- multiple refactorings:
  - modularize macros bodies.
  - refactor manifest.
  - reformat crate.
- update CI.

## [0.10.0] - 2024-04-16

### Changed
- conditional compilation macros now panic when encountering an unrecognized compilation predicate, instead of returning `false`.
  - add explicit compilation eval branches for `"true"`, `"false"` and `""`.

### Added
- add compilation predicates: `pointer_width_eq`, `pointer_width_ne`, `pointer_width_ge`, `pointer_width_gt`, `pointer_width_le`, `pointer_width_lt`.
- add compilation predicates: `little_endian`, `big_endian`.

### Fixed
- add missing `alloc` feature gate attribute.

## [0.9.0] - 2024-02-26

### Added
- add macros: `ident_total`, `ident_unique`, `ident_total_unique`.

### Changed
- enable `std` feature by default.
- rename `nightly_docs` feature to `docsrs`.

### Removed
- remove features: `safest`, `unsafest`.

## [0.8.0] - 2024-01-15

### Added
- new `compile_doc` attribute macro.
- new `compile_doc` example.
- new `eq` and `ne` predicates compare numeric literals.

### Removed
- remove the old `ne` predicate, use `not(equal)` instead.

### Changed
- rename the old `eq` predicate to `equal`.

### Fixed
- fixed warnings.

## [0.7.1] - 2023-12-11

### Added
- new `coalesce` macro.

### Changed
- make `cif` example standalone.

## [0.7.0] - 2023-10-07

### Changed
- update MSRV to `1.72.1`.

### Fixed
- update intra-doc links.
- refactor manifest.
- update keywords.
- update docs.

## [0.6.1] - 2023-09-02

### Added
- add binary number comparisons: `ge`, `gt`, `le`, `lt`

## [0.6.0] - 2023-08-29

### Added
- new features `alloc`, `std`, `safe`, `safest`, `unsafe`, `unsafest` and `nightly`.
- make `alloc` and `safe` default features.
- add global warning config and attributes.
- add `docs.rs` package metadata.
- new `xone()` predicate.

### Changed
- rename `xome()` predicate to `xany()`.
- depend on `alloc` instead of `std`.
- include any markdown files inside `src` in the crate.

### Fixed
- update CI and readme.
- improve docs and tests.

## [0.5.0] - 2023-08-23

### Added
- add category `depevelopment-tools::procedural-macro-helpers`.
- add new attribute macro: `compile_attr`.
- add new procedural macro: `cif!`.
- add new predicates:
  - binary: `eq()`, `ne()`, `xor()`.
  - non binary: `xodd`, `xome`, `same()`, `diff()`.
- add unit tests.
- add a new example.
- update documentation.
- new feature `nightly_docs`.

## [0.4.0] - 2023-08-20

### Added
- new argument wrapper modifiers for the `compile` macro:
  `all()`, `any()`, `none()`, `some()`.
- improved docs and examples.
- add changelog.
- add CI.

### Changed
- split the project into its own repository.
- bump MSRV to `1.60.0`.

### Fixed
- update manifest repository key.

## [0.3.1] - 2023-08-06

### Added
- add MSRV to readme.
- add manifest categories and keywords.

### Fixed
- update manifest repository key.

## [0.3.0] - 2023-05-09

### Added
- add add the `not()` argument wrapper modifier to the `compile` macro.
- update docs, examples and licenses.

## [0.2.0] - 2023-05-07

### Added
- add docs and examples.

### Changed
- rename `include_block` macro to `compile`.

## [0.1.0] - 2023-05-07

### Added
- new attribute macro `include_block`.

[unreleased]: https://github.com/andamira/devela_macros/compare/v0.11.0...HEAD
[0.12.0]: https://github.com/andamira/devela_macros/releases/tag/v0.12.0
[0.11.0]: https://github.com/andamira/devela_macros/releases/tag/v0.11.0
[0.10.0]: https://github.com/andamira/devela_macros/releases/tag/v0.10.0
[0.9.0]: https://github.com/andamira/devela_macros/releases/tag/v0.9.0
[0.8.0]: https://github.com/andamira/devela_macros/releases/tag/v0.8.0
[0.7.0]: https://github.com/andamira/devela_macros/releases/tag/v0.7.0
[0.6.1]: https://github.com/andamira/devela_macros/releases/tag/v0.6.1
[0.6.0]: https://github.com/andamira/devela_macros/releases/tag/v0.6.0
[0.5.0]: https://github.com/andamira/devela_macros/releases/tag/v0.5.0
[0.4.0]: https://github.com/andamira/devela_macros/releases/tag/v0.4.0
[0.3.1]: https://github.com/andamira/devela_macros/releases/tag/v0.3.1
[0.3.0]: https://github.com/andamira/devela_macros/releases/tag/v0.3.0
[0.2.0]: https://github.com/andamira/devela_macros/releases/tag/v0.2.0
[0.1.0]: https://github.com/andamira/devela_macros/releases/tag/v0.1.0

[Keep a Changelog]: https://keepachangelog.com/en/1.0.0/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html
