// devela::os::linux
//
//!
//
// improve set flags

use super::super::{linux_sys_ioctl, LINUX_ERRNO, LINUX_FILENO, LINUX_IOCTL, LINUX_TERMIOS_LFLAG};
use crate::ops::iif;
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
impl LinuxTermios {
    /// Gets the current termios state into `state`.
    ///
    /// # Errors
    /// In case of an error returns the [`LINUX_ERRNO`] value from [`linux_sys_ioctl`].
    pub fn get_state() -> Result<LinuxTermios, isize> {
        let mut state = LinuxTermios::new();
        let res = unsafe {
            linux_sys_ioctl(
                LINUX_FILENO::STDIN,
                LINUX_IOCTL::TCGETS,
                state.as_mut_bytes_ptr(),
            )
        };
        iif![res >= 0; Ok(state); Err(res)]
    }

    /// Sets the current termios `state`.
    ///
    /// Returns the [`LINUX_ERRNO`] value from [`linux_sys_ioctl`].
    pub fn set_state(mut state: LinuxTermios) -> Result<(), isize> {
        let res = unsafe {
            linux_sys_ioctl(
                LINUX_FILENO::STDIN,
                LINUX_IOCTL::TCSETS,
                state.as_mut_bytes_ptr(),
            )
        };
        iif![res >= 0; Ok(()); Err(res)]
    }

    /// Returns `true` if we're in a terminal context.
    /// IMPROVE DOC low lovel
    pub fn is_terminal() -> bool {
        match Self::get_state() {
            Ok(_) => true,
            Err(e) => e != -LINUX_ERRNO::ENOTTY && e != -LINUX_ERRNO::EINVAL,
        }
    }

    /// Disables raw mode.
    pub fn disable_raw_mode() -> Result<(), isize> {
        let mut state = LinuxTermios::get_state()?;
        state.c_lflag |= LINUX_TERMIOS_LFLAG::ICANON;
        state.c_lflag |= LINUX_TERMIOS_LFLAG::ECHO;
        LinuxTermios::set_state(state)
    }

    /// Enables raw mode.
    pub fn enable_raw_mode() -> Result<(), isize> {
        let mut state = Self::get_state()?;
        state.c_lflag &= !LINUX_TERMIOS_LFLAG::ICANON;
        state.c_lflag &= !LINUX_TERMIOS_LFLAG::ECHO;
        LinuxTermios::set_state(state)
    }
}

unsafe impl bytemuck::NoUninit for LinuxTermios {}
