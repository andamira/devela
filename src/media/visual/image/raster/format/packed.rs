// devela::media::visual::image::raster::format::packed
//
//! Defines [`RasterPackedChannels`].
//

use crate::_impl_init;

/// Packed channel bit layout inside one scalar raster element.
///
/// Variant names describe channel order from the most-significant field to the
/// least-significant field of the scalar value. For example, `Rgb565` means red
/// occupies the high 5 bits, green the middle 6 bits, and blue the low 5 bits.
///
/// This is not the same as native memory byte order. A packed `u16` or `u32`
/// stored as bytes will appear according to the target endianness. On
/// little-endian machines, the least-significant byte is stored first.
///
/// Use byte-interleaved formats such as `Rgba` + `U8` when each channel is
/// stored as its own sequential byte sample. Use `Packed(_)` when one scalar
/// element contains multiple channel fields.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[allow(
    dead_code,
    reason = "Only exposed: Rgb332, Rgb565, Xrgb8888, Argb8888 Bgr332, Bgr565, Xbgr8888, Abgr8888"
)]
pub(crate) enum RasterPackedChannels {
    /// Unknown or unspecified packed layout.
    #[default]
    Unknown,

    /* rgb */
    /// Packed 8-bit RGB: 3 red bits, 3 green bits, 2 blue bits.
    Rgb332,
    /// Packed 16-bit RGB: 5 red bits, 6 green bits, 5 blue bits.
    Rgb565,
    /// Packed 15-bit RGB: 5 red bits, 5 green bits, 5 blue bits.
    Rgb555,
    /// Packed 16-bit RGBA: 5 red bits, 5 green bits, 5 blue bits, 1 alpha bit.
    Rgba5551,
    /// Packed 16-bit ARGB: 1 alpha bit, 5 red bits, 5 green bits, 5 blue bits.
    Argb1555,
    /// Packed 16-bit RGBA: 4 bits per channel.
    Rgba4444,
    /// Packed 32-bit XRGB: 8 padding bits, then 8 bits per color channel.
    Xrgb8888,
    /// Packed 32-bit RGBX: 8 bits per color channel, then 8 padding bits.
    Rgbx8888,
    /// Packed 32-bit ARGB: 8 alpha bits, then 8 bits per color channel.
    Argb8888,
    /// Packed 32-bit RGBA: 8 bits per channel.
    Rgba8888,

    /* bgr */
    /// Packed 8-bit BGR: 3 blue bits, 3 green bits, 2 red bits.
    Bgr332,
    /// Packed 16-bit BGR: 5 blue bits, 6 green bits, 5 red bits.
    Bgr565,
    /// Packed 15-bit BGR: 5 blue bits, 5 green bits, 5 red bits.
    Bgr555,
    /// Packed 16-bit BGRA: 5 blue bits, 5 green bits, 5 red bits, 1 alpha bit.
    Bgra5551,
    /// Packed 16-bit ABGR: 1 alpha bit, 5 blue bits, 5 green bits, 5 red bits.
    Abgr1555,
    /// Packed 16-bit BGRA: 4 bits per channel.
    Bgra4444,
    /// Packed 32-bit XBGR: 8 padding bits, then 8 bits per color channel.
    Xbgr8888,
    /// Packed 32-bit BGRX: 8 bits per color channel, then 8 padding bits.
    Bgrx8888,
    /// Packed 32-bit ABGR: 8 alpha bits, then 8 bits per color channel.
    Abgr8888,
    /// Packed 32-bit BGRA: 8 bits per channel.
    Bgra8888,
}
_impl_init![ConstInit: Self::Unknown => RasterPackedChannels];

impl RasterPackedChannels {
    /// Returns whether this packed layout is unknown.
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }
    /// Returns the number of stored channel fields.
    ///
    /// Padding fields such as `X` in `Xrgb8888` are counted here.
    pub const fn channel_count(self) -> Option<u8> {
        match self {
            Self::Unknown => None,
            Self::Rgb332
            | Self::Rgb565
            | Self::Rgb555
            | Self::Bgr332
            | Self::Bgr565
            | Self::Bgr555 => Some(3),
            _ => Some(4),
        }
    }
    /// Returns the number of direct color channel fields.
    ///
    /// Alpha and padding fields are not counted.
    pub const fn color_channel_count(self) -> Option<u8> {
        match self {
            Self::Unknown => None,
            _ => Some(3),
        }
    }
    /// Returns whether the packed layout contains an alpha field.
    pub const fn has_alpha(self) -> bool {
        matches!(
            self,
            Self::Rgba5551
                | Self::Argb1555
                | Self::Rgba4444
                | Self::Argb8888
                | Self::Rgba8888
                | Self::Bgra5551
                | Self::Abgr1555
                | Self::Bgra4444
                | Self::Abgr8888
                | Self::Bgra8888
        )
    }
    /// Returns whether the packed layout contains a padding field.
    pub const fn has_padding_channel(self) -> bool {
        matches!(self, Self::Xrgb8888 | Self::Rgbx8888 | Self::Xbgr8888 | Self::Bgrx8888)
    }
    /// Returns the number of meaningful bits in the packed pixel.
    ///
    /// Padding bits are excluded. Alpha bits are included when present.
    pub const fn depth_bits(self) -> Option<u16> {
        match self {
            Self::Unknown => None,
            Self::Rgb332 | Self::Bgr332 => Some(8),
            Self::Rgb555 | Self::Bgr555 => Some(15),
            Self::Rgb565
            | Self::Rgba5551
            | Self::Argb1555
            | Self::Rgba4444
            | Self::Bgr565
            | Self::Bgra5551
            | Self::Abgr1555
            | Self::Bgra4444 => Some(16),
            Self::Xrgb8888 | Self::Rgbx8888 | Self::Xbgr8888 | Self::Bgrx8888 => Some(24),
            Self::Argb8888 | Self::Rgba8888 | Self::Abgr8888 | Self::Bgra8888 => Some(32),
        }
    }
    /// Returns the number of stored bits per packed pixel.
    ///
    /// Padding bits are included.
    /// For example, `Xrgb8888` has depth 24 but occupies 32 stored bits.
    pub const fn bits_per_pixel(self) -> Option<u16> {
        match self {
            Self::Unknown => None,
            Self::Rgb332 | Self::Bgr332 => Some(8),
            // RGB555 is normally carried in a 16-bit storage element.
            Self::Rgb555 | Self::Bgr555 => Some(16),
            Self::Rgb565
            | Self::Rgba5551
            | Self::Argb1555
            | Self::Rgba4444
            | Self::Bgr565
            | Self::Bgra5551
            | Self::Abgr1555
            | Self::Bgra4444 => Some(16),

            Self::Xrgb8888
            | Self::Rgbx8888
            | Self::Argb8888
            | Self::Rgba8888
            | Self::Xbgr8888
            | Self::Bgrx8888
            | Self::Abgr8888
            | Self::Bgra8888 => Some(32),
        }
    }
    /// Returns the stored bytes per packed pixel.
    #[allow(dead_code, reason = "WIP")]
    pub const fn stored_bytes_per_pixel(self) -> Option<u16> {
        match self.bits_per_pixel() {
            Some(bits) if bits % 8 == 0 => Some(bits / 8),
            _ => None,
        }
    }
}
