// devela::sys::os::linux::io::term::termios
//
//! Terminal I/O configuration structure.
//!
//! Defines the [`LinuxTermios`] structure used to query and configure terminal
//! attributes via ioctl operations.
//
// TOC
// - struct LinuxTermios
// - impls
// - struct LinuxTermiosCharSize

use crate::c_uint;
#[crate::macro_apply(crate::_unsafe_syscall_not_miri)]
use crate::{
    LINUX_ERRNO, LINUX_FILENO, LINUX_IOCTL, Linux, LinuxError, LinuxResult as Result, TermSize, is,
};
use crate::{
    LinuxTermiosCc as Cc, LinuxTermiosControlFlags as C, LinuxTermiosInputFlags as I,
    LinuxTermiosLocalFlags as L, LinuxTermiosOutputFlags as O,
};

#[doc = crate::_tags!(linux term)]
/// Represents the [`termios`] structure from libc, used to control terminal I/O.
#[doc = crate::_doc_meta!{location("sys/os/linux/io")}]
///
/// It has fields for input, output, control, and local modes,
/// as well as a line discipline and control characters.
///
/// [`termios`]: https://man7.org/linux/man-pages/man3/termios.3.html
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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

#[crate::macro_apply(crate::__doc_show(feature = "dep_bytemuck"))]
#[cfg(all(feature = "unsafe_syscall", feature = "dep_bytemuck"))]
unsafe impl crate::_dep::bytemuck::NoUninit for LinuxTermios {}

/// # Raw representation helpers
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

/// # Linux terminal state operations
#[cfg(any_target_arch_linux)]
#[crate::macro_apply(crate::_unsafe_syscall_not_miri)]
impl LinuxTermios {
    /// Gets the current termios state into `state`.
    pub fn get_state() -> Result<LinuxTermios> {
        let mut state = LinuxTermios::new();
        let res = unsafe {
            Linux::sys_ioctl(LINUX_FILENO::STDIN, LINUX_IOCTL::TCGETS, state.as_mut_bytes_ptr())
        };
        is![res >= 0, Ok(state), Err(LinuxError::Sys(res))]
    }
    /// Sets the current termios `state`.
    pub fn set_state(mut state: LinuxTermios) -> Result<()> {
        let res = unsafe {
            Linux::sys_ioctl(LINUX_FILENO::STDIN, LINUX_IOCTL::TCSETS, state.as_mut_bytes_ptr())
        };
        is![res >= 0, Ok(()), Err(LinuxError::Sys(res))]
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
        state.insert_local_flags(L::ICANON | L::ECHO);
        LinuxTermios::set_state(state)
    }
    /// Enables raw mode.
    pub fn enable_raw_mode() -> Result<()> {
        let mut state = Self::get_state()?;
        state.remove_local_flags(L::ICANON | L::ECHO);
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
        is![res >= 0, Ok(winsize), Err(LinuxError::Sys(res))]
    }
}

/// # Typed flag accessors
#[rustfmt::skip]
impl LinuxTermios {

    /// Returns the input flags.
    #[must_use]
    pub const fn input_flags(&self) -> I { I::from_c_uint(self.c_iflag) }
    /// Replaces the input flags.
    pub const fn set_input_flags(&mut self, flags: I) { self.c_iflag = flags.as_c_uint(); }
    /// Inserts input flags.
    pub const fn insert_input_flags(&mut self, flags: I) { self.c_iflag |= flags.as_c_uint(); }
    /// Removes input flags.
    pub const fn remove_input_flags(&mut self, flags: I) { self.c_iflag &= !flags.as_c_uint(); }
    /// Returns whether all the given input flags are set.
    #[must_use]
    pub const fn has_input_flags(&self, flags: I) -> bool {
        (self.c_iflag & flags.as_c_uint()) == flags.as_c_uint()
    }

    /// Returns the output flags.
    #[must_use]
    pub const fn output_flags(&self) -> O { O::from_c_uint(self.c_oflag) }
    /// Replaces the output flags.
    pub const fn set_output_flags(&mut self, flags: O) { self.c_oflag = flags.as_c_uint(); }
    /// Inserts output flags.
    pub const fn insert_output_flags(&mut self, flags: O) { self.c_oflag |= flags.as_c_uint(); }
    /// Removes output flags.
    pub const fn remove_output_flags(&mut self, flags: O) { self.c_oflag &= !flags.as_c_uint(); }
    /// Returns whether all the given output flags are set.
    #[must_use]
    pub const fn has_output_flags(&self, flags: O) -> bool {
        (self.c_oflag & flags.as_c_uint()) == flags.as_c_uint()
    }

    /// Returns the control flags.
    #[must_use]
    pub const fn control_flags(&self) -> C { C::from_c_uint(self.c_cflag) }
    /// Replaces the control flags.
    pub const fn set_control_flags(&mut self, flags: C) { self.c_cflag = flags.as_c_uint(); }
    /// Inserts control flags.
    ///
    /// For masked fields such as character size, prefer field-specific helpers.
    pub const fn insert_control_flags(&mut self, flags: C) { self.c_cflag |= flags.as_c_uint(); }
    /// Removes control flags.
    ///
    /// For masked fields such as character size, prefer field-specific helpers.
    pub const fn remove_control_flags(&mut self, flags: C) { self.c_cflag &= !flags.as_c_uint(); }
    /// Returns whether all the given control flags are set.
    #[must_use]
    pub const fn has_control_flags(&self, flags: C) -> bool {
        (self.c_cflag & flags.as_c_uint()) == flags.as_c_uint()
    }

    /// Returns the local flags.
    #[must_use]
    pub const fn local_flags(&self) -> L { L::from_c_uint(self.c_lflag) }
    /// Replaces the local flags.
    pub const fn set_local_flags(&mut self, flags: L) { self.c_lflag = flags.as_c_uint(); }
    /// Inserts local flags.
    pub const fn insert_local_flags(&mut self, flags: L) { self.c_lflag |= flags.as_c_uint(); }
    /// Removes local flags.
    pub const fn remove_local_flags(&mut self, flags: L) { self.c_lflag &= !flags.as_c_uint(); }
    /// Returns whether all the given local flags are set.
    #[must_use]
    pub const fn has_local_flags(&self, flags: L) -> bool {
        (self.c_lflag & flags.as_c_uint()) == flags.as_c_uint()
    }
}

/// # Typed mode-field accessors
impl LinuxTermios {
    /// Returns the configured character size.
    #[must_use]
    pub const fn char_size(&self) -> LinuxTermiosCharSize {
        let bits = self.c_cflag & C::CSIZE.as_c_uint();

        if bits == C::CS6.as_c_uint() {
            LinuxTermiosCharSize::Bits6
        } else if bits == C::CS7.as_c_uint() {
            LinuxTermiosCharSize::Bits7
        } else if bits == C::CS8.as_c_uint() {
            LinuxTermiosCharSize::Bits8
        } else {
            LinuxTermiosCharSize::Bits5
        }
    }
    /// Sets the configured character size.
    pub const fn set_char_size(&mut self, size: LinuxTermiosCharSize) {
        let mask = C::CSIZE.as_c_uint();
        self.c_cflag = (self.c_cflag & !mask) | size.to_control_flags().as_c_uint();
    }
}

/// # Control-character accessors
#[rustfmt::skip]
impl LinuxTermios {
    /// Returns a terminal control-character value.
    #[must_use]
    pub const fn cc(&self, cc: Cc) -> u8 { self.c_cc[cc.index()] }
    /// Sets a terminal control-character value.
    pub const fn set_cc(&mut self, cc: Cc, value: u8) { self.c_cc[cc.index()] = value; }
    /// Returns a shared view of all terminal control-character values.
    #[must_use]
    pub const fn cc_array(&self) -> &[u8; Cc::COUNT] { &self.c_cc }
    /// Returns an exclusive view of all terminal control-character values.
    #[must_use]
    pub const fn cc_array_mut(&mut self) -> &mut [u8; Cc::COUNT] { &mut self.c_cc }
}

#[doc = crate::_tags!(linux term)]
/// A Linux termios character size.
#[doc = crate::_doc_meta!{location("sys/os/linux/io/term")}]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum LinuxTermiosCharSize {
    /// 5-bit characters.
    Bits5 = 5,
    /// 6-bit characters.
    Bits6 = 6,
    /// 7-bit characters.
    Bits7 = 7,
    /// 8-bit characters. (This the default)
    #[default]
    Bits8 = 8,
}
impl LinuxTermiosCharSize {
    /// Returns the corresponding control-flag field value.
    #[must_use]
    pub const fn to_control_flags(self) -> C {
        match self {
            Self::Bits5 => C::CS5,
            Self::Bits6 => C::CS6,
            Self::Bits7 => C::CS7,
            Self::Bits8 => C::CS8,
        }
    }
}
