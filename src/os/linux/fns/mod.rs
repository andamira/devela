// devela::os::linux::fns
//
//!
//

pub use {all_targets::*, no_riscv::*};

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
mod all_targets {
    // reexport syscalls
    use super::super::consts::{ERRNO, FILENO, IOCTL};
    pub use super::super::syscalls::{sys_exit, sys_ioctl, sys_read, sys_write, SysTermios};

    /// Returns `true` if this is a terminal.
    #[cfg_attr(
        feature = "nightly",
        doc(cfg(all(target_os = "linux", feature = "unsafe_os")))
    )]
    pub fn is_terminal() -> bool {
        let mut termios = SysTermios::default();
        let res = unsafe { sys_ioctl(FILENO::STDOUT, IOCTL::TCGETS, termios.as_mut_bytes_ptr()) };
        res != -ERRNO::ENOTTY && res != -ERRNO::EINVAL
    }

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
            let n = unsafe { sys_write(FILENO::STDOUT, s.as_ptr(), s.len()) };
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
            let n = unsafe { sys_write(FILENO::STDOUT, b.as_ptr(), b.len()) };
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
            let n = unsafe { sys_read(FILENO::STDIN, &mut c as *mut u8, 1) };
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
mod no_riscv {
    use super::{print, sys_exit};
    use core::{cmp::Ordering, time::Duration};

    // reexport syscalls
    pub use super::super::syscalls::{sys_nanosleep, SysTimeSpec};

    /// Suspends execution of calling thread.
    #[cfg_attr(
        feature = "nightly",
        doc(cfg(all(target_os = "linux", feature = "unsafe_os")))
    )]
    pub fn sleep(duration: Duration) {
        let mut req = SysTimeSpec::with(duration);
        let mut rem = SysTimeSpec::default();

        loop {
            let n = unsafe { sys_nanosleep(req.as_ptr(), rem.as_mut_ptr()) };
            match n.cmp(&0) {
                Ordering::Less => {
                    print("nanosleep failed");
                    unsafe { sys_exit(13) };
                }
                Ordering::Equal => break,
                Ordering::Greater => req = rem,
            }
        }
    }
}
