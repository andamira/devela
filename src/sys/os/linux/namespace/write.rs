// devela/src/sys/os/linux/namespace/write.rs

use crate::{LINUX_FILENO as FILENO, Linux, LinuxError, LinuxResult as Result, MaybeUninit, c_int};

/// # Write-related methods.
#[rustfmt::skip]
impl Linux {
    /* core helpers */

    const STACK_BUFFER_LEN: usize = 512; // PIPE_BUF is typically 4096 but 512 is safe

    /// Writes all bytes to a file descriptor, handling partial writes.
    fn write_all(fd: c_int, mut buf: &[u8]) -> Result<()> {
        while !buf.is_empty() {
            let n = unsafe { Linux::sys_write(fd, buf.as_ptr(), buf.len()) };
            if n < 0 { return Err(LinuxError::Sys(n)); }
            buf = &buf[n as usize..];
        }
        Ok(())
    }
    /// Writes all bytes to stdout. Returns error on syscall failure.
    fn write_all_unchecked(fd: c_int, mut buf: &[u8]) {
        while !buf.is_empty() {
            let n = unsafe { Linux::sys_write(fd, buf.as_ptr(), buf.len()) };
            if n <= 0 { panic!("write failed with return value {n}"); }
            buf = &buf[n as usize..];
        }
    }

    /* public methods */

    /// Writes string + newline to stdout. Optimized for small strings.
    pub fn print(s: &str) -> Result<()> {
        Self::write_all(FILENO::STDOUT, s.as_bytes())
    }
    /// Like `print`, but panics on failure instead of returning errors.
    pub fn print_unchecked(s: &str) { Self::write_all_unchecked(FILENO::STDOUT, s.as_bytes()); }

    /// Writes string + newline to stdout. Optimized for small strings.
    pub fn println(s: &str) -> Result<()> {
        if s.len() <= Self::STACK_BUFFER_LEN { // small string optimization:
            let mut buf = [0u8; Self::STACK_BUFFER_LEN];
            let bytes = s.as_bytes();
            buf[..bytes.len()].copy_from_slice(bytes);
            buf[bytes.len()] = b'\n';
            Self::write_all(FILENO::STDOUT, &buf[..bytes.len() + 1])
        } else { // fallback for large strings:
            Self::write_all(FILENO::STDOUT, s.as_bytes())?;
            Self::write_all(FILENO::STDOUT, b"\n")
        }
    }
    /// Like `println`, but panics on failure instead of returning errors.
    pub fn println_unchecked(s: &str) {
        if s.len() <= Self::STACK_BUFFER_LEN {
            let mut buf = [0u8; Self::STACK_BUFFER_LEN];
            let bytes = s.as_bytes();
            buf[..bytes.len()].copy_from_slice(bytes);
            buf[bytes.len()] = b'\n';
            Self::write_all_unchecked(FILENO::STDOUT, &buf[..bytes.len() + 1]);
        } else {
            Self::write_all_unchecked(FILENO::STDOUT, s.as_bytes());
            Self::write_all_unchecked(FILENO::STDOUT, b"\n");
        }
    }
    /// Ultra-fast stdout write. Panics if not all bytes are written at once.
    /// Only use for small strings (<512 bytes).
    pub fn print_unchecked_fast(s: &str) {
        let bytes = s.as_bytes();
        let n = unsafe { Linux::sys_write(FILENO::STDOUT, bytes.as_ptr(), bytes.len()) };
        if n != bytes.len() as isize {
            panic!("write failed with return value {n}");
        }
    }
    /// Ultra-fast stdout write with newline. Panics if write isn't atomic.
    /// Only use for small strings (<511 bytes).
    pub fn println_unchecked_fast(s: &str) {
        let bytes = s.as_bytes();
        let mut buf = MaybeUninit::<[u8; Self::STACK_BUFFER_LEN]>::uninit();
        // SAFETY: we're about to initialize all accessed bytes
        unsafe {
            let buf_ptr = buf.as_mut_ptr() as *mut u8;
            core::ptr::copy_nonoverlapping(bytes.as_ptr(), buf_ptr, bytes.len());
            *buf_ptr.add(bytes.len()) = b'\n'; // Add newline
            let n = Linux::sys_write(FILENO::STDOUT, buf_ptr, bytes.len() + 1); // single syscall
            if n != (bytes.len() + 1) as isize { panic!("write failed with return value {n}"); }
        }
    }
    /// Writes bytes to stdout. Returns error on syscall failure.
    pub fn print_bytes(b: &[u8]) -> Result<()> { Self::write_all(FILENO::STDOUT, b) }
    /// Like `print_bytes`, but panics on failure instead of returning errors.
    pub fn print_bytes_unchecked(b: &[u8]) { Self::write_all_unchecked(FILENO::STDOUT, b); }

    /// Writes all bytes to stderr. Returns error on syscall failure.
    pub fn eprint(s: &str) -> Result<()> { Self::write_all(FILENO::STDERR, s.as_bytes()) }
    /// Writes string + newline to stderr. Optimized for small strings.
    pub fn eprintln(s: &str) -> Result<()> {
        if s.len() <= Self::STACK_BUFFER_LEN { // small string optimization:
            let mut buf = [0u8; Self::STACK_BUFFER_LEN];
            let bytes = s.as_bytes();
            buf[..bytes.len()].copy_from_slice(bytes);
            buf[bytes.len()] = b'\n';
            Self::write_all(FILENO::STDERR, &buf[..bytes.len() + 1])
        } else { // fallback for large strings:
            Self::write_all(FILENO::STDERR, s.as_bytes())?;
            Self::write_all(FILENO::STDERR, b"\n")
        }
    }
    /// Writes bytes to stderr. Returns error on syscall failure.
    pub fn eprint_bytes(b: &[u8]) -> Result<()> {
        Self::write_all(FILENO::STDERR, b)
    }
}
