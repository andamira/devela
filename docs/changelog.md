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

#### num::dom::int
- rename `define_divisor!` to `divisor!`.

## run
- new items: `RunDriver`, `RunDriverError`.

### sys::os
- remove macros: `os_print!`, `os_println!`, `os_eprint!`, `os_eprintln!`.

#### sys::os::linux
- update `LinuxError` conversion to `IoError`.

## yard
- update `_use_or_shim!` to add `_doc!` macro support.
- split a new `_doc_vendor!` macro out of `_doc!`
- update syntax of `_devela_policy`.rs

[0.28.0]: https://github.com/andamira/devela/releases/tag/v0.28.0
