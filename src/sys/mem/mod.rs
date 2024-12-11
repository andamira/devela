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

crate::items! { // structural access: doc_inline, doc_hidden, items_hidden, all, always
    #[allow(unused)]
    pub use {doc_hidden::*, doc_inline::*, items_hidden::*};
    #[allow(unused)] #[doc(hidden)] #[doc(no_inline)]
    pub use always::*;

    mod doc_inline {
        pub use super::{
            aligned::*, cache_align::*, cswap::*, ext::*, namespace::*, ptr::all::*,
            reexports::*, size::all::*, slice::all::*, storage::*,
        };
        #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_ptr"))]
        pub use super::pin::Pinned;
        #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_layout"))]
        pub use super::pod::MemPod;
    }
    mod doc_hidden { #[doc(hidden)] #[doc(no_inline)]
        pub use super::cell::all::*;
    }
    pub(super) mod items_hidden {
        pub use super::size::items_hidden::*;
    }
    pub(super) mod all {
        #[doc(inline)]
        pub use super::{doc_hidden::*, doc_inline::*};
    }
    pub(super) mod always { #![allow(unused)]
        pub use super::{cell::always::*, ptr::always::*, reexports::*};
    }
}
