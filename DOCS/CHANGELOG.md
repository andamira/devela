# Changelog

[0.24.0-wip] unreleased
=======================

## manifest
- bump MSRV to 1.88.0.
- convert library to standalone, remove workspace config.
- bump dependencies:
  - `allocator-api2` → 0.3.
  - `bumpalo` → 3.18.
  - `bytemuck` → 1.23.
  - `crossterm` → 0.29.
  - `libm` → 2.15.
  - `pyo3` → 0.25.1.
  - `rodio` → 0.21.1.
  - `sdl2` → 0.38.
  - `sysinfo` → 0.36.
  - `toml_edit` → 0.23.
  - `tokio` → 1.47.
- disable dependencies: `sdl3`.
- disable `nightly_autodiff` flag.
- remove `std` requirement for optional dependencies: `crossterm`, `pyo3`, `regex_lite`, `sysinfo`.

## documentation
- improve rustdoc header loading, make loading more reliable.
- fix multiple katex warnings.
- new doc tag: `TAG_NICHE`.

## examples
- new `js_web_worker` example.
- delete the `niche` example.

## utils
- change `check.rs`:
  - add new args: `-A` | `--install-arches`, `-N` | `--install-nightly`
  - behavior change: no longer installs components automatically.

---

## code
- new trait `Introspect`.
- new doc tag: `TAG_ALLOCATOR`.
- rename `set_panic_handler!`'s macro `web_api` arm to `web`.

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

## lang
### ffi
- new types: `Js`, `JsConsole`, `JsValue`, `WebDocument`, `WebWindow`, `WebWindowState`.
- move and rename the `js_str*` fns as public `Js` `read_str*` methods.
- new `Web` methods:
  - `console_count[_reset]`.
- rename types:
  - `Js` to `Web`.
  - `JsEvent*` to `WebEvent*`.
  - `JsPermission*` to `WebPermission*`.
  - `JsWorker*` to `WebWorker*`.

## media
- delete: `[Audio|Color|Draw|Font|Media|Midi][Result|Error]`.

### color
- add types: `Gamma`, `Lum`, `Rgb`, `Rgba`.
  - implement for `u8`, `u16`, `f32`, `f64`.
- add aliases for different `Rgb` color types.
- add `Lum` sub-type aliases: `Lightness`, `LinearLightness`, `Luma`, `Luminance`.
- add module: `media::color::rgb`.
- remove the `Color` namespace.
  - move all its functionality to `Gamma`.
- rename the `ColorBase` trait to `Color`.
- update the `Color` trait:
  - make type `Component` bound on `NumConst`.
  - add constants: `COLOR_[BITS|COUNT|HAS_ALPHA]`, `COLOR_IS_[INT|LINEAR|PREMUL]`.
  - add methods: `color_[bits|has_alpha]`, `color_[red|green|blue|alpha]`, `color_is_[int|linear|premul]`.

## num
- update the `NumConst` trait.
- require the trait bound `PartialEq<Self::Num>`.
- make all its associated constant values be `Option`al.
- add consts: `NUM_[MAX|MIN][_NORM]`, `NUM_IS_[BIG|INT|FLOAT|FIXED|SIGNED|NICHE]`.
- add auto-implemented methods over `&self`, to query the associated constant values.

### float
- add new `Float` methods: `classify`, `next_down`, `next_up`.
- delete the `alg` feature.

### geom
#### metric
- rename `Extent` field `size` to `dim` for consitency.
- add missing attributes `must_use` and `repr(transparent)`.
- remove type aliases: `Extent2d`, `Extent3d`, `Region2d`, `Region3d`.
- remove `metric` feature-gate for `Distance`, `Extent` and `Position`.
- impl `From` arrays and tuples for `Distance`, `Extent`, `Orientation`, `Position` and `Stride`.

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

### arch
- new `Wasm` methods: `heap_base`, `remaining_memory`.
- change `Wasm::memory_grow` to have the same signature as `core::arch::wasm32::memory_grow`.
- add support for new:
  - architectures: `amdgpu`.
  - OS targets: `amdhsa`, `cygwin`, `psx`.
  - vendor targets: `amd`, `mti`, `openwrt`.

### mem
- new `Mem` methods: `align_down`, `align_up`, `is_aligned`, `is_aligned_to`.
- remove macros: `addr_of!`, `addr_of_mut!`.

#### alloc
- new types: `BumpAlloc`, `WasmAlloc`.
- vendor `mini-alloc` as `WasmAlloc`.

#### borrow
- new types: `Backing`, `MaybeOwned`.
- new trait: `Ownership`.

### os
#### linux
- add methods:
  - `Linux`: `print[ln]_unchecked[_fast]`, `eprint_bytes`.

## ui
### layout
- delete:
  - types: `LayoutError`, `LayoutResult`.

[0.24.0-wip]: https://github.com/andamira/devela/releases/tag/v0.23.0...HEAD
