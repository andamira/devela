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
- update `RunRender` and `RunPresent` to support borrowed render artifacts with GATs.

## run::time
- update `RunPacer`:
  - change requiring `Duration` for `T: TimeSpan`.
  - make the constructor fallible.
  - add new methods: `interval`, `accum`, `allow`, `allow_checked`, `cycles`, `cycles_checked`.

### sys::mem
- rename `define_arena!` to `arena!`

### sys::os
- remove macros: `os_print!`, `os_println!`, `os_eprint!`, `os_eprintln!`.

#### sys::os::linux
- update `LinuxError` conversion to `IoError`.

## yard
- update `_use_or_shim!` to add `_doc!` macro support.
- split a new `_doc_vendor!` macro out of `_doc!`
- update syntax of `_devela_policy`.rs

[0.28.0]: https://github.com/andamira/devela/releases/tag/v0.28.0
