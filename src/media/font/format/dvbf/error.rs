// devela/src/media/font/format/dvbf/error.rs
//
//! Defines [`DvbfError`].
//!
//! This is intentionally kept together for the first integration pass.

use crate::{Debug, Display, Error, FmtResult, Formatter, Version};

#[doc = crate::_tags!(font error_composite)]
/// An error encountered while validating or reading DVBF data.
#[doc = crate::_doc_meta!{location("media/font")}]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DvbfError {
    /// The input ends before the complete DVBF header.
    TooShort,
    /// The input does not begin with [`Dvbf::MAGIC`][crate::Dvbf::MAGIC].
    InvalidMagic,
    /// The encoded format version is not supported.
    UnsupportedVersion(Version),
    /// The header size or a reserved header field is invalid.
    InvalidHeader,
    /// The header contains unsupported flag bits.
    UnsupportedFlags(u32),
    /// The declared bitmap bit depth is unsupported.
    UnsupportedBitDepth(u8),
    /// The declared file length differs from the input length.
    InvalidFileSize {
        /// The byte length declared by the header.
        declared: u32,
        /// The actual input byte length.
        actual: usize,
    },
    /// The font declares unusable or internally inconsistent fixed metrics.
    InvalidMetrics,
    /// The encoded strides, offsets or section lengths are invalid.
    InvalidLayout,
    /// A scalar-table entry is not a valid Unicode scalar value.
    InvalidScalar {
        /// The zero-based scalar-table index.
        index: u32,
        /// The invalid encoded value.
        scalar: u32,
    },
    /// The scalar table is not strictly increasing at this index.
    ///
    /// This includes both duplicate and descending scalar values.
    UnsortedScalars {
        /// The first index whose value is not greater than its predecessor.
        index: u32,
    },
    /// The declared default value is not a Unicode scalar.
    InvalidDefaultScalar(u32),
    /// The declared default scalar has no corresponding glyph.
    MissingDefaultGlyph(u32),
}

impl Error for DvbfError {}
impl Display for DvbfError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
        use DvbfError as E;
        match *self {
            E::TooShort => f.write_str("the DVBF data is shorter than its header"),
            E::InvalidMagic => f.write_str("invalid DVBF magic"),
            E::UnsupportedVersion(v) => {
                write!(f, "unsupported DVBF version {}.{}.{}", v.major, v.minor, v.patch)
            }
            E::InvalidHeader => f.write_str("invalid DVBF header"),
            E::UnsupportedFlags(flags) => write!(f, "unsupported DVBF flags: {flags:#010x}"),
            E::UnsupportedBitDepth(d) => write!(f, "unsupported DVBF bitmap bit depth: {d}"),
            E::InvalidFileSize { declared, actual } => {
                write!(f, "invalid DVBF file size: declared {declared}, actual {actual}")
            }
            E::InvalidMetrics => f.write_str("invalid DVBF font metrics"),
            E::InvalidLayout => f.write_str("invalid DVBF section layout"),
            E::InvalidScalar { index, scalar } => {
                write!(f, "invalid DVBF scalar {scalar:#x} at index {index}")
            }
            E::UnsortedScalars { index } => {
                write!(f, "DVBF scalars are not strictly sorted at index {index}")
            }
            E::InvalidDefaultScalar(cp) => write!(f, "DVBF default scalar {cp:#x} is invalid"),
            E::MissingDefaultGlyph(cp) => write!(f, "DVBF default scalar {cp:#x} has no glyph"),
        }
    }
}
