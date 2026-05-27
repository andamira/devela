// devela::media::audio::format::raw::error
//
//! Defines [`PcmRawError`].
//

#[cfg(feature = "std")]
use crate::IoError;
#[cfg(feature = "std")]
use crate::IoErrorKind;

crate::test_size_of![PcmRawError = 1]; // 8 bits

#[doc = crate::_tags!(audio error)]
/// Raw PCM encoding and decoding error.
#[doc = crate::_doc_meta!{location("media/audio")}]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PcmRawError {
    /// The PCM stream specification is incomplete or invalid.
    InvalidSpec,

    /// The byte length is not a multiple of the PCM frame size.
    InvalidDataLength,

    /// The PCM sample format does not match the requested operation.
    MismatchedSampleFormat,

    /// A typed sample value is outside the target PCM range.
    SampleOutOfRange,

    /// The destination buffer is too small.
    NotEnoughSpace,

    /// Reading or writing a file failed.
    #[cfg(feature = "std")]
    Io(IoErrorKind),
}

crate::impl_trait![fmt::Display+Error for PcmRawError |self, f| match self {
    Self::InvalidSpec => f.write_str("invalid raw PCM stream specification"),
    Self::InvalidDataLength => {
        f.write_str("raw PCM data length is not a multiple of frame size")
    }
    Self::MismatchedSampleFormat => {
        f.write_str("PCM sample format does not match the requested operation")
    }
    Self::SampleOutOfRange => f.write_str("PCM sample value is outside the target range"),
    Self::NotEnoughSpace => f.write_str("not enough space to write raw PCM data"),
    #[cfg(feature = "std")]
    Self::Io(err) => write!(f, "raw PCM file operation failed: {err}"),
}];

#[cfg(feature = "std")]
impl From<IoError> for PcmRawError {
    fn from(err: IoError) -> Self {
        Self::Io(err.kind())
    }
}
