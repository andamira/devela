// devela::media::visual::image::raster::format::channels
//
//! Defines [`RasterChannels`].
//

use crate::{_impl_init, RasterPackedChannels, RasterSampleFormat};

/// Channel order, packing, or indexing model of raster elements.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[allow(dead_code, reason = "Not exposed: Xrgb, Xbgr")]
pub(crate) enum RasterChannels {
    /// Unknown or unspecified channel model.
    #[default]
    Unknown,

    /* gray */
    /// Single luminance-like channel.
    Gray,
    /// Gray channel followed by alpha.
    GrayAlpha,

    /* rgb */
    /// Red, green, blue.
    Rgb,
    /// Red, green, blue, alpha.
    Rgba,
    /// Alpha, red, green, blue.
    Argb,
    /// Padding, red, green, blue.
    Xrgb,
    /// Red, green, blue, padding.
    Rgbx,

    /* bgr */
    /// Blue, green, red.
    Bgr,
    /// Blue, green, red, alpha.
    Bgra,
    /// Alpha, blue, green, red.
    Abgr,
    /// Padding, blue, green, red.
    Xbgr,
    /// Blue, green, red, padding.
    Bgrx,

    /* indexed */
    /// Palette index instead of direct color channels.
    Indexed,

    /// Channel fields packed inside one scalar raster element.
    ///
    /// The packed format describes the bit layout of that scalar element,
    /// not the byte order of a borrowed byte slice.
    Packed(RasterPackedChannels),
}
_impl_init![ConstInit: Self::Unknown => RasterChannels];

impl RasterChannels {
    /// Returns whether this channel model is unknown or incomplete.
    pub const fn is_unknown(self) -> bool {
        match self {
            Self::Unknown => true,
            Self::Packed(packed) => packed.is_unknown(),
            _ => false,
        }
    }

    /// Returns the number of stored channel fields.
    ///
    /// Padding fields such as `X` in `Xrgb` are counted here.
    pub const fn channel_count(self) -> Option<u8> {
        match self {
            Self::Unknown => None,

            Self::Gray | Self::Indexed => Some(1),
            Self::GrayAlpha => Some(2),

            Self::Rgb | Self::Bgr => Some(3),

            Self::Rgba
            | Self::Bgra
            | Self::Argb
            | Self::Abgr
            | Self::Xrgb
            | Self::Xbgr
            | Self::Rgbx
            | Self::Bgrx => Some(4),

            Self::Packed(packed) => packed.channel_count(),
        }
    }

    /// Returns the number of direct color channel fields.
    ///
    /// Alpha and padding fields are not counted. Indexed formats return
    /// `None`, since the direct element is a palette index, not a color field.
    pub const fn color_channel_count(self) -> Option<u8> {
        match self {
            Self::Unknown | Self::Indexed => None,

            Self::Gray | Self::GrayAlpha => Some(1),

            Self::Rgb
            | Self::Bgr
            | Self::Rgba
            | Self::Bgra
            | Self::Argb
            | Self::Abgr
            | Self::Xrgb
            | Self::Xbgr
            | Self::Rgbx
            | Self::Bgrx => Some(3),

            Self::Packed(packed) => packed.color_channel_count(),
        }
    }

    /// Returns whether the channel model has an alpha field.
    ///
    /// This only describes the stored channel topology. Semantic alpha
    /// interpretation is still controlled by `RasterAlpha`.
    pub const fn has_alpha_field(self) -> bool {
        match self {
            Self::GrayAlpha | Self::Rgba | Self::Bgra | Self::Argb | Self::Abgr => true,

            Self::Packed(packed) => packed.has_alpha(),

            Self::Unknown
            | Self::Gray
            | Self::Rgb
            | Self::Bgr
            | Self::Xrgb
            | Self::Xbgr
            | Self::Rgbx
            | Self::Bgrx
            | Self::Indexed => false,
        }
    }

    /// Returns whether the channel model has a padding field.
    pub const fn has_padding_channel(self) -> bool {
        match self {
            Self::Xrgb | Self::Xbgr | Self::Rgbx | Self::Bgrx => true,
            Self::Packed(packed) => packed.has_padding_channel(),

            Self::Unknown
            | Self::Gray
            | Self::GrayAlpha
            | Self::Rgb
            | Self::Bgr
            | Self::Rgba
            | Self::Bgra
            | Self::Argb
            | Self::Abgr
            | Self::Indexed => false,
        }
    }

    /// Returns whether this channel model stores palette indices.
    pub const fn is_indexed(self) -> bool {
        matches!(self, Self::Indexed)
    }

    /// Returns whether this channel model uses packed scalar pixels.
    pub const fn is_packed(self) -> bool {
        matches!(self, Self::Packed(_))
    }

    /// Returns the number of meaningful bits per pixel for this channel model.
    ///
    /// Padding bits are excluded. Alpha bits are included when present.
    pub const fn depth_bits(self, sample: RasterSampleFormat) -> Option<u16> {
        match self {
            Self::Unknown => None,
            Self::Packed(packed) => packed.depth_bits(),
            Self::Xrgb | Self::Xbgr | Self::Rgbx | Self::Bgrx => match sample.bits() {
                Some(bits) => Some(3 * bits),
                None => None,
            },
            _ => match (self.channel_count(), sample.bits()) {
                (Some(channels), Some(bits)) => Some(channels as u16 * bits),
                _ => None,
            },
        }
    }

    /// Returns the number of stored bits per pixel for this channel model.
    ///
    /// Padding bits are included.
    pub const fn bits_per_pixel(self, sample: RasterSampleFormat) -> Option<u16> {
        match self {
            Self::Unknown => None,
            Self::Packed(packed) => packed.bits_per_pixel(),

            _ => match (self.channel_count(), sample.bits()) {
                (Some(channels), Some(bits)) => Some(channels as u16 * bits),
                _ => None,
            },
        }
    }
}
