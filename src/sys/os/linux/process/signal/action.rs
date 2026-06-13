// devela/src/sys/os/linux/process/signal/action.rs
//
//! Signal action configuration and delivery metadata.
//!
//! Defines the `sigaction` structure, signal handler configuration,
//! signal masks, and associated metadata passed to handlers.
//!
//! Defines [`LinuxSigaction`], [`LinuxSigset`], and (`LinuxSigactionHandler`).
//

#[cfg(feature = "unsafe_syscall")]
use crate::LINUX_SIGACTION;
use crate::{Debug, FmtResult, Formatter, c_size_t, c_void};
use crate::{LinuxSigactionFlags, LinuxSigactionHandler, LinuxSiginfo, LinuxSigset};

#[doc = crate::_tags!(linux signal abi)]
/// Examine and change a signal action.
#[doc = crate::_doc_meta!{
    location("sys/os/linux/process"),
    #[cfg(target_pointer_width = "32")]
    test_size_of(LinuxSigaction = 16|128),
    #[cfg(target_pointer_width = "64")]
    test_size_of(LinuxSigaction = 32|256),
}]
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
    sa_flags: c_size_t,

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
        flags: LinuxSigactionFlags,
        mask: LinuxSigset,
        restorer: Option<extern "C" fn()>,
    ) -> Self {
        Self {
            sa_handler: LinuxSigactionHandler { sa_handler: handler },
            sa_flags: flags.as_c_size_t(),
            sa_restorer: restorer,
            sa_mask: mask,
        }
    }
    /// Returns a new `LinuxSigaction`
    /// for a [`SA_SIGINFO`][LinuxSigactionFlags::SA_SIGINFO] handler.
    pub const fn new_siginfo(
        handler: extern "C" fn(i32, *mut LinuxSiginfo, *mut c_void),
        flags: LinuxSigactionFlags,
        mask: LinuxSigset,
        restorer: Option<extern "C" fn()>,
    ) -> Self {
        Self {
            sa_handler: LinuxSigactionHandler { sa_sigaction: handler },
            sa_flags: flags.as_c_size_t(),
            sa_restorer: restorer,
            sa_mask: mask,
        }
    }

    /// Returns the simple handler, if set.
    #[cfg(feature = "unsafe_syscall")]
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
    pub const fn sa_sigaction(&self) -> Option<extern "C" fn(i32, *mut LinuxSiginfo, *mut c_void)> {
        if self.sa_flags & LINUX_SIGACTION::SA_SIGINFO != 0 {
            Some(unsafe { self.sa_handler.sa_sigaction })
        } else {
            None
        }
    }

    /// Returns the signal action flags.
    #[must_use]
    pub const fn flags(&self) -> LinuxSigactionFlags {
        LinuxSigactionFlags::from_c_size_t(self.sa_flags as c_size_t)
    }
    /// Returns the raw Linux `sa_flags` word.
    #[must_use]
    pub const fn raw_flags(&self) -> c_size_t {
        self.sa_flags
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
