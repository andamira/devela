// devela::media::image::sixel::output::builder
//
//! Defines the [`LegacySixel`] builder struct.
//
// TOC
// - LegacySixel
// - common methods
// - extra methods
// - sixel_string

use super::{
    LegacySixelDitherConf, LegacySixelEncodePolicy, LegacySixelError, LegacySixelMean,
    LegacySixelOutput, LegacySixelPixelFormat, LegacySixelQuality, LegacySixelResult,
    LegacySixelSplit,
};
use crate::{ConstDefault, LegacySixelDither, String, ToString, Vec};

/// A configurable sixel string builder from a slice of pixel data bytes.
///
/// By default it assumes `RGB888` LegacySixelPixelFormat, and `Auto`matic `LegacySixelDither`,
/// `LegacySixelSplit`, `LegacySixelMean` and `LegacySixelQuality`.
///
/// # Example
/// ```
/// # use devela::LegacySixel;
/// // 2x2 pixels (Red, Green, Blue, White)
/// const IMAGE_HEX: &[u8] = b"FF000000FF000000FFFFFFFF";
/// //                         RRGGBBrrggbbRRGGBBrrggbb
/// println!("{}", LegacySixel::with_bytes_size(IMAGE_HEX, 2, 2).build().unwrap());
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct LegacySixel<'a> {
    ///
    pub bytes: Option<&'a [u8]>,
    ///
    pub width: i32,
    ///
    pub height: i32,
    ///
    pub format: LegacySixelPixelFormat,
    ///
    pub dither: LegacySixelDither,
    ///
    pub split: LegacySixelSplit,
    /// Method for choosing a representative mean color for the box.
    pub mean: LegacySixelMean,
    ///
    pub quality: LegacySixelQuality,
}

impl ConstDefault for LegacySixel<'_> {
    const DEFAULT: Self = Self {
        bytes: None,
        width: 0,
        height: 0,
        format: LegacySixelPixelFormat::DEFAULT,
        dither: LegacySixelDither::DEFAULT,
        split: LegacySixelSplit::DEFAULT,
        mean: LegacySixelMean::DEFAULT,
        quality: LegacySixelQuality::DEFAULT,
    };
}

/// # Common methods
#[rustfmt::skip]
impl<'bytes> LegacySixel<'bytes> {
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
    pub fn build(self) -> LegacySixelResult<String> {
        if self.width == 0 || self.height == 0 {
            return Err(LegacySixelError::BadInput);
        }
        if let Some(bytes) = self.bytes {
            if bytes.len() < self.format.required_bytes(self.width, self.height) {
                Err(LegacySixelError::BadInput)
            } else {
                sixel_string(bytes, self.width, self.height,
                    self.format, self.dither, self.split, self.mean, self.quality)
            }
        } else {
            Err(LegacySixelError::BadInput)
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
    pub const fn format(mut self, format: LegacySixelPixelFormat) -> Self {
        self.format = format; self
    }
    /// Sets the method for dither diffusion.
    #[must_use]
    pub const fn dither(mut self, dither: LegacySixelDither) -> Self {
        self.dither = dither; self
    }
    /// Sets the method for largest dimension for splitting.
    #[must_use]
    pub const fn split(mut self, split: LegacySixelSplit) -> Self {
        self.split = split; self
    }
    /// Sets the method for mean.
    #[must_use]
    pub const fn mean(mut self, mean: LegacySixelMean) -> Self {
        self.mean = mean; self
    }
    /// Sets the quality.
    #[must_use]
    pub const fn quality(mut self, quality: LegacySixelQuality) -> Self {
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
impl LegacySixel<'_> {
    add_method![format_rgb555, format, LegacySixelPixelFormat::RGB555];
    add_method![format_rgb565, format, LegacySixelPixelFormat::RGB565];
    add_method![format_rgb888, format, LegacySixelPixelFormat::RGB888];
    add_method![format_bgr555, format, LegacySixelPixelFormat::BGR555];

    add_method![format_bgr565, format, LegacySixelPixelFormat::BGR565];
    add_method![format_bgr888, format, LegacySixelPixelFormat::BGR888];
    add_method![format_argb8888, format, LegacySixelPixelFormat::ARGB8888];
    add_method![format_rgba8888, format, LegacySixelPixelFormat::RGBA8888];
    add_method![format_abgr8888, format, LegacySixelPixelFormat::ABGR8888];
    add_method![format_bgra8888, format, LegacySixelPixelFormat::BGRA8888];
    add_method![format_g1, format, LegacySixelPixelFormat::G1];
    add_method![format_g2, format, LegacySixelPixelFormat::G2];
    add_method![format_g4, format, LegacySixelPixelFormat::G4];
    add_method![format_g8, format, LegacySixelPixelFormat::G8];
    add_method![format_ag88, format, LegacySixelPixelFormat::AG88];
    add_method![format_ga88, format, LegacySixelPixelFormat::GA88];
    add_method![format_pal1, format, LegacySixelPixelFormat::PAL1];
    add_method![format_pal2, format, LegacySixelPixelFormat::PAL2];
    add_method![format_pal4, format, LegacySixelPixelFormat::PAL4];
    add_method![format_pal8, format, LegacySixelPixelFormat::PAL8];
    //
    add_method![split_auto, split, LegacySixelSplit::Auto];
    add_method![split_norm, split, LegacySixelSplit::Norm];
    add_method![split_lum, split, LegacySixelSplit::Lum];
    //
    add_method![mean_auto, mean, LegacySixelMean::Auto];
    add_method![mean_center, mean, LegacySixelMean::Center];
    add_method![mean_colors, mean, LegacySixelMean::Colors];
    add_method![mean_pixels, mean, LegacySixelMean::Pixels];
    //
    add_method![dither_auto, dither, LegacySixelDither::Auto];
    add_method![dither_none, dither, LegacySixelDither::None];
    add_method![dither_atkinson, dither, LegacySixelDither::Atkinson];
    add_method![dither_fs, dither, LegacySixelDither::FS];
    add_method![dither_jajuni, dither, LegacySixelDither::JaJuNi];
    add_method![dither_stucki, dither, LegacySixelDither::Stucki];
    add_method![dither_burkes, dither, LegacySixelDither::Burkes];
    add_method![dither_adither, dither, LegacySixelDither::ADither];
    add_method![dither_xdither, dither, LegacySixelDither::XDither];
    //
    add_method![quality_auto, quality, LegacySixelQuality::Auto];
    add_method![quality_high, quality, LegacySixelQuality::High];
    add_method![quality_low, quality, LegacySixelQuality::Low];
    add_method![quality_full, quality, LegacySixelQuality::Full];
    add_method![quality_high_color, quality, LegacySixelQuality::HighColor];
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
///     LegacySixelPixelFormat::RGB888,
///     LegacySixelDither::Stucki,
///     LegacySixelSplit::Auto,
///     LegacySixelMean::Auto,
///     LegacySixelQuality::Auto
/// ).unwrap());
/// ```
#[expect(clippy::too_many_arguments)]
fn sixel_string(
    bytes: &[u8],
    width: i32,
    height: i32,
    pixel_format: LegacySixelPixelFormat,
    dither: LegacySixelDither,
    split: LegacySixelSplit,
    mean: LegacySixelMean,
    quality: LegacySixelQuality,
) -> LegacySixelResult<String> {
    let mut sixel_data: Vec<u8> = Vec::new(); // MAYBE with_capacity

    let mut sixel_output = LegacySixelOutput::new(&mut sixel_data);
    sixel_output.set_encode_policy(LegacySixelEncodePolicy::Auto);
    let mut dither_conf = LegacySixelDitherConf::new(256).unwrap();

    dither_conf.set_optimize_palette(true);

    dither_conf.initialize(bytes, width, height, pixel_format, split, mean, quality)?;
    dither_conf.set_pixel_format(pixel_format);
    dither_conf.set_diffusion_method(dither);

    let mut bytes = bytes.to_vec();
    sixel_output.encode(&mut bytes, width, height, 0, &mut dither_conf)?;

    Ok(String::from_utf8_lossy(&sixel_data).to_string())
}
