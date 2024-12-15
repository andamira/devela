// devela::sys
//
//! System interfaces and hardware abstractions.
#![doc = crate::doc_!(modules: crate; sys: arch, io, mem, os)]
#![doc = crate::doc_!(newline)]
//!
#![doc = crate::doc_!(extends: alloc, arch, borrow, boxed, cell, env, fs, mem,
    io, net, os, path, pin, ptr, rc, slice, simd)]
//
//
/* NOTE: You can get the full list of: `arch`, `os`, `target` and `target`-family, like this:
```shell
rustc +nightly -Z unstable-options --print all-target-specs-json | jq '[ to_entries[] | {"arch": .value.arch, "target": .key, "target-family": (.value."target-family" // [] | join(", ")), "os": (.value.os // "") } ]' | grep -v '""'
```
*/
// safety
#![cfg_attr(feature = "safe_sys", forbid(unsafe_code))]

mod env;
mod sound; // IMPROVE

#[cfg(feature = "sys")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "sys")))]
mod path;

pub mod arch;
pub mod io;
pub mod log;
pub mod mem;
pub mod os;

crate::items! { // structural access: _mods, _pub_mods, _hidden, _all, _always
    #[allow(unused)]
    pub use {_mods::*, _pub_mods::*, _hidden::*};
    #[allow(unused)] #[doc(hidden)] #[doc(no_inline)]
    pub use _always::*;

    mod _mods { #![allow(unused)]
        pub use super::{env::_all::*, sound::_all::*};
        #[cfg(feature = "sys")]
        pub use super::path::_all::*;
    }
    mod _pub_mods { #![allow(unused)]
        #[doc(hidden)] #[doc(no_inline)]
        pub use super::{arch::_all::*, io::_all::*, log::_all::*, mem::_all::*, os::_all::*};
    }
    pub(super) mod _hidden {
        pub use super::mem::_hidden::*;
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
