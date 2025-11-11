// devela::media::image::sixel::output::enums
//
//! Defines: [`LegacySixelPixelFormat`],
//
// TOC
// - enum LegacySixelSplit
// - enum LegacySixelMean
// - enum LegacySixelQuality
//
// private:
// - enum LegacySixelEncodePolicy
// - enum LegacySixelColorModel
// - enum Loop
// - //
//   - enum LegacySixelResampleMethod
//   - enum ImageFormat
//   - enum LegacySixelPixelFormatType

crate::impl_cdef! { ConstDefault: Self::Auto => LegacySixelSplit, LegacySixelMean,
LegacySixelQuality, Loop, LegacySixelEncodePolicy, LegacySixelColorModel }

/// Method for finding the largest dimension for splitting,
/// and sorting by that component.
//
// # Adaptation
// - Derived from `methodForLargest` enum in the `libsixel` C library.
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub enum LegacySixelSplit {
    /// Choose automatically the method for finding the largest dimension. (default)
    #[default]
    Auto,
    /// Simply comparing the range in RGB space.
    Norm,
    /// Transforming into luminosities before the comparison.
    Lum,
}

/// Method for selecting a representative color from a color space partition (box).
//
// # Adaptation
// - Derived from `methodForRep` enum in the `libsixel` C library.
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub enum LegacySixelMean {
    /// Choose automatically the method for selecting representative color from each box.
    /// (default)
    #[default]
    Auto,
    /// Choose the geometric center of the box.
    Center,
    /// Choose the mean of all unique colors in the box (specified in Heckbert's paper).
    Colors,
    /// Computes the mean weighted by pixel count.
    Pixels,
}

/// Quality modes.
//
// # Adaptation
// Derived from `qualityMode` enum in the `libsixel` C library.
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub enum LegacySixelQuality {
    /// Choose quality mode automatically.
    #[default]
    Auto,
    /// High quality palette construction.
    High,
    /// Low quality palette construction.
    Low,
    /// Full quality palette construction.
    Full,
    /// High color.
    HighColor,
}

/* private items */

/// Policies of SIXEL encoding.
//
// # Adaptation
// Derived from `encodePolicy` enum in the `libsixel` C library.
#[repr(u8)]
#[allow(dead_code, reason = "Fast variant never constructed")]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub(crate) enum LegacySixelEncodePolicy {
    /// Choose encoding policy automatically (default).
    #[default]
    Auto = 0,
    /// Encode as fast as possible.
    Fast = 1,
    /// Encode to as small sixel sequence as possible.
    Size = 2,
}

/// Color model used for palette generation in Sixel output.
///
/// This defines **how colors are represented or generated** in a Sixel image,
/// rather than the specific colors used.
///
//
// # Adaptation
// Derived from `paletteType` enum in the `libsixel` C library.
#[repr(u8)]
#[allow(dead_code, reason = "Some variants are never constructed")]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub(crate) enum LegacySixelColorModel {
    /// Automatically chooses a color model based on output settings.
    #[default]
    Auto,
    /// Uses the **HLS (Hue, Lightness, Saturation)** colorspace.
    Hls,
    /// Uses the **RGB (Red, Green, Blue)** colorspace.
    Rgb,
}

/// Loop mode.
//
// # Adaptation
// Derived from `loopControl` enum in the `libsixel` C library.
#[repr(u8)]
#[expect(dead_code, reason = "Only using `Auto` for now")]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
enum Loop {
    /// Honer the setting of GIF header.
    #[default]
    Auto,
    /// Always enable loop.
    Force,
    /// Always disable loop.
    Disable,
}

// /// Method of resampling.
// //
// // # Adaptation
// // Derived from `methodForResampling` enum in the `libsixel` C library.
// #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
// pub enum LegacySixelResampleMethod { // TODO:MAYBE
//     /// Use nearest neighbor method
//     Nearest,
//     /// Use guaussian filter
//     Gaussian,
//     /// Use hanning filter
//     Hanning,
//     /// Use hamming filter
//     Hamming,
//     /// Use bilinear filter
//     Bilinear,
//     /// Use welfilter
//     Welsh,
//     /// Use bicubic filter
//     Bicubic,
//     /// Use lanczos-2 filter
//     Lanczos2,
//     /// Use lanczos-3 filter
//     Lanczos3,
//     /// Use lanczos-4 filter
//     Lanczos4,
// }

// /// Image format
// //
// // # Adaptation
// // Derived from `imageFormat` enum in the `libsixel` C library.
// #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
// enum ImageFormat { // TODO:MAYBE
//     GIF,   //         0x0 /* read only */
//     PNG,   //         0x1 /* read/write */
//     BMP,   //         0x2 /* read only */
//     JPG,   //         0x3 /* read only */
//     TGA,   //         0x4 /* read only */
//     WBMP,  //         0x5 /* read only with --with-gd configure option */
//     TIFF,  //         0x6 /* read only */
//     SIXEL, //         0x7 /* read only */
//     PNM,   //         0x8 /* read only */
//     GD2,   //         0x9 /* read only with --with-gd configure option */
//     PSD,   //         0xa /* read only */
//     HDR,   //         0xb /* read only */
// }

// /// Offset value of `LegacySixelPixelFormat`.
// //
// // # Adaptation
// // Derived from `formatType` enum in the `libsixel` C library.
// #[repr(u8)]
// #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
// pub enum LegacySixelPixelFormatType { // TODO:MAYBE
//     Color,     // 0
//     Grayscale, // (1 << 6)
//     Palette,   // (1 << 7)
// }
