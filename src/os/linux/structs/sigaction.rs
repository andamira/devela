/// Represents the [`sigaction`] structure from libc.
/// Examine and change a signal action.
///
/// [`sigaction`]: https://man7.org/linux/man-pages/man2/sigaction.2.html
#[derive(Debug)]
#[repr(C)]
pub struct SysSigaction {
    /// Specifies the action to be associated with signum.
    ///
    /// - [`SIG_DFL`][Self::SIG_DFL] for the default action.
    /// - [`SIG_IGN`][Self::SIG_IGN] to ignore this signal.
    /// - A pointer to a signal handling function.
    ///   This function receives the signal number as its only argument.
    // pub sa_sigaction: unsafe exern "C" fn(i32),
    pub sa_handler: unsafe extern "C" fn(i32),

    /// Specifies a mask of signals which should be blocked.
    pub sa_flags: u64,

    // a legacy field that is not used on modern Linux systems, but must be
    // filled in for compatibility
    pub sa_restorer: unsafe extern "C" fn(),

    pub sa_mask: u64,
}

/// [`sa_handler`][Self::sa_handler] field constants.
impl SysSigaction {
    /// The default signal handling.
    pub const SIG_DFL: isize = 0;

    /// Ignore this signal.
    pub const SIG_IGN: isize = 1;

    // /// Error return from signal.
    // pub const SIG_ERR: isize = -1;
}
