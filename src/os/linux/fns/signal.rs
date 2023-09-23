// devela::os::linux::fns::signal
//
//! signal related functions
//

use super::super::{linux_sys_rt_sigaction, LinuxSigaction, LINUX_SIGACTION};

/// Register a single signal `handler` with multiple `signals`.
//
// IMPROVE documentation; add an example.
#[inline]
pub fn linux_handle_signals(handler: extern "C" fn(i32), signals: &[i32]) {
    for s in signals {
        unsafe {
            register_signal_handler(handler, *s, LINUX_SIGACTION::SA_RESTORER, 0);
        }
    }
}

/// Registers a multiple signals using the same handler function.
//
// THINK MAYBE make public
#[inline]
unsafe fn register_signal_handler(handler: extern "C" fn(i32), signal: i32, flags: u64, mask: u64) {
    let sigaction = LinuxSigaction::new(handler, flags, mask);
    unsafe {
        linux_sys_rt_sigaction(signal, &sigaction, core::ptr::null_mut(), 8);
    }
}
