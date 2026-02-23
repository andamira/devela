// devela::sys::mem::alloc::bump
//
//! Defines [`BumpAlloc`].
//

use crate::{
    AtomicOrdering::{Relaxed, SeqCst},
    AtomicUsize, GlobalAlloc, MemLayout, Ptr, is,
};

#[doc = crate::_tags!(allocation)]
/// A simple, single-threaded bump allocator.
#[doc = crate::_doc_location!("sys/mem")]
///
/// - Allocates memory linearly from a fixed memory region.
/// - Does not support deallocation.
/// - Suitable for embedded and WASM.
///
/// # Safety
/// This allocator is thread-safe only when allocations are uncontended.
/// For full thread safety, use a lock or atomic coordination externally.
///
/// # Example
/// ```ignore
/// # use devela::BumpAlloc;
/// #[global_allocator]
/// static ALLOCATOR: BumpAlloc = BumpAlloc::new(0x10000, 0x20000);
/// ```
#[derive(Debug)]
pub struct BumpAlloc {
    current: AtomicUsize,
    end: usize,
}

impl BumpAlloc {
    /// Creates a new `BumpAlloc` from `start..end` addresses.
    ///
    /// # Panics
    /// Panics if `start >= end`.
    pub const fn new(start: usize, end: usize) -> Self {
        debug_assert![start < end];
        BumpAlloc { current: AtomicUsize::new(start), end }
    }
}

unsafe impl GlobalAlloc for BumpAlloc {
    unsafe fn alloc(&self, layout: MemLayout) -> *mut u8 {
        let (align, size) = (layout.align(), layout.size());
        // Load the current allocation pointer atomically (no ordering dependency)
        let mut current = self.current.load(Relaxed);

        loop {
            // Align the current address up to the requested alignment
            let aligned = (current + align - 1) & !(align - 1);
            // Compute the next allocation pointer (may overflow)
            let next = aligned.checked_add(size).unwrap_or(self.end + 1);
            // If allocation would exceed the end of the memory region, fail
            is![next > self.end, return Ptr::null_mut()];
            // Attempt to atomically reserve the space by advancing the pointer
            match self.current.compare_exchange_weak(current, next, SeqCst, Relaxed) {
                Ok(_) => return aligned as *mut u8, // Success: return aligned pointer
                Err(updated) => current = updated,  // Fail: retry with updated `current`
            }
        }
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: MemLayout) {} // IMPROVE
}
