// devela::sys::os::linux::file::fd
//
//! File-descriptor identity and positioning.
//!
//! Contains constants related to file descriptor numbers, open flags, and seek operations.
//!
//! Defines [`LINUX_FILENO`], [`LINUX_O_FLAGS`], [`LINUX_SEEK`].
//

#![allow(non_camel_case_types)]

use crate::c_int;

#[doc = crate::_TAG_LINUX!()]
#[doc = crate::_TAG_FS!()]
/// [`Linux`][crate::Linux] Standard file descriptor numbers.
#[doc = crate::_doc_location!("sys/os/linux")]
#[derive(Debug)]
pub struct LINUX_FILENO;
impl LINUX_FILENO {
    /// Standard input.
    pub const STDIN: c_int = 0;
    /// Standard output.
    pub const STDOUT: c_int = 1;
    /// Standard error.
    pub const STDERR: c_int = 2;
}

#[doc = crate::_TAG_LINUX!()]
#[doc = crate::_TAG_FS!()]
/// [`Linux`][crate::Linux] file creation and status flags.
#[doc = crate::_doc_location!("sys/os/linux")]
///
/// Used with `sys_open`, `sys_fcntl`, etc.
//
// - /usr/include/asm-generic/fcntl.h
// - /usr/include/linux/fcntl.h
#[derive(Debug)]
pub struct LINUX_O_FLAGS;
impl LINUX_O_FLAGS {
    /* basic access */

    /// Open for reading only.
    pub const RDONLY: c_int = 0o0;
    /// Open for writing only.
    pub const WRONLY: c_int = 0o1;
    /// Open for reading and writing.
    pub const RDWR: c_int = 0o2;
    /// Mask for access mode.
    pub const ACCMODE: c_int = 0o3;

    /* File creation and status flags */

    /// Create file if it doesn't exist.
    pub const CREAT: c_int = 0o100;
    /// Fail if file exists (when used with CREAT).
    pub const EXCL: c_int = 0o200;
    /// Don't make this fd the controlling terminal (for devices).
    pub const NOCTTY: c_int = 0o400;
    /// Truncate file upon open.
    pub const TRUNC: c_int = 0o1_000;
    /// Append to file on each write.
    pub const APPEND: c_int = 0o2_000;
    /// Non-blocking mode.
    pub const NONBLOCK: c_int = 0o4_000;
    /// Synchronous I/O.
    pub const SYNC: c_int = 0o4_010_000;
    /// Synchronized I/O data integrity (wait for data writes only).
    pub const DSYNC: c_int = 0o10_000;
    /// Direct I/O (no buffering).
    pub const DIRECT: c_int = 0o40_000;
    /// Large file support.
    pub const LARGEFILE: c_int = 0o100_000;
    /// Don't follow symbolic links.
    pub const NOFOLLOW: c_int = 0o200_000;
    /// Create a directory if one doesn't exist.
    pub const DIRECTORY: c_int = 0o400_000;
    /// Don't update file access time on reads.
    pub const NOATIME: c_int = 0o1_000_000;
    /// Close on exec.
    pub const CLOEXEC: c_int = 0o2_000_000;
    /// Obtain fd for path resolution only (no file access).
    pub const PATH: c_int = 0o10_000_000; // For openat2()

    /* Special flags for tmpfile */

    /// Create an unnamed temporary file (automatically deleted on close).
    ///
    /// Must be used with `DIRECTORY` to specify where to create it.
    pub const TMPFILE: c_int = 0o20_000_000 | Self::DIRECTORY;

    /* Async I/O (not available on all architectures) */

    /// Enable signal-driven I/O (SIGIO on file activity).
    pub const ASYNC: c_int = 0o20_000;

    /* Additional flags from newer kernels */

    /// Resolve path without escaping starting dir (openat2()).
    pub const RESOLVE_BENEATH: c_int = 0o100_000_000;
    /// Don't cross mount points during resolution.
    pub const RESOLVE_NO_XDEV: c_int = 0o200_000_000;
    /// Block magic-link traversal (procfs-style links).
    pub const RESOLVE_NO_MAGICLINKS: c_int = 0o400_000_000;
    /// Block all symlink traversal.
    pub const RESOLVE_NO_SYMLINKS: c_int = 0o1_000_000_000;
    /// Treat starting dir as root during resolution.
    pub const RESOLVE_IN_ROOT: c_int = 0o2_000_000_000;
}

#[doc = crate::_TAG_LINUX!()]
#[doc = crate::_TAG_FS!()]
/// [`Linux`][crate::Linux] Seek commands (for `lseek`).
#[doc = crate::_doc_location!("sys/os/linux")]
//
// Architecture independent
// - /usr/include/unistd.h
// - /usr/include/bits/fcntl-linux.h
#[derive(Debug)]
pub struct LINUX_SEEK;
impl LINUX_SEEK {
    /// Seek from start of file.
    pub const SET: c_int = 0;
    /// Seek from current position.
    pub const CUR: c_int = 1;
    /// Seek from end of file.
    pub const END: c_int = 2;

    /// Seek to next data (for sparse files).
    pub const DATA: c_int = 3;
    /// Seek to next hole (for sparse files).
    pub const HOLE: c_int = 4;

    /// Max supported seek type.
    pub const MAX: c_int = Self::HOLE;
}
