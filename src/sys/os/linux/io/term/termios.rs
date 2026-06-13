// devela/src/sys/os/linux/io/term/termios.rs
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

#[doc = crate::_tags!(linux term abi)]
/// Represents the [`termios`] structure from libc, used to control terminal I/O.
#[doc = crate::_doc_meta!{
    location("sys/os/linux/io"),
    test_size_of(LinuxTermios = 36|288),
}]
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

    /// Special control characters.
    pub c_cc: [u8; 19],
}

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
    pub const fn as_mut_bytes_ptr(&mut self) -> *mut u8 {
        self as *mut Self as *mut u8
    }
}

/// # Linux terminal state syscalls
#[cfg(any_target_arch_linux)]
#[crate::macro_apply(crate::_unsafe_syscall_not_miri)]
impl LinuxTermios {
    /// Reads the current terminal termios state.
    pub fn read_state() -> Result<LinuxTermios> {
        let mut state = LinuxTermios::new();
        let res = unsafe {
            Linux::sys_ioctl(LINUX_FILENO::STDIN, LINUX_IOCTL::TCGETS, state.as_mut_bytes_ptr())
        };
        is![res >= 0, Ok(state), Err(LinuxError::Sys(res))]
    }
    /// Writes this termios state as the current terminal state.
    pub fn write_state(mut self) -> Result<()> {
        let res = unsafe {
            Linux::sys_ioctl(LINUX_FILENO::STDIN, LINUX_IOCTL::TCSETS, self.as_mut_bytes_ptr())
        };
        is![res >= 0, Ok(()), Err(LinuxError::Sys(res))]
    }
    /// Reads the current termios state, mutates it, and writes it back.
    pub fn update_state(f: impl FnOnce(&mut Self)) -> Result<()> {
        let mut state = Self::read_state()?;
        f(&mut state);
        state.write_state()
    }

    /// Returns `true` if standard input is attached to a terminal.
    #[must_use]
    pub fn is_terminal() -> bool {
        match Self::read_state() {
            Ok(_) => true,
            Err(LinuxError::Sys(err)) => err != -LINUX_ERRNO::ENOTTY && err != -LINUX_ERRNO::EINVAL,
            Err(_) => false, // Other errors are not related to terminal checks
        }
    }
    /// Reads the terminal window size, in cells and pixels.
    pub fn read_window_size() -> Result<TermSize> {
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

    /// Enables event-oriented terminal mode.
    pub fn enable_event_mode() -> Result<()> {
        Self::update_state(|state| state.make_event())
    }
    /// Enables raw terminal mode.
    pub fn enable_raw_mode() -> Result<()> {
        Self::update_state(|state| state.make_raw())
    }
    /// Best-effort reset to ordinary line-buffered terminal behavior.
    ///
    /// This does not restore the exact previous terminal state.
    /// Prefer a scoped terminal session for exact restoration.
    pub fn reset_cooked_mode() -> Result<()> {
        Self::update_state(|state| state.make_cooked_reset())
    }
}

/// # Terminal state transforms
#[rustfmt::skip]
impl LinuxTermios {
    /// Makes this state event-oriented.
    ///
    /// Event mode disables canonical line buffering and echo, allowing input to
    /// be read immediately while preserving terminal-generated signals.
    pub const fn make_event(&mut self) {
        self.remove_input_flags(I::IXON);
        self.remove_local_flags(L::ECHO);
        self.remove_local_flags(L::ICANON);
        // self.remove_local_flags(L::IEXTEN);
        self.set_read_min_timeout(1, 0);
    }
    /// Makes this state raw.
    ///
    /// Raw mode disables canonical input, echo, software flow control, input
    /// translations, output processing, extensions, and terminal-generated
    /// signal characters.
    ///
    /// In this mode, control keys such as Ctrl-C and Ctrl-Q are delivered as
    /// input bytes instead of being handled by the terminal line discipline.
    pub const fn make_raw(&mut self) {
        self.remove_input_flags(I::BRKINT);
        self.remove_input_flags(I::ICRNL);
        self.remove_input_flags(I::INPCK);
        self.remove_input_flags(I::ISTRIP);
        self.set_software_flow(false);

        self.remove_output_flags(O::OPOST);

        self.remove_local_flags(L::ECHO);
        self.remove_local_flags(L::ICANON);
        self.remove_local_flags(L::IEXTEN);
        self.remove_local_flags(L::ISIG);

        self.set_char_size(LinuxTermiosCharSize::Bits8);
        self.set_read_min_timeout(1, 0);
    }
    /// Makes this state conventionally line-buffered.
    ///
    /// This is a best-effort convenience reset, not an exact restoration of the
    /// previous terminal state. For exact restoration, use a scoped terminal session.
    pub const fn make_cooked_reset(&mut self) {
        // input
        self.set_software_flow(true);
        self.set_input_cr_to_lf(true);
        // local
        self.set_canonical(true);
        self.set_echo(true);
        self.set_signals(true);
        self.set_extensions(true);
        self.set_output_processing(true);
        self.set_read_min_timeout(1, 0);
    }

    /* local */
    /// Enables or disables echoing typed input.
    pub const fn set_echo(&mut self, enabled: bool) {
        if enabled { self.insert_local_flags(L::ECHO); }
        else { self.remove_local_flags(L::ECHO); }
    }
    /// Enables or disables canonical line buffering.
    pub const fn set_canonical(&mut self, enabled: bool) {
        if enabled { self.insert_local_flags(L::ICANON); }
        else { self.remove_local_flags(L::ICANON); }
    }
    /// Enables or disables terminal-generated signal characters.
    ///
    /// When enabled, characters such as Ctrl-C and Ctrl-Z are handled by the
    /// terminal line discipline instead of being delivered as ordinary input.
    pub const fn set_signals(&mut self, enabled: bool) {
        if enabled { self.insert_local_flags(L::ISIG); }
        else { self.remove_local_flags(L::ISIG); }
    }
    /// Enables or disables implementation-defined line discipline extensions.
    pub const fn set_extensions(&mut self, enabled: bool) {
        if enabled { self.insert_local_flags(L::IEXTEN); }
        else { self.remove_local_flags(L::IEXTEN); }
    }

    /* input */
    /// Enables or disables interrupt generation on BREAK.
    pub const fn set_break_interrupt(&mut self, enabled: bool) {
        if enabled { self.insert_input_flags(I::BRKINT); }
        else { self.remove_input_flags(I::BRKINT); }
    }
    /// Enables or disables translating carriage return to newline on input.
    pub const fn set_input_cr_to_lf(&mut self, enabled: bool) {
        if enabled { self.insert_input_flags(I::ICRNL); }
        else { self.remove_input_flags(I::ICRNL); }
    }
    /// Enables or disables XON/XOFF software flow control.
    pub const fn set_software_flow(&mut self, enabled: bool) {
        if enabled {
            // decides whether Ctrl-S/Ctrl-Q are intercepted for output flow control
            self.insert_input_flags(I::IXON);
            // sends start/stop control characters to the other side when input queues fill
            self.insert_input_flags(I::IXOFF);
        } else {
            self.remove_input_flags(I::IXON);
            self.remove_input_flags(I::IXOFF);
        }
    }

    /* output */
    /// Enables or disables terminal output post-processing.
    pub const fn set_output_processing(&mut self, enabled: bool) {
        if enabled { self.insert_output_flags(O::OPOST); }
        else { self.remove_output_flags(O::OPOST); }
    }

    /* control characters */
    /// Sets noncanonical read blocking behavior.
    ///
    /// - `min` is the minimum number of bytes required to satisfy a read.
    /// - `timeout_deciseconds` is the inter-byte timeout in tenths of a second.
    pub const fn set_read_min_timeout(&mut self, min: u8, timeout_deciseconds: u8) {
        self.set_cc(Cc::VMIN, min);
        self.set_cc(Cc::VTIME, timeout_deciseconds);
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
