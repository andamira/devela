// devela::sys::os::c::namespace
//
//! Defines [`Libc`].
//

use super::raw;
use crate::{c_char, c_int, c_mode_t, c_off_t, c_void};

#[doc = crate::_tags!(platform namespace)]
/// Raw C/POSIX system interfaces exposed through the platform’s libc.
#[doc = crate::_doc_location!("sys/os")]
///
/// Thin, unsafe bindings that provide direct access to C ABI functions
/// for memory mapping, shared memory, file descriptors, and other
/// low-level operations. These are unmodified system calls as provided
/// by the host libc.
///
/// - <https://www.gnu.org/software/libc/manual/html_node>
#[derive(Debug)]
pub struct Libc;

/// Constants from POSIX and libc.
impl Libc {
    /// POSIX open flag: open for read-only access.
    pub const O_RDONLY: c_int = 0o0;
    /// POSIX open flag: open for write-only access.
    pub const O_WRONLY: c_int = 0o1;
    /// POSIX open flag: open for reading and writing.
    pub const O_RDWR: c_int = 0o2;
    /// POSIX open flag: create the file if it does not exist.
    pub const O_CREAT: c_int = 0o100;

    /// POSIX mmap protection: pages are readable.
    pub const PROT_READ: c_int = 0x1;
    /// POSIX mmap protection: pages are writable.
    pub const PROT_WRITE: c_int = 0x2;

    /// POSIX mmap flag: mapping is shared with other processes.
    pub const MAP_SHARED: c_int = 0x01;
    /// POSIX mmap failure sentinel returned on error.
    pub const MAP_FAILED: *mut c_void = !0 as *mut c_void;
}

/// Convenience helpers for libc return values.
impl Libc {
    /// Returns `true` if `ptr` matches the POSIX `MAP_FAILED` sentinel.
    #[inline(always)]
    pub fn is_map_failed(ptr: *mut c_void) -> bool {
        ptr == Self::MAP_FAILED
    }
}

/// Direct wrappers around libc/POSIX calls.
#[allow(clippy::missing_safety_doc)]
impl Libc {
    /// Create/open POSIX shared memory.
    /// - <https://www.man7.org/linux/man-pages/man3/shm_open.3.html>
    #[inline(always)]
    pub unsafe fn shm_open(name: *const c_char, oflag: c_int, mode: c_mode_t) -> c_int {
        unsafe { raw::shm_open(name, oflag, mode) }
    }

    /// Unlink POSIX shared memory.
    /// - <https://www.man7.org/linux/man-pages/man3/shm_open.3.html>
    #[inline(always)]
    pub unsafe fn shm_unlink(name: *const c_char) -> c_int {
        unsafe { raw::shm_unlink(name) }
    }

    /// Truncate a file to a specified length.
    /// - <https://man7.org/linux/man-pages/man3/ftruncate.3p.html>
    #[inline(always)]
    pub unsafe fn ftruncate(fd: c_int, length: c_off_t) -> c_int {
        unsafe { raw::ftruncate(fd, length) }
    }

    /// Map pages of memory.
    /// - <https://man7.org/linux/man-pages/man3/mmap.3p.html>
    // /// - <https://man7.org/linux/man-pages/man2/mmap.2.html>
    #[inline(always)]
    pub unsafe fn mmap(
        addr: *mut c_void,
        length: usize,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: c_off_t,
    ) -> *mut c_void {
        unsafe { raw::mmap(addr, length, prot, flags, fd, offset) }
    }

    ///  Unmap pages of memory.
    /// - <https://man7.org/linux/man-pages/man3/munmap.3p.html>
    // /// - <https://man7.org/linux/man-pages/man2/mmap.2.html>
    #[inline(always)]
    pub unsafe fn munmap(addr: *mut c_void, length: usize) -> c_int {
        unsafe { raw::munmap(addr, length) }
    }

    /// Free allocated memory.
    /// - <https://man7.org/linux/man-pages/man3/free.3p.html>
    #[inline(always)]
    pub unsafe fn free(ptr: *mut c_void) {
        unsafe { raw::free(ptr) }
    }

    /// Closes a file descriptor.
    /// - <https://man7.org/linux/man-pages/man3/close.3p.html>
    // /// - <https://man7.org/linux/man-pages/man2/close.2.html>
    #[inline(always)]
    pub unsafe fn close(fd: c_int) -> c_int {
        unsafe { raw::close(fd) }
    }
}
