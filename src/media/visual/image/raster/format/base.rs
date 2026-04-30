// devela::media::visual::image::raster::format::base
//
//! Defines [`RasterFormat`], (`RasterAlpha`, `RasterTransfer`).
//
// TOC
// - struct RasterFormat
// - (struct RasterAlpha)
// - (struct RasterTransfer)

use crate::{_impl_init, RasterChannels, RasterSampleFormat};

#[doc = crate::_tags!(image)]
/// Sample layout and color meaning of raster memory.
#[doc = crate::_doc_location!("media/visual/image")]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct RasterFormat {
    /// Channel order, packing, or indexing model.
    pub(super) channels: RasterChannels,
    /// Primitive representation used by each channel or packed element.
    pub(super) sample: RasterSampleFormat,
    /// Transfer curve used to interpret color component values.
    pub(super) transfer: RasterTransfer,
    /// Meaning of the alpha component, if present.
    pub(super) alpha: RasterAlpha,
}
_impl_init![ConstInit: Self::UNKNOWN => RasterFormat];

/// # Queries
impl RasterFormat {
    /// Returns whether this format is unknown or incomplete.
    pub const fn is_unknown(self) -> bool {
        self.channels.is_unknown() || self.sample.is_unknown() || self.transfer.is_unknown()
    }

    /// Returns the number of stored channel fields.
    ///
    /// Padding fields such as `X` in `Xrgb` are counted here.
    pub const fn channel_count(self) -> Option<u8> {
        self.channels.channel_count()
    }

    /// Returns the number of direct color channel fields.
    ///
    /// Alpha and padding fields are not counted. Indexed color returns `None`.
    pub const fn color_channel_count(self) -> Option<u8> {
        self.channels.color_channel_count()
    }

    /// Returns whether this format stores palette indices.
    pub const fn is_indexed(self) -> bool {
        self.channels.is_indexed()
    }

    /// Returns whether this format uses a packed scalar pixel layout.
    pub const fn is_packed(self) -> bool {
        self.channels.is_packed()
    }

    /// Returns whether this format has a padding channel or padding field.
    pub const fn has_padding_channel(self) -> bool {
        self.channels.has_padding_channel()
    }

    /// Returns whether this format has semantic alpha.
    ///
    /// Padding fields such as `X` do not count as alpha.
    pub const fn has_alpha(self) -> bool {
        matches!(self.alpha, RasterAlpha::Straight | RasterAlpha::Premultiplied)
    }

    /// Returns whether the channel model stores an alpha field.
    ///
    /// This can be true even if semantic alpha handling is decided separately.
    pub const fn has_alpha_field(self) -> bool {
        self.channels.has_alpha_field()
    }

    /// Returns whether color channels are independent from alpha.
    pub const fn is_straight_alpha(self) -> bool {
        matches!(self.alpha, RasterAlpha::Straight)
    }

    /// Returns whether color channels are already multiplied by alpha.
    pub const fn is_premultiplied(self) -> bool {
        matches!(self.alpha, RasterAlpha::Premultiplied)
    }

    /// Returns whether this format should be treated as fully opaque.
    pub const fn is_opaque(self) -> bool {
        matches!(self.alpha, RasterAlpha::None | RasterAlpha::Opaque)
    }

    /// Returns whether color values use the sRGB transfer curve.
    pub const fn is_srgb(self) -> bool {
        matches!(self.transfer, RasterTransfer::Srgb)
    }

    /// Returns whether color values are linear-light values.
    pub const fn is_linear(self) -> bool {
        matches!(self.transfer, RasterTransfer::Linear)
    }

    /// Returns whether samples use floating-point values.
    pub const fn is_float(self) -> bool {
        self.sample.is_float()
    }

    /// Returns whether samples use integer values.
    pub const fn is_integer(self) -> bool {
        self.sample.is_integer()
    }

    /// Returns the number of bits used by one unpacked sample.
    ///
    /// For packed formats, prefer [`bits_per_pixel`](Self::bits_per_pixel),
    /// since channels are bit fields inside one scalar element.
    pub const fn bits_per_sample(self) -> Option<u16> {
        self.sample.bits()
    }

    /// Returns the number of meaningful bits per pixel.
    ///
    /// Padding bits are excluded. Alpha bits are included when present.
    pub const fn depth_bits(self) -> Option<u16> {
        self.channels.depth_bits(self.sample)
    }

    /// Returns the number of stored bits per pixel.
    ///
    /// Padding bits are included.
    pub const fn bits_per_pixel(self) -> Option<u16> {
        self.channels.bits_per_pixel(self.sample)
    }

    /// Returns the number of stored bytes per pixel, if byte-aligned.
    pub const fn bytes_per_pixel(self) -> Option<u16> {
        match self.bits_per_pixel() {
            Some(bits) if bits % 8 == 0 => Some(bits / 8),
            _ => None,
        }
    }
}

/// Alpha-channel interpretation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub(crate) enum RasterAlpha {
    /// No alpha component is present.
    #[default]
    None,
    /// Alpha is absent or ignored, and pixels are treated as fully opaque.
    Opaque,
    /// Color channels are independent from alpha.
    Straight,
    /// Color channels are already multiplied by alpha.
    Premultiplied,
}
_impl_init![ConstInit: Self::None => RasterAlpha];

/// Transfer curve used to interpret color sample values.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub(crate) enum RasterTransfer {
    /// Unknown or unspecified transfer curve.
    #[default]
    Unknown,
    /// Linear-light sample values.
    Linear,
    /// sRGB transfer-encoded sample values.
    Srgb,
}
_impl_init![ConstInit: Self::Unknown => RasterTransfer];
impl RasterTransfer {
    /// Returns whether this transfer curve is unknown.
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }
}
