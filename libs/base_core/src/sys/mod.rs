// devela_base_core::sys
//
#![doc = crate::_DOC_SYS!()]
#![doc = crate::_doc!(modules: crate; sys: arch, env, log, mem, net)] // fs, io, os
#![doc = crate::_doc!(newline)]
//!
#![doc = crate::_doc!(extends: alloc, arch, borrow, boxed, cell, env, fs, mem,
    io, net, os, path, pin, ptr, rc, slice, simd)]
//
/* Notes:
- To get the full list of: `arch`, `os`, `target` and `target-family`:
```sh
rustc +nightly -Z unstable-options --print all-target-specs-json | jq '[ to_entries[] | {"arch": .value.arch, "target": .key, "target-family": (.value."target-family" // [] | join(", ")), "os": (.value.os // "") } ]' | grep -v '""'
```
- Altenatively:
```sh
rustc --print target-list | cut -f1 -d'-'| sort | uniq # List of arches supported
rustc --print target-list | cut -f2 -d'-'| sort | uniq # List of vendors supported
rustc --print target-list | cut -f3 -d'-'| sort | uniq # List of OSes supported
```
*/
//
// safety
#![cfg_attr(base_safe_sys, forbid(unsafe_code))]

pub mod arch;
pub mod env;
pub mod log;
pub mod mem;
pub mod net;
pub mod os;

crate::structural_mods! { // _pub_mods, _hidden
    _pub_mods {
        pub use super::{
            arch::_all::*,
            env::_all::*,
            log::_all::*,
            mem::_all::*,
            net::_all::*,
            os::_all::*,
        };
    }
    _hidden {
        use super::mem::_hidden::*;
    }
}
