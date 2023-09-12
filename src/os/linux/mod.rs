// devela::os::linux
//
//! Linux-specific definitions
//
// NOTE: doc cfg attributes for target_arch are hidden from reexports
// in order to be have a more concise documentation in the libera crate.
// This is achieved by attaching a brief version to the item itself,
// and attaching a complete version to the module that reexports them.
//
// This is so both for syscalls and safe syscall wrappers. And when more
// platforms are supported they will all need to be updated accordingly.

/* public modules */

pub mod consts;

/// Linux-specific extensions to [`std::io`].
pub mod io {
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
    pub use super::fns::{
        get_byte, print, print_bytes, sys_ioctl, sys_read, sys_write, SysTermios,
    };
}

/// Linux-specific extensions to [`std::process`].
pub mod process {
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

/// Linux-specific extensions to [`std::thread`].
pub mod thread {
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
    pub use super::{consts::all::*, io::*, process::*, thread::*};
}

#[cfg(all(feature = "unsafe_os", not(miri)))]
mod syscalls;

// functions supported by all targets
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
    // reexport syscalls
    pub use super::syscalls::{sys_exit, sys_ioctl, sys_read, sys_write, SysTermios};

    /// Prints a string to *stdout*.
    ///
    /// This function makes use of the `write` syscall to print a string.
    ///
    /// # Error Handling
    /// If the write fails, it prints an error message and exits with status code 10.
    #[cfg_attr(
        feature = "nightly",
        doc(cfg(all(target_os = "linux", feature = "unsafe_os")))
    )]
    pub fn print(s: &str) {
        let mut s = s.as_bytes();
        while !s.is_empty() {
            let n = unsafe { sys_write(super::consts::STDOUT, s.as_ptr(), s.len()) };
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
    #[cfg_attr(
        feature = "nightly",
        doc(cfg(all(target_os = "linux", feature = "unsafe_os")))
    )]
    pub fn print_bytes(b: &[u8]) {
        let mut b = b;
        while !b.is_empty() {
            let n = unsafe { sys_write(super::consts::STDOUT, b.as_ptr(), b.len()) };
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
    #[cfg_attr(
        feature = "nightly",
        doc(cfg(all(target_os = "linux", feature = "unsafe_os")))
    )]
    pub fn get_byte() -> u8 {
        let mut c = 0;
        loop {
            let n = unsafe { sys_read(super::consts::STDIN, &mut c as *mut u8, 1) };
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

// functions not supported for risc-v
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
    use super::{print, sys_exit};
    use core::time::Duration;

    // reexport syscalls
    pub use super::syscalls::{sys_nanosleep, SysTimeSpec};

    /// Suspends execution of calling thread.
    #[cfg_attr(
        feature = "nightly",
        doc(cfg(all(target_os = "linux", feature = "unsafe_os")))
    )]
    pub fn sleep(duration: Duration) {
        let mut req = SysTimeSpec::with(duration);
        let mut rem = SysTimeSpec::default();
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
