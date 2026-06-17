// devela/src/sys/os/linux/namespace/file.rs
//
//! File-descriptor-oriented Linux operations.
//

use crate::{
    CStr, FdRaw, LINUX_AT, LINUX_ERRNO, Linux, LinuxError, LinuxFd, LinuxOpenOptions, LinuxResult,
};

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
