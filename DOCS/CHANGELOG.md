# Changelog from 2025 on

[0.24.0-wip] unreleased
=======================

## manifest
- changed:
  - bump MSRV to 1.86.0.

## utils
- change `check.rs`:
  - add new args: `-A` | `--install-arches`, `-N` | `--install-nightly`
  - behavior change: no longer installs components automatically.

## code
- relocate and rename `code::result::Enum` → `data::list::Oneof`.

## data
- add:
  - re-exports: `IterFromCoroutine`.

## game
- add:
  - module: `game`.
  - features: `game`, `game_safe`.
  - reflection flags: `game··`.

## num
### float
- add:
  - new `Float` methods: `classify`, `next_down`, `next_up`.

## sys
### os
#### linux
- add methods:
  - `Linux`: `print[ln]_unchecked[_fast]`, `eprint_bytes`.

### arch
- add support for new:
  - architectures: `amdgpu`.
  - OS targets: `amdhsa`, `cygwin`, `psx`.
  - vendor targets: `amd`, `mti`, `openwrt`.

## ui
### layout
- delete:
  - types: `LayoutError`, `LayoutResult`.

[0.24.0-wip]: https://github.com/andamira/devela/releases/tag/v0.23.0...HEAD
