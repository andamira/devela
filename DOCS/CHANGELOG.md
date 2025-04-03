# Changelog from 2025 on

[0.24.0-wip] unreleased
=======================

### Added
- new methods:
  - `Float`: `classify`, `next_down`, `next_up`.
  - `Linux`: `print[ln]_unchecked[_fast]`, `eprint_bytes`.
- new re-exports:
  - `IterFromCoroutine`.
- new target arches: `amdgpu`.
- new target oses: `amdhsa`, `cygwin`, `psx`.
- new target vendors: `amd`, `mti`, `openwrt`.

### Removed
- remove types:
  - ui:
    - layout: `LayoutError`, `LayoutResult`.

### Changed
- bump MSRV to v1.86.0.
- Update `check.rs` script
  - do not install components automatically.
  - add new args: `-A` | `--install-arches`, `-N` | `--install-nightly`

### Fixed

[0.24.0-wip]: https://github.com/andamira/devela/releases/tag/v0.23.0...HEAD
