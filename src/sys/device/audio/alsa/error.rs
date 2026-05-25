// devela::sys::device::audio::alsa::error
//
//! Defines [`AlsaError`].
//

use crate::{c_int, impl_trait};

#[doc = crate::_tags!(audio linux error)]
/// ALSA operation error.
#[doc = crate::_doc_location!("sys/device/audio")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AlsaError {
    /// Negative ALSA/libc-style error code.
    pub code: c_int,
}
#[rustfmt::skip]
impl AlsaError {
    /// Invalid argument.
    ///
    /// Matches the usual negative `EINVAL` value used by ALSA/libc-style APIs.
    pub const INVALID_ARGUMENT: c_int = -22;

    /// Creates a new ALSA error from a negative error code.
    pub const fn new(code: c_int) -> Self {
        Self { code }
    }

    /// Creates an invalid-argument error.
    pub const fn invalid_argument() -> Self {
        Self::new(Self::INVALID_ARGUMENT)
    }

    #[inline(always)]
    pub(crate) const fn result(code: c_int) -> Result<(), AlsaError> {
        if code < 0 { Err(AlsaError::new(code)) } else { Ok(()) }
    }
}
impl_trait![fmt::Display+Error for AlsaError |self, f| write![f, "Alsa error: {}", self.code]];
