// devela/src/sys/os/linux/io/file/fd/fd.rs
//
//! Defines [`LinuxFd`].
//

use crate::{
    FdRaw, IoRead, IoResult, IoWrite, Linux, LinuxResult, LinuxSeekFrom, ManuallyDrop, c_off_t,
};
#[cfg(any(feature = "std", feature = "io"))]
use crate::{IoError, IoErrorKind, IoSeek, IoSeekFrom};

const INVALID_FD: FdRaw = -1;

#[doc = crate::_tags!(linux fs abi)]
/// An owned Linux file descriptor.
#[doc = crate::_doc_meta!{
    location("sys/os/linux/io"),
    test_size_of(LinuxFd = 4|32),
}]
#[must_use]
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

    /// Repositions this descriptor's file offset.
    pub fn seek_raw(&mut self, pos: LinuxSeekFrom) -> LinuxResult<c_off_t> {
        Linux::seek_fd(self.fd, pos)
    }
    /// Rewinds this descriptor to the start.
    pub fn rewind_raw(&mut self) -> LinuxResult<()> {
        self.seek_raw(LinuxSeekFrom::Start(0)).map(|_| ())
    }

    /// Reads bytes from this descriptor.
    pub fn read_raw(&mut self, buf: &mut [u8]) -> LinuxResult<usize> {
        Linux::read_fd(self.fd, buf)
    }
    /// Reads exactly enough bytes to fill `buf`.
    pub fn read_exact_raw(&mut self, buf: &mut [u8]) -> LinuxResult<()> {
        Linux::read_fd_exact(self.fd, buf)
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
#[cfg(any(feature = "std", feature = "io"))]
impl crate::IoSeek for LinuxFd {
    fn seek(&mut self, pos: IoSeekFrom) -> IoResult<u64> {
        let pos = match pos {
            IoSeekFrom::Start(offset) => {
                let offset = offset.try_into().map_err(|_| {
                    IoError::new(IoErrorKind::InvalidInput, "seek offset out of range")
                })?;
                LinuxSeekFrom::Start(offset)
            }
            IoSeekFrom::Current(offset) => {
                let offset = offset.try_into().map_err(|_| {
                    IoError::new(IoErrorKind::InvalidInput, "seek offset out of range")
                })?;
                LinuxSeekFrom::Current(offset)
            }
            IoSeekFrom::End(offset) => {
                let offset = offset.try_into().map_err(|_| {
                    IoError::new(IoErrorKind::InvalidInput, "seek offset out of range")
                })?;
                LinuxSeekFrom::End(offset)
            }
        };
        let offset = self.seek_raw(pos).map_err(IoError::from)?;
        offset
            .try_into()
            .map_err(|_| IoError::new(IoErrorKind::InvalidInput, "seek returned negative offset"))
    }
}
