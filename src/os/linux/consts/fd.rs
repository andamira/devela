// devela::os::linux::consts::fd
//
//! File descriptors.
//

use core::ffi::c_int;

/// Linux file descriptor constants.
#[allow(non_camel_case_types)]
pub struct LINUX_FILENO;

impl LINUX_FILENO {
    /// File descriptor for standard input.
    pub const STDIN: c_int = 0;

    /// File descriptor for standard output.
    pub const STDOUT: c_int = 1;

    /// File descriptor for standard error.
    pub const STDERR: c_int = 2;
}
