// devela/src/sys/os/linux/io/file/_raw.rs
//
//! Defines [`LINUX_S_IFMT`].
//

#![allow(dead_code, non_camel_case_types)]

use crate::c_uint;

#[doc = crate::_tags!(linux fs)]
/// [`Linux`][crate::Linux] File mode and permission bits.
#[doc = crate::_doc_meta!{location("sys/os/linux/io")}]
//
// Architecture independent
// - /usr/include/linux/stat.h
// - /usr/include/bits/stat.h (glibc)
#[derive(Debug)]
pub(crate) struct LINUX_S_IFMT;
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
