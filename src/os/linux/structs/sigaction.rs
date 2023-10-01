/// Represents the [`sigaction`] structure from libc.
/// Examine and change a signal action.
///
/// [`sigaction`]: https://man7.org/linux/man-pages/man2/sigaction.2.html
#[derive(Debug)]
#[repr(C)]
pub struct LinuxSigaction {
    /// A pointer to a signal handling function.
    ///
    /// This function receives the signal number as its only argument.
    pub sa_handler: extern "C" fn(i32),

    /// A set of flags which modify the behavior of the signal.
    pub sa_flags: usize,

    /// A legacy field that is not used on modern Linux systems, but must be
    /// filled in for compatibility
    pub sa_restorer: Option<extern "C" fn()>,

    /// A mask of signals that should be blocked.
    pub sa_mask: LinuxSigset,
}

impl LinuxSigaction {
    /// Retuns a new `LinuxSigation`.
    #[inline]
    #[must_use]
    pub fn new(handler: extern "C" fn(i32), flags: usize, mask: LinuxSigset) -> Self {
        Self {
            sa_handler: handler,
            sa_flags: flags,
            sa_restorer: None,
            sa_mask: mask,
        }
    }
}

/// [`sa_handler`][Self::sa_handler] field constants.
impl LinuxSigaction {
    /// The default signal handling.
    pub const SIG_DFL: isize = 0;

    /// Ignore this signal.
    pub const SIG_IGN: isize = 1;

    // /// Error return from signal.
    // pub const SIG_ERR: isize = -1;
}

/// A set of signals.
#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct LinuxSigset {
    /// An array of signals.
    ///
    /// Its lenght is calculated from the number of signals divided by the bits of a usize.
    pub sig: [usize; Self::LEN],
}
impl LinuxSigset {
    const BITS_PER_USIZE: usize = usize::BITS as usize;
    // The hardcoded number of system signals defined in `LINUX_SIGNAL`.
    const NSIG: usize = 36;
    // The size of the array is the number of signals divided by the bits of an usize.
    const LEN: usize = { (Self::NSIG + Self::BITS_PER_USIZE - 1) / Self::BITS_PER_USIZE };

    /// Returns the size in bytes of `LinuxSigset`.
    #[inline]
    #[must_use]
    pub const fn size() -> usize {
        core::mem::size_of::<Self>()
    }
}
impl LinuxSigset {
    /// Sets the bit corresponding to a `signal` in the `sig` array.
    ///
    /// # Arguments
    /// * `signum` - The number of the signal. This should be between 1 and `NSIG`.
    ///
    /// # Panics
    /// Panics if `signum` < 1 or > 36.
    pub fn set_signal(&mut self, signal: i32) {
        let signal = signal as usize;
        assert![(1..=Self::NSIG).contains(&signal)];

        // Subtract 1 from the signal number because signal numbers start from 1
        // but array indices start from 0
        let signal_index = (signal - 1) / Self::BITS_PER_USIZE;
        let bit_position = (signal - 1) % Self::BITS_PER_USIZE;
        self.sig[signal_index] |= 1 << bit_position;
    }
}
