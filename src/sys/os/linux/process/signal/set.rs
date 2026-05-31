// devela::sys::os::linux::process::signal::set
//
//! Defines [`LinuxSignalSet`], [`LinuxSigset`].
//

use crate::{c_int, c_size_t};

crate::enumset! {
    #[doc = crate::_tags!(linux signal member)]
    /// A linux signal
    #[doc = crate::_doc_meta!{
        location("sys/os/linux/process"),
        test_size_of(LinuxSignal = 1|8; niche Option),
    }]
    /// The enum discriminants are zero-based semantic indices, not raw Linux
    /// signal numbers. Use [`as_c_int`][Self::as_c_int] for the raw signal number.
    //
    // For the terminal, the key signals are:
    // - SIGINT    // Ctrl+C
    // - SIGQUIT   // Ctrl+\
    // - SIGTSTP   // Ctrl+Z
    // - SIGTTIN   // background terminal read
    // - SIGTTOU   // background terminal write
    // - SIGWINCH  // terminal resize
    // - SIGHUP    // terminal/session hangup
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum LinuxSignal(
        #[doc = crate::_tags!(linux signal set)]
        /// A compact semantic set of standard Linux signals.
        #[doc = crate::_doc_meta!{
            location("sys/os/linux/process"),
            test_size_of(LinuxSignalSet = 4|32),
        }]
        /// This is not the raw Linux `sigset_t` mask. Signal bits are stored
        /// compactly from zero: `SIGHUP` is bit `0`, `SIGINT` is bit `1`, …
        pub LinuxSignalSet: u32
    ) {
        /// Terminal hangup or controlling process ended.
        SIGHUP // = 0
        /// Keyboard interrupt, usually `Ctrl+C`.
        SIGINT, // = 1
        /// Keyboard quit, usually `Ctrl+\`; often produces a core dump.
        SIGQUIT, // = 2
        /// Illegal CPU instruction.
        SIGILL, // = 3
        /// Trace or breakpoint trap, commonly used by debuggers.
        SIGTRAP, // = 4
        /// Process abort, usually from `abort()`.
        SIGABRT, // = 5
        /// Bus error, usually invalid or misaligned memory access.
        SIGBUS, // = 6
        /// Floating-point exception, such as division by zero in some contexts.
        SIGFPE, // = 7
        /// Uncatchable forced termination.
        SIGKILL, // = 8
        /// User-defined signal 1.
        SIGUSR1, // = 9
        /// Invalid memory reference, usually a segmentation fault.
        SIGSEGV, // = 10
        /// User-defined signal 2.
        SIGUSR2, // = 11
        /// Broken pipe, usually writing to a pipe or socket with no reader.
        SIGPIPE, // = 12
        /// Alarm timer expired, usually from `alarm()`.
        SIGALRM, // = 13
        /// Polite termination request, commonly sent by `kill` or service managers.
        SIGTERM, // = 14
        /// Stack fault signal, obsolete on most modern Linux systems.
        SIGSTKFLT, // = 15
        /// Child process stopped, continued, or exited.
        SIGCHLD, // = 16
        /// Continue a stopped process.
        SIGCONT, // = 17
        /// Uncatchable stop signal.
        SIGSTOP, // = 18
        /// Terminal stop, usually `Ctrl+Z`.
        SIGTSTP, // = 19
        /// Background process tried to read from the terminal.
        SIGTTIN, // = 20
        /// Background process tried to write to the terminal.
        SIGTTOU, // = 21
        /// Urgent condition on a socket.
        SIGURG, // = 22
        /// CPU time limit exceeded.
        SIGXCPU, // = 23
        /// File size limit exceeded.
        SIGXFSZ, // = 24
        /// Virtual interval timer expired.
        SIGVTALRM, // = 25
        /// Profiling timer expired.
        SIGPROF, // = 26
        /// Terminal or window size changed.
        SIGWINCH, // = 27
        /// Asynchronous I/O event is available.
        SIGIO, // = 28
        /// Power failure or power-management event.
        SIGPWR, // = 29
        /// Bad system call, often from seccomp filtering.
        SIGSYS, // = 30
    }
    // Constants
    impl enum {
        /// All supported standard Linux signals, in signal-number order.
        pub const ALL: [Self; Self::VARIANTS] = [
            Self::SIGHUP, Self::SIGINT, Self::SIGQUIT, Self::SIGILL,
            Self::SIGTRAP, Self::SIGABRT, Self::SIGBUS, Self::SIGFPE,
            Self::SIGKILL, Self::SIGUSR1, Self::SIGSEGV, Self::SIGUSR2,
            Self::SIGPIPE, Self::SIGALRM, Self::SIGTERM, Self::SIGSTKFLT,
            Self::SIGCHLD, Self::SIGCONT, Self::SIGSTOP, Self::SIGTSTP,
            Self::SIGTTIN, Self::SIGTTOU, Self::SIGURG, Self::SIGXCPU,
            Self::SIGXFSZ, Self::SIGVTALRM, Self::SIGPROF, Self::SIGWINCH,
            Self::SIGIO, Self::SIGPWR, Self::SIGSYS,
        ];
        /// Lowest supported standard Linux signal number.
        pub const MIN_NUMBER: c_int = 1;
        /// Highest supported standard Linux signal number.
        pub const MAX_NUMBER: c_int = Self::SIGSYS.as_c_int();
        /// Number of slots needed when indexing directly by raw signal number.
        ///
        /// Slot `0` is unused.
        pub(crate) const TABLE_LEN: usize = Self::MAX_NUMBER as usize + 1;
        const __: () = {
            assert!(LinuxSignal::SIGHUP.as_c_int() == 1);
            assert!(LinuxSignal::SIGINT.as_c_int() == 2);
            assert!(LinuxSignal::SIGWINCH.as_c_int() == 28);
            assert!(LinuxSignal::SIGSYS.as_c_int() == 31);

            assert!(LinuxSignal::SIGHUP.as_index() == 0);
            assert!(LinuxSignal::SIGINT.as_index() == 1);
            assert!(LinuxSignal::SIGWINCH.as_index() == 27);
            assert!(LinuxSignal::SIGSYS.as_index() == 30);

            assert!(LinuxSignal::MAX_NUMBER == 31);
            assert!(LinuxSignal::TABLE_LEN == 32);
        };
    }
    // Methods
    impl enum {
        /// Returns the raw Linux signal number.
        #[must_use]
        pub const fn as_c_int(self) -> c_int { self as c_int + 1 }
        /// Returns the zero-based set/mask index for this signal.
        #[must_use]
        pub const fn as_index(self) -> usize { self as usize }
        /// Returns a signal from a raw Linux signal number.
        #[must_use]
        pub const fn from_c_int(signal: c_int) -> Option<Self> {
            match signal {
                1 => Some(Self::SIGHUP),
                2 => Some(Self::SIGINT),
                3 => Some(Self::SIGQUIT),
                4 => Some(Self::SIGILL),
                5 => Some(Self::SIGTRAP),
                6 => Some(Self::SIGABRT),
                7 => Some(Self::SIGBUS),
                8 => Some(Self::SIGFPE),
                9 => Some(Self::SIGKILL),
                10 => Some(Self::SIGUSR1),
                11 => Some(Self::SIGSEGV),
                12 => Some(Self::SIGUSR2),
                13 => Some(Self::SIGPIPE),
                14 => Some(Self::SIGALRM),
                15 => Some(Self::SIGTERM),
                16 => Some(Self::SIGSTKFLT),
                17 => Some(Self::SIGCHLD),
                18 => Some(Self::SIGCONT),
                19 => Some(Self::SIGSTOP),
                20 => Some(Self::SIGTSTP),
                21 => Some(Self::SIGTTIN),
                22 => Some(Self::SIGTTOU),
                23 => Some(Self::SIGURG),
                24 => Some(Self::SIGXCPU),
                25 => Some(Self::SIGXFSZ),
                26 => Some(Self::SIGVTALRM),
                27 => Some(Self::SIGPROF),
                28 => Some(Self::SIGWINCH),
                29 => Some(Self::SIGIO),
                30 => Some(Self::SIGPWR),
                31 => Some(Self::SIGSYS),
                _ => None,
            }
        }
        /// Returns whether this signal can be caught by a user handler.
        #[must_use]
        pub const fn is_catchable(self) -> bool {
            !matches!(self, Self::SIGKILL | Self::SIGSTOP)
        }
        /// Returns whether the raw signal number is supported.
        #[must_use]
        pub const fn is_standard_number(signal: c_int) -> bool {
            Self::from_c_int(signal).is_some()
        }
        /// Returns whether the raw signal number can be caught by a user handler.
        #[must_use]
        pub const fn is_catchable_number(signal: c_int) -> bool {
            match Self::from_c_int(signal) {
                Some(signal) => signal.is_catchable(),
                None => false,
            }
        }
    }
    // Aliases
    impl enum {
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
        // /// Obsolete compatibility alias for [`SIGSYS`][Self::SIGSYS].
        // pub const SIGUNUSED: Self = Self::SIGSYS;
    }

    impl set {
        /// Returns a singleton set containing `signal`.
        #[must_use]
        pub const fn from_signal(signal: LinuxSignal) -> Self {
            signal.to_set()
        }
        /// Returns the only signal in this set, if it contains exactly one.
        #[must_use]
        pub const fn to_signal(self) -> Option<LinuxSignal> {
            let bits = self.bits();
            if bits != 0 && bits.is_power_of_two() {
                LinuxSignal::from_c_int(bits.trailing_zeros() as c_int + 1)
            } else {
                None
            }
        }
        /// Calls `f` for each signal in this set.
        pub fn for_each(self, mut f: impl FnMut(LinuxSignal)) {
            for signal in LinuxSignal::ALL {
                if signal.is_in(self) { f(signal); }
            }
        }
        /// Calls `f` for each catchable signal in this set.
        pub fn for_each_catchable(self, mut f: impl FnMut(LinuxSignal)) {
            self.catchable_only().for_each(|signal| f(signal));
        }

        /// Returns whether all signals in this set are catchable.
        #[must_use]
        pub const fn all_catchable(self) -> bool {
            !self.has(Self::SIGKILL) && !self.has(Self::SIGSTOP)
        }
        /// Returns this set without without signals that cannot be caught.
        #[must_use]
        pub const fn catchable_only(self) -> Self {
            self.without(Self::SIGKILL).without(Self::SIGSTOP)
        }
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

#[doc = crate::_tags!(linux signal set abi)]
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
    pub const WORD_BITS: usize = usize::BITS as usize;
    /// Highest supported standard Linux signal number.
    pub const MAX_SIGNAL: usize = LinuxSignal::MAX_NUMBER as usize;
    /// Number of raw mask words needed for the supported signal range.
    pub const WORDS: usize = Self::MAX_SIGNAL.div_ceil(Self::WORD_BITS);

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
    #[must_use]
    pub const fn with_signal(mut self, signal: LinuxSignal) -> Self {
        let bit = signal.as_index();
        let word = bit / Self::WORD_BITS;
        let shift = bit % Self::WORD_BITS;
        self.sig[word] |= 1usize << shift;
        self
    }
    /// Returns a copy with `signal` removed.
    ///
    /// Ignores unsupported signal numbers.
    pub const fn without_signal(mut self, signal: LinuxSignal) -> Self {
        let bit = signal.as_index();
        let word = bit / Self::WORD_BITS;
        let shift = bit % Self::WORD_BITS;
        self.sig[word] &= !(1usize << shift);
        self
    }
    /// Returns whether this raw mask contains `signal`.
    #[must_use]
    pub const fn has_signal(self, signal: LinuxSignal) -> bool {
        let bit = signal.as_index();
        let word = bit / Self::WORD_BITS;
        let shift = bit % Self::WORD_BITS;
        self.sig[word] & (1usize << shift) != 0
    }
    /// Adds `signal` to this raw mask.
    pub fn insert_signal(&mut self, signal: LinuxSignal) {
        *self = self.with_signal(signal);
    }
    /// Removes `signal` from this raw mask.
    ///
    /// # Panics
    /// Panics if `signal` is not a supported standard Linux signal.
    pub fn remove_signal(&mut self, signal: LinuxSignal) {
        *self = self.without_signal(signal);
    }
    /// Returns a copy with the raw signal number added, if supported.
    #[must_use]
    pub const fn with_signal_number(self, signal: c_int) -> Self {
        match LinuxSignal::from_c_int(signal) {
            Some(signal) => self.with_signal(signal),
            None => self,
        }
    }
}
