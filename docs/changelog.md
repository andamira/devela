# devela changelog

[0.28.0-wip] unreleased
=======================

> 
> 

```
```

# Key changes:

------------------------------------------------------------------------------

# Project

## infra
### cargo
- bump MSRV to 1.95.0.

## workspace
### examples
- new example scripts: `scope_guard`, `current_guard`.

## dependencies
- bump `portable-atomic-util` to 0.2.7.
- bump `hashbrown` to 0.17.
- bump `wide` to 1.3.

## features & flags
- update the intended scope of the `_docs_examples` feature.

---

# Modules

## code
- impl `ConstInit` for more types.

### code::util
- new macro `maybe_slot!`.
- reexport `cfg_select!`, `cold_path()`.
- remove `cfg_if!`, replace with `cfg_select!`.

#### data::codec
- update `bitfield!`
  - rename method `without_fields` with `new`, and make it always visible.
  - derive `ConstInit`.

### data::id
- rename `define_handle!` to `handle!`

#### data storage::key
- rename `define_static_map!` to `map!`

### geom::metric
- new macros: `dis!`, `ext!`, `ori!`, `pos!`, `region!`.
- new methods for `geom::metric` *dim* types: `map`, `map_into`, `try_map`, `try_map_into`.
- new methods for `Region`: `map`, `map_ext`, `map_pos`, `try_map`, `try_map_ext`, `try_map_pos`.

#### media::visual::image
- new trait `RasterView`.
- remove `image` feature-gate from the module.
- update `ImageError`.
  - make `InvalidParsedInteger` variant contain `ParseIntErrorKind`.
  - derive `Hash`.

#### num::dom::int
- rename `define_divisor!` to `divisor!`.

#### num::dom::real
- make `Float`'s std methods const: `ceil`, `floor`, `mul_add`, `round_ties_away`, `round_ties_even`.

### num::grain
- rename `Primitive*` traits to `Prim*`.
- rename `define_lane!` to `lane!`

### num::prob
- new module `phys::prob::markov`.

#### num::prob::rand
- rename `define_pcg!` to `rand_pcg!`
- rename `define_xorshift!` to `rand_xorshift!`

## phys
- new module `phys::subs`.

#### phys::time
- update `NoTime`:
  - implement `TimePoint`, `TimeSource` and `TimeSpan`.

#### phys::time::source
- update `TimePoint`:
  - make `Elapsed` require `TimeSpan`.
  - implement for `Duration`.
- new trait `TimeSpan`.

## run
- new items: `RunDriver`, `RunDriverError`, `RunDriverFrameError`.
- update `RunRender` and `RunPresent`:
  - support borrowed render artifacts with GATs.
  - make them use a borrowed `RunFrame`.
  - make `RunRender`'s `S: ?Sized`.

## run::time
- update `RunPacer`:
  - change requiring `Duration` for `T: TimeSpan`.
  - make the constructor fallible.
  - add new methods: `interval`, `accum`, `allow`, `allow_checked`, `cycles`, `cycles_checked`.

### sys::io
- new traits `TextIn`, `TextOut`.

### sys::log
- new modules `sys::log::{bench, trace}`;

### sys::mem
- rename `define_arena!` to `arena!`

### sys::mem::cell
- new items: `MemHedgeCtrl`, `MemHedgeError`, `MemHedgeRead`, `MemHedgeState`.

### sys::mem::view
- update `Slice`: add methods: `get`, `get_mut`:
- new items: `MemReplicaError`, `MemReplicaSlice`.

### sys::os
- remove macros: `os_print!`, `os_println!`, `os_eprint!`, `os_eprintln!`.

##### sys::os::browser::web
- bring here impls from `ui::event`.
- feature-gate with `event` the event-related impls.

#### sys::os::linux
- update `LinuxError` conversion to `IoError`.

### text::fmt
- make `Fmt::from_fn` const.

### ui::event
- new types: `EventButtons`, `EventWheelUnit`.
- update `EventMouse`:
  - change the `buttons` field to use `EventButtons`.
- update `EventButtonState`: make `Pressed` the default.
- update `EventKind` and `EventTag`:
  - add `Wheel` variant.
  - add methods: `is_wheel`, `some_wheel`.
- update `EventWheel`:
  - add fields: `unit`, `buttons`.
  - remove field: `timestamp`.
  - add method: `new`.

## yard
- update `_use_or_shim!` to add `_doc!` macro support.
- split a new `_doc_vendor!` macro out of `_doc!`
- update syntax of `_devela_policy`.rs

[0.28.0]: https://github.com/andamira/devela/releases/tag/v0.28.0
