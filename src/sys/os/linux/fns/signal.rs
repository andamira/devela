// devela::sys::os::linux::fns::signal
//
//! signal related functions
//

use crate::{
    linux_sys_rt_sigaction, AtomicOrdering, AtomicPtr, LinuxSigaction, LinuxSigset,
    LINUX_SIGACTION as SA,
};
use core::{mem::transmute, ptr::null_mut};

/// Registers multiple signals using a handler function that never returns.
///
/// # Examples
/// ```no_run
/// use std::{process::exit, time::Duration, thread::sleep};
/// use devela::{linux_sig_handler_no_return, LINUX_SIGNAL as LS};
///
/// fn handler(sig: i32) -> ! {
///    println!("\nsignal `{sig}` received! exiting. . .");
///    exit(1);
/// }
///
/// fn main() {
///     // handle all the signals used to quit
///     linux_sig_handler_no_return(handler, &[LS::SIGINT, LS::SIGQUIT, LS::SIGSEGV, LS::SIGABRT]);
///     // press Ctrl+C before the time expires to catch the SIGINT signal
///     sleep(Duration::from_secs(2));
///     println!("bye");
/// }
/// ```
///
/// # Rationale
/// It would be very nice to be able to register a signal handler that can return,
/// unfortunately I've been unable to make it work.
///
/// Apparently the handler needs the [`SA_RESTORER`] flag to run, but doing so
/// without providing a restorer function produces a segmentation fault. The only
/// way to simply avoid that is to not return from the handler function.
///
/// The `libc` library sets it up correctly but doing so manually seems a too
/// complex too low level task.
///
/// [`SA_RESTORER`]: SA::SA_RESTORER
pub fn linux_sig_handler_no_return(handler: fn(i32) -> !, signals: &[i32]) {
    // We store the given `handler` function in a static to be able to call it
    // from the new extern function which can't capture its environment.
    static HANDLER: AtomicPtr<fn(i32) -> !> = AtomicPtr::new(null_mut());
    HANDLER.store(handler as *mut _, AtomicOrdering::SeqCst);

    extern "C" fn c_handler(sig: i32) {
        let handler = HANDLER.load(AtomicOrdering::SeqCst);
        if !handler.is_null() {
            #[allow(clippy::crosspointer_transmute)]
            // SAFETY: The non-null pointer is originally created from a `fn(i32) -> !` pointer.
            let handler = unsafe { transmute::<*mut fn(i32) -> !, fn(i32) -> !>(handler) };
            handler(sig);
        }
    }

    // Apparently Rust doesn't call the handler unless we set the SA_RESTORER flag.
    let flags = SA::SA_RESETHAND | SA::SA_RESTORER;
    let mask = LinuxSigset::default();
    let sigaction = LinuxSigaction::new(c_handler, flags, mask);

    for s in signals {
        // make sure the signal is a valid number
        if (1..=36).contains(s) {
            unsafe {
                let _ = linux_sys_rt_sigaction(*s, &sigaction, null_mut(), LinuxSigset::size());
            }
        }
    }
}
