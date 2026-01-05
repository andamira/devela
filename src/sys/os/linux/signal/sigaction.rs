// devela::sys::os::linux::signal::sigaction
//
//! Signal action configuration and delivery metadata.
//!
//! Defines the `sigaction` structure, signal handler configuration,
//! signal masks, and associated metadata passed to handlers.
//!
//! Defines [`LinuxSigaction`], [`LinuxSiginfo`], [`LinuxSigset`].
//! As well as the private items: `LinuxSigactionHandler`, `LinuxSigval`.
//

#![cfg_attr(not(feature = "unsafe_syscall"), allow(dead_code))]

use crate::{Debug, FmtResult, Formatter, c_void};
#[cfg(feature = "unsafe_syscall")]
use crate::{LINUX_SIGACTION, impl_trait};

#[doc = crate::_TAG_LINUX!()]
/// Examine and change a signal action.
#[doc = crate::_doc_location!("sys/os/linux")]
///
/// Represents the [`sigaction`] structure from libc.
///
/// [`sigaction`]: https://man7.org/linux/man-pages/man2/sigaction.2.html
#[repr(C)]
#[must_use]
pub struct LinuxSigaction {
    /// A union that stores either a simple handler or a `SA_SIGINFO` handler.
    sa_handler: LinuxSigactionHandler,

    /// A set of flags which modify the behavior of the signal.
    sa_flags: usize,

    /// A function that restores the program's state after the signal handler runs.
    ///
    /// This is required for proper signal handling on some architectures.
    sa_restorer: Option<extern "C" fn()>,

    /// A mask of signals that should be blocked.
    sa_mask: LinuxSigset,
}
#[rustfmt::skip]
impl Debug for LinuxSigaction {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
        write!(f, "LinuxSigaction {{ ")?;
        #[cfg(feature = "unsafe_syscall")]
        if self.sa_flags & LINUX_SIGACTION::SA_SIGINFO == 0 {
            write!(f, "sa_handler: {:p}, ", unsafe { self.sa_handler.sa_handler })?;
        } else {
            write!(f, "sa_sigaction: {:p}, ", unsafe { self.sa_handler.sa_sigaction })?;
        }
        write!(f, "sa_flags: {:#x}, ", self.sa_flags)?;
        match self.sa_restorer {
            Some(ptr) => write!(f, "sa_restorer: {ptr:p}, ")?,
            None => write!(f, "sa_restorer: None, ")?,
        }
        write!(f, "sa_mask: {:?}", self.sa_mask)?;
        write!(f, " }}")
    }
}

impl LinuxSigaction {
    /// Returns a new `LinuxSigaction` for a simple handler.
    pub const fn new(
        handler: extern "C" fn(i32),
        flags: usize,
        mask: LinuxSigset,
        restorer: Option<extern "C" fn()>,
    ) -> Self {
        Self {
            sa_handler: LinuxSigactionHandler { sa_handler: handler },
            sa_flags: flags,
            sa_restorer: restorer,
            sa_mask: mask,
        }
    }
    /// Returns a new `LinuxSigaction` for a [`SA_SIGINFO`][LINUX_SIGACTION::SA_SIGINFO] handler.
    pub const fn new_siginfo(
        handler: extern "C" fn(i32, LinuxSiginfo, *mut c_void),
        flags: usize,
        mask: LinuxSigset,
        restorer: Option<extern "C" fn()>,
    ) -> Self {
        Self {
            sa_handler: LinuxSigactionHandler { sa_sigaction: handler },
            sa_flags: flags,
            sa_restorer: restorer,
            sa_mask: mask,
        }
    }
    /// Returns the simple handler, if set.
    #[cfg(feature = "unsafe_syscall")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_syscall")))]
    pub const fn sa_handler(&self) -> Option<extern "C" fn(i32)> {
        if self.sa_flags & LINUX_SIGACTION::SA_SIGINFO == 0 {
            Some(unsafe { self.sa_handler.sa_handler })
        } else {
            None
        }
    }
    /// Returns the `SA_SIGINFO` handler, if set.
    #[must_use]
    #[cfg(feature = "unsafe_syscall")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_syscall")))]
    pub const fn sa_sigaction(&self) -> Option<extern "C" fn(i32, LinuxSiginfo, *mut c_void)> {
        if self.sa_flags & LINUX_SIGACTION::SA_SIGINFO != 0 {
            Some(unsafe { self.sa_handler.sa_sigaction })
        } else {
            None
        }
    }
}
/// `Self::sa_handler` field constants.
impl LinuxSigaction {
    /// The default signal handling.
    pub const SIG_DFL: isize = 0;
    /// Ignore this signal.
    pub const SIG_IGN: isize = 1;
    // /// Error return from signal.
    // pub const SIG_ERR: isize = -1;
}

#[doc = crate::_TAG_LINUX!()]
/// A set of linux signals.
#[doc = crate::_doc_location!("sys/os/linux")]
///
/// Represents the [`sigset_t`] structure from libc.
///
/// [`sigset_t`]: https://man7.org/linux/man-pages/man7/system_data_types.7.html
#[repr(C)]
#[must_use]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LinuxSigset {
    /// An array of signals.
    ///
    /// Its lenght is calculated from the number of signals divided by the bits of a usize.
    pub sig: [usize; Self::LEN],
}
#[rustfmt::skip]
impl LinuxSigset {
    /// The number of bits in a `usize`.
    const BITS_PER_USIZE: usize = usize::BITS as usize;
    /// The hardcoded number of system signals defined in `LINUX_SIGNAL`.
    const NSIG: usize = 31;
    /// The size of the array is the number of signals divided by the bits of an usize.
    const LEN: usize = { Self::NSIG.div_ceil(Self::BITS_PER_USIZE) };

    /// Returns an empty set of signals.
    pub const fn empty() -> Self { Self { sig: [0; Self::LEN] } }
    /// Returns the size in bytes of `LinuxSigset`.
    #[must_use]
    pub const fn size() -> usize { size_of::<Self>() }
}
impl LinuxSigset {
    /// Sets the bit corresponding to a `signal` in the `sig` array.
    ///
    /// # Arguments
    /// * `signum` - The number of the signal. This should be between 1 and `NSIG`.
    ///
    /// # Panics
    /// Panics if `signum` < 1 or > 36.
    pub fn set_signal(&mut self, signal: i32) {
        let signal = signal as usize;
        assert![(1..=Self::NSIG).contains(&signal)];

        // Subtract 1 from the signal number because signal numbers start from 1
        // but array indices start from 0
        let signal_index = (signal - 1) / Self::BITS_PER_USIZE;
        let bit_position = (signal - 1) % Self::BITS_PER_USIZE;
        self.sig[signal_index] |= 1 << bit_position;
    }
}

#[doc = crate::_TAG_LINUX!()]
/// Additional information about a signal.
#[doc = crate::_doc_location!("sys/os/linux")]
///
/// Represents the [`siginfo_t`] structure from libc.
///
/// [`siginfo_t`]: https://man7.org/linux/man-pages/man2/sigaction.2.html
#[repr(C)]
#[must_use]
pub struct LinuxSiginfo {
    /// Signal number.
    si_signo: i32,
    /// Error number (if applicable).
    si_errno: i32,
    /// Indicates the reason or source of the signal.
    si_code: i32,
    /// Sender's PID.
    si_pid: i32,
    /// Sender's UID.
    si_uid: u32,
    /// Exit value or signal.
    si_status: i32,
    /// Faulting address (e.g., for SIGSEGV).
    si_addr: *mut c_void,
    /// Band event (for SIGPOLL).
    si_band: i64,
    /// Signal value.
    si_value: LinuxSigval,
}
#[rustfmt::skip]
impl LinuxSiginfo {
    /// Returns the signal number.
    pub fn signo(&self) -> i32 { self.si_signo }
    /// Returns the error number (if applicable).
    pub fn errno(&self) -> i32 { self.si_errno }
    /// Returns the signal code.
    pub fn code(&self) -> i32 { self.si_code } // IMPROVE: return si_codes
    /// Returns the sender's PID.
    pub fn pid(&self) -> i32 { self.si_pid }
    /// Returns the sender's UID.
    pub fn uid(&self) -> u32 { self.si_uid }
    /// Returns the exit value or signal.
    pub fn status(&self) -> i32 { self.si_status }
    /// Returns the faulting address (e.g., for SIGSEGV).
    pub fn addr(&self) -> *mut c_void { self.si_addr }
    /// Returns the band event (for SIGPOLL).
    pub fn band(&self) -> i64 { self.si_band }
    /// Returns the signal value as an integer.
    #[cfg(feature = "unsafe_syscall")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_syscall")))]
    pub fn value_int(&self) -> i32 { unsafe { self.si_value.int } }
    /// Returns the signal value as a pointer.
    #[cfg(feature = "unsafe_syscall")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_syscall")))]
    pub fn value_ptr(&self) -> *mut c_void { unsafe { self.si_value.ptr } }
}
#[rustfmt::skip]
impl Debug for LinuxSiginfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
        write!(f, "LinuxSiginfo {{ ")?;
        write!(f, "si_signo: {}, ", self.si_signo)?;
        write!(f, "si_errno: {}, ", self.si_errno)?;
        write!(f, "si_code: {}, ", self.si_code)?;
        write!(f, "si_pid: {}, ", self.si_pid)?;
        write!(f, "si_uid: {}, ", self.si_uid)?;
        write!(f, "si_status: {}, ", self.si_status)?;
        write!(f, "si_addr: {:?}, ", self.si_addr)?;
        write!(f, "si_band: {}, ", self.si_band)?;
        // IMPROVE: use si_codes
        #[cfg(feature = "unsafe_syscall")]
        write!(f, "si_value: LinuxSigval {{ sival_int: {}, sival_ptr: {:?} }}",
            unsafe { self.si_value.int }, unsafe { self.si_value.ptr })?;
        write!(f, " }}")
    }
}

/* private items */

/// A union representing either a simple handler or a `SA_SIGINFO` handler.
#[repr(C)]
union LinuxSigactionHandler {
    sa_handler: extern "C" fn(i32),
    sa_sigaction: extern "C" fn(i32, LinuxSiginfo, *mut c_void),
}

/// A union representing the signal value.
///
/// [`sigval`]: https://man7.org/linux/man-pages/man7/system_data_types.7.html
#[repr(C)]
union LinuxSigval {
    /// Integer value
    int: i32,
    /// Pointer value
    ptr: *mut c_void,
}
#[cfg(feature = "unsafe_syscall")]
impl_trait![fmt::Debug for LinuxSigval |self, f|
    write!(f, "LinuxSigval {{ int: {}, ptr: {:?} }}", unsafe { self.int }, unsafe { self.ptr })
];
