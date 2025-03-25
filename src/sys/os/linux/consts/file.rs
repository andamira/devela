// devela::sys::os::linux::consts::file
//
//! Linux file-related constants.
//!
//! Defines:
//! - Standard file descriptors ([`LINUX_FILENO`])
//! - File opening flags ([`LINUX_O_FLAGS`])
//! - File mode/permission bits ([`LINUX_S_IFMT`]).
//! - File seek commands ([`LINUX_SEEK`])
//! - File descriptor commands ([`LINUX_F_CMD`] for fcntl)
//

#![allow(non_camel_case_types)]

use core::ffi::{c_int, c_uint};

/// [`Linux`][crate::Linux] Standard file descriptor numbers.
#[allow(non_camel_case_types)]
pub struct LINUX_FILENO;
impl LINUX_FILENO {
    /// Standard input.
    pub const STDIN: c_int = 0;
    /// Standard output.
    pub const STDOUT: c_int = 1;
    /// Standard error.
    pub const STDERR: c_int = 2;
}

/// [`Linux`][crate::Linux] file creation and status flags.
///
/// Used with `sys_open`, `sys_fcntl`, etc.
//
// - /usr/include/asm-generic/fcntl.h
// - /usr/include/linux/fcntl.h
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

/// [`Linux`][crate::Linux] File mode and permission bits.
//
// Architecture independent
// - /usr/include/linux/stat.h
// - /usr/include/bits/stat.h (glibc)
//
pub struct LINUX_S_IFMT;
impl LINUX_S_IFMT {
    /// File type mask (S_IFMT in POSIX).
    pub const TYPE_MASK: c_uint = 0o170_000;

    /* File types */

    /// Regular file (S_IFREG in POSIX).
    pub const FILE: c_uint = 0o100_000;
    /// Directory (S_IFDIR in POSIX).
    pub const DIRECTORY: c_uint = 0o040_000;
    /// Character device (S_IFCHR in POSIX).
    pub const CHAR_DEVICE: c_uint = 0o020_000;
    /// Block device (S_IFBLK in POSIX).
    pub const BLOCK_DEVICE: c_uint = 0o060_000;
    /// FIFO/pipe (S_IFIFO in POSIX).
    pub const FIFO: c_uint = 0o010_000;
    /// Symbolic link (S_IFLNK in POSIX).
    pub const SYMLINK: c_uint = 0o120_000;
    /// Socket (S_IFSOCK in POSIX).
    pub const SOCKET: c_uint = 0o140_000;

    /* Special mode bits */

    /// Set user ID on execution (`S_ISUID` in POSIX).
    pub const SET_UID: c_uint = 0o4_000;
    /// Set group ID on execution (`S_ISGID` in POSIX).
    pub const SET_GID: c_uint = 0o2_000;
    /// Sticky bit - restricted deletion (`S_ISVTX` in POSIX).
    pub const STICKY_BIT: c_uint = 0o1_000;

    /* Permission bits */

    /// User has read permission (`S_IRUSR` in POSIX).
    pub const USER_READ: c_uint = 0o400;
    /// User has write permission (`S_IWUSR` in POSIX).
    pub const USER_WRITE: c_uint = 0o200;
    /// User has execute permission (`S_IXUSR` in POSIX).
    pub const USER_EXEC: c_uint = 0o100;

    /// Group has read permission (`S_IRGRP` in POSIX).
    pub const GROUP_READ: c_uint = 0o040;
    /// Group has write permission (`S_IWGRP` in POSIX).
    pub const GROUP_WRITE: c_uint = 0o020;
    /// Group has execute permission (`S_IXGRP` in POSIX).
    pub const GROUP_EXEC: c_uint = 0o010;

    /// Others have read permission (`S_IROTH` in POSIX).
    pub const OTHER_READ: c_uint = 0o004;
    /// Others have write permission (`S_IWOTH` in POSIX).
    pub const OTHER_WRITE: c_uint = 0o002;
    /// Others have execute permission (`S_IXOTH` in POSIX).
    pub const OTHER_EXEC: c_uint = 0o001;
}

/// [`Linux`][crate::Linux] Seek commands (for `lseek`).
//
// Architecture independent
// - /usr/include/unistd.h
// - /usr/include/bits/fcntl-linux.h
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

/// [`Linux`][crate::Linux] File descriptor commands (for `fcntl`).
//
// - /usr/include/asm-generic/fcntl.h
// - /usr/include/bits/fcntl-linux.h
//
// Possible arch-specific variations (F_GETLK, F_SETLK, F_SETLKW may have different numeric values)
// - /usr/include/asm/fcntl.h
pub struct LINUX_F_CMD;
impl LINUX_F_CMD {
    /// Duplicate file descriptor.
    pub const DUPFD: c_int = 0;
    /// Get file descriptor flags.
    pub const GETFD: c_int = 1;
    /// Set file descriptor flags.
    pub const SETFD: c_int = 2;

    /// Get file status flags.
    pub const GETFL: c_int = 3;
    /// Set file status flags.
    pub const SETFL: c_int = 4;

    /// Get record locking info.
    pub const GETLK: c_int = 5;
    /// Set record locking info.
    pub const SETLK: c_int = 6;
    /// Set record locking wait.
    pub const SETLKW: c_int = 7;

    /// Duplicate FD with CLOSE_ON_EXEC.
    pub const DUPFD_CLOEXEC: c_int = 1024;

    /// Get pipe buffer size.
    pub const GETPIPE_SZ: c_int = 1032;
    /// Set pipe buffer size.
    pub const SETPIPE_SZ: c_int = 1031;

    /// Add seals to file.
    pub const ADD_SEALS: c_int = 1033;
    /// Get seals from file.
    pub const GET_SEALS: c_int = 1034;

    /// Get owner's process ID.
    pub const GETOWN: c_int = 9;
    /// Set owner's process ID.
    pub const SETOWN: c_int = 8;

    /// Get SIGIO/SIGURG signals.
    pub const GETSIG: c_int = 11;
    /// Set SIGIO/SIGURG signals.
    pub const SETSIG: c_int = 10;
}
