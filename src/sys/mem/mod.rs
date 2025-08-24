// devela::sys::mem
//
#![doc = crate::_DOC_SYS_MEM!()]
//!
#![doc = crate::doc_!(modules: crate::sys; mem: cell)]
#![doc = crate::doc_!(newline)]
//!
#![doc = crate::doc_!(extends: alloc, borrow, boxed, cell, mem, pin, ptr, rc, slice)]
#![cfg_attr(
    any(not(feature = "bit")),
    doc = "## Features\nTo compile the missing items, enable the `bit` feature."
)]
//
// safety
#![cfg_attr(feature = "safe_mem", forbid(unsafe_code))]

mod aligned; // MemAligned
mod alloc; // Alloc, ::alloc::alloc::*
mod borrow; // Mow
mod cache_align; // CacheAlign
mod cswap; // cswap!
mod ext; // ExtMem
mod namespace; // Mem
mod pin; // Pinned, ::core::pin::*
mod ptr; // Ptr, ::core::ptr::*
mod reexports; // ::core::mem::*
mod size; // size_of_expr!, BitSized, ByteSized,
mod slice; // Slice, ExtSlice
mod storage; // Bare, BareBox, Boxed, Storage

#[cfg(all(not(feature = "safe_mem"), feature = "unsafe_layout"))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_layout")))]
mod pod; // MemPod

#[cfg(feature = "std")]
#[cfg(all(not(feature = "safe_mem"), feature = "unsafe_layout"))]
#[cfg_attr(nightly_doc, doc(cfg(all(feature = "std", feature = "unsafe_layout"))))]
mod guard; // Current, CurrrentGuard

pub mod cell; // ExtCellOption, ::core::cell::*

crate::items! { // structural access: _mods, _pub_mods, _hidden, _all, _always
    #[allow(unused)]
    pub use {_mods::*, _hidden::*};
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use {_always::*, _pub_mods::*};

    mod _mods { #![allow(unused)]
        pub use super::{
            aligned::*, alloc::_all::*, borrow::_all::*, cache_align::*, cswap::*, ext::*,
            namespace::*, pin::_all::*, ptr::_all::*, reexports::*, size::_all::*, slice::_all::*,
            storage::*,
        };
        #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_layout"))]
        pub use super::pod::MemPod;
        #[cfg(feature = "std")]
        #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_layout"))]
        pub use super::guard::{Current, CurrentGuard};
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
        pub use super::{cell::_always::*, pin::_always::*, ptr::_always::*, reexports::*};
    }
}
