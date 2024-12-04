// devela::sys::mem
//
//! Memory management.
#![doc = crate::doc_!(modules: crate; mem: cell)]
#![doc = crate::doc_!(newline)]
//!
#![doc = crate::doc_!(extends: alloc, borrow, boxed, cell, mem, pin, ptr, rc, slice)]
//
// safety
#![cfg_attr(feature = "safe_mem", forbid(unsafe_code))]

#[allow(unused_imports)]
use crate::code::items;

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
#[allow(unused_imports)]
pub use {
    aligned::*, cache_align::*, cswap::*, ext::*, namespace::*, ptr::all::*, reexports::*, size::*,
    slice::*, storage::*,
};

#[cfg(all(not(feature = "safe_mem"), feature = "unsafe_ptr"))]
items! {
    mod pin;
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_ptr")))]
    pub use pin::Pinned;
}

#[cfg(all(not(feature = "safe_mem"), feature = "unsafe_layout"))]
items! {
    mod pod;
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_layout")))]
    pub use pod::MemPod;
}

pub mod cell;
#[doc(no_inline)]
pub use cell::all::*;

pub(crate) mod all {
    #![allow(unused_imports)]

    #[doc(inline)]
    pub use super::{
        aligned::*, cache_align::*, cell::all::*, cswap::*, ext::*, namespace::*, ptr::all::*,
        reexports::*, size::all::*, slice::all::*, storage::*,
    };

    #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_ptr"))]
    pub use super::pin::Pinned;

    #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_layout"))]
    pub use super::pod::MemPod;
}
