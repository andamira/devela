// devela::os::linux
//
//!
//

use core::ffi::c_uint;

/// Represents the [`termios`] structure from libc,
/// used to control terminal I/O.
///
/// It has fields for input, output, control, and local modes,
/// as well as a line discipline and control characters.
///
/// [`termios`]: https://man7.org/linux/man-pages/man3/termios.3.html
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct LinuxTermios {
    /// Input flags.
    pub c_iflag: c_uint,

    /// Output flags.
    pub c_oflag: c_uint,

    /// Control flags.
    pub c_cflag: c_uint,

    /// Local flags.
    pub c_lflag: c_uint,

    pub c_line: u8,

    pub c_cc: [u8; 19],
}

impl LinuxTermios {
    /// Returns a new empty struct.
    pub const fn new() -> Self {
        Self {
            c_iflag: 0,
            c_oflag: 0,
            c_cflag: 0,
            c_lflag: 0,
            c_line: 0,
            c_cc: [0; 19],
        }
    }

    /// Returns a raw byte pointer to self.
    pub const fn as_bytes_ptr(&self) -> *const u8 {
        self as *const Self as *const u8
    }

    /// Returns a raw mutable byte pointer to self.
    pub fn as_mut_bytes_ptr(&mut self) -> *mut u8 {
        self as *mut Self as *mut u8
    }
}

unsafe impl bytemuck::NoUninit for LinuxTermios {}
