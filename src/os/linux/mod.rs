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
    pub use super::fns::{get_byte, print, read, write};
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
    pub use super::fns::exit;
}

/* private modules */

pub use all::*;
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{fd::*, io::*, process::*};
}

#[cfg(all(feature = "unsafe_os", not(miri)))]
mod syscalls;

#[cfg(all(feature = "unsafe_os", not(miri)))]
pub use fns::*;

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
    /// use devela::os::linux::exit;
    ///
    /// unsafe { exit(0) };
    /// ```
    pub use syscalls::exit;

    /// Performs a `read` syscall.
    ///
    /// Reads `count` bytes from a file descriptor `fd` into a buffer `buf`.
    ///
    /// # Examples
    /// ```
    /// use devela::os::linux::{STDIN, read};
    ///
    /// let mut buf: [u8; 1024] = [0; 1024];
    /// let bytes_read: isize = unsafe { read(STDIN, buf.as_mut_ptr(), buf.len()) };
    /// ```
    pub use syscalls::read;

    /// Performs a `write` syscall.
    ///
    /// Writes `count` bytes from a buffer `buf` into a file descriptor `fd`.
    ///
    /// # Safety
    ///
    /// # Examples
    /// ```
    /// use devela::os::linux::{STDOUT, write};
    ///
    /// let buf = "Hello\n".as_bytes();
    /// let bytes_written: isize = unsafe { write(STDOUT, buf.as_ptr(), buf.len()) };
    /// ```
    pub use syscalls::write;

    /* new */

    /// Prints a string to *stdout*.
    ///
    /// This function makes use of the `write` syscall to print a string.
    ///
    /// If the write fails, it prints an error message and exits with status code 10.
    pub fn print(s: &str) {
        let mut s = s.as_bytes();
        unsafe {
            while !s.is_empty() {
                // Write the bytes to the standard output
                let n = write(super::fd::STDOUT, s.as_ptr(), s.len());
                if n < 0 || n as usize > s.len() {
                    print("write failed");
                    exit(10);
                }
                // Update the byte slice to exclude the bytes that have been written
                s = &s[n as usize..];
            }
        }
    }

    /// Gets a single byte from *stdin*.
    ///
    /// This function makes use of the `read` syscall to read a byte.
    ///
    /// If the read fails, it prints an error message and exits with status code 11.
    pub fn get_byte() -> u8 {
        let mut c = 0;
        unsafe {
            loop {
                let n = read(super::fd::STDIN, &mut c as *mut u8, 1);
                if n < 0 {
                    print("read failed");
                    exit(11);
                }
                if n == 1 {
                    break;
                }
            }
        }
        c
    }
}
