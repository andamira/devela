// devela::sys
//
#![doc = crate::_DOC_SYS!()]
#![doc = crate::_doc!(modules: crate; sys: arch, env, fs, io, log, mem, net, os)]
#![doc = crate::_doc!(newline)]
//!
#![doc = crate::_doc!(extends: alloc, arch, borrow, boxed, cell, env, fs, mem,
    io, net, os, path, pin, ptr, rc, slice, simd)]
//
//
/* NOTES
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
// safety
#![cfg_attr(feature = "safe_sys", forbid(unsafe_code))]

mod sound; // IMPROVE

pub mod arch; // Arch, *asm, detect_*, m128* m256*
pub mod env; // App*, Arg*, Env
pub mod fs; // Fs
pub mod io; // Io*
pub mod log;
pub mod mem; // Mem,
pub mod net; // Ip*, Socket*, Tcp*, Udp*
pub mod os; // Linux,

// WIPZONE
// #[cfg(feature = "std")]
// mod bench;

crate::structural_mods! { // _mods, _pub_mods, _hidden, _crate_internals, _always
    _mods {
        pub use super::sound::_all::*;
    }
    _pub_mods {
        pub use super::{
            arch::_all::*, env::_all::*, fs::_all::*, io::_all::*,
            log::_all::*, mem::_all::*, net::_all::*, os::_all::*,
        };
        // WIPZONE
        // #[cfg(feature = "std")]
        // pub use super::bench::*;
        // pub use super::bench::_all::*;
    }
    _crate_internals {}
    _always {
        pub use super::{
            arch::_always::*, env::_always::*, io::_always::*, mem::_always::*,
        };
    }
    _hidden {
        pub use super::mem::_hidden::*;
    }
}
