// devela/src/sys/mem/alloc/alloc/namespace.rs
//
//! Defines the [`Alloc`] namespace.
//

#[cfg(feature = "alloc")]
#[allow(unused_imports, reason = "±unsafe")]
use crate::{
    _dep::_alloc::alloc::{alloc, alloc_zeroed, dealloc, handle_alloc_error, realloc},
    MemLayout,
};
#[cfg(feature = "alloc")]
use crate::{Box, Infallible, RandQualities, RandTry, SplitMix64};

#[doc = crate::_tags!(allocation namespace)]
/// Memory-allocation-related operations.
#[doc = crate::_doc_meta!{location("sys/mem")}]
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

#[cfg(feature = "alloc")]
impl Alloc {
    /// Returns weak address-derived seed material.
    ///
    /// This is not cryptographic entropy.
    #[must_use]
    #[inline(never)]
    pub fn random_weak_u64() -> u64 {
        let a = 0usize;
        let b = Box::new(0usize);
        let stack = &a as *const _ as usize as u64;
        let heap = &*b as *const _ as usize as u64;
        SplitMix64::mix64(stack ^ heap.rotate_left(32))
    }

    /// Fills `buf` with weak address-derived random-looking bytes.
    ///
    /// This is only suitable for weak fallback seeding and tests.
    #[inline(never)]
    pub fn random_weak_bytes(buf: &mut [u8]) {
        let mut sm = SplitMix64::new(Self::random_weak_u64());
        let mut i = 0;
        while i < buf.len() {
            let bytes = sm.next_u64().to_ne_bytes();
            let n = usize::min(8, buf.len() - i);
            buf[i..i + n].copy_from_slice(&bytes[..n]);
            i += n;
        }
    }
}

#[cfg(feature = "alloc")]
impl RandTry for Alloc {
    type Error = Infallible;
    const RAND_OUTPUT_BITS: u32 = 64;
    const RAND_STATE_BITS: u32 = 0;
    const RAND_QUALITIES: RandQualities = RandQualities::EXTERNAL.with_entropy_weak();

    #[inline(always)]
    fn rand_try_next_u64(&mut self) -> Result<u64, Self::Error> {
        Ok(Alloc::random_weak_u64())
    }
    #[inline(always)]
    fn rand_try_fill_bytes(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
        Alloc::random_weak_bytes(buf);
        Ok(())
    }
}
