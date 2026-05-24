// devela::data::codec::pack::wrap::riff::error
//
//! Defines [`RiffError`].
//

#[cfg(feature = "audio")]
use crate::PcmWavError;

#[doc = crate::_tags!(data codec error)]
/// RIFF parsing error.
#[doc = crate::_doc_location!("data/codec/pack")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RiffError {
    /// The chunk header is incomplete.
    TruncatedHeader,
    /// The declared chunk data is incomplete.
    TruncatedData,
    /// The declared pad byte is incomplete.
    TruncatedPad,
    /// A size computation overflowed.
    Overflow,
    /// The root chunk is not `RIFF`.
    NotRiff,
    /// The destination buffer is too small.
    NotEnoughSpace,
    /// The root chunk is `RIFX`, which is not supported yet.
    UnsupportedRifx,
    /// A container chunk is missing its form/list type.
    MissingContainerType,
}
crate::impl_trait![fmt::Display+Error for RiffError |self, f| match self {
    Self::TruncatedHeader => f.write_str("incomplete RIFF chunk header"),
    Self::TruncatedData => f.write_str("incomplete RIFF chunk data"),
    Self::TruncatedPad => f.write_str("missing RIFF chunk pad byte"),
    Self::Overflow => f.write_str("RIFF chunk size computation overflowed"),
    Self::NotEnoughSpace => f.write_str("not enough space to write RIFF data"),
    Self::NotRiff => f.write_str("root chunk is not `RIFF`"),
    Self::UnsupportedRifx => f.write_str("`RIFX` big-endian RIFF is not supported"),
    Self::MissingContainerType => f.write_str("RIFF container chunk is missing its form/list type"),
}];
impl RiffError {
    /// Converts the error into a WAVE error.
    #[cfg(feature = "audio")]
    pub const fn to_wav(self) -> PcmWavError {
        PcmWavError::Riff(self)
    }
}
