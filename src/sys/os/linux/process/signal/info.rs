// devela/src/sys/os/linux/process/signal/info.rs
//
//! Defines [`LinuxSiginfo`] and (`LinuxSigval`).
//
// IMPROVE: add LinuxSiginfoCode

use crate::{c_void, impl_trait};

#[doc = crate::_tags!(linux signal abi)]
/// Additional information about a signal.
#[doc = crate::_doc_meta!{
    location("sys/os/linux/process"),
    #[cfg(target_pointer_width = "32")]
    test_size_of(LinuxSiginfo = 40|320),
    #[cfg(target_pointer_width = "64")]
    test_size_of(LinuxSiginfo = 48|384),
}]
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
    pub fn code(&self) -> i32 { self.si_code }
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
    pub fn value_int(&self) -> i32 { unsafe { self.si_value.int } }
    /// Returns the signal value as a pointer.
    #[cfg(feature = "unsafe_syscall")]
    pub fn value_ptr(&self) -> *mut c_void { unsafe { self.si_value.ptr } }
}
impl_trait! { fmt::Debug for LinuxSiginfo |self, f| {
    let mut fmt = f.debug_struct("LinuxSiginfo");
    fmt.field("si_signo", &self.si_signo)
        .field("si_errno", &self.si_errno)
        .field("si_code", &self.si_code)
        .field("si_pid", &self.si_pid)
        .field("si_uid", &self.si_uid)
        .field("si_status", &self.si_status)
        .field("si_addr", &self.si_addr)
        .field("si_band", &self.si_band);
    #[cfg(feature = "unsafe_syscall")]
    fmt.field("si_value", &self.si_value);
    fmt.finish()
}}

#[cfg(target_pointer_width = "32")]
crate::test_size_of!(LinuxSigval = 4 | 32);
#[cfg(target_pointer_width = "64")]
crate::test_size_of!(LinuxSigval = 8 | 64);
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
    write!(
        f,
        "LinuxSigval {{ sival_int: {}, sival_ptr: {:?} }}",
        unsafe { self.int },
        unsafe { self.ptr },
    )
];
