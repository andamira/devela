// devela/src/sys/os/linux/io/file/fd/definition.rs
//
//! Defines [`LinuxFd`].
//

#![allow(missing_docs, unused)] // WIP

use crate::{FdRaw, IoRead, IoResult, IoWrite, Linux, LinuxResult, ManuallyDrop};

const INVALID_FD: FdRaw = -1;

#[doc = crate::_tags!(linux fs abi)]
/// An owned Linux file descriptor.
#[doc = crate::_doc_meta!{location("sys/os/linux/io")}]
#[derive(Debug)]
pub struct LinuxFd {
    fd: FdRaw,
}

impl LinuxFd {
    /// Creates an owned file descriptor from a raw descriptor.
    ///
    /// # Safety
    /// The descriptor must be valid, open, and uniquely owned by the returned
    /// `LinuxFd`. It will be closed on drop.
    #[must_use]
    pub const unsafe fn from_raw(fd: FdRaw) -> Self {
        Self { fd }
    }
    /// Returns the raw file descriptor.
    #[must_use]
    pub const fn as_raw(&self) -> FdRaw {
        self.fd
    }
    /// Consumes this wrapper without closing the descriptor.
    #[must_use]
    pub fn into_raw(self) -> FdRaw {
        let fd = self.fd;
        core::mem::forget(self);
        fd
    }

    /// Closes this descriptor.
    pub fn close(self) -> LinuxResult<()> {
        let mut this = ManuallyDrop::new(self);
        let fd = this.fd;
        this.fd = INVALID_FD;
        Linux::close_fd(fd)
    }
    /// Duplicates this descriptor into a new owned descriptor.
    pub fn try_clone(&self) -> LinuxResult<LinuxFd> {
        Linux::dup_fd(self.fd)
    }

    /// Reads bytes from this descriptor.
    pub fn read_raw(&mut self, buf: &mut [u8]) -> LinuxResult<usize> {
        Linux::read_fd(self.fd, buf)
    }
    /// Writes bytes to this descriptor.
    pub fn write_raw(&mut self, buf: &[u8]) -> LinuxResult<usize> {
        Linux::write_fd(self.fd, buf)
    }
    /// Writes all bytes to this descriptor.
    pub fn write_all_raw(&mut self, buf: &[u8]) -> LinuxResult<()> {
        Linux::write_fd_all(self.fd, buf)
    }
}

impl Drop for LinuxFd {
    fn drop(&mut self) {
        if self.fd >= 0 {
            let _ = Linux::close_fd(self.fd);
            self.fd = INVALID_FD;
        }
    }
}
impl IoRead for LinuxFd {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        self.read_raw(buf).map_err(Into::into)
    }
}
impl IoWrite for LinuxFd {
    fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
        self.write_raw(buf).map_err(Into::into)
    }
    fn flush(&mut self) -> IoResult<()> {
        Ok(())
    }
}
