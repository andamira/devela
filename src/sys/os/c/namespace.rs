// devela/src/sys/os/c/namespace.rs
//
//! Defines [`Libc`].
//

#[cfg(unix)]
use {
    super::_raw,
    crate::{c_char, c_int, c_mode_t, c_off_t, c_void},
};

#[doc = crate::_tags!(platform namespace)]
/// Raw C/POSIX system interfaces exposed through the platform's libc.
#[doc = crate::_doc_meta!{location("sys/os")}]
///
/// Thin, unsafe bindings that provide direct access to C ABI functions
/// for memory mapping, shared memory, file descriptors, and other
/// low-level operations. These are unmodified system calls as provided
/// by the host libc.
///
/// - <https://www.gnu.org/software/libc/manual/html_node>
#[derive(Debug)]
pub struct Libc;

/// Constants common to Unix libc implementations.
#[cfg(unix)]
impl Libc {
    /// POSIX `mmap` failure sentinel.
    pub const MAP_FAILED: *mut c_void = !0 as *mut c_void;
}

/// Linux libc constants.
#[cfg(target_os = "linux")]
impl Libc {
    /* open */

    /// Opens a file for read-only access.
    pub const O_RDONLY: c_int = 0o0;
    /// Opens a file for write-only access.
    pub const O_WRONLY: c_int = 0o1;
    /// Opens a file for reading and writing.
    pub const O_RDWR: c_int = 0o2;
    /// Creates the file if it does not exist.
    pub const O_CREAT: c_int = 0o100;

    /* mmap protection */

    /// Allows pages to be read.
    pub const PROT_READ: c_int = 0x1;
    /// Allows pages to be written.
    pub const PROT_WRITE: c_int = 0x2;

    /* mmap flags */

    /// Shares updates with other mappings of the same region.
    pub const MAP_SHARED: c_int = 0x01;
    /// Keeps updates private to this mapping.
    pub const MAP_PRIVATE: c_int = 0x02;
    /// Creates a mapping not backed by a file.
    pub const MAP_ANONYMOUS: c_int = 0x20;
    /// Does not reserve swap space for the mapping.
    pub const MAP_NORESERVE: c_int = 0x4000;
}

#[cfg(unix)]
/// Convenience helpers for libc return values.
impl Libc {
    /// Returns whether `ptr` is the POSIX `mmap` failure sentinel.
    pub fn is_map_failed(ptr: *mut c_void) -> bool {
        ptr == Self::MAP_FAILED
    }
}

#[cfg(unix)]
/// Direct wrappers around libc/POSIX calls.
#[allow(clippy::missing_safety_doc)]
impl Libc {
    /// Create/open POSIX shared memory.
    /// - <https://www.man7.org/linux/man-pages/man3/shm_open.3.html>
    pub unsafe fn shm_open(name: *const c_char, oflag: c_int, mode: c_mode_t) -> c_int {
        unsafe { _raw::shm_open(name, oflag, mode) }
    }

    /// Unlink POSIX shared memory.
    /// - <https://www.man7.org/linux/man-pages/man3/shm_open.3.html>
    pub unsafe fn shm_unlink(name: *const c_char) -> c_int {
        unsafe { _raw::shm_unlink(name) }
    }

    /// Truncate a file to a specified length.
    /// - <https://man7.org/linux/man-pages/man3/ftruncate.3p.html>
    pub unsafe fn ftruncate(fd: c_int, length: c_off_t) -> c_int {
        unsafe { _raw::ftruncate(fd, length) }
    }

    /// Map pages of memory.
    /// - <https://man7.org/linux/man-pages/man3/mmap.3p.html>
    // /// - <https://man7.org/linux/man-pages/man2/mmap.2.html>
    pub unsafe fn mmap(
        addr: *mut c_void,
        length: usize,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: c_off_t,
    ) -> *mut c_void {
        unsafe { _raw::mmap(addr, length, prot, flags, fd, offset) }
    }

    ///  Unmap pages of memory.
    /// - <https://man7.org/linux/man-pages/man3/munmap.3p.html>
    // /// - <https://man7.org/linux/man-pages/man2/mmap.2.html>
    pub unsafe fn munmap(addr: *mut c_void, length: usize) -> c_int {
        unsafe { _raw::munmap(addr, length) }
    }

    /// Free allocated memory.
    /// - <https://man7.org/linux/man-pages/man3/free.3p.html>
    pub unsafe fn free(ptr: *mut c_void) {
        unsafe { _raw::free(ptr) }
    }

    /// Closes a file descriptor.
    /// - <https://man7.org/linux/man-pages/man3/close.3p.html>
    // /// - <https://man7.org/linux/man-pages/man2/close.2.html>
    pub unsafe fn close(fd: c_int) -> c_int {
        unsafe { _raw::close(fd) }
    }
}
