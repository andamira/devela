// devela::sys::mem::alloc
//
//! Defines [`WasmAlloc`].
//

use crate::{GlobalAlloc, MemLayout, Ptr};
#[cfg(target_arch = "wasm32")]
use crate::{Mem, NonZeroUsize, Wasm};

#[doc = crate::_TAG_RUNTIME!()]
#[doc = crate::_TAG_ALLOCATION!()]
/// A WebAssembly global memory allocator that uses a bump allocation strategy.
#[doc = crate::_doc_location!("sys/mem")]
///
/// This allocator manages memory starting from `__heap_base` (the WASM heap start)
/// and grows memory as needed using `memory.grow`. It's designed for single-threaded
/// WASM environments where synchronization isn't required.
///
/// # Safety
/// - This allocator is not thread-safe. The unsafe `Sync` implementation is only
///   valid in single-threaded WASM environments.
///
/// # Example
/// ```ignore
/// # use devela::WasmAlloc;
/// #[global_allocator]
/// static ALLOCATOR: WasmAlloc = WasmAlloc::INIT;
/// ```
#[doc = crate::_doc!(vendor: "mini-alloc")]
#[derive(Debug)]
pub struct WasmAlloc;
impl WasmAlloc {
    /// Initial value for `WasmAlloc`.
    pub const INIT: Self = WasmAlloc;
}
/// This is safe in single-threaded WASM environments bug invalid in multi-threaded contexts.
unsafe impl Sync for WasmAlloc {}

#[rustfmt::skip]
unsafe impl GlobalAlloc for WasmAlloc {
    /// Allocates memory with the given layout.
    #[allow(unused_variables, reason = "cross-platform")]
    unsafe fn alloc(&self, l: MemLayout) -> *mut u8 {
        #[cfg(target_arch = "wasm32")]
        return unsafe { alloc_impl(l) }.unwrap_or(Ptr::null_mut());

        #[cfg(not(target_arch = "wasm32"))]
        Ptr::null_mut()
    }
    /// Allocates zeroed memory with the given layout.
    /// This is the same as `alloc` since WASM memory is always zero-initialized.
    #[inline(always)]
    unsafe fn alloc_zeroed(&self, l: MemLayout) -> *mut u8 { unsafe { self.alloc(l) } }
    /// No-op. Memory is only reclaimed when the entire WASM instance is dropped.
    #[inline(always)]
    unsafe fn dealloc(&self, _ptr: *mut u8, _l: MemLayout) {}
}

/// Global allocator state tracking the current bump offset and memory boundary.
///
/// The state stores negative values because:
/// 1. Aligning negative offsets requires fewer instructions than positive offsets.
/// 2. Allows using saturating arithmetic to detect out-of-memory conditions.
///
/// The tuple contains:
/// - `.0`: Negative of the current bump pointer offset from `__heap_base`
/// - `.1`: Negative of the current memory boundary (total available memory)
#[cfg(target_arch = "wasm32")]
static mut WASM_ALLOC_STATE: Option<(NonZeroUsize, usize)> = None;

/// Internal allocation implementation using bump allocation strategy.
///
/// # Safety
/// - This function accesses and modifies global mutable state.
/// - The caller must ensure no other allocations are happening concurrently.
///
/// Returns `None` if allocation fails (cannot grow memory or layout is invalid).
#[cfg(target_arch = "wasm32")]
#[allow(clippy::deref_addrof, reason = "safe reference to static mut")]
unsafe fn alloc_impl(layout: MemLayout) -> Option<*mut u8> {
    // SAFETY: Single-threaded WASM, the static is only referenced here, the function not reentrant.
    // https://doc.rust-lang.org/nightly/edition-guide/rust-2024/static-mut-references.html#safe-references
    let state_ref = unsafe { &mut *&raw mut WASM_ALLOC_STATE };
    let (neg_offset, neg_bound) = state_ref.get_or_insert_with(|| {
        let bound = Wasm::PAGE_BYTES * Wasm::memory_pages() - 1;
        (
            unsafe { NonZeroUsize::new_unchecked(Wasm::heap_base().wrapping_neg()) },
            bound.wrapping_neg(),
        )
    });
    // Align the current offset for the requested alignment
    let neg_aligned = Mem::align_down(neg_offset.get(), layout.align());
    // Calculate new offset after allocating the requested size
    let next_neg_offset = neg_aligned.checked_sub(layout.size())?;
    // Check if we need to grow memory
    let bytes_needed = neg_bound.saturating_sub(next_neg_offset + 1);
    if bytes_needed != 0 {
        let pages_needed = 1 + (bytes_needed - 1) / Wasm::PAGE_BYTES;
        Wasm::memory_grow_checked(pages_needed)?;
        *neg_bound -= Wasm::PAGE_BYTES * pages_needed;
    }
    // Update state and return the allocated pointer
    *neg_offset = unsafe { NonZeroUsize::new_unchecked(next_neg_offset) };
    Some(neg_aligned.wrapping_neg() as *mut u8)
}
