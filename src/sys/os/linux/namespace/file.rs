// devela/src/sys/os/linux/namespace/file.rs
//
//! File-descriptor-oriented Linux operations.
//

use crate::{CStr, FdRaw, MaybeUninit, c_off_t};
use crate::{LINUX_AT, LINUX_ERRNO, Linux, LinuxError, LinuxResult};
use crate::{LinuxFd, LinuxOpenOptions, LinuxPipe, LinuxPipeFlags, LinuxSeekFrom, LinuxStat};
#[cfg(feature = "alloc")]
use crate::{Vec, vec_};

/// # File-descriptor-related methods.
#[rustfmt::skip]
impl Linux {
    /* open, close and duplicate */

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

    /* path */

    /// Returns whether a path exists.
    pub fn path_exists(path: &CStr) -> bool {
        Self::stat_path(path).is_ok()
    }
    /// Returns whether a path points to a regular file.
    pub fn path_is_file(path: &CStr) -> LinuxResult<bool> {
        Ok(Self::stat_path(path)?.is_file())
    }
    /// Returns whether a path points to a directory.
    pub fn path_is_dir(path: &CStr) -> LinuxResult<bool> {
        Ok(Self::stat_path(path)?.is_dir())
    }
    /// Returns the size of a path in bytes, if representable as `usize`.
    pub fn path_size(path: &CStr) -> LinuxResult<usize> {
        Self::stat_path(path)?.size_usize().ok_or(LinuxError::Other("file size out of range"))
    }

    /* pipe */

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
        if n < 0 { return Err(LinuxError::Sys(n)); }
        let read = unsafe { LinuxFd::from_raw(fds[0]) };
        let write = unsafe { LinuxFd::from_raw(fds[1]) };
        Ok(LinuxPipe { read, write })
    }
    /// Creates a close-on-exec nonblocking anonymous pipe.
    pub fn pipe_fd_nonblock() -> LinuxResult<LinuxPipe> {
        Self::pipe_fd_with(LinuxPipeFlags::CLOEXEC.with(LinuxPipeFlags::NONBLOCK))
    }

    /* read & seek */

    /// Reads bytes from a file descriptor.
    ///
    /// Retries if interrupted before reading any bytes.
    pub fn read_fd(fd: FdRaw, buf: &mut [u8]) -> LinuxResult<usize> {
        if buf.is_empty() { return Ok(0) }
        loop {
            let n = unsafe { Self::sys_read(fd, buf.as_mut_ptr(), buf.len()) };
            if n >= 0 { return Ok(n as usize); }
            else if n == -LINUX_ERRNO::EINTR { continue; }
            else { return Err(LinuxError::Sys(n)); }
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
    /// Reads a file from the beginning into `buf`.
    ///
    /// Stops at EOF or when `buf` is full, and returns the number of bytes read.
    pub fn read_file_buf(path: &CStr, buf: &mut [u8]) -> LinuxResult<usize> {
        let mut fd = Self::open_fd(path, LinuxOpenOptions::read_only())?;
        fd.read_to_buf_raw(buf)
    }
    /// Reads exactly enough bytes from a file to fill `buf`.
    pub fn read_file_exact(path: &CStr, buf: &mut [u8]) -> LinuxResult<()> {
        let mut fd = Self::open_fd(path, LinuxOpenOptions::read_only())?;
        fd.read_exact_raw(buf)
    }
    /// Reads an entire regular file into an allocated buffer.
    #[cfg(feature = "alloc")]
    pub fn read_file_alloc(path: &CStr) -> LinuxResult<Vec<u8>> {
        let mut fd = Self::open_fd(path, LinuxOpenOptions::read_only())?;
        let mut stat = LinuxStat::default();
        let n = unsafe { Self::sys_fstat(fd.as_raw(), &mut stat) };
        if n < 0 { return Err(LinuxError::Sys(n as isize)); }
        let len = stat.st_size as usize;
        let mut buf = vec_![0u8; len];
        let read = fd.read_to_buf_raw(&mut buf)?;
        buf.truncate(read);
        Ok(buf)
    }

    /// Repositions the file offset for a descriptor.
    pub fn seek_fd(fd: FdRaw, pos: LinuxSeekFrom) -> LinuxResult<c_off_t> {
        let (offset, whence) = pos.raw();
        let n = unsafe { Self::sys_lseek(fd, offset, whence) };
        if n < 0 { Err(LinuxError::Sys(n as isize)) } else { Ok(n) }
    }

    /* stat */

    /// Gets file status for an open file descriptor.
    pub fn stat_fd(fd: FdRaw) -> LinuxResult<LinuxStat> {
        let mut stat = MaybeUninit::<LinuxStat>::uninit();
        let n = unsafe { Self::sys_fstat(fd, stat.as_mut_ptr()) };
        if n < 0 { Err(LinuxError::Sys(n)) } else { Ok(unsafe { stat.assume_init() }) }
    }
    /// Gets file status for a path.
    ///
    /// Follows symbolic links.
    pub fn stat_path(path: &CStr) -> LinuxResult<LinuxStat> {
        let mut stat = MaybeUninit::<LinuxStat>::uninit();
        let n = unsafe { Self::sys_stat(path.as_ptr(), stat.as_mut_ptr()) };
        if n < 0 { Err(LinuxError::Sys(n)) } else { Ok(unsafe { stat.assume_init() }) }
    }

    /* write & append */

    /// Writes bytes to a file descriptor.
    ///
    /// Retries if interrupted before writing any bytes.
    pub fn write_fd(fd: FdRaw, buf: &[u8]) -> LinuxResult<usize> {
        if buf.is_empty() { return Ok(0) }
        loop {
            let n = unsafe { Self::sys_write(fd, buf.as_ptr(), buf.len()) };
            if n >= 0 { return Ok(n as usize); }
            else if n == -LINUX_ERRNO::EINTR { continue; }
            else { return Err(LinuxError::Sys(n)); }
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
    /// Writes all bytes to a file.
    ///
    /// Creates the file if missing and truncates it if it already exists.
    pub fn write_file(path: &CStr, bytes: &[u8]) -> LinuxResult<()> {
        let mut fd = Self::open_fd(path, LinuxOpenOptions::write_only().create() .truncate())?;
        fd.write_all_raw(bytes)
    }
    /// Appends all bytes to a file.
    ///
    /// Creates the file if missing.
    pub fn append_file(path: &CStr, bytes: &[u8]) -> LinuxResult<()> {
        let mut fd = Self::open_fd(path, LinuxOpenOptions::write_only().create().append())?;
        fd.write_all_raw(bytes)
    }
}
