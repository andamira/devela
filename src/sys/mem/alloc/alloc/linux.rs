// devela::sys::mem::alloc::linux
//
//! Defines [`LinuxMmapAlloc`].

use crate::{GlobalAlloc, Libc, Mem, MemLayout, Ptr, c_void, is};

#[doc = crate::_tags!(linux allocation)]
/// A Linux global memory allocator backed directly by anonymous `mmap`.
#[doc = crate::_doc_meta!{location("sys/mem/alloc")}]
///
/// This allocator maps one anonymous private memory region per allocation
/// and unmaps it on deallocation.
///
/// It is intentionally simple:
/// - no freelist
/// - no size classes
/// - no coalescing
/// - no shared Rust allocator state
///
/// # Safety
/// This allocator is thread-safe in the sense that it keeps no mutable Rust state.
/// Synchronization is delegated to the kernel/libc memory mapping operations.
///
/// # Examples
/// ```ignore
/// # use devela::LinuxMmapAlloc;
/// #[global_allocator]
/// static ALLOCATOR: LinuxMmapAlloc = LinuxMmapAlloc::INIT;
/// ```
#[derive(Debug)]
pub struct LinuxMmapAlloc;

impl LinuxMmapAlloc {
    /// Initial value for `LinuxMmapAlloc`.
    pub const INIT: Self = LinuxMmapAlloc;
}

unsafe impl Sync for LinuxMmapAlloc {}

#[derive(Clone, Copy)]
#[repr(C)]
struct LinuxMmapAllocHeader {
    base: *mut u8,
    len: usize,
}

unsafe impl GlobalAlloc for LinuxMmapAlloc {
    /// Allocates memory with the given layout.
    #[allow(unused_variables, reason = "cross-platform")]
    unsafe fn alloc(&self, layout: MemLayout) -> *mut u8 {
        unsafe { linux_mmap_alloc_impl(layout) }.unwrap_or(Ptr::null_mut())
    }

    /// Allocates zeroed memory with the given layout.
    ///
    /// Anonymous Linux mappings are initially zero-filled.
    #[inline(always)]
    unsafe fn alloc_zeroed(&self, layout: MemLayout) -> *mut u8 {
        unsafe { self.alloc(layout) }
    }

    /// Deallocates memory with `munmap`.
    #[inline(always)]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: MemLayout) {
        unsafe {
            is! { !ptr.is_null(), linux_mmap_dealloc_impl(ptr) }
        }
    }
}
#[rustfmt::skip]
unsafe fn linux_mmap_alloc_impl(layout: MemLayout) -> Option<*mut u8> {
    let size = layout.size().max(1);
    let align = layout.align();
    let header_len = size_of::<LinuxMmapAllocHeader>();
    // Extra `align` bytes leave enough room to align the user pointer after
    // the header while still storing metadata immediately before it.
    let map_len = size.checked_add(align)?.checked_add(header_len)?;
    let base = unsafe {
        Libc::mmap(Ptr::null_mut(), map_len,
            Libc::PROT_READ | Libc::PROT_WRITE,
            Libc::MAP_PRIVATE | Libc::MAP_ANONYMOUS,
            -1, 0)
    };
    is! { Libc::is_map_failed(base), return None }
    let base = base.cast::<u8>();
    let start = unsafe { base.add(header_len) } as usize;
    let user = Mem::align_up(start, align) as *mut u8;
    let header = unsafe { user.cast::<LinuxMmapAllocHeader>().sub(1) };
    unsafe { header.write(LinuxMmapAllocHeader { base, len: map_len }); }
    Some(user)
}

unsafe fn linux_mmap_dealloc_impl(ptr: *mut u8) {
    let header = unsafe { ptr.cast::<LinuxMmapAllocHeader>().sub(1).read() };
    let _ = unsafe { Libc::munmap(header.base.cast::<c_void>(), header.len) };
}
