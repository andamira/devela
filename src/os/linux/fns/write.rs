// devela::os::linux::fns::write
//
//! write related functions
//

use super::{sys_exit, sys_write, FILENO};

/// Prints a string slice to standard output.
///
/// This function makes use of [`sys_write`].
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

/// Prints a string slice to standard output, with a newline.
///
/// This function makes use of [`sys_write`].
///
/// # Error Handling
/// If the write fails, it prints an error message and exits with status code 10.
#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(target_os = "linux", feature = "unsafe_os")))
)]
#[inline]
pub fn println(s: &str) {
    print(s);
    print("\n");
}

/// Prints a string slice to standard error.
///
/// This function makes use of [`sys_write`].
///
/// # Error Handling
/// If the write fails, it prints an error message and exits with status code 10.
#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(target_os = "linux", feature = "unsafe_os")))
)]
pub fn eprint(s: &str) {
    let mut s = s.as_bytes();
    while !s.is_empty() {
        let n = unsafe { sys_write(FILENO::STDERR, s.as_ptr(), s.len()) };
        if n < 0 || n as usize > s.len() {
            print("write failed");
            unsafe { sys_exit(10) };
        }
        // Update the byte slice to exclude the bytes that have been written
        s = &s[n as usize..];
    }
}

/// Prints a string slice to standard error, with a newline.
///
/// This function makes use of the [`sys_write`] syscall to print a string.
///
/// # Error Handling
/// If the write fails, it prints an error message and exits with status code 10.
#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(target_os = "linux", feature = "unsafe_os")))
)]
#[inline]
pub fn eprintln(s: &str) {
    eprint(s);
    eprint("\n");
}

/// Prints a byte slice to *stdout*.
///
/// This function makes use of the [`sys_write`] syscall to print bytes.
///
/// # Error Handling
/// If the write fails, it prints an error message and exits with status code 10.
#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(target_os = "linux", feature = "unsafe_os")))
)]
#[inline]
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
