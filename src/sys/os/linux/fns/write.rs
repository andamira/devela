// devela::sys::os::linux::fns::write
//
//! write related functions
//

use super::{linux_sys_exit, linux_sys_write, LINUX_FILENO as FILENO};

/// Prints a string slice to standard output.
///
/// This function makes use of [`linux_sys_write`].
///
/// # Error Handling
/// If the write fails, it prints an error message and exits with status code 10.
pub fn linux_print(s: &str) {
    let mut s = s.as_bytes();
    while !s.is_empty() {
        let n = unsafe { linux_sys_write(FILENO::STDOUT, s.as_ptr(), s.len()) };
        if n < 0 || n as usize > s.len() {
            linux_print("write failed");
            unsafe { linux_sys_exit(10) };
        }
        // Update the byte slice to exclude the bytes that have been written
        s = &s[n as usize..];
    }
}

/// Prints a string slice to standard output, with a newline.
///
/// This function makes use of [`linux_sys_write`].
///
/// # Error Handling
/// If the write fails, it prints an error message and exits with status code 10.
pub fn linux_println(s: &str) {
    linux_print(s);
    linux_print("\n");
}

/// Prints a string slice to standard error.
///
/// This function makes use of [`linux_sys_write`].
///
/// # Error Handling
/// If the write fails, it prints an error message and exits with status code 10.
pub fn linux_eprint(s: &str) {
    let mut s = s.as_bytes();
    while !s.is_empty() {
        let n = unsafe { linux_sys_write(FILENO::STDERR, s.as_ptr(), s.len()) };
        if n < 0 || n as usize > s.len() {
            linux_print("write failed");
            unsafe { linux_sys_exit(10) };
        }
        // Update the byte slice to exclude the bytes that have been written
        s = &s[n as usize..];
    }
}

/// Prints a string slice to standard error, with a newline.
///
/// This function makes use of the [`linux_sys_write`] syscall to print a string.
///
/// # Error Handling
/// If the write fails, it prints an error message and exits with status code 10.
pub fn linux_eprintln(s: &str) {
    linux_eprint(s);
    linux_eprint("\n");
}

/// Prints a byte slice to *stdout*.
///
/// This function makes use of the [`linux_sys_write`] syscall to print bytes.
///
/// # Error Handling
/// If the write fails, it prints an error message and exits with status code 10.
pub fn linux_print_bytes(b: &[u8]) {
    let mut b = b;
    while !b.is_empty() {
        let n = unsafe { linux_sys_write(FILENO::STDOUT, b.as_ptr(), b.len()) };
        if n < 0 || n as usize > b.len() {
            linux_print("write failed");
            unsafe { linux_sys_exit(10) };
        }
        // Update the byte slice to exclude the bytes that have been written
        b = &b[n as usize..];
    }
}
