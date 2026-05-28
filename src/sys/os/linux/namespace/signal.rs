// devela::sys::os::linux::namespace::signal

use crate::{
    AtomicOrdering, AtomicPtr, LINUX_SIGACTION as SIGACTION, LINUX_SIGNAL as SIGNAL, Linux,
    LinuxSigaction, LinuxSiginfo, LinuxSigset, Ptr, c_void, is, transmute,
};

/// Highest supported standard Linux signal number.
///
/// Signal numbers are used directly as table indices, so slot `0` is unused.
/// Handler tables must therefore have length `LINUX_SIG_MAX + 1`.
const LINUX_SIG_MAX: usize = 31;

/// Number of slots in the signal-handler tables.
const LINUX_SIG_TABLE_LEN: usize = LINUX_SIG_MAX + 1;

/// Simple Rust handlers indexed by signal number.
///
/// This table stores user-provided `fn(i32)` handlers registered through
/// [`Linux::sig_handler`]. They are not installed directly as kernel handlers.
/// Instead, the RT signal trampoline calls a siginfo-shaped adapter,
/// and that adapter looks up the simple handler here.
///
/// The stored pointer is the function's code address, erased through a raw pointer
/// for atomic storage. A null value means no simple handler is installed for that signal.
static LINUX_SIG_HANDLERS: [AtomicPtr<()>; LINUX_SIG_TABLE_LEN] =
    [const { AtomicPtr::new(Ptr::null_mut()) }; LINUX_SIG_TABLE_LEN];

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
static LINUX_SIGINFO_HANDLERS: [AtomicPtr<()>; LINUX_SIG_TABLE_LEN] =
    [const { AtomicPtr::new(Ptr::null_mut()) }; LINUX_SIG_TABLE_LEN];

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
    /// - `flags`: Optional [`LINUX_SIGACTION`][crate::LINUX_SIGACTION] flags.
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
    /// use devela::{Linux, LINUX_SIGNAL};
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
    ///     Linux::sig_handler(handler, &[LINUX_SIGNAL::SIGINT, LINUX_SIGNAL::SIGWINCH], None);
    ///     println!("press Ctrl+C or resize the terminal");
    ///     loop {
    ///         if GOT_SIGNAL.swap(false, AtomicOrdering::SeqCst) {
    ///             let sig = LAST_SIGNAL.load(AtomicOrdering::SeqCst);
    ///             println!("received signal {sig}");
    ///             if sig == LINUX_SIGNAL::SIGINT {
    ///                 break;
    ///             }
    ///         }
    ///         Thread::sleep_ms(50);
    ///     }
    ///     println!("bye");
    /// }
    /// # }
    /// ```
    pub fn sig_handler(handler: fn(i32), signals: &[i32], flags: Option<&[usize]>) {
        fn simple_adapter(sig: i32, _info: &LinuxSiginfo, _context: *mut c_void) {
            is![sig < 1 || sig > LINUX_SIG_MAX as i32, return];
            let handler = LINUX_SIG_HANDLERS[sig as usize].load(AtomicOrdering::SeqCst);
            if !handler.is_null() {
                #[expect(clippy::crosspointer_transmute, reason = "pointer to fn pointer")]
                // SAFETY: we control both storage and retrieval and check for null.
                let handler: fn(i32) = unsafe { transmute(handler) };
                handler(sig);
            }
        }
        for &sig in signals {
            if Linux::is_catchable_signal(sig) {
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
    /// - `flags`: Optional [`LINUX_SIGACTION`][crate::LINUX_SIGACTION] flags.
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
    /// use devela::{Linux, LinuxSiginfo, LINUX_SIGNAL, LINUX_SIGACTION};
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
    ///     Linux::sig_handler_info(
    ///         handler,
    ///         &[LINUX_SIGNAL::SIGINT],
    ///         Some(&[LINUX_SIGACTION::SA_RESTART]),
    ///     );
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
        extern "C" fn c_handler_siginfo(sig: i32, info: *mut LinuxSiginfo, context: *mut c_void) {
            is![sig < 1 || sig > LINUX_SIG_MAX as i32 || info.is_null(), return];
            let handler = LINUX_SIGINFO_HANDLERS[sig as usize].load(AtomicOrdering::SeqCst);
            if !handler.is_null() {
                #[expect(clippy::crosspointer_transmute, reason = "pointer to fn pointer")]
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
            if Linux::is_catchable_signal(sig) {
                LINUX_SIGINFO_HANDLERS[sig as usize]
                    .store(handler as *mut (), AtomicOrdering::SeqCst);
                unsafe {
                    let _ = Linux::sys_rt_sigaction(sig, &sigaction,
                        Ptr::null_mut(), LinuxSigset::size());
                }
            }
        }
    }
    /// Returns whether `sig` can be installed as a user signal handler.
    ///
    /// Accepts only supported standard Linux signal numbers and excludes
    /// `SIGKILL` and `SIGSTOP`, which the kernel never allows to be caught.
    const fn is_catchable_signal(sig: i32) -> bool {
        (sig as usize) >= 1
            && (sig as usize) <= LINUX_SIG_MAX
            && sig != SIGNAL::SIGKILL
            && sig != SIGNAL::SIGSTOP
    }
}
