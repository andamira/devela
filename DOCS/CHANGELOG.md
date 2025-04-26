# Changelog

[0.24.0-wip] unreleased
=======================

## manifest
- bump MSRV to 1.86.0.
- convert library to standalone, remove workspace config.
- bump dependencies:
  - `crossterm` → 0.29.
- remove `std` requirement for optional dependencies: `crossterm`, `pyo3`, `regex_lite`, `sysinfo`.

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
- new trait `Introspect`.

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

## media
- delete: `[Audio|Color|Draw|Font|Media|Midi][Result|Error]`.

### color
- add types: `Gamma`, `Lum`, `Rgb`, `Rgba`.
  - implement for `u8`, `u16`, `f32`, `f64`.
- add `Lum` sub-type aliases: `Lightness`, `LinearLightness`, `Luma`, `Luminance`.
- add module: `media::color::rgb`.
- remove the `Color` namespace.
  - move all its functionality to `Gamma`.
- rename the `ColorBase` trait to `Color`.
- add new constants to the `Color` trait:
  - add constants: `COLOR_COUNT`, `COLOR_IS_LINEAR`.
  - add methods: `color_is_linear`.

## num
### float
- add new `Float` methods: `classify`, `next_down`, `next_up`.
- delete the `alg` feature.

### geom
#### shape
- update `Point:` make mut accesors *const*.
- add new alias: `Points2d`.
- add new method: `Points::new`.

### quant
- new macro: `interval!`.

### niche
- add new:
  - macros: `ne!`, `nz!`.
  - types: `Non[Extreme|Value][I|U]size`.
  - methods to `NonValue*`: `new_lossy`.
- make `impl_non_value!` private.
- pre-generate all `NonValue*` types.
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
