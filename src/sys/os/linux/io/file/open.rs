// devela/src/sys/os/linux/io/file/open.rs
//
//! Linux file-opening options.
//

use crate::{LINUX_O_FLAGS as O, c_int, c_mode_t};

#[doc = crate::_tags!(linux fs)]
/// Options used to open a Linux file descriptor.
#[doc = crate::_doc_meta!{location("sys/os/linux/io")}]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LinuxOpenOptions {
    flags: c_int,
    mode: c_mode_t,
}

impl LinuxOpenOptions {
    /// Opens for reading only.
    #[must_use]
    pub const fn read_only() -> Self {
        Self { flags: O::RDONLY | O::CLOEXEC, mode: 0o666 }
    }
    /// Opens for writing only.
    #[must_use]
    pub const fn write_only() -> Self {
        Self { flags: O::WRONLY | O::CLOEXEC, mode: 0o666 }
    }
    /// Opens for reading and writing.
    #[must_use]
    pub const fn read_write() -> Self {
        Self { flags: O::RDWR | O::CLOEXEC, mode: 0o666 }
    }
    /// Creates the file if it does not exist.
    #[must_use]
    pub const fn create(mut self) -> Self {
        self.flags |= O::CREAT;
        self
    }
    /// Fails if the file already exists.
    #[must_use]
    pub const fn exclusive(mut self) -> Self {
        self.flags |= O::EXCL;
        self
    }
    /// Truncates the file on open.
    #[must_use]
    pub const fn truncate(mut self) -> Self {
        self.flags |= O::TRUNC;
        self
    }
    /// Appends writes to the end of the file.
    #[must_use]
    pub const fn append(mut self) -> Self {
        self.flags |= O::APPEND;
        self
    }
    /// Opens in nonblocking mode.
    #[must_use]
    pub const fn nonblock(mut self) -> Self {
        self.flags |= O::NONBLOCK;
        self
    }
    /// Requires the path to resolve to a directory.
    #[must_use]
    pub const fn directory(mut self) -> Self {
        self.flags |= O::DIRECTORY;
        self
    }
    /// Prevents following a final symbolic link.
    #[must_use]
    pub const fn nofollow(mut self) -> Self {
        self.flags |= O::NOFOLLOW;
        self
    }
    /// Sets the creation mode used with `create`.
    #[must_use]
    pub const fn mode(mut self, mode: c_mode_t) -> Self {
        self.mode = mode;
        self
    }
    /// Allows the opened descriptor to remain open across `exec`.
    ///
    /// This clears `O_CLOEXEC`, overriding the safer default.
    pub const fn inherit_on_exec(mut self) -> Self {
        self.flags &= !O::CLOEXEC;
        self
    }

    /* internals */

    /// Returns the raw Linux open flags.
    #[must_use]
    pub(crate) const fn flags(self) -> c_int {
        self.flags
    }
    /// Returns the raw Linux creation mode.
    #[must_use]
    pub(crate) const fn mode_raw(self) -> c_mode_t {
        self.mode
    }
}
