// devela::sys::os::linux::process::signal::set
//
//! Defines [`LinuxSignalSet`], [`LinuxSigset`].
//

use crate::{LINUX_SIGNAL as S, c_int, c_size_t};

crate::set! {
    #[doc = crate::_tags!(linux signal)]
    /// A compact semantic set of standard Linux signals.
    #[doc = crate::_doc_meta!{
        location("sys/os/term/session"),
        test_size_of(LinuxSignalSet = 4|32),
    }]
    /// This is not the raw Linux `sigset_t` mask. Signal bits are stored
    /// compactly from zero: `SIGHUP` is bit `0`, `SIGINT` is bit `1`, and so on.
    //
    // For the terminal, the key ones are:
    // - SIGINT    // Ctrl+C
    // - SIGQUIT   // Ctrl+\
    // - SIGTSTP   // Ctrl+Z
    // - SIGTTIN   // background terminal read
    // - SIGTTOU   // background terminal write
    // - SIGWINCH  // terminal resize
    // - SIGHUP    // terminal/session hangup
    pub struct LinuxSignalSet(u32) {
        /// Terminal hangup or controlling process ended.
        SIGHUP = 0;
        /// Keyboard interrupt, usually `Ctrl+C`.
        SIGINT = 1;
        /// Keyboard quit, usually `Ctrl+\`; often produces a core dump.
        SIGQUIT = 2;
        /// Illegal CPU instruction.
        SIGILL = 3;
        /// Trace or breakpoint trap, commonly used by debuggers.
        SIGTRAP = 4;
        /// Process abort, usually from `abort()`.
        SIGABRT = 5;
        /// Bus error, usually invalid or misaligned memory access.
        SIGBUS = 6;
        /// Floating-point exception, such as division by zero in some contexts.
        SIGFPE = 7;
        /// Uncatchable forced termination.
        SIGKILL = 8;
        /// User-defined signal 1.
        SIGUSR1 = 9;
        /// Invalid memory reference, usually a segmentation fault.
        SIGSEGV = 10;
        /// User-defined signal 1.
        SIGUSR2 = 11;
        /// Broken pipe, usually writing to a pipe or socket with no reader.
        SIGPIPE = 12;
        /// Alarm timer expired, usually from `alarm()`.
        SIGALRM = 13;
        /// Polite termination request, commonly sent by `kill` or service managers.
        SIGTERM = 14;
        /// Stack fault signal, obsolete on most modern Linux systems.
        SIGSTKFLT = 15;
        /// Child process stopped, continued, or exited.
        SIGCHLD = 16;
        /// Continue a stopped process.
        SIGCONT = 17;
        /// Uncatchable stop signal.
        SIGSTOP = 18;
        /// Terminal stop, usually `Ctrl+Z`.
        SIGTSTP = 19;
        /// Background process tried to read from the terminal.
        SIGTTIN = 20;
        /// Background process tried to write to the terminal.
        SIGTTOU = 21;
        /// Urgent condition on a socket.
        SIGURG = 22;
        /// CPU time limit exceeded.
        SIGXCPU = 23;
        /// File size limit exceeded.
        SIGXFSZ = 24;
        /// Virtual interval timer expired.
        SIGVTALRM = 25;
        /// Profiling timer expired.
        SIGPROF = 26;
        /// Terminal or window size changed.
        SIGWINCH = 27;
        /// Asynchronous I/O event is available.
        SIGIO = 28;
        /// Power failure or power-management event.
        SIGPWR = 29;
        /// Bad system call, often from seccomp filtering.
        SIGSYS = 30;
    }
    /// Aliases
    impl {
        /// IOT trap. Compatibility alias for [`SIGABRT`][Self::SIGABRT].
        pub const SIGIOT: Self = Self::SIGABRT;
        /// Compatibility alias for [`SIGCHLD`][Self::SIGCHLD].
        pub const SIGCLD: Self = Self::SIGCHLD;
        /// Pollable event. Compatibility alias for [`SIGIO`][Self::SIGIO].
        pub const SIGPOLL: Self = Self::SIGIO;
        // /// Compatibility alias for [`SIGPWR`][Self::SIGPWR].
        // ///
        // /// This is not the BSD terminal status signal commonly associated with `Ctrl+T`.
        // pub const SIGINFO: Self = Self::SIGPWR;
        //
        // /// Obsolete compatibility alias for [`SIGSYS`][Self::SIGSYS].
        // pub const SIGUNUSED: Self = Self::SIGSYS;
    }
    impl {
        /// Lowest supported standard Linux signal number.
        pub const MIN: c_int = 1;
        /// Highest supported standard Linux signal number.
        pub const MAX: c_int = S::SIGSYS;

        /// Returns whether `signal` is in the supported standard Linux range.
        #[must_use]
        pub const fn is_standard_signal(signal: c_int) -> bool {
            signal >= Self::MIN && signal <= Self::MAX
        }

        /// Returns whether `signal` can be installed as a user handler.
        #[must_use]
        pub const fn is_catchable_signal(signal: c_int) -> bool {
            Self::is_standard_signal(signal)
                && signal != crate::LINUX_SIGNAL::SIGKILL
                && signal != crate::LINUX_SIGNAL::SIGSTOP
        }

        /// Returns a set containing one raw Linux signal number.
        ///
        /// Returns an empty set if `signal` is outside the supported range.
        pub const fn from_c_int(signal: c_int) -> Self {
            if Self::is_standard_signal(signal) {
                Self::from_bits(1_u32 << ((signal - 1) as u32))
            } else {
                Self::new()
            }
        }

        /// Returns the raw Linux signal number if this set contains exactly one signal.
        #[must_use]
        pub const fn to_c_int(self) -> Option<c_int> {
            let bits = self.bits();
            if bits != 0 && bits.is_power_of_two() {
                Some(bits.trailing_zeros() as c_int + 1)
            } else {
                None
            }
        }
        /// Returns this set as a raw Linux signal number, or `0` if not exactly one signal.
        ///
        /// Signal number `0` is used here as an invalid sentinel for this semantic set.
        #[must_use]
        pub const fn as_c_int(self) -> c_int {
            match self.to_c_int() {
                Some(signal) => signal,
                None => 0,
            }
        }
        /// Returns this semantic set encoded as a raw Linux signal mask.
        pub const fn to_sigset(self) -> LinuxSigset {
            LinuxSigset::from_signal_set(self)
        }
        const _ASSERT_RAW_VALUES: () = {
            assert!(Self::SIGHUP.as_c_int()    == S::SIGHUP);
            assert!(Self::SIGINT.as_c_int()    == S::SIGINT);
            assert!(Self::SIGQUIT.as_c_int()   == S::SIGQUIT);
            assert!(Self::SIGILL.as_c_int()    == S::SIGILL);
            assert!(Self::SIGTRAP.as_c_int()   == S::SIGTRAP);
            assert!(Self::SIGABRT.as_c_int()   == S::SIGABRT);
            assert!(Self::SIGIOT.as_c_int()    == S::SIGIOT);
            assert!(Self::SIGBUS.as_c_int()    == S::SIGBUS);
            assert!(Self::SIGFPE.as_c_int()    == S::SIGFPE);
            assert!(Self::SIGKILL.as_c_int()   == S::SIGKILL);
            assert!(Self::SIGUSR1.as_c_int()   == S::SIGUSR1);
            assert!(Self::SIGSEGV.as_c_int()   == S::SIGSEGV);
            assert!(Self::SIGUSR2.as_c_int()   == S::SIGUSR2);
            assert!(Self::SIGPIPE.as_c_int()   == S::SIGPIPE);
            assert!(Self::SIGALRM.as_c_int()   == S::SIGALRM);
            assert!(Self::SIGTERM.as_c_int()   == S::SIGTERM);
            assert!(Self::SIGSTKFLT.as_c_int() == S::SIGSTKFLT);
            assert!(Self::SIGCHLD.as_c_int()   == S::SIGCHLD);
            assert!(Self::SIGCLD.as_c_int()    == S::SIGCLD);
            assert!(Self::SIGCONT.as_c_int()   == S::SIGCONT);
            assert!(Self::SIGSTOP.as_c_int()   == S::SIGSTOP);
            assert!(Self::SIGTSTP.as_c_int()   == S::SIGTSTP);
            assert!(Self::SIGTTIN.as_c_int()   == S::SIGTTIN);
            assert!(Self::SIGTTOU.as_c_int()   == S::SIGTTOU);
            assert!(Self::SIGURG.as_c_int()    == S::SIGURG);
            assert!(Self::SIGXCPU.as_c_int()   == S::SIGXCPU);
            assert!(Self::SIGXFSZ.as_c_int()   == S::SIGXFSZ);
            assert!(Self::SIGVTALRM.as_c_int() == S::SIGVTALRM);
            assert!(Self::SIGPROF.as_c_int()   == S::SIGPROF);
            assert!(Self::SIGWINCH.as_c_int()  == S::SIGWINCH);
            assert!(Self::SIGIO.as_c_int()     == S::SIGIO);
            assert!(Self::SIGPOLL.as_c_int()   == S::SIGPOLL);
            assert!(Self::SIGPWR.as_c_int()    == S::SIGPWR);
            // assert!(Self::SIGINFO.as_c_int()   == S::SIGINFO);
            assert!(Self::SIGSYS.as_c_int()    == S::SIGSYS);
            // assert!(Self::SIGUNUSED.as_c_int() == S::SIGUNUSED);
            assert!(Self::from_c_int(S::SIGHUP).has(Self::SIGHUP));
            assert!(Self::from_c_int(S::SIGSYS).has(Self::SIGSYS));
            assert!(Self::from_c_int(0).is_empty());
            assert!(Self::from_c_int(32).is_empty());
        };
    }
}
impl From<LinuxSignalSet> for LinuxSigset {
    fn from(signals: LinuxSignalSet) -> Self {
        Self::from_signal_set(signals)
    }
}
impl From<LinuxSigset> for LinuxSignalSet {
    fn from(mask: LinuxSigset) -> Self {
        mask.to_signal_set()
    }
}

#[doc = crate::_tags!(linux signal abi)]
/// A raw Linux kernel signal mask.
#[doc = crate::_doc_meta!{
    location("sys/os/linux/process"),
    #[cfg(target_pointer_width = "32")]
    test_size_of(LinuxSigset = 4|32),
    #[cfg(target_pointer_width = "64")]
    test_size_of(LinuxSigset = 8|64),
}]
/// This is the mask representation passed to raw Linux signal syscalls.
///
/// Signal numbers start at `1`, while mask bits start at `0`,
/// so signal `n` is stored in bit `n - 1`.
///
/// For the semantic Devela set, use [`LinuxSignalSet`].
//
// This one resembles the [`sigset_t`] structure from libc.
// [`sigset_t`]: https://man7.org/linux/man-pages/man7/system_data_types.7.html
#[repr(C)]
#[must_use]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LinuxSigset {
    /// Raw signal mask words.
    pub sig: [c_size_t; Self::WORDS],
}
#[rustfmt::skip]
impl LinuxSigset {
    /// Number of bits in one raw mask word.
    pub const WORD_BITS: c_size_t = usize::BITS as usize;
    /// Highest supported standard Linux signal number.
    pub const MAX_SIGNAL: c_size_t = LinuxSignalSet::MAX as usize;
    /// Number of raw mask words needed for the supported signal range.
    pub const WORDS: c_size_t = Self::MAX_SIGNAL.div_ceil(Self::WORD_BITS);

    /// Returns an empty signal mask.
    pub const fn empty() -> Self { Self { sig: [0; Self::WORDS] } }
    /// Returns the size in bytes passed to raw Linux signal syscalls.
    #[must_use]
    pub const fn size() -> c_size_t { size_of::<Self>() }


    /// Returns a raw mask from a semantic signal set.
    pub const fn from_signal_set(signals: LinuxSignalSet) -> Self {
        Self { sig: [signals.bits() as usize; Self::WORDS] }
    }
    /// Returns this raw mask as a semantic signal set.
    #[must_use]
    pub const fn to_signal_set(self) -> LinuxSignalSet {
        LinuxSignalSet::from_bits(self.sig[0] as u32)
    }
    /// Returns a copy with `signal` added.
    ///
    /// Ignores unsupported signal numbers.
    pub const fn with_signal(mut self, signal: c_int) -> Self {
        if LinuxSignalSet::is_standard_signal(signal) {
            let bit = signal as usize - 1;
            let word = bit / Self::WORD_BITS;
            let shift = bit % Self::WORD_BITS;
            self.sig[word] |= 1usize << shift;
        }
        self
    }
    /// Returns a copy with `signal` removed.
    ///
    /// Ignores unsupported signal numbers.
    pub const fn without_signal(mut self, signal: c_int) -> Self {
        if LinuxSignalSet::is_standard_signal(signal) {
            let bit = signal as usize - 1;
            let word = bit / Self::WORD_BITS;
            let shift = bit % Self::WORD_BITS;
            self.sig[word] &= !(1usize << shift);
        }
        self
    }
    /// Returns whether this raw mask contains `signal`.
    #[must_use]
    pub const fn has_signal(self, signal: c_int) -> bool {
        if LinuxSignalSet::is_standard_signal(signal) {
            let bit = signal as usize - 1;
            let word = bit / Self::WORD_BITS;
            let shift = bit % Self::WORD_BITS;
            self.sig[word] & (1usize << shift) != 0
        } else {
            false
        }
    }
    /// Adds `signal` to this raw mask.
    ///
    /// # Panics
    /// Panics if `signal` is not supported.
    pub fn insert_signal(&mut self, signal: c_int) {
        assert!(LinuxSignalSet::is_standard_signal(signal));
        *self = self.with_signal(signal);
    }
    /// Removes `signal` from this raw mask.
    ///
    /// # Panics
    /// Panics if `signal` is not a supported standard Linux signal.
    pub fn remove_signal(&mut self, signal: c_int) {
        assert!(LinuxSignalSet::is_standard_signal(signal));
        *self = self.without_signal(signal);
    }
}
