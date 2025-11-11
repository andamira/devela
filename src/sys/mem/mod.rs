// devela::sys::mem
//
#![doc = crate::_DOC_SYS_MEM!()]
//!
#![doc = crate::_doc!(modules: crate::sys; mem: cell)]
#![doc = crate::_doc!(newline)]
//!
#![doc = crate::_doc!(extends: alloc, borrow, boxed, cell, mem, pin, ptr, rc, slice)]
#![cfg_attr(
    any(not(feature = "bit")),
    doc = "## Features\nTo compile the missing items, enable the `bit` feature."
)]
//
// safety
#![cfg_attr(feature = "safe_mem", forbid(unsafe_code))]

mod alloc; // Alloc, ::alloc::alloc::*
mod borrow; // Mow
mod ext; // MemExt
mod pin; // Pinned, ::core::pin::*
mod ptr; // Ptr, ::core::ptr::*
mod reexports;
mod size; // size_of_expr!, BitSized, ByteSized,
mod slice; // Slice, SliceExt
mod storage; // Bare, BareBox, Boxed, Storage

#[cfg(all(not(feature = "safe_mem"), feature = "unsafe_layout"))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_layout")))]
mod pod; // MemPod

#[cfg(feature = "std")]
#[cfg(all(not(feature = "safe_mem"), feature = "unsafe_layout"))]
#[cfg_attr(nightly_doc, doc(cfg(all(feature = "std", feature = "unsafe_layout"))))]
mod guard; // Current, CurrrentGuard

pub mod cell; // ExtCellOption, ::core::cell::*

crate::structural_mods! { // _mods, _pub_mods, _hidden
    _mods {
        pub use super::{
            alloc::_all::*,
            borrow::_all::*,
            ext::*,
            pin::_all::*,
            ptr::_all::*,
            reexports::*,
            size::_all::*,
            slice::_all::*,
            storage::*,
        };
        #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_layout"))]
        pub use super::pod::MemPod;
        #[cfg(feature = "std")]
        #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_layout"))]
        pub use super::guard::{Current, CurrentGuard};
    }
    _pub_mods {
        pub use super::cell::_all::*;
    }
    _hidden {
        pub use super::size::_hidden::*;
    }
}
