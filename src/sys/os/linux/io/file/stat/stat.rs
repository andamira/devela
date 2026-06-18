// devela/src/sys/os/linux/io/file/stat.rs
//
//! File metadata and type classification.
//!
//! Contains structures and constants used to query file metadata and determine file types.
//!
//! Defines `LinuxFileType`, [`LinuxStat`].
//

#![allow(non_camel_case_types)]

use crate::{LINUX_S_IFMT, c_long, c_uint, c_ulong};

#[doc = crate::_tags!(linux fs)]
/// A Linux file type.
#[doc = crate::_doc_meta!{location("sys/os/linux/io")}]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LinuxFileType {
    /// A regular file.
    File,

    /// A directory.
    Directory,

    /// A symbolic link.
    Symlink,

    /// A character device.
    CharDevice,

    /// A block device.
    BlockDevice,

    /// A FIFO.
    Fifo,

    /// A socket.
    Socket,

    /// An unknown or unsupported file type.
    Unknown(c_uint),
}

#[doc = crate::_tags!(linux fs abi)]
/// File status structure matching libc's stat ([man 2 stat])
#[doc = crate::_doc_meta!{
    location("sys/os/linux/io"),
    #[cfg(target_pointer_width = "32")]
    test_size_of(LinuxStat = 76|608),
    #[cfg(target_pointer_width = "64")]
    test_size_of(LinuxStat = 144|1152), // TODO verify actual runtime stat values per Linux arch
}]
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

    /// Last access time.
    pub st_atime: c_long,
    /// Last access time, nanoseconds part.
    pub st_atime_nsec: c_long,

    /// Last modification time.
    pub st_mtime: c_long,
    /// Last modification time, nanoseconds part.
    pub st_mtime_nsec: c_long,

    /// Last status change time.
    pub st_ctime: c_long,
    /// Last status change time, nanoseconds part.
    pub st_ctime_nsec: c_long,

    // Padding for future expansion
    _unused: [c_long; 3],
}

#[rustfmt::skip]
impl LinuxStat {
    /// Returns the raw file mode.
    #[must_use]
    pub const fn mode(&self) -> c_uint { self.st_mode }

    /// Returns the Linux file type.
    #[must_use]
    pub const fn file_type(&self) -> LinuxFileType {
        match self.st_mode & LINUX_S_IFMT::TYPE_MASK {
            LINUX_S_IFMT::FILE => LinuxFileType::File,
            LINUX_S_IFMT::DIRECTORY => LinuxFileType::Directory,
            LINUX_S_IFMT::SYMLINK => LinuxFileType::Symlink,
            LINUX_S_IFMT::CHAR_DEVICE => LinuxFileType::CharDevice,
            LINUX_S_IFMT::BLOCK_DEVICE => LinuxFileType::BlockDevice,
            LINUX_S_IFMT::FIFO => LinuxFileType::Fifo,
            LINUX_S_IFMT::SOCKET => LinuxFileType::Socket,
            other => LinuxFileType::Unknown(other),
        }
    }

    /// Returns whether this is a regular file.
    #[must_use]
    pub const fn is_file(&self) -> bool { matches!(self.file_type(), LinuxFileType::File) }

    /// Returns whether this is a directory.
    #[must_use]
    pub const fn is_dir(&self) -> bool { matches!(self.file_type(), LinuxFileType::Directory) }

    /// Returns whether this is a symbolic link.
    #[must_use]
    pub const fn is_symlink(&self) -> bool { matches!(self.file_type(), LinuxFileType::Symlink) }

    /// Returns the Unix permission bits.
    #[must_use]
    pub const fn permissions(&self) -> u16 { (self.st_mode & 0o777) as u16 }

    /// Returns the Unix mode bits, including setuid, setgid, and sticky bits.
    #[must_use]
    pub const fn mode_bits(&self) -> u16 { (self.st_mode & 0o7777) as u16 }

    /// Returns the file size in bytes.
    #[must_use]
    pub const fn size(&self) -> c_long { self.st_size }

    /// Returns the file size as `usize`, or `None` if it does not fit.
    #[must_use]
    pub const fn size_usize(&self) -> Option<usize> {
        if self.st_size < 0 { None } else { Some(self.st_size as usize) }
    }

    /// Returns the preferred block size for filesystem I/O.
    #[must_use]
    pub const fn block_size(&self) -> c_long { self.st_blksize }

    /// Returns the number of allocated 512-byte blocks.
    #[must_use]
    pub const fn blocks(&self) -> c_long { self.st_blocks }
}
