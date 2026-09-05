// devela/src/sys/os/linux/rand.rs
//
//! Defines [`LinuxRandomMode`].
//

use crate::c_uint;

#[doc = crate::_tags!(rand linux)]
/// Linux `getrandom` randomness mode.
#[doc = crate::_doc_meta!{
    location("sys/os/linux", enum LinuxRandomMode),
    test_size_of(LinuxRandomMode = 1|8; niche Option),
}]
/// This selects the Linux `getrandom` policy
/// used by the configurable `Linux::random_*_with` methods.
///
/// The default mode is [`SecureNonblock`][Self::SecureNonblock], exposed as
/// [`Linux::RANDOM_MODE`]. It requests cryptographic random bytes without
/// blocking; if the kernel randomness source is not ready, the operation
/// returns an error instead.
///
///  [`Linux::RANDOM_MODE`]: crate::Linux::RANDOM_MODE
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LinuxRandomMode {
    /// Cryptographic randomness, allowing the syscall to block.
    ///
    /// This uses no `getrandom` flags.
    Secure,

    #[doc = crate::_tags!(init)]
    /// Cryptographic randomness without blocking.
    ///
    /// This uses `GRND_NONBLOCK`. If the kernel randomness source is not ready,
    /// the syscall may fail with `EAGAIN`.
    #[default]
    SecureNonblock,

    /// Non-cryptographic random bytes.
    ///
    /// This uses `GRND_INSECURE` and is suitable only
    /// when cryptographic quality is not required.
    Insecure,
}
impl LinuxRandomMode {
    // from `sys/random.h`
    const GRND_NONBLOCK: c_uint = 0x0001;
    const GRND_INSECURE: c_uint = 0x0004;

    /* queries */

    /// Returns the Linux `getrandom` flags for this mode.
    pub const fn flags(self) -> c_uint {
        match self {
            Self::Secure => 0,
            Self::SecureNonblock => Self::GRND_NONBLOCK,
            Self::Insecure => Self::GRND_INSECURE,
        }
    }

    /// Returns whether this mode is intended for cryptographic randomness.
    pub const fn is_cryptographic(self) -> bool {
        matches!(self, Self::Secure | Self::SecureNonblock)
    }
    /// Returns whether this mode is known to be weak.
    pub const fn is_weak(self) -> bool {
        matches!(self, Self::Insecure)
    }
    /// Returns whether this mode may block while waiting for randomness.
    pub const fn may_block(self) -> bool {
        matches!(self, Self::Secure)
    }
}
