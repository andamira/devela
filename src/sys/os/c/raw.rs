// devela::sys::os::c::raw
//
//!
//

pub(super) use crate::{c_char, c_int, c_mode_t, c_off_t, c_void};

unsafe extern "C" {
    /// Creates or opens a POSIX shared memory object.
    /// - <https://www.man7.org/linux/man-pages/man3/shm_open.3.html>
    pub(super) fn shm_open(name: *const c_char, oflag: c_int, mode: c_mode_t) -> c_int;

    /// Removes a POSIX shared memory object.
    /// - <https://www.man7.org/linux/man-pages/man3/shm_unlink.3.html>
    pub(super) fn shm_unlink(name: *const c_char) -> c_int;

    /// Truncates an open file or shared memory object to `length`.
    /// - <https://man7.org/linux/man-pages/man3/ftruncate.3p.html>
    pub(super) fn ftruncate(fd: c_int, length: c_off_t) -> c_int;

    /// Maps a file or shared memory object into memory.
    /// - <https://man7.org/linux/man-pages/man3/mmap.3p.html>
    // /// - <https://man7.org/linux/man-pages/man2/mmap.2.html>
    pub(super) fn mmap(
        addr: *mut c_void,
        length: usize,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: c_off_t,
    ) -> *mut c_void;

    /// Unmaps a previously mapped memory region.
    /// - <https://man7.org/linux/man-pages/man3/munmap.3p.html>
    // /// - <https://man7.org/linux/man-pages/man2/mmap.2.html>
    pub(super) fn munmap(addr: *mut c_void, length: usize) -> c_int;

    /// Frees memory previously allocated by C APIs.
    /// - <https://man7.org/linux/man-pages/man3/free.3p.html>
    pub(super) fn free(ptr: *mut c_void);

    /// Closes a file descriptor.
    // /// - <https://man7.org/linux/man-pages/man2/close.2.html>
    /// - <https://man7.org/linux/man-pages/man3/close.3p.html>
    pub(super) fn close(fd: c_int) -> c_int;
}
