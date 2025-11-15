// devela::sys::os::linux::types::stat
//
//! Defines [`LinuxStat`].
//

use crate::{LINUX_S_IFMT, c_long, c_uint, c_ulong};

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
