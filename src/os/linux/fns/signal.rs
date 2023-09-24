// devela::os::linux::fns::signal
//
//! signal related functions
//

use super::super::{linux_sys_rt_sigaction, LinuxSigaction, LinuxSigset, LINUX_SIGACTION as SA};
use core::ptr::null_mut;

/// Registers multiple signals using a handler function that never returns.
///
/// # Examples
/// ```ignore
/// use std::{process::exit, time::Duration, thread::sleep};
/// use devela::os::linux::LINUX_SIGNAL as LS;
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
#[inline]
pub fn linux_sig_handler_no_return(handler: fn(i32) -> !, signals: &[i32]) {
    // We store the given `handler` function in a static to be able to call it
    // from the new extern function which can't capture its environment.
    static mut HANDLER: Option<fn(i32) -> !> = None;
    unsafe {
        HANDLER = Some(handler);
    }

    extern "C" fn c_handler(sig: i32) {
        unsafe {
            if let Some(handler) = HANDLER {
                handler(sig);
            }
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
                linux_sys_rt_sigaction(*s, &sigaction, null_mut(), LinuxSigset::size());
            }
        }
    }
}
