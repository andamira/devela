// devela::sys
//
//! System interfaces and hardware abstractions.
#![doc = crate::doc_!(modules: crate; sys: arch, env, fs, io, log, mem, net, os)]
#![doc = crate::doc_!(newline)]
//!
#![doc = crate::doc_!(extends: alloc, arch, borrow, boxed, cell, env, fs, mem,
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

crate::items! { // structural access: _mods, _pub_mods, _hidden, _internals, _all, _always
    #[allow(unused)]
    pub use {_mods::*, _hidden::*, _internals::*};
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use {_always::*, _pub_mods::*};

    mod _mods { #![allow(unused)]
        pub use super::sound::_all::*;
    }
    mod _pub_mods { #![allow(unused)]
        pub use super::{
            arch::_all::*, env::_all::*, fs::_all::*, io::_all::*,
            log::_all::*, mem::_all::*, net::_all::*, os::_all::*,
        };
        // WIPZONE
        // #[cfg(feature = "std")]
        // pub use super::bench::*;
        // pub use super::bench::_all::*;
    }
    pub(super) mod _hidden {
        pub use super::mem::_hidden::*;
    }
    pub(super) mod _internals { #![allow(unused)]
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::{_mods::*, _pub_mods::*};
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::{
            arch::_always::*, env::_always::*, io::_always::*, mem::_always::*,
        };
    }
}
// WIPZONE
// #[cfg(feature = "std")]
// mod bench;
