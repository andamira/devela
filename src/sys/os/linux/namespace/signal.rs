// devela::sys::os::linux::namespace::signal

use crate::{AtomicOrdering, Ptr, c_int, c_void, is, transmute};
use crate::{
    LINUX_SIG_HANDLERS, LINUX_SIGACTION as SIGACTION, LINUX_SIGINFO_HANDLERS, Linux,
    LinuxSigaction, LinuxSiginfo, LinuxSignalSet, LinuxSigset,
};

// RT signal-restorer routine defined in architecture-specific assembly.
//
// This routine is passed to `rt_sigaction` with `SA_RESTORER`,
// and performs the `rt_sigreturn` syscall needed to leave a Linux signal frame.
unsafe extern "C" {
    safe fn __devela_linux_restore_rt();
}

/// # Signaling-related methods.
#[rustfmt::skip]
impl Linux {
    /// Registers multiple signals using a simple handler function.
    ///
    /// This is a convenience wrapper over [`sig_handler_info`][Self::sig_handler_info].
    /// Internally it installs an `SA_SIGINFO` handler and discards the extra signal
    /// information before calling `handler`.
    ///
    /// # Arguments
    /// - `handler`: Function called with the received signal number.
    /// - `signals`: Signal numbers to handle.
    /// - `flags`: Optional `LinuxSigactionFlags`.
    ///
    /// # Notes
    /// - `SA_RESTORER` and `SA_SIGINFO` are always included internally.
    /// - `SIGKILL`, `SIGSTOP`, and out-of-range signal numbers are ignored.
    /// - Unknown flags in `flags` are ignored, and a warning is printed with `std`.
    /// - Signal handlers run asynchronously; keep them small and prefer atomics.
    ///
    /// # Examples
    /// ```no_run
    /// # #[cfg(feature = "std")] {
    /// use devela::{AtomicBool, AtomicI32, AtomicOrdering, Thread, ThreadExt};
    /// use devela::{Linux, LinuxSignalSet};
    ///
    /// static GOT_SIGNAL: AtomicBool = AtomicBool::new(false);
    /// static LAST_SIGNAL: AtomicI32 = AtomicI32::new(0);
    ///
    /// fn handler(sig: i32) {
    ///     LAST_SIGNAL.store(sig, AtomicOrdering::SeqCst);
    ///     GOT_SIGNAL.store(true, AtomicOrdering::SeqCst);
    /// }
    ///
    /// fn main() {
    ///     let sigint = LinuxSignalSet::SIGINT.as_c_int();
    ///     let sigwinch = LinuxSignalSet::SIGWINCH.as_c_int();
    ///     Linux::sig_handler(handler, &[sigint, sigwinch], None);
    ///
    ///     println!("press Ctrl+C or resize the terminal");
    ///     loop {
    ///         if GOT_SIGNAL.swap(false, AtomicOrdering::SeqCst) {
    ///             let sig = LAST_SIGNAL.load(AtomicOrdering::SeqCst);
    ///             println!("received signal {sig}");
    ///             if sig == sigint {
    ///                 break;
    ///             }
    ///         }
    ///         Thread::sleep_ms(50);
    ///     }
    ///     println!("bye");
    /// }
    /// # }
    /// ```
    // pub fn sig_handler(handler: fn(c_int), signals: LinuxSignalSet, flags: Option<LinuxSigactionFlags>) {
    pub fn sig_handler(handler: fn(c_int), signals: &[i32], flags: Option<&[usize]>) {
        #[allow(clippy::manual_range_contains)]
        fn simple_adapter(sig: c_int, _info: &LinuxSiginfo, _context: *mut c_void) {
            is![sig < LinuxSignalSet::MIN || sig > LinuxSignalSet::MAX, return];
            let handler = LINUX_SIG_HANDLERS[sig as usize].load(AtomicOrdering::SeqCst);
            if !handler.is_null() {
                // SAFETY: we control both storage and retrieval and check for null.
                let handler: fn(c_int) = unsafe { transmute(handler) };
                handler(sig);
            }
        }
        for &sig in signals {
            if LinuxSignalSet::is_catchable_signal(sig) {
                LINUX_SIG_HANDLERS[sig as usize].store(handler as *mut (), AtomicOrdering::SeqCst);
            }
        }
        Self::sig_handler_info(simple_adapter, signals, flags);
    }

    /// Registers multiple signals using a handler that receives signal information.
    ///
    /// The handler receives the signal number, a borrowed [`LinuxSiginfo`], and the
    /// raw signal context pointer supplied by the kernel.
    ///
    /// # Arguments
    /// - `handler`: Function called with the signal number, signal info, and context.
    /// - `signals`: Signal numbers to handle.
    /// - `flags`: Optional `LinuxSigactionFlags`.
    ///
    /// # Notes
    /// - `SA_RESTORER` and `SA_SIGINFO` are always included internally.
    /// - The `LinuxSiginfo` borrow is valid only during the handler call.
    /// - `SIGKILL`, `SIGSTOP`, and out-of-range signal numbers are ignored.
    /// - Unknown flags in `flags` are ignored, and a warning is printed with `std`.
    /// - Signal handlers run asynchronously; keep them small and prefer atomics.
    ///
    /// # Examples
    /// ```no_run
    /// # #[cfg(feature = "std")] {
    /// use devela::{AtomicBool, AtomicI32, AtomicOrdering, Thread, ThreadExt, c_void};
    /// use devela::{Linux, LinuxSigactionFlags, LinuxSiginfo, LinuxSignalSet};
    ///
    /// static GOT_SIGNAL: AtomicBool = AtomicBool::new(false);
    /// static LAST_SIGNAL: AtomicI32 = AtomicI32::new(0);
    /// static LAST_PID: AtomicI32 = AtomicI32::new(0);
    ///
    /// fn handler(sig: i32, info: &LinuxSiginfo, _context: *mut c_void) {
    ///     LAST_SIGNAL.store(sig, AtomicOrdering::SeqCst);
    ///     LAST_PID.store(info.pid(), AtomicOrdering::SeqCst);
    ///     GOT_SIGNAL.store(true, AtomicOrdering::SeqCst);
    /// }
    ///
    /// fn main() {
    ///     let sigint = LinuxSignalSet::SIGINT.as_c_int();
    ///     let restart = LinuxSigactionFlags::SA_RESTART.as_c_size_t();
    ///     Linux::sig_handler_info(handler, &[sigint], Some(&[restart]));
    ///
    ///     println!("press Ctrl+C");
    ///     loop {
    ///         if GOT_SIGNAL.swap(false, AtomicOrdering::SeqCst) {
    ///             let sig = LAST_SIGNAL.load(AtomicOrdering::SeqCst);
    ///             let pid = LAST_PID.load(AtomicOrdering::SeqCst);
    ///             println!("received signal {sig} from pid {pid}");
    ///             break;
    ///         }
    ///         Thread::sleep_ms(50);
    ///     }
    ///     println!("bye");
    /// }
    /// # }
    /// ```
    pub fn sig_handler_info(
        handler: fn(i32, &LinuxSiginfo, *mut c_void),
        signals: &[i32],
        flags: Option<&[usize]>,
    ) {
        #[allow(clippy::manual_range_contains)]
        extern "C" fn c_handler_siginfo(sig: i32, info: *mut LinuxSiginfo, context: *mut c_void) {
            is![sig < LinuxSignalSet::MIN || sig > LinuxSignalSet::MAX || info.is_null(), return];
            let handler = LINUX_SIGINFO_HANDLERS[sig as usize].load(AtomicOrdering::SeqCst);
            if !handler.is_null() {
                // SAFETY: we control both the storage and retrieval and ensure null checks.
                let handler: fn(i32, &LinuxSiginfo, *mut c_void) = unsafe { transmute(handler) };

                // SAFETY: kernel-provided for this SA_SIGINFO handler call.
                let info = unsafe { &*info };
                handler(sig, info, context);
            }
        }
        let mut combined_flags = SIGACTION::SA_RESTORER | SIGACTION::SA_SIGINFO;
        if let Some(provided_flags) = flags {
            for &flag in provided_flags {
                match flag {
                    SIGACTION::SA_NOCLDSTOP
                    | SIGACTION::SA_NOCLDWAIT
                    | SIGACTION::SA_NODEFER
                    | SIGACTION::SA_ONSTACK
                    | SIGACTION::SA_RESETHAND
                    | SIGACTION::SA_RESTART => { combined_flags |= flag; }
                    SIGACTION::SA_RESTORER | SIGACTION::SA_SIGINFO => {} // already forced
                    _ => {
                        #[cfg(feature = "std")]
                        eprintln!("Warning: Ignoring unknown flag: {flag:#x}");
                    }
                }
            }
        }
        let mask = LinuxSigset::empty();
        let sigaction = LinuxSigaction::new_siginfo(c_handler_siginfo,
            combined_flags, mask, Some(__devela_linux_restore_rt));
        for &sig in signals {
            if LinuxSignalSet::is_catchable_signal(sig) {
                LINUX_SIGINFO_HANDLERS[sig as usize]
                    .store(handler as *mut (), AtomicOrdering::SeqCst);
                unsafe {
                    let _ = Linux::sys_rt_sigaction(sig, &sigaction,
                        Ptr::null_mut(), LinuxSigset::size());
                }
            }
        }
    }
}
