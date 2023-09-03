# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to
[Semantic Versioning].

## [Unreleased]

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

[unreleased]: https://github.com/andamira/devela_macros/compare/v0.6.1...HEAD
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
