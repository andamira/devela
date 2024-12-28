// devela::sys::mem
//
//! Memory management.
#![doc = crate::doc_!(modules: crate::sys; mem: cell)]
#![doc = crate::doc_!(newline)]
//!
#![doc = crate::doc_!(extends: alloc, borrow, boxed, cell, mem, pin, ptr, rc, slice)]
//
// safety
#![cfg_attr(feature = "safe_mem", forbid(unsafe_code))]

mod aligned;
mod cache_align;
mod cswap;
mod ext;
mod namespace;
mod ptr;
mod reexports;
mod size;
mod slice;
mod storage;

#[cfg(all(not(feature = "safe_mem"), feature = "unsafe_ptr"))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_ptr")))]
mod pin;
#[cfg(all(not(feature = "safe_mem"), feature = "unsafe_layout"))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_layout")))]
mod pod;

pub mod cell;

crate::items! { // structural access: _mods, _pub_mods, _hidden, _all, _always
    #[allow(unused)]
    pub use {_mods::*, _hidden::*};
    #[allow(unused)] #[doc(hidden)] #[doc(no_inline)]
    pub use {_always::*, _pub_mods::*};

    mod _mods {
        pub use super::{
            aligned::*, cache_align::*, cswap::*, ext::*, namespace::*, ptr::_all::*,
            reexports::*, size::_all::*, slice::_all::*, storage::*,
        };
        #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_ptr"))]
        pub use super::pin::Pinned;
        #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_layout"))]
        pub use super::pod::MemPod;
    }
    mod _pub_mods {
        pub use super::cell::_all::*;
    }
    pub(super) mod _hidden {
        pub use super::size::_hidden::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::{_mods::*, _pub_mods::*};
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::{cell::_always::*, ptr::_always::*, reexports::*};
    }
}
