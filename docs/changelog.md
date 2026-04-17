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
- bump `hashbrown` to 0.17.

## features & flags
- update the intended scope of the `_docs_examples` feature.

---

# Modules

### data::id
- rename `define_handle!` to `handle!`

#### data storage::key
- rename `define_static_map!` to `map!`

### geom::metric
- new macros: `dis!`, `ext!`, `ori!`, `pos!`.
- new methods for `geom::metric` types: `map`, `map_into`, `try_map`, `try_map_into`.

#### media::visual::image
- new trait `RasterView`.
- remove `image` feature-gate from the module.

#### num::dom::int
- rename `define_divisor!` to `divisor!`.

#### num::grain::wide
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

## run::time
- update `RunPacer`:
  - change requiring `Duration` for `T: TimeSpan`.
  - make the constructor fallible.
  - add new methods: `interval`, `accum`, `allow`, `allow_checked`, `cycles`, `cycles_checked`.

### sys::mem
- rename `define_arena!` to `arena!`

### sys::mem::cell
- new items: `MemHedgeCtrl`, `MemHedgeError`, `MemHedgeRead`, `MemHedgeState`.

### sys::mem::view
- update `Slice`: add methods: `get`, `get_mut`:
- new items: `MemReplicaError`, `MemReplicaSlice`.

### sys::os
- remove macros: `os_print!`, `os_println!`, `os_eprint!`, `os_eprintln!`.

#### sys::os::linux
- update `LinuxError` conversion to `IoError`.

## text
- make `Fmt::from_fn` const.

## yard
- update `_use_or_shim!` to add `_doc!` macro support.
- split a new `_doc_vendor!` macro out of `_doc!`
- update syntax of `_devela_policy`.rs

[0.28.0]: https://github.com/andamira/devela/releases/tag/v0.28.0
