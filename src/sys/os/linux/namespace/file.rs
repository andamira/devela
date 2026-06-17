// devela/src/sys/os/linux/namespace/file.rs
//
//! File-descriptor-oriented Linux operations.
//

use crate::{CStr, FdRaw, LINUX_AT, LINUX_ERRNO, LinuxError, LinuxResult, c_int, c_off_t};
use crate::{Linux, LinuxFd, LinuxOpenOptions, LinuxPipe, LinuxPipeFlags, LinuxSeekFrom};

/// # File-descriptor-related methods.
#[rustfmt::skip]
impl Linux {
    /// Opens a path as an owned Linux file descriptor.
    ///
    /// Uses `openat` with `AT_FDCWD`, so relative paths
    /// are resolved from the current working directory.
    ///
    /// # Example
    /// ```no_run
    /// # use devela::{Linux, LinuxOpenOptions};
    /// let fd = Linux::open_fd(
    ///     c"/tmp/test",
    ///     LinuxOpenOptions::read_write().create().mode(0o644),
    /// )?;
    /// # Ok::<(), devela::LinuxError>(())
    /// ```
    pub fn open_fd(path: &CStr, options: LinuxOpenOptions) -> LinuxResult<LinuxFd> {
        let fd = unsafe {
            Self::sys_openat(LINUX_AT::FDCWD, path.as_ptr(), options.flags(), options.mode_raw())
        };
        if fd < 0 { Err(LinuxError::Sys(fd as isize)) }
        else { Ok(unsafe { LinuxFd::from_raw(fd) }) }
    }
    /// Opens a path relative to a directory file descriptor.
    ///
    /// Passing `AT_FDCWD` resolves relative paths from the current working directory.
    pub fn open_fd_at(dirfd: FdRaw, path: &CStr, options: LinuxOpenOptions) -> LinuxResult<LinuxFd> {
        let fd = unsafe {
            Self::sys_openat(dirfd, path.as_ptr(), options.flags(), options.mode_raw())
        };
        if fd < 0 { Err(LinuxError::Sys(fd as isize)) }
        else { Ok(unsafe { LinuxFd::from_raw(fd) }) }
    }
    /// Closes a file descriptor.
    ///
    /// This does not retry on failure.
    pub fn close_fd(fd: FdRaw) -> LinuxResult<()> {
        let n = unsafe { Self::sys_close(fd) };
        if n < 0 { Err(LinuxError::Sys(n)) } else { Ok(()) }
    }
    /// Duplicates a file descriptor into a new owned descriptor.
    pub fn dup_fd(fd: FdRaw) -> LinuxResult<LinuxFd> {
        let new_fd = unsafe { Self::sys_dup(fd) };
        if new_fd < 0 { Err(LinuxError::Sys(new_fd as isize)) }
        else { Ok(unsafe { LinuxFd::from_raw(new_fd) }) }
    }

    /// Creates an anonymous pipe.
    ///
    /// The returned descriptors are closed during `exec`.
    pub fn pipe_fd() -> LinuxResult<LinuxPipe> {
        Self::pipe_fd_with(LinuxPipeFlags::CLOEXEC)
    }
    /// Creates an anonymous pipe with the given flags.
    pub fn pipe_fd_with(flags: LinuxPipeFlags) -> LinuxResult<LinuxPipe> {
        let mut fds = [0 as FdRaw; 2];
        let n = unsafe { Self::sys_pipe2(fds.as_mut_ptr(), flags.bits()) };
        if n < 0 { return Err(LinuxError::Sys(n as isize)); }
        let read = unsafe { LinuxFd::from_raw(fds[0]) };
        let write = unsafe { LinuxFd::from_raw(fds[1]) };
        Ok(LinuxPipe { read, write })
    }

    /// Repositions the file offset for a descriptor.
    pub fn seek_fd(fd: FdRaw, pos: LinuxSeekFrom) -> LinuxResult<c_off_t> {
        let (offset, whence) = pos.raw();
        let n = unsafe { Self::sys_lseek(fd, offset, whence) };
        if n < 0 { Err(LinuxError::Sys(n as isize)) } else { Ok(n) }
    }

    /// Reads bytes from a file descriptor.
    ///
    /// Retries if interrupted before reading any bytes.
    pub fn read_fd(fd: FdRaw, buf: &mut [u8]) -> LinuxResult<usize> {
        if buf.is_empty() { return Ok(0) }
        loop {
            let n = unsafe { Self::sys_read(fd, buf.as_mut_ptr(), buf.len()) };
            if n >= 0 {
                return Ok(n as usize);
            } else if n == -LINUX_ERRNO::EINTR {
                continue;
            } else {
                return Err(LinuxError::Sys(n));
            }
        }
    }
    /// Reads exactly enough bytes to fill `buf`.
    ///
    /// Returns an error if EOF is reached before the buffer is full.
    pub fn read_fd_exact(fd: FdRaw, mut buf: &mut [u8]) -> LinuxResult<()> {
        while !buf.is_empty() {
            let n = Self::read_fd(fd, buf)?;
            if n == 0 { return Err(LinuxError::Other("unexpected end of file")); }
            buf = &mut buf[n..];
        }
        Ok(())
    }

    /// Writes bytes to a file descriptor.
    ///
    /// Retries if interrupted before writing any bytes.
    pub fn write_fd(fd: FdRaw, buf: &[u8]) -> LinuxResult<usize> {
        if buf.is_empty() { return Ok(0) }
        loop {
            let n = unsafe { Self::sys_write(fd, buf.as_ptr(), buf.len()) };
            if n >= 0 {
                return Ok(n as usize);
            } else if n == -LINUX_ERRNO::EINTR {
                continue;
            } else {
                return Err(LinuxError::Sys(n));
            }
        }
    }
    /// Writes all bytes to a file descriptor, handling partial writes.
    pub fn write_fd_all(fd: FdRaw, mut buf: &[u8]) -> LinuxResult<()> {
        while !buf.is_empty() {
            let n = Self::write_fd(fd, buf)?;
            if n == 0 { return Err(LinuxError::Other("write returned zero")); }
            buf = &buf[n..];
        }
        Ok(())
    }
}
