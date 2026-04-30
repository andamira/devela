// devela::media::visual::image::raster::format::impl_constants

use crate::{
    RasterAlpha as Alpha, RasterChannels as Channels, RasterFormat, RasterPackedFormat as Packed,
    RasterSampleFormat as Sample, RasterTransfer as Transfer,
};

/// # Constructors
impl RasterFormat {
    pub(super) const fn _new(
        channels: Channels,
        sample: Sample,
        transfer: Transfer,
        alpha: Alpha,
    ) -> Self {
        Self { channels, sample, transfer, alpha }
    }
    pub(super) const fn _new_packed(
        channels: Packed,
        sample: Sample,
        transfer: Transfer,
        alpha: Alpha,
    ) -> Self {
        Self::_new(Channels::Packed(channels), sample, transfer, alpha)
    }

    /// Unknown or unspecified raster format.
    pub const UNKNOWN: Self =
        Self::_new(Channels::Unknown, Sample::Unknown, Transfer::Unknown, Alpha::None);

    /* gray */

    /// 8-bit grayscale in sRGB space.
    pub const GRAY8: Self = Self::_new(Channels::Gray, Sample::U8, Transfer::Srgb, Alpha::None);
    /// 8-bit grayscale with straight alpha in sRGB space.
    pub const GRAYA8: Self =
        Self::_new(Channels::GrayAlpha, Sample::U8, Transfer::Srgb, Alpha::Straight);

    /* byte-interleaved rgb */

    /// 24-bit RGB in sRGB space.
    pub const RGB8: Self = Self::_new(Channels::Rgb, Sample::U8, Transfer::Srgb, Alpha::None);
    /// 32-bit RGBA with straight alpha in sRGB space. (byte-interleaved)
    pub const RGBA8: Self = Self::_new(Channels::Rgba, Sample::U8, Transfer::Srgb, Alpha::Straight);
    /// 32-bit RGBA with premultiplied alpha in sRGB space.
    pub const RGBA_PRE8: Self =
        Self::_new(Channels::Rgba, Sample::U8, Transfer::Srgb, Alpha::Premultiplied);
    /// 32-bit RGBX storage with 24 meaningful color bits in sRGB space.
    pub const RGBX8: Self = Self::_new(Channels::Rgbx, Sample::U8, Transfer::Srgb, Alpha::Opaque);
    /// Byte-interleaved 32-bit ARGB with straight alpha in sRGB space.
    pub const ARGB8: Self = Self::_new(Channels::Argb, Sample::U8, Transfer::Srgb, Alpha::Straight);

    /* byte-interleaved bgr */

    /// 24-bit BGR in sRGB space.
    pub const BGR8: Self = Self::_new(Channels::Bgr, Sample::U8, Transfer::Srgb, Alpha::None);
    /// 32-bit BGRA with straight alpha in sRGB space.
    pub const BGRA8: Self = Self::_new(Channels::Bgra, Sample::U8, Transfer::Srgb, Alpha::Straight);
    /// 32-bit BGRA with premultiplied alpha in sRGB space.
    pub const BGRA_PRE8: Self =
        Self::_new(Channels::Bgra, Sample::U8, Transfer::Srgb, Alpha::Premultiplied);
    /// 32-bit BGRX storage with 24 meaningful color bits in sRGB space.
    pub const BGRX8: Self = Self::_new(Channels::Bgrx, Sample::U8, Transfer::Srgb, Alpha::Opaque);
    /// Byte-interleaved 32-bit ABGR with straight alpha in sRGB space.
    pub const ABGR8: Self = Self::_new(Channels::Abgr, Sample::U8, Transfer::Srgb, Alpha::Straight);

    /* packed rgb */

    /// Packed 8-bit RGB332 in sRGB space.
    pub const RGB332: Self =
        Self::_new_packed(Packed::Rgb332, Sample::U8, Transfer::Srgb, Alpha::None);
    /// Packed 16-bit RGB565 in sRGB space.
    pub const RGB565: Self =
        Self::_new_packed(Packed::Rgb565, Sample::U16, Transfer::Srgb, Alpha::None);
    /// Packed 32-bit RGBX8888 in sRGB space.
    pub const RGBX8888: Self =
        Self::_new_packed(Packed::Rgbx8888, Sample::U32, Transfer::Srgb, Alpha::Opaque);
    /// Packed 32-bit RGBA8888 with straight alpha in sRGB space.
    pub const RGBA8888: Self =
        Self::_new_packed(Packed::Rgba8888, Sample::U32, Transfer::Srgb, Alpha::Straight);

    /// Packed 32-bit XRGB8888 in sRGB space.
    pub const XRGB8888: Self =
        Self::_new_packed(Packed::Xrgb8888, Sample::U32, Transfer::Srgb, Alpha::Opaque);
    /// Packed 32-bit ARGB8888 with straight alpha in sRGB space.
    pub const ARGB8888: Self =
        Self::_new_packed(Packed::Argb8888, Sample::U32, Transfer::Srgb, Alpha::Straight);

    /* packed bgr */

    /// Packed 8-bit BGR332 in sRGB space.
    pub const BGR332: Self =
        Self::_new_packed(Packed::Bgr332, Sample::U8, Transfer::Srgb, Alpha::None);
    /// Packed 16-bit BGR565 in sRGB space.
    pub const BGR565: Self =
        Self::_new_packed(Packed::Bgr565, Sample::U16, Transfer::Srgb, Alpha::None);
    /// Packed 32-bit BGRX8888 in sRGB space.
    pub const BGRX8888: Self =
        Self::_new_packed(Packed::Bgrx8888, Sample::U32, Transfer::Srgb, Alpha::Opaque);
    /// Packed 32-bit BGRA8888 with straight alpha in sRGB space.
    pub const BGRA8888: Self =
        Self::_new_packed(Packed::Bgra8888, Sample::U32, Transfer::Srgb, Alpha::Straight);
    /// Packed 32-bit XBGR8888 in sRGB space.
    pub const XBGR8888: Self =
        Self::_new_packed(Packed::Xbgr8888, Sample::U32, Transfer::Srgb, Alpha::Opaque);
    /// Packed 32-bit ABGR8888 with straight alpha in sRGB space.
    pub const ABGR8888: Self =
        Self::_new_packed(Packed::Abgr8888, Sample::U32, Transfer::Srgb, Alpha::Straight);

    /* indexed*/

    /// 8-bit palette-indexed raster format.
    pub const INDEXED8: Self =
        Self::_new(Channels::Indexed, Sample::U8, Transfer::Srgb, Alpha::None);

    /* float */

    /// 96-bit linear RGB using 32-bit float channels.
    pub const RGB_LIN_F32: Self =
        Self::_new(Channels::Rgb, Sample::F32, Transfer::Linear, Alpha::None);
    /// 128-bit linear RGBA using 32-bit float channels and straight alpha.
    pub const RGBA_LIN_F32: Self =
        Self::_new(Channels::Rgba, Sample::F32, Transfer::Linear, Alpha::Straight);
    /// 128-bit linear RGBA using 32-bit float channels and premultiplied alpha.
    pub const RGBA_LIN_PRE_F32: Self =
        Self::_new(Channels::Rgba, Sample::F32, Transfer::Linear, Alpha::Premultiplied);
}
