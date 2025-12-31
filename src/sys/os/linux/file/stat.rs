// devela::sys::os::linux::types::stat
//
//! File metadata and type classification.
//!
//! Contains structures and constants used to query file metadata and determine file types.
//!
//! Defines [`LinuxStat`], [`LINUX_S_IFMT`].
//

#![allow(non_camel_case_types)]

use crate::{c_long, c_uint, c_ulong};

#[doc = crate::_TAG_LINUX!()]
#[doc = crate::_TAG_FS!()]
/// File status structure matching libc's stat ([man 2 stat])
///
/// [man 2 stat]: https://man7.org/linux/man-pages/man2/stat.2.html
#[repr(C)]
#[derive(Debug, Default)]
pub struct LinuxStat {
    /// Device ID containing file.
    pub st_dev: c_ulong,
    /// Inode number.
    pub st_ino: c_ulong,
    /// File type and mode.
    pub st_mode: c_uint,
    /// Hard link count.
    pub st_nlink: c_ulong,
    /// User ID of owner.
    pub st_uid: c_uint,
    /// Group ID of owner.
    pub st_gid: c_uint,
    /// Device ID (if special file).
    pub st_rdev: c_ulong,
    /// Total size, in bytes.
    pub st_size: c_long,
    /// Block size for filesystem I/O.
    pub st_blksize: c_long,
    /// Number of 512B blocks allocated.
    pub st_blocks: c_long,
    /// Last access time (nanoseconds).
    pub st_atime: c_long,
    /// Last access time (nanoseconds).
    pub st_atime_nsec: c_long,
    /// Last modification time.
    pub st_mtime: c_long,
    /// Last modification time (nanoseconds).
    pub st_mtime_nsec: c_long,
    /// Last status change time.
    pub st_ctime: c_long,
    /// Last status change time (nanoseconds).
    pub st_ctime_nsec: c_long,
    // Padding for future expansion
    _unused: [c_long; 3],
}

impl LinuxStat {
    /// Test if file is a regular file (S_ISREG)
    pub const fn is_file(&self) -> bool {
        self.st_mode & LINUX_S_IFMT::TYPE_MASK == LINUX_S_IFMT::FILE
    }
    /// Test if file is a directory (S_ISDIR)
    pub const fn is_dir(&self) -> bool {
        self.st_mode & LINUX_S_IFMT::TYPE_MASK == LINUX_S_IFMT::DIRECTORY
    }
    /// Get file permissions (lower 12 bits)
    pub const fn permissions(&self) -> u16 {
        (self.st_mode & 0o777) as u16
    }
}

#[doc = crate::_TAG_LINUX!()]
#[doc = crate::_TAG_FS!()]
/// [`Linux`][crate::Linux] File mode and permission bits.
//
// Architecture independent
// - /usr/include/linux/stat.h
// - /usr/include/bits/stat.h (glibc)
//
#[derive(Debug)]
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
