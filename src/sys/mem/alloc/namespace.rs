// devela::sys::mem::alloc::namespace
//
//! Defines the [`Alloc`] namespace.
//

#[allow(unused_imports, reason = "unsafe feature-gated")]
#[cfg(feature = "alloc")]
use crate::{
    _dep::_alloc::alloc::{alloc, alloc_zeroed, dealloc, handle_alloc_error, realloc},
    MemLayout,
};

#[doc = crate::_TAG_ALLOCATION!()]
#[doc = crate::_TAG_NAMESPACE!()]
/// Memory-allocation-related operations.
///
/// See also: [`Mem`][crate::Mem], [`MemExt`][crate::MemExt],
/// [`Ptr`][crate::Ptr], [`Slice`][crate::Slice].
#[derive(Debug)]
pub struct Alloc;

#[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
#[cfg(feature = "alloc")]
impl Alloc {
    /// Signals a memory allocation error.
    ///
    /// # Safety
    /// See `std::alloc::`[`handle_alloc_error()`].
    pub fn handle_alloc_error(layout: MemLayout) -> ! {
        handle_alloc_error(layout)
    }
}

/// # Unsafe methods
///
/// ## Features
/// They depend on enabling any `unsafe*` feature, and not enabling `safe_mem`.
#[cfg_attr(nightly_doc, doc(cfg(unsafe··)))]
#[cfg(all(not(feature = "safe_mem"), unsafe··))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
#[cfg(feature = "alloc")]
#[rustfmt::skip]
impl Alloc {
    /// Allocates memory with the global allocator.
    ///
    /// # Safety
    /// See `std::alloc::`[`alloc()`].
    #[must_use]
    pub unsafe fn alloc(layout: MemLayout) -> *mut u8 {
        // SAFETY: Caller must uphold the safety contract.
        unsafe { alloc(layout) }
    }
    /// Allocates zero-initialized memory with the global allocator.
    ///
    /// # Safety
    /// See `std::alloc::`[`alloc_zeroed()`].
    #[must_use]
    pub unsafe fn alloc_zeroed(layout: MemLayout) -> *mut u8 {
        // SAFETY: Caller must uphold the safety contract.
        unsafe { alloc_zeroed(layout) }
    }
    /// Deallocates memory with the global allocator.
    ///
    /// # Safety
    /// See `std::alloc::`[`dealloc()`].
    pub unsafe fn dealloc(ptr: *mut u8, layout: MemLayout) {
        // SAFETY: Caller must uphold the safety contract.
        unsafe { dealloc(ptr, layout) }
    }
    /// Reallocates memory with the global allocator.
    ///
    /// # Safety
    /// See `std::alloc::`[`realloc()`].
    pub unsafe fn realloc(ptr: *mut u8, layout: MemLayout, new_size: usize) -> *mut u8 {
        // SAFETY: Caller must uphold the safety contract.
        unsafe { realloc(ptr, layout, new_size) }
    }
}
