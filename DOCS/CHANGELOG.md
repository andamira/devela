# Changelog from 2025 on

[0.24.0-wip] unreleased
=======================

## manifest
- changed:
  - bump MSRV to 1.86.0.

## documentation
- improve rustdoc header loading, make loading more reliable.
- fix multiple katex warnings.
- new doc tag: `TAG_NICHE`.

## examples
- delete the `niche` example.

## utils
- change `check.rs`:
  - add new args: `-A` | `--install-arches`, `-N` | `--install-nightly`
  - behavior change: no longer installs components automatically.

---

## code
### result
- add re-exports: `core::option::*`, `core::result::*`.

## data
- add re-exports: `IterFromCoroutine`.
- add modules: `data::list::of`.
- relocate and rename `code::result::Enum` → `data::list::of::Oneof`.
- rename `Oneof` methods to ordinals: `A` → `_0`, `B` → `_1`, ….

## game
- add:
  - module: `game`.
  - features: `game`, `game_safe`.
  - reflection flags: `game··`.

## num
### float
- add:
  - new `Float` methods: `classify`, `next_down`, `next_up`.

### niche
- make `impl_non_value!` private.
- pre-generate all `NonValue*` types.
- add `Non[Extreme|Value][I|U]size` types.
- add new method to `NonValue*`: `new_lossy`.
- improve the efficiency of `NonValue*<MAX>`.

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
