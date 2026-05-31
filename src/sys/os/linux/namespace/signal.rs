// devela::sys::os::linux::namespace::signal

use crate::{AtomicOrdering::SeqCst, Ptr, c_int, c_void, is, transmute};
use crate::{
    LINUX_SIG_HANDLERS, LINUX_SIGINFO_HANDLERS, Linux, LinuxSigaction, LinuxSigactionFlags,
    LinuxSiginfo, LinuxSignal, LinuxSignalSet, LinuxSigset,
};

// RT signal-restorer routine defined in architecture-specific assembly.
//
// This routine is passed to `rt_sigaction` with `SA_RESTORER`,
// and performs the `rt_sigreturn` syscall needed to leave a Linux signal frame.
unsafe extern "C" {
    safe fn __devela_linux_restore_rt();
}

/// # Signaling-related methods.
impl Linux {
    /// Registers multiple signals using a simple handler function.
    ///
    /// This is a convenience wrapper over [`sig_handler_info`][Self::sig_handler_info].
    /// Internally it installs an `SA_SIGINFO` handler and discards the extra signal
    /// information before calling `handler`.
    ///
    /// # Arguments
    /// - `handler`: Function called with the received signal number.
    /// - `signals`: Signals to handle.
    /// - `flags`: Additional signal action flags.
    ///
    /// # Notes
    /// - `SA_RESTORER` and `SA_SIGINFO` are always included internally.
    /// - `SIGKILL` and `SIGSTOP` are ignored because they cannot be caught.
    /// - Signal handlers run asynchronously; keep them small and prefer atomics.
    ///
    /// # Examples
    /// ```no_run
    /// # #[cfg(feature = "std")] {
    /// use devela::{AtomicBool, AtomicI32, AtomicOrdering::SeqCst, Thread, ThreadExt};
    /// use devela::{Linux, LinuxSignal, LinuxSignalSet};
    ///
    /// static GOT_SIGNAL: AtomicBool = AtomicBool::new(false);
    /// static LAST_SIGNAL: AtomicI32 = AtomicI32::new(0);
    ///
    /// fn handler(sig: i32) {
    ///     LAST_SIGNAL.store(sig, SeqCst);
    ///     GOT_SIGNAL.store(true, SeqCst);
    /// }
    ///
    /// fn main() {
    ///     Linux::sig_handler(handler, LinuxSignalSet::SIGINT | LinuxSignalSet::SIGWINCH, None);
    ///
    ///     println!("press Ctrl+C or resize the terminal");
    ///     loop {
    ///         if GOT_SIGNAL.swap(false, SeqCst) {
    ///             let sig = LAST_SIGNAL.load(SeqCst);
    ///             println!("received signal {sig}");
    ///             if sig == LinuxSignal::SIGINT.as_c_int() {
    ///                 break;
    ///             }
    ///         }
    ///         Thread::sleep_ms(50);
    ///     }
    ///     println!("bye");
    /// }
    /// # }
    /// ```
    pub fn sig_handler(
        handler: fn(c_int),
        signals: LinuxSignalSet,
        flags: impl Into<LinuxSigactionFlags>,
    ) {
        fn simple_adapter(sig: c_int, _info: &LinuxSiginfo, _context: *mut c_void) {
            is! { LinuxSignal::from_c_int(sig).is_none(), return }
            let handler = LINUX_SIG_HANDLERS[sig as usize].load(SeqCst);
            if !handler.is_null() {
                // SAFETY: we control both storage and retrieval and check for null.
                let handler: fn(c_int) = unsafe { transmute(handler) };
                handler(sig);
            }
        }
        signals.for_each_catchable(|signal| {
            LINUX_SIG_HANDLERS[signal.as_c_int() as usize].store(handler as *mut (), SeqCst);
        });
        Self::sig_handler_info(simple_adapter, signals, flags);
    }

    /// Registers multiple signals using a handler that receives signal information.
    ///
    /// The handler receives the signal number, a borrowed [`LinuxSiginfo`], and the
    /// raw signal context pointer supplied by the kernel.
    ///
    /// # Arguments
    /// - `handler`: Function called with the signal number, signal info, and context.
    /// - `signals`: Signals to handle.
    /// - `flags`: Additional signal action flags.
    ///
    /// # Notes
    /// - `SA_RESTORER` and `SA_SIGINFO` are always included internally.
    /// - `SIGKILL` and `SIGSTOP` are ignored because they cannot be caught.
    /// - Signal handlers run asynchronously; keep them small and prefer atomics.
    /// - The `LinuxSiginfo` borrow is valid only during the handler call.
    ///
    /// # Examples
    /// ```no_run
    /// # #[cfg(feature = "std")] {
    /// use devela::{AtomicBool, AtomicI32, AtomicOrdering::SeqCst, Thread, ThreadExt, c_void};
    /// use devela::{Linux, LinuxSigactionFlags, LinuxSiginfo, LinuxSignalSet};
    ///
    /// static GOT_SIGNAL: AtomicBool = AtomicBool::new(false);
    /// static LAST_SIGNAL: AtomicI32 = AtomicI32::new(0);
    /// static LAST_PID: AtomicI32 = AtomicI32::new(0);
    ///
    /// fn handler(sig: i32, info: &LinuxSiginfo, _context: *mut c_void) {
    ///     LAST_SIGNAL.store(sig, SeqCst);
    ///     LAST_PID.store(info.pid(), SeqCst);
    ///     GOT_SIGNAL.store(true, SeqCst);
    /// }
    ///
    /// fn main() {
    ///     Linux::sig_handler_info(
    ///         handler,
    ///         LinuxSignalSet::SIGINT,
    ///         LinuxSigactionFlags::SA_RESTART,
    ///     );
    ///
    ///     println!("press Ctrl+C");
    ///     loop {
    ///         if GOT_SIGNAL.swap(false, SeqCst) {
    ///             let sig = LAST_SIGNAL.load(SeqCst);
    ///             let pid = LAST_PID.load(SeqCst);
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
        signals: LinuxSignalSet,
        flags: impl Into<LinuxSigactionFlags>,
    ) {
        extern "C" fn c_handler_siginfo(sig: c_int, info: *mut LinuxSiginfo, context: *mut c_void) {
            is! { LinuxSignal::from_c_int(sig).is_none() || info.is_null(), return }
            let handler = LINUX_SIGINFO_HANDLERS[sig as usize].load(SeqCst);
            if !handler.is_null() {
                // SAFETY: we control both storage and retrieval and check for null.
                let handler: fn(c_int, &LinuxSiginfo, *mut c_void) = unsafe { transmute(handler) };

                // SAFETY: kernel-provided for this SA_SIGINFO handler call.
                let info = unsafe { &*info };
                handler(sig, info, context);
            }
        }
        // let signals = signals.catchable_only();
        let flags = flags.into().with_sa_restorer().with_sa_siginfo();
        let sigaction = LinuxSigaction::new_siginfo(
            c_handler_siginfo,
            flags,
            LinuxSigset::empty(),
            Some(__devela_linux_restore_rt),
        );
        signals.for_each_catchable(|signal| {
            let raw = signal.as_c_int();
            LINUX_SIGINFO_HANDLERS[raw as usize].store(handler as *mut (), SeqCst);
            let _res = unsafe {
                Linux::sys_rt_sigaction(raw, &sigaction, Ptr::null_mut(), LinuxSigset::size())
            };
        });
    }
}
