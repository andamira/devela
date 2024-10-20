// devela::mem
//
//! Memory management.
#![doc = crate::code::doc_!(extends: alloc, borrow, boxed, cell, mem, pin, ptr, rc, slice)]
#![doc = crate::code::doc_!(modules: crate; mem: cell, ptr)]
#![doc = crate::code::doc_!(newline)]
//!
//

// safety:
#![cfg_attr(feature = "safe_mem", forbid(unsafe_code))]

#[allow(unused_imports)]
use crate::code::items;

mod aligned;
mod ext;
mod fns_macros;
mod reexports;
mod size;
mod slice;
mod storage;
#[allow(unused_imports)]
pub use {aligned::*, ext::*, fns_macros::*, reexports::*, size::*, slice::*, storage::*};

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
pub mod ptr;
#[doc(no_inline)]
pub use {cell::*, ptr::*};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{
        aligned::*, cell::*, ext::*, fns_macros::*, ptr::all::*, reexports::*, size::all::*,
        slice::all::*, storage::*,
    };

    #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_ptr"))]
    pub use super::pin::Pinned;

    #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_layout"))]
    pub use super::pod::MemPod;
}
