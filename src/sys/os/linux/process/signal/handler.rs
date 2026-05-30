// devela::sys::os::linux::process::signal::handler
//
//! Defines `LinuxSigactionHandler`, and static tables.
//

#[cfg(doc)]
use crate::Linux;
use crate::{AtomicPtr, LinuxSiginfo, LinuxSignalSet, Ptr, c_void};

#[cfg(target_pointer_width = "32")]
crate::test_size_of!(LinuxSigactionHandler = 4 | 32);
#[cfg(target_pointer_width = "64")]
crate::test_size_of!(LinuxSigactionHandler = 8 | 64);

const LINUX_SIG_HANDLER_TABLE_LEN: usize = LinuxSignalSet::MAX as usize + 1;
const __: () = {
    assert![LINUX_SIG_HANDLER_TABLE_LEN == 32];
};

/// A union representing either a simple handler or a `SA_SIGINFO` handler.
#[repr(C)]
pub(crate) union LinuxSigactionHandler {
    pub(super) sa_handler: extern "C" fn(i32),
    pub(super) sa_sigaction: extern "C" fn(i32, *mut LinuxSiginfo, *mut c_void),
}

/// Simple Rust handlers indexed by signal number.
///
/// This table stores user-provided `fn(i32)` handlers registered through
/// [`Linux::sig_handler`]. They are not installed directly as kernel handlers.
/// Instead, the RT signal trampoline calls a siginfo-shaped adapter,
/// and that adapter looks up the simple handler here.
///
/// The stored pointer is the function's code address, erased through a raw pointer
/// for atomic storage. A null value means no simple handler is installed for that signal.
pub(crate) static LINUX_SIG_HANDLERS: [AtomicPtr<()>; LINUX_SIG_HANDLER_TABLE_LEN] =
    [const { AtomicPtr::new(Ptr::null_mut()) }; LINUX_SIG_HANDLER_TABLE_LEN];

/// RT/siginfo-shaped Rust handlers indexed by signal number.
///
/// This is the canonical dispatch table used by the kernel-facing `SA_SIGINFO`
/// trampoline. Entries registered through [`Linux::sig_handler_info`] point
/// to the user-provided info handler directly. Entries registered through
/// [`Linux::sig_handler`] point to a small adapter that discards the extra
/// signal information and then dispatches through [`LINUX_SIG_HANDLERS`].
///
/// The stored pointer is the function's code address, erased through a raw pointer
/// for atomic storage. A null value means no siginfo handler is installed for that signal.
pub(crate) static LINUX_SIGINFO_HANDLERS: [AtomicPtr<()>; LINUX_SIG_HANDLER_TABLE_LEN] =
    [const { AtomicPtr::new(Ptr::null_mut()) }; LINUX_SIG_HANDLER_TABLE_LEN];
