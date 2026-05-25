// devela::media::audio::format::wav::error
//
//! Defines [`PcmWavError`].
//

use crate::{PcmRawError, RiffError};
#[cfg(feature = "std")]
use crate::{IoError, IoErrorKind};

crate::test_size_of![PcmWavError = 4]; // 32 bits

#[doc = crate::_tags!(audio error)]
/// WAVE encoding and decoding error.
#[doc = crate::_doc_location!("media/audio")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PcmWavError {
    /// Raw PCM byte/sample conversion failed.
    Raw(PcmRawError),

    /// RIFF-level parsing failed.
    Riff(RiffError),

    /// The RIFF form type is not `WAVE`.
    NotWave,

    /// The required `fmt` chunk was not found.
    MissingFmt,

    /// The required `data` chunk was not found.
    MissingData,

    /// The `fmt` chunk is shorter than the base 16-byte PCM format.
    TruncatedFmt,

    /// The WAVE format code is not supported by this parser.
    UnsupportedFormat(u16),

    /// The channel count is zero or cannot map to current PCM metadata.
    UnsupportedChannelCount(u16),

    /// The bits-per-sample field is not supported by current PCM metadata.
    UnsupportedBitsPerSample(u16),

    /// `block_align` does not match the parsed format.
    InvalidBlockAlign,

    /// `byte_rate` does not match the parsed format.
    InvalidByteRate,

    /// The `data` chunk length is not a multiple of `block_align`.
    InvalidDataLength,

    /// A size computation overflowed.
    Overflow,

    /// The destination buffer is too small.
    NotEnoughSpace,

    /// The encoded WAVE file would exceed RIFF's 32-bit size fields.
    SizeOutOfRange,

    /// The WAVE format metadata cannot be represented by the current writer.
    UnsupportedEncoding,

    /// The WAVE extensible `fmt` extension is malformed.
    InvalidExtensibleFormat,

    /// The WAVE extensible subformat GUID is not supported.
    UnsupportedSubformat,

    /// Reading the file failed.
    #[cfg(feature = "std")]
    Io(IoErrorKind),
}
crate::impl_trait![fmt::Display+Error for PcmWavError |self, f| match self {
    Self::Raw(err) => write!(f, "raw PCM operation failed: {err}"),
    Self::Riff(err) => write!(f, "RIFF parsing failed: {err}"),
    Self::NotWave => f.write_str("RIFF form type is not `WAVE`"),
    Self::MissingFmt => f.write_str("missing required `fmt` chunk"),
    Self::MissingData => f.write_str("missing required `data` chunk"),
    Self::TruncatedFmt => f.write_str("`fmt` chunk is shorter than the 16-byte PCM base format"),
    Self::UnsupportedFormat(code) => write!(f, "unsupported WAVE format code: 0x{code:04X}"),
    Self::UnsupportedChannelCount(chans) => write!(f, "unsupported WAVE channel count: {chans}"),
    Self::UnsupportedBitsPerSample(bits) => write!(f, "unsupported WAVE bits per sample: {bits}"),
    Self::InvalidBlockAlign => f.write_str("invalid WAVE block alignment"),
    Self::InvalidByteRate => f.write_str("invalid WAVE byte rate"),
    Self::InvalidDataLength => f.write_str("WAVE data length is not a multiple of block alignment"),
    Self::Overflow => f.write_str("WAVE chunk size computation overflowed"),
    Self::NotEnoughSpace => f.write_str("not enough space to write WAVE data"),
    Self::SizeOutOfRange => f.write_str("encoded WAVE data exceeds RIFF size limits"),
    Self::UnsupportedEncoding => f.write_str("unsupported WAVE encoding"),
    Self::InvalidExtensibleFormat => f.write_str("invalid WAVE extensible format"),
    Self::UnsupportedSubformat => f.write_str("unsupported WAVE extensible subformat"),
    #[cfg(feature = "std")]
    Self::Io(err) => write!(f, "WAVE file read failed.: {err}"),
}];
impl From<PcmRawError> for PcmWavError {
    fn from(err: PcmRawError) -> Self {
        Self::Raw(err)
    }
}
impl From<RiffError> for PcmWavError {
    fn from(err: RiffError) -> Self {
        Self::Riff(err)
    }
}
#[cfg(feature = "std")]
impl From<IoError> for PcmWavError {
    fn from(err: IoError) -> Self {
        Self::Io(err.kind())
    }
}
