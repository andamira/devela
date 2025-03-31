// devela::sys::os::linux::structs::termios
//
//! Defines [`LinuxTermios`]
//

#![cfg_attr(not(feature = "unsafe_syscall"), allow(dead_code))]

#[cfg(all(feature = "unsafe_syscall", not(miri)))]
use crate::{
    LINUX_ERRNO, LINUX_FILENO, LINUX_IOCTL, LINUX_TERMIOS_LFLAG, Linux, LinuxError,
    LinuxResult as Result, is,
};
use crate::{TermSize, c_uint};

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

    /// …
    pub c_line: u8,

    /// …
    pub c_cc: [u8; 19],
}

#[cfg_attr(nightly_doc, doc(cfg(all(feature = "unsafe_syscall", feature = "dep_bytemuck"))))]
#[cfg(all(feature = "unsafe_syscall", feature = "dep_bytemuck"))]
unsafe impl crate::_dep::bytemuck::NoUninit for LinuxTermios {}

impl LinuxTermios {
    /// Returns a new empty struct.
    #[must_use]
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
    #[must_use]
    pub const fn as_bytes_ptr(&self) -> *const u8 {
        self as *const Self as *const u8
    }

    /// Returns a raw mutable byte pointer to self.
    #[must_use]
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
    feature = "unsafe_syscall",
    not(miri),
))]
impl LinuxTermios {
    /// Gets the current termios state into `state`.
    pub fn get_state() -> Result<LinuxTermios> {
        let mut state = LinuxTermios::new();
        let res = unsafe {
            Linux::sys_ioctl(LINUX_FILENO::STDIN, LINUX_IOCTL::TCGETS, state.as_mut_bytes_ptr())
        };
        is![res >= 0; Ok(state); Err(LinuxError::Sys(res))]
    }

    /// Sets the current termios `state`.
    pub fn set_state(mut state: LinuxTermios) -> Result<()> {
        let res = unsafe {
            Linux::sys_ioctl(LINUX_FILENO::STDIN, LINUX_IOCTL::TCSETS, state.as_mut_bytes_ptr())
        };
        is![res >= 0; Ok(()); Err(LinuxError::Sys(res))]
    }

    /// Returns `true` if we're in a terminal context.
    #[must_use]
    pub fn is_terminal() -> bool {
        match Self::get_state() {
            Ok(_) => true,
            Err(LinuxError::Sys(err)) => err != -LINUX_ERRNO::ENOTTY && err != -LINUX_ERRNO::EINVAL,
            Err(_) => false, // Other errors are not related to terminal checks
        }
    }

    /// Disables raw mode.
    pub fn disable_raw_mode() -> Result<()> {
        let mut state = LinuxTermios::get_state()?;
        state.c_lflag |= LINUX_TERMIOS_LFLAG::ICANON;
        state.c_lflag |= LINUX_TERMIOS_LFLAG::ECHO;
        LinuxTermios::set_state(state)
    }

    /// Enables raw mode.
    pub fn enable_raw_mode() -> Result<()> {
        let mut state = Self::get_state()?;
        state.c_lflag &= !LINUX_TERMIOS_LFLAG::ICANON;
        state.c_lflag &= !LINUX_TERMIOS_LFLAG::ECHO;
        LinuxTermios::set_state(state)
    }

    /// Returns the size of the window, in cells and pixels.
    pub fn get_winsize() -> Result<TermSize> {
        let mut winsize = TermSize::default();
        let res = unsafe {
            Linux::sys_ioctl(
                LINUX_FILENO::STDIN,
                LINUX_IOCTL::TIOCGWINSZ,
                &mut winsize as *mut TermSize as *mut u8,
            )
        };
        is![res >= 0; Ok(winsize); Err(LinuxError::Sys(res))]
    }
}
