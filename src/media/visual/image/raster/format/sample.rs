// devela::media::visual::image::raster::format::sample
//
//! Defines [`RasterSampleFormat`].
//

use crate::_impl_init;

/// Primitive representation of raster samples.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[allow(dead_code, reason = "Not exposed: U16, U32, F64")]
pub(crate) enum RasterSampleFormat {
    /// Unknown or unspecified sample representation.
    #[default]
    Unknown,
    /// Unsigned 8-bit integer sample.
    U8,
    /// Unsigned 16-bit integer sample.
    U16,
    /// Unsigned 32-bit integer sample.
    U32,
    /// 32-bit floating-point sample.
    F32,
    /// 64-bit floating-point sample.
    F64,
}
_impl_init![ConstInit: Self::Unknown => RasterSampleFormat];

impl RasterSampleFormat {
    /// Returns the number of bits used by one sample.
    pub const fn bits(self) -> Option<u16> {
        match self {
            Self::Unknown => None,
            Self::U8 => Some(8),
            Self::U16 => Some(16),
            Self::U32 => Some(32),
            Self::F32 => Some(32),
            Self::F64 => Some(64),
        }
    }
    /// Returns whether this sample format is unknown.
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }
    /// Returns whether this sample format uses floating-point values.
    pub const fn is_float(self) -> bool {
        matches!(self, Self::F32 | Self::F64)
    }
    /// Returns whether this sample format uses integer values.
    pub const fn is_integer(self) -> bool {
        matches!(self, Self::U8 | Self::U16 | Self::U32)
    }
}
