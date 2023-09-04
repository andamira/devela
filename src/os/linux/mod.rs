// devela::os::linux
//
//! Linux-specific definitions
//

/* public modules */

pub mod fd;

pub mod io {
    //! Linux-specific extensions to [`std::io`].
    #[cfg(all(
        any(
            target_arch = "x86_64",
            target_arch = "x86",
            target_arch = "arm",
            target_arch = "aarch64",
            target_arch = "riscv32",
            target_arch = "riscv64"
        ),
        feature = "unsafe_os",
        not(miri),
    ))]
    pub use super::fns::{get_byte, print, print_bytes, sys_read, sys_write};
}

pub mod process {
    //! Linux-specific extensions to [`std::process`].
    #[cfg(all(
        any(
            target_arch = "x86_64",
            target_arch = "x86",
            target_arch = "arm",
            target_arch = "aarch64",
            target_arch = "riscv32",
            target_arch = "riscv64"
        ),
        feature = "unsafe_os",
        not(miri),
    ))]
    pub use super::fns::sys_exit;
}

pub mod thread {
    //! Linux-specific extensions to [`std::thread`].
    #[cfg(all(
        any(
            target_arch = "x86_64",
            target_arch = "x86",
            target_arch = "arm",
            target_arch = "aarch64",
        ),
        feature = "unsafe_os",
        not(miri),
    ))]
    pub use super::fns_no_riscv::{sleep, sys_nanosleep, SysTimeSpec};
}

/* private modules */

pub use all::*;
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{fd::*, io::*, process::*, thread::*};
}

#[cfg(all(feature = "unsafe_os", not(miri)))]
mod syscalls;

#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(
        target_os = "linux",
        any(
            target_arch = "x86_64",
            target_arch = "x86",
            target_arch = "arm",
            target_arch = "aarch64",
            target_arch = "riscv32",
            target_arch = "riscv64"
        ),
        feature = "unsafe_os",
    )))
)]
#[cfg(all(feature = "unsafe_os", not(miri)))]
mod fns {
    use super::syscalls;

    /* reexported */

    /// Performs an `exit` syscall.
    ///
    /// Terminates the process with an exit status.
    ///
    /// # Examples
    /// ```
    /// use devela::os::linux::sys_exit;
    ///
    /// unsafe { sys_exit(0) };
    /// ```
    ///
    /// # Safety
    /// TODO
    pub use syscalls::sys_exit;

    /// Performs a `read` syscall.
    ///
    /// Reads `count` bytes from a file descriptor `fd` into a buffer `buf`.
    ///
    /// # Examples
    /// ```ignore
    // IMPROVE: The test doc example fails for lack of input
    /// use devela::os::linux::{STDIN, sys_read};
    ///
    /// let mut buf: [u8; 1024] = [0; 1024];
    /// let bytes_read: isize = unsafe { sys_read(STDIN, buf.as_mut_ptr(), buf.len()) };
    /// assert![bytes_read > 0];
    /// ```
    ///
    /// # Safety
    /// TODO
    pub use syscalls::sys_read;

    /// Performs a `write` syscall.
    ///
    /// Writes `count` bytes from a buffer `buf` into a file descriptor `fd`.
    ///
    /// Returns the syscall return value.
    ///
    /// # Examples
    /// ```
    /// use devela::os::linux::{STDOUT, sys_write};
    ///
    /// let buf = "Hello\n".as_bytes();
    /// let bytes_written: isize = unsafe { sys_write(STDOUT, buf.as_ptr(), buf.len()) };
    /// assert![bytes_written > 0];
    /// ```
    ///
    /// # Safety
    /// TODO
    pub use syscalls::sys_write;

    /* new */

    /// Prints a string to *stdout*.
    ///
    /// This function makes use of the `write` syscall to print a string.
    ///
    /// # Error Handling
    /// If the write fails, it prints an error message and exits with status code 10.
    pub fn print(s: &str) {
        let mut s = s.as_bytes();
        while !s.is_empty() {
            let n = unsafe { sys_write(super::fd::STDOUT, s.as_ptr(), s.len()) };
            if n < 0 || n as usize > s.len() {
                print("write failed");
                unsafe { sys_exit(10) };
            }
            // Update the byte slice to exclude the bytes that have been written
            s = &s[n as usize..];
        }
    }

    /// Prints bytes to *stdout*.
    ///
    /// This function makes use of the `write` syscall to print bytes.
    ///
    /// # Error Handling
    /// If the write fails, it prints an error message and exits with status code 10.
    pub fn print_bytes(b: &[u8]) {
        let mut b = b;
        while !b.is_empty() {
            let n = unsafe { sys_write(super::fd::STDOUT, b.as_ptr(), b.len()) };
            if n < 0 || n as usize > b.len() {
                print("write failed");
                unsafe { sys_exit(10) };
            }
            // Update the byte slice to exclude the bytes that have been written
            b = &b[n as usize..];
        }
    }

    /// Gets a single byte from *stdin*.
    ///
    /// This function makes use of the `read` syscall to read a byte.
    ///
    /// # Error Handling
    /// If the read fails, it prints an error message and exits with status code 11.
    pub fn get_byte() -> u8 {
        let mut c = 0;
        loop {
            let n = unsafe { sys_read(super::fd::STDIN, &mut c as *mut u8, 1) };
            if n < 0 {
                print("read failed");
                unsafe { sys_exit(11) };
            }
            if n == 1 {
                break;
            }
        }
        c
    }
}

#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(
        target_os = "linux",
        any(
            target_arch = "x86_64",
            target_arch = "x86",
            target_arch = "arm",
            target_arch = "aarch64",
        ),
        feature = "unsafe_os",
    )))
)]
#[cfg(all(feature = "unsafe_os", not(miri)))]
mod fns_no_riscv {
    use super::{print, sys_exit, syscalls};
    use core::time::Duration;

    pub use syscalls::SysTimeSpec;

    /// Suspends execution of calling thread.
    ///
    /// Suspension will last until either the time interval specified in `*req`
    /// has elapsed or a signal is delivered to the calling thread, in which
    /// case the remaining time will be stored in `rem`.
    ///
    /// Returns the syscall return value.
    ///
    /// # Examples
    /// ```
    /// use devela::os::linux::{sys_nanosleep, SysTimeSpec};
    /// use core::time::Duration;
    ///
    /// let mut req = SysTimeSpec::from(Duration::from_millis(99));
    /// let mut rem = SysTimeSpec::new();
    /// assert_eq![0, unsafe { sys_nanosleep(&mut req, &mut rem) }];
    /// ```
    ///
    /// # Safety
    /// TODO
    pub use syscalls::sys_nanosleep;

    /// Suspends execution of calling thread.
    pub fn sleep(duration: Duration) {
        let mut req = SysTimeSpec::with(duration);
        let mut rem = SysTimeSpec::new();
        loop {
            let n =
                unsafe { sys_nanosleep(&req as *const SysTimeSpec, &mut rem as *mut SysTimeSpec) };
            if n < 0 {
                print("nanosleep failed");
                unsafe { sys_exit(13) };
            }
            if n == 0 {
                break;
            }
            req = rem;
        }
    }
}
