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

mod _reexport_core; // SYMLINK to /libs/base_core/src/sys/mem/_reexport.rs
#[cfg(feature = "alloc")]
mod _reexport_alloc; // SYMLINK to /libs/base_alloc/src/sys/mem/_reexport.rs

mod alloc; // Alloc, ::alloc::alloc::*
mod borrow; // Mow
mod ext; // MemExt
mod pin; // Pinned, ::core::pin::*
mod ptr; // Ptr, ::core::ptr::*
mod size; // size_of_expr!, BitSized, ByteSized,
mod slice; // Slice, SliceExt
mod storage; // Bare, BareBox, Boxed, Storage

#[cfg(all(not(feature = "safe_mem"), feature = "unsafe_layout"))]
mod pod; // MemPod

#[cfg(feature = "std")]
#[cfg(all(not(feature = "safe_mem"), feature = "unsafe_layout"))]
mod guard; // Current, CurrrentGuard

pub mod cell; // ExtCellOption, ::core::cell::*

crate::structural_mods! { // _mods, _pub_mods, _reexports, _hidden
    _mods {
        pub use super::{
            alloc::_all::*,
            borrow::_all::*,
            ext::*,
            pin::_all::*,
            ptr::_all::*,
            size::_all::*,
            slice::_all::*,
            storage::*,
        };
        #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_layout"))]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_layout")))]
        pub use super::pod::MemPod;

        #[cfg(feature = "std")]
        #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_layout"))]
        #[cfg_attr(nightly_doc, doc(cfg(all(feature = "std", feature = "unsafe_layout"))))]
        pub use super::guard::{Current, CurrentGuard};
    }
    _pub_mods {
        pub use super::cell::_all::*;
    }
    _reexports {
        pub use super::_reexport_core::*;
        #[cfg(feature = "alloc")]
        pub use super::_reexport_alloc::*;

        #[doc(inline)]
        pub use devela_base_core::sys::mem::{
            CacheAlign, MaybeByte, Mem, MemAligned, cswap, define_arena,
        };

        #[doc(inline)]
        pub use crate::Sized;
    }
    _hidden {
        pub use super::size::_hidden::*;
    }
}
