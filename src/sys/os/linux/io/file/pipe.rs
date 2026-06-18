// devela/src/sys/os/linux/io/file/pipe.rs
//
//! Defines [`LinuxPipe`], [`LinuxPipeFlags`].
//

use crate::{LINUX_O_FLAGS as O, Linux, LinuxFd, LinuxResult, c_int};

#[doc = crate::_tags!(linux fs io)]
/// An owned Linux anonymous pipe.
#[doc = crate::_doc_meta!{
    location("sys/os/linux/io"),
    test_size_of(LinuxPipe = 8|64),
}]
#[must_use]
#[derive(Debug)]
pub struct LinuxPipe {
    /// The read end of the pipe.
    pub read: LinuxFd,

    /// The write end of the pipe.
    pub write: LinuxFd,
}

impl LinuxPipe {
    /// Creates an anonymous pipe.
    pub fn new() -> LinuxResult<Self> {
        Linux::pipe_fd()
    }
    /// Creates an anonymous pipe with the given flags.
    pub fn with_flags(flags: LinuxPipeFlags) -> LinuxResult<Self> {
        Linux::pipe_fd_with(flags)
    }
    /// Creates a pipe from raw read and write descriptors.
    ///
    /// # Safety
    /// Both descriptors must be valid, open, uniquely owned pipe endpoints.
    pub const unsafe fn from_raw(read: LinuxFd, write: LinuxFd) -> Self {
        Self { read, write }
    }

    /// Splits the pipe into its read and write ends.
    pub fn into_inner(self) -> (LinuxFd, LinuxFd) {
        (self.read, self.write)
    }
}

#[doc = crate::_tags!(linux fs io)]
/// Flags used when creating a Linux pipe.
#[doc = crate::_doc_meta!{location("sys/os/linux/io")}]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct LinuxPipeFlags {
    bits: c_int,
}

/// # Flags
impl LinuxPipeFlags {
    /// No pipe flags.
    pub const NONE: Self = Self { bits: 0 };
    /// Close pipe descriptors during `exec`.
    pub const CLOEXEC: Self = Self { bits: O::CLOEXEC };
    /// Open both pipe ends in nonblocking mode.
    pub const NONBLOCK: Self = Self { bits: O::NONBLOCK };
    /// Use packet-mode pipe I/O.
    ///
    /// This corresponds to Linux `O_DIRECT` for pipes.
    pub const DIRECT: Self = Self { bits: O::DIRECT };
    /// Create a notification pipe.
    ///
    /// This reuses the same raw bit as `O_EXCL`,
    /// but in the `pipe2` context it means `O_NOTIFICATION_PIPE`.
    pub const NOTIFICATION: Self = Self { bits: O::EXCL };
}

/// # Methods
#[rustfmt::skip]
impl LinuxPipeFlags {
    /* constructors */

    /// Returns an empty flag set.
    pub const fn new() -> Self { Self::NONE }

    /// Returns flags from raw Linux bits.
    pub const fn from_bits(bits: c_int) -> Self { Self { bits } }

    /// Returns the default pipe flags.
    ///
    /// The default closes pipe descriptors during `exec`.
    #[must_use]
    pub const fn default_exec_safe() -> Self { Self::CLOEXEC }

    /* queries */

    /// Returns the raw Linux bits.
    #[must_use]
    pub const fn bits(self) -> c_int { self.bits }

    /// Returns whether all flags in `other` are present.
    #[must_use]
    pub const fn contains(self, other: Self) -> bool { self.bits & other.bits == other.bits }
    /// Returns whether any flag in `other` is present.
    #[must_use]
    pub const fn intersects(self, other: Self) -> bool { self.bits & other.bits != 0 }

    /* modifiers */

    /// Returns this flag set with `other` included.
    pub const fn with(self, other: Self) -> Self { Self { bits: self.bits | other.bits } }
    /// Returns this flag set without `other`.
    pub const fn without(self, other: Self) -> Self { Self { bits: self.bits & !other.bits } }
    /// Includes close-on-exec.
    pub const fn close_on_exec(self) -> Self { self.with(Self::CLOEXEC) }
    /// Allows inheritance across `exec`.
    pub const fn inherit_on_exec(self) -> Self { self.without(Self::CLOEXEC) }
    /// Includes nonblocking mode.
    pub const fn nonblock(self) -> Self { self.with(Self::NONBLOCK) }
    /// Includes packet-mode pipe I/O.
    pub const fn direct(self) -> Self { self.with(Self::DIRECT) }
    /// Includes notification-pipe mode.
    pub const fn notification(self) -> Self { self.with(Self::NOTIFICATION) }
}
