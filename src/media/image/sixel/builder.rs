// devela::media::image::sixel::output::builder
//
//! Defines the [`Sixel`] builder struct.
//
// TOC
// - Sixel
// - common methods
// - extra methods
// - sixel_string

use super::{
    DitherConf, PixelFormat, SixelEncodePolicy, SixelError, SixelMean, SixelOutput, SixelQuality,
    SixelResult, SixelSplit,
};
use crate::{ConstDefault, Dither, String, ToString, Vec};

/// A configurable sixel string builder from a slice of pixel data bytes.
///
/// By default it assumes `RGB888` PixelFormat, and `Auto`matic `Dither`,
/// `SixelSplit`, `SixelMean` and `SixelQuality`.
///
/// # Example
/// ```
/// # use devela::Sixel;
/// // 2x2 pixels (Red, Green, Blue, White)
/// const IMAGE_HEX: &[u8] = b"FF000000FF000000FFFFFFFF";
/// //                         RRGGBBrrggbbRRGGBBrrggbb
/// println!("{}", Sixel::with_bytes_size(IMAGE_HEX, 2, 2).build().unwrap());
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Sixel<'a> {
    ///
    pub bytes: Option<&'a [u8]>,
    ///
    pub width: i32,
    ///
    pub height: i32,
    ///
    pub format: PixelFormat,
    ///
    pub dither: Dither,
    ///
    pub split: SixelSplit,
    /// Method for choosing a representative mean color for the box.
    pub mean: SixelMean,
    ///
    pub quality: SixelQuality,
}

impl ConstDefault for Sixel<'_> {
    const DEFAULT: Self = Self {
        bytes: None,
        width: 0,
        height: 0,
        format: PixelFormat::DEFAULT,
        dither: Dither::DEFAULT,
        split: SixelSplit::DEFAULT,
        mean: SixelMean::DEFAULT,
        quality: SixelQuality::DEFAULT,
    };
}

/// # Common methods
#[rustfmt::skip]
impl<'bytes> Sixel<'bytes> {
    /// Returns a new empty sixel builder.
    #[must_use]
    pub const fn new() -> Self { Self::DEFAULT }

    /// Returns a new empty sixel builder with the given byte slice.
    #[must_use]
    pub const fn with_bytes(bytes: &'bytes [u8]) -> Self {
        Self::DEFAULT.bytes(bytes)
    }

    /// Returns a new empty sixel builder with the given size.
    #[must_use]
    pub const fn with_size(width: i32, height: i32) -> Self {
        Self::DEFAULT.size(width, height)
    }

    /// Returns a new empty sixel builder with the given byte slize and size.
    #[must_use]
    pub const fn with_bytes_size(bytes: &'bytes [u8], width: i32, height: i32) -> Self {
        Self::DEFAULT.bytes(bytes).size(width, height)
    }

    /* */

    /// Builds a sixel formatted string with the configured options.
    ///
    /// # Errors
    /// Returns an error if
    /// the bytes slice have not been set,
    /// if either the width or height is 0,
    /// or the slice is not long enough.
    pub fn build(self) -> SixelResult<String> {
        if self.width == 0 || self.height == 0 {
            return Err(SixelError::BadInput);
        }
        if let Some(bytes) = self.bytes {
            if bytes.len() < self.format.required_bytes(self.width, self.height) {
                Err(SixelError::BadInput)
            } else {
                sixel_string(bytes, self.width, self.height,
                    self.format, self.dither, self.split, self.mean, self.quality)
            }
        } else {
            Err(SixelError::BadInput)
        }
    }

    /* */

    /// Sets the byte slice of image data.
    #[must_use]
    pub const fn bytes(mut self, bytes: &'bytes [u8]) -> Self {
        self.bytes = Some(bytes); self
    }
    /// Sets the width.
    #[must_use]
    pub const fn width(mut self, width: i32) -> Self {
        self.width = width; self
    }
    /// Sets the height.
    #[must_use]
    pub const fn height(mut self, height: i32) -> Self {
        self.height = height; self
    }
    /// Sets the size (width, height).
    #[must_use]
    pub const fn size(mut self, width: i32, height: i32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /**/

    /// Sets the pixel format.
    #[must_use]
    pub const fn format(mut self, format: PixelFormat) -> Self {
        self.format = format; self
    }
    /// Sets the method for dither diffusion.
    #[must_use]
    pub const fn dither(mut self, dither: Dither) -> Self {
        self.dither = dither; self
    }
    /// Sets the method for largest dimension for splitting.
    #[must_use]
    pub const fn split(mut self, split: SixelSplit) -> Self {
        self.split = split; self
    }
    /// Sets the method for mean.
    #[must_use]
    pub const fn mean(mut self, mean: SixelMean) -> Self {
        self.mean = mean; self
    }
    /// Sets the quality.
    #[must_use]
    pub const fn quality(mut self, quality: SixelQuality) -> Self {
        self.quality = quality; self
    }
}

macro_rules! add_method {
    ($fn:ident, $field:ident, $variant:expr) => {
        #[doc = concat!["Sets the `", stringify!($field), "` field to [`", stringify!($variant), "`]."]]
        #[must_use]
        pub const fn $fn(mut self) -> Self {
            self.$field = $variant;
            self
        }
    };
}
/// # Extra methods
#[rustfmt::skip]
impl Sixel<'_> {
    add_method![format_rgb555, format, PixelFormat::RGB555];
    add_method![format_rgb565, format, PixelFormat::RGB565];
    add_method![format_rgb888, format, PixelFormat::RGB888];
    add_method![format_bgr555, format, PixelFormat::BGR555];

    add_method![format_bgr565, format, PixelFormat::BGR565];
    add_method![format_bgr888, format, PixelFormat::BGR888];
    add_method![format_argb8888, format, PixelFormat::ARGB8888];
    add_method![format_rgba8888, format, PixelFormat::RGBA8888];
    add_method![format_abgr8888, format, PixelFormat::ABGR8888];
    add_method![format_bgra8888, format, PixelFormat::BGRA8888];
    add_method![format_g1, format, PixelFormat::G1];
    add_method![format_g2, format, PixelFormat::G2];
    add_method![format_g4, format, PixelFormat::G4];
    add_method![format_g8, format, PixelFormat::G8];
    add_method![format_ag88, format, PixelFormat::AG88];
    add_method![format_ga88, format, PixelFormat::GA88];
    add_method![format_pal1, format, PixelFormat::PAL1];
    add_method![format_pal2, format, PixelFormat::PAL2];
    add_method![format_pal4, format, PixelFormat::PAL4];
    add_method![format_pal8, format, PixelFormat::PAL8];
    //
    add_method![split_auto, split, SixelSplit::Auto];
    add_method![split_norm, split, SixelSplit::Norm];
    add_method![split_lum, split, SixelSplit::Lum];
    //
    add_method![mean_auto, mean, SixelMean::Auto];
    add_method![mean_center, mean, SixelMean::Center];
    add_method![mean_colors, mean, SixelMean::Colors];
    add_method![mean_pixels, mean, SixelMean::Pixels];
    //
    add_method![dither_auto, dither, Dither::Auto];
    add_method![dither_none, dither, Dither::None];
    add_method![dither_atkinson, dither, Dither::Atkinson];
    add_method![dither_fs, dither, Dither::FS];
    add_method![dither_jajuni, dither, Dither::JaJuNi];
    add_method![dither_stucki, dither, Dither::Stucki];
    add_method![dither_burkes, dither, Dither::Burkes];
    add_method![dither_adither, dither, Dither::ADither];
    add_method![dither_xdither, dither, Dither::XDither];
    //
    add_method![quality_auto, quality, SixelQuality::Auto];
    add_method![quality_high, quality, SixelQuality::High];
    add_method![quality_low, quality, SixelQuality::Low];
    add_method![quality_full, quality, SixelQuality::Full];
    add_method![quality_high_color, quality, SixelQuality::HighColor];
}

/// Writes a string of sixel data.
///
/// # Example
/// ```ignore
/// # use sixela::*;
/// // 2x2 pixels (Red, Green, Blue, White)
/// const IMAGE_HEX: &[u8] = b"FF000000FF000000FFFFFFFF";
///                          //RRGGBBrrggbbRRGGBBrrggbb
///
/// println!("{}", sixel_string(
///     IMAGE_HEX, 2, 2,
///     PixelFormat::RGB888,
///     Dither::Stucki,
///     SixelSplit::Auto,
///     SixelMean::Auto,
///     SixelQuality::Auto
/// ).unwrap());
/// ```
#[expect(clippy::too_many_arguments)]
fn sixel_string(
    bytes: &[u8],
    width: i32,
    height: i32,
    pixel_format: PixelFormat,
    dither: Dither,
    split: SixelSplit,
    mean: SixelMean,
    quality: SixelQuality,
) -> SixelResult<String> {
    let mut sixel_data: Vec<u8> = Vec::new(); // MAYBE with_capacity

    let mut sixel_output = SixelOutput::new(&mut sixel_data);
    sixel_output.set_encode_policy(SixelEncodePolicy::Auto);
    let mut dither_conf = DitherConf::new(256).unwrap();

    dither_conf.set_optimize_palette(true);

    dither_conf.initialize(bytes, width, height, pixel_format, split, mean, quality)?;
    dither_conf.set_pixel_format(pixel_format);
    dither_conf.set_diffusion_method(dither);

    let mut bytes = bytes.to_vec();
    sixel_output.encode(&mut bytes, width, height, 0, &mut dither_conf)?;

    Ok(String::from_utf8_lossy(&sixel_data).to_string())
}
