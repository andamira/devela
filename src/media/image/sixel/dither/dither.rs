// devela::media::image::sixel::dither::dither
//
//! Defines the [`Dither`] enum.
//
// TOC
// - Dither
// - fn sixel_apply_15bpp_dither
// - fn dither_none
// - fn dither_fs_15bpp
// - fn dither_atkinson_15bpp
// - fn dither_jajuni_15bpp
// - fn dither_stucki_15bpp
// - fn dither_burkes_15bpp
// - fn dither_a_dither_15bpp
// - fn dither_x_dither_15bpp

#![allow(clippy::erasing_op, clippy::identity_op, reason = "symmetry")]

crate::impl_cdef! { Self::Auto => Dither }

/// Dithering methods of error diffusion.
///
/// Dithering helps improve image quality when reducing color depth, commonly
/// used in **Sixel**, **GIF**, and other indexed-color formats.
//
// # Adaptation
// - Derived from `methodForDiffuse` enum in the `libsixel` C library.
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub enum Dither {
    /// Choose diffusion type automatically. (Default)
    #[default]
    Auto = 0,

    /// No dithering is applied. Pixels are directly quantized without modification.
    None = 1,

    /// Error diffusion dithering using Bill Atkinson's method.
    /// Produces a softer dithering effect with a limited error spread.
    /// Often used in early Macintosh graphics.
    Atkinson = 2,

    /// Floyd-Steinberg error diffusion dithering.
    /// A widely used method that spreads error to neighboring pixels for smooth gradients.
    /// Produces good results with minimal artifacts.
    FS = 3,

    /// Jarvis, Judice & Ninke (JaJuNi) error diffusion dithering.
    /// Spreads quantization error further across neighboring pixels,
    /// resulting in smoother transitions but requiring more computation.
    JaJuNi = 4,

    /// Stucki error diffusion dithering.
    /// Similar to JaJuNi but slightly optimized for faster computation.
    /// Produces high-quality results with minimal artifacts.
    Stucki = 5,

    /// Burkes error diffusion dithering.
    /// A simplified version of Stucki with a smaller diffusion matrix,
    /// reducing computation while maintaining good quality.
    Burkes = 6,

    /// Positionally stable arithmetic dithering.
    /// Applies a deterministic arithmetic transformation to each pixel,
    /// ensuring consistency without propagating errors.
    ADither = 7,

    /// Positionally stable XOR-based dithering.
    /// Uses bitwise XOR operations for structured noise generation,
    /// creating a high-frequency dithering pattern without error diffusion.
    XDither = 8,
}

impl Dither {
    /// Applies dithering to a pixel array in **15-bit color mode (5-5-5 RGB)**.
    ///
    /// This method modifies pixel values based on the selected **dithering algorithm**.
    /// It is designed for **15bpp color depth**, commonly used in retro graphics
    /// and hardware-limited displays.
    ///
    /// # Behavior
    /// - **Error diffusion dithering** (e.g., Floyd-Steinberg, Atkinson) is only applied
    ///   when there is enough space for propagation.
    /// - **Positionally stable dithering** (e.g., `ADither`, `XDither`) is applied
    ///   unconditionally per pixel.
    /// - `None` disables dithering, and `Auto` currently behaves the same.
    #[rustfmt::skip]
    pub fn apply_15bpp(self, pixels: &mut [u8], x: i32, y: i32, width: i32, height: i32) {
        match self {
            Dither::None | Dither::Auto => {
                dither_none(pixels, width); }
            /* only run when enough neighboring pixels exist. */
            Dither::Atkinson => {
                if x < width - 2 && y < height - 2 { dither_atkinson_15bpp(pixels, width); } }
            Dither::FS => {
                if x < width - 1 && y < height - 1 { dither_fs_15bpp(pixels, width); } }
            Dither::JaJuNi => {
                if x < width - 2 && y < height - 2 { dither_jajuni_15bpp(pixels, width); } }
            Dither::Stucki => {
                if x < width - 2 && y < height - 2 { dither_stucki_15bpp(pixels, width); } }
            Dither::Burkes => {
                if x < width - 2 && y < height - 1 { dither_burkes_15bpp(pixels, width); } }
            /* apply immediately without boundary checks */
            Dither::ADither => {
                dither_a_dither_15bpp(pixels, width, x, y); }
            Dither::XDither => {
                dither_x_dither_15bpp(pixels, width, x, y); }
        }
    }
}

/// No dithering
fn dither_none(_data: &mut [u8], _width: i32) {}

/// Floyd Steinberg dithering
///
/// ```txt
///         curr    7/16
/// 3/16    5/48    1/16
/// ```
fn dither_fs_15bpp(data: &mut [u8], width: i32) {
    let error_r = data[0] as i32 & 0x7;
    let error_g = data[1] as i32 & 0x7;
    let error_b = data[2] as i32 & 0x7;
    let width = width as usize;
    let mut r = data[3 + 0] as i32 + ((error_r * 5) >> 4);
    let mut g = data[3 + 1] as i32 + ((error_g * 5) >> 4);
    let mut b = data[3 + 2] as i32 + ((error_b * 5) >> 4);
    data[3 + 0] = if r > 0xff { 0xff } else { r as u8 };
    data[3 + 1] = if g > 0xff { 0xff } else { g as u8 };
    data[3 + 2] = if b > 0xff { 0xff } else { b as u8 };
    r = data[width * 3 - 3 + 0] as i32 + ((error_r * 3) >> 4);
    g = data[width * 3 - 3 + 1] as i32 + ((error_g * 3) >> 4);
    b = data[width * 3 - 3 + 2] as i32 + ((error_b * 3) >> 4);
    data[width * 3 - 3 + 0] = if r > 0xff { 0xff } else { r as u8 };
    data[width * 3 - 3 + 1] = if g > 0xff { 0xff } else { g as u8 };
    data[width * 3 - 3 + 2] = if b > 0xff { 0xff } else { b as u8 };
    r = data[width * 3 + 0] as i32 + ((error_r * 5) >> 4);
    g = data[width * 3 + 1] as i32 + ((error_g * 5) >> 4);
    b = data[width * 3 + 2] as i32 + ((error_b * 5) >> 4);
    data[width * 3 + 0] = if r > 0xff { 0xff } else { r as u8 };
    data[width * 3 + 1] = if g > 0xff { 0xff } else { g as u8 };
    data[width * 3 + 2] = if b > 0xff { 0xff } else { b as u8 };
}

/// Atkinson's dithering
///
/// ```txt
///         curr    1/8    1/8
///  1/8     1/8    1/8
/// ```
fn dither_atkinson_15bpp(data: &mut [u8], width: i32) {
    let mut error_r = data[0] as i32 & 0x7;
    let mut error_g = data[1] as i32 & 0x7;
    let mut error_b = data[2] as i32 & 0x7;
    error_r += 4;
    error_g += 4;
    error_b += 4;
    let width = width as usize;

    let mut r = data[(width * 0 + 1) * 3 + 0] as i32 + (error_r >> 3);
    let mut g = data[(width * 0 + 1) * 3 + 1] as i32 + (error_g >> 3);
    let mut b = data[(width * 0 + 1) * 3 + 2] as i32 + (error_b >> 3);
    data[(width * 0 + 1) * 3 + 0] = if r > 0xff { 0xff } else { r as u8 };
    data[(width * 0 + 1) * 3 + 1] = if g > 0xff { 0xff } else { g as u8 };
    data[(width * 0 + 1) * 3 + 2] = if b > 0xff { 0xff } else { b as u8 };
    r = data[(width * 0 + 2) * 3 + 0] as i32 + (error_r >> 3);
    g = data[(width * 0 + 2) * 3 + 1] as i32 + (error_g >> 3);
    b = data[(width * 0 + 2) * 3 + 2] as i32 + (error_b >> 3);
    data[(width * 0 + 2) * 3 + 0] = if r > 0xff { 0xff } else { r as u8 };
    data[(width * 0 + 2) * 3 + 1] = if g > 0xff { 0xff } else { g as u8 };
    data[(width * 0 + 2) * 3 + 2] = if b > 0xff { 0xff } else { b as u8 };
    r = data[(width * 1 - 1) * 3 + 0] as i32 + (error_r >> 3);
    g = data[(width * 1 - 1) * 3 + 1] as i32 + (error_g >> 3);
    b = data[(width * 1 - 1) * 3 + 2] as i32 + (error_b >> 3);
    data[(width * 1 - 1) * 3 + 0] = if r > 0xff { 0xff } else { r as u8 };
    data[(width * 1 - 1) * 3 + 1] = if g > 0xff { 0xff } else { g as u8 };
    data[(width * 1 - 1) * 3 + 2] = if b > 0xff { 0xff } else { b as u8 };
    r = data[(width * 1 + 0) * 3 + 0] as i32 + (error_r >> 3);
    g = data[(width * 1 + 0) * 3 + 1] as i32 + (error_g >> 3);
    b = data[(width * 1 + 0) * 3 + 2] as i32 + (error_b >> 3);
    data[(width * 1 + 0) * 3 + 0] = if r > 0xff { 0xff } else { r as u8 };
    data[(width * 1 + 0) * 3 + 1] = if g > 0xff { 0xff } else { g as u8 };
    data[(width * 1 + 0) * 3 + 2] = if b > 0xff { 0xff } else { b as u8 };
    r = data[(width * 1 + 1) * 3 + 0] as i32 + (error_r >> 3);
    g = data[(width * 1 + 1) * 3 + 1] as i32 + (error_g >> 3);
    b = data[(width * 1 + 1) * 3 + 2] as i32 + (error_b >> 3);
    data[(width * 1 + 1) * 3 + 0] = if r > 0xff { 0xff } else { r as u8 };
    data[(width * 1 + 1) * 3 + 1] = if g > 0xff { 0xff } else { g as u8 };
    data[(width * 1 + 1) * 3 + 2] = if b > 0xff { 0xff } else { b as u8 };
    r = data[(width * 2 + 0) * 3 + 0] as i32 + (error_r >> 3);
    g = data[(width * 2 + 0) * 3 + 1] as i32 + (error_g >> 3);
    b = data[(width * 2 + 0) * 3 + 2] as i32 + (error_b >> 3);
    data[(width * 2 + 0) * 3 + 0] = if r > 0xff { 0xff } else { r as u8 };
    data[(width * 2 + 0) * 3 + 1] = if g > 0xff { 0xff } else { g as u8 };
    data[(width * 2 + 0) * 3 + 2] = if b > 0xff { 0xff } else { b as u8 };
}

/// Jarvis, Judice & Ninke dithering
///
/// ```txt
///                 curr    7/48    5/48
/// 3/48    5/48    7/48    5/48    3/48
/// 1/48    3/48    5/48    3/48    1/48
/// ```
fn dither_jajuni_15bpp(data: &mut [u8], width: i32) {
    let mut error_r = data[0] as i32 & 0x7;
    let mut error_g = data[1] as i32 & 0x7;
    let mut error_b = data[2] as i32 & 0x7;
    error_r += 4;
    error_g += 4;
    error_b += 4;
    let width = width as usize;

    let mut r = data[(width * 0 + 1) * 3 + 0] as i32 + (error_r * 7 / 48);
    let mut g = data[(width * 0 + 1) * 3 + 1] as i32 + (error_g * 7 / 48);
    let mut b = data[(width * 0 + 1) * 3 + 2] as i32 + (error_b * 7 / 48);
    data[(width * 0 + 1) * 3 + 0] = if r > 0xff { 0xff } else { r as u8 };
    data[(width * 0 + 1) * 3 + 1] = if g > 0xff { 0xff } else { g as u8 };
    data[(width * 0 + 1) * 3 + 2] = if b > 0xff { 0xff } else { b as u8 };
    r = data[(width * 0 + 2) * 3 + 0] as i32 + (error_r * 5 / 48);
    g = data[(width * 0 + 2) * 3 + 1] as i32 + (error_g * 5 / 48);
    b = data[(width * 0 + 2) * 3 + 2] as i32 + (error_b * 5 / 48);
    data[(width * 0 + 2) * 3 + 0] = if r > 0xff { 0xff } else { r as u8 };
    data[(width * 0 + 2) * 3 + 1] = if g > 0xff { 0xff } else { g as u8 };
    data[(width * 0 + 2) * 3 + 2] = if b > 0xff { 0xff } else { b as u8 };
    r = data[(width * 1 - 2) * 3 + 0] as i32 + (error_r * 3 / 48);
    g = data[(width * 1 - 2) * 3 + 1] as i32 + (error_g * 3 / 48);
    b = data[(width * 1 - 2) * 3 + 2] as i32 + (error_b * 3 / 48);
    data[(width * 1 - 2) * 3 + 0] = if r > 0xff { 0xff } else { r as u8 };
    data[(width * 1 - 2) * 3 + 1] = if g > 0xff { 0xff } else { g as u8 };
    data[(width * 1 - 2) * 3 + 2] = if b > 0xff { 0xff } else { b as u8 };
    r = data[(width * 1 - 1) * 3 + 0] as i32 + (error_r * 5 / 48);
    g = data[(width * 1 - 1) * 3 + 1] as i32 + (error_g * 5 / 48);
    b = data[(width * 1 - 1) * 3 + 2] as i32 + (error_b * 5 / 48);
    data[(width * 1 - 1) * 3 + 0] = if r > 0xff { 0xff } else { r as u8 };
    data[(width * 1 - 1) * 3 + 1] = if g > 0xff { 0xff } else { g as u8 };
    data[(width * 1 - 1) * 3 + 2] = if b > 0xff { 0xff } else { b as u8 };
    r = data[(width * 1 + 0) * 3 + 0] as i32 + (error_r * 7 / 48);
    g = data[(width * 1 + 0) * 3 + 1] as i32 + (error_g * 7 / 48);
    b = data[(width * 1 + 0) * 3 + 2] as i32 + (error_b * 7 / 48);
    data[(width * 1 + 0) * 3 + 0] = if r > 0xff { 0xff } else { r as u8 };
    data[(width * 1 + 0) * 3 + 1] = if g > 0xff { 0xff } else { g as u8 };
    data[(width * 1 + 0) * 3 + 2] = if b > 0xff { 0xff } else { b as u8 };
    r = data[(width * 1 + 1) * 3 + 0] as i32 + (error_r * 5 / 48);
    g = data[(width * 1 + 1) * 3 + 1] as i32 + (error_g * 5 / 48);
    b = data[(width * 1 + 1) * 3 + 2] as i32 + (error_b * 5 / 48);
    data[(width * 1 + 1) * 3 + 0] = if r > 0xff { 0xff } else { r as u8 };
    data[(width * 1 + 1) * 3 + 1] = if g > 0xff { 0xff } else { g as u8 };
    data[(width * 1 + 1) * 3 + 2] = if b > 0xff { 0xff } else { b as u8 };
    r = data[(width * 1 + 2) * 3 + 0] as i32 + (error_r * 3 / 48);
    g = data[(width * 1 + 2) * 3 + 1] as i32 + (error_g * 3 / 48);
    b = data[(width * 1 + 2) * 3 + 2] as i32 + (error_b * 3 / 48);
    data[(width * 1 + 2) * 3 + 0] = if r > 0xff { 0xff } else { r as u8 };
    data[(width * 1 + 2) * 3 + 1] = if g > 0xff { 0xff } else { g as u8 };
    data[(width * 1 + 2) * 3 + 2] = if b > 0xff { 0xff } else { b as u8 };
    r = data[(width * 2 - 2) * 3 + 0] as i32 + (error_r * 1 / 48);
    g = data[(width * 2 - 2) * 3 + 1] as i32 + (error_g * 1 / 48);
    b = data[(width * 2 - 2) * 3 + 2] as i32 + (error_b * 1 / 48);
    data[(width * 2 - 2) * 3 + 0] = if r > 0xff { 0xff } else { r as u8 };
    data[(width * 2 - 2) * 3 + 1] = if g > 0xff { 0xff } else { g as u8 };
    data[(width * 2 - 2) * 3 + 2] = if b > 0xff { 0xff } else { b as u8 };
    r = data[(width * 2 - 1) * 3 + 0] as i32 + (error_r * 3 / 48);
    g = data[(width * 2 - 1) * 3 + 1] as i32 + (error_g * 3 / 48);
    b = data[(width * 2 - 1) * 3 + 2] as i32 + (error_b * 3 / 48);
    data[(width * 2 - 1) * 3 + 0] = if r > 0xff { 0xff } else { r as u8 };
    data[(width * 2 - 1) * 3 + 1] = if g > 0xff { 0xff } else { g as u8 };
    data[(width * 2 - 1) * 3 + 2] = if b > 0xff { 0xff } else { b as u8 };
    r = data[(width * 2 + 0) * 3 + 0] as i32 + (error_r * 5 / 48);
    g = data[(width * 2 + 0) * 3 + 1] as i32 + (error_g * 5 / 48);
    b = data[(width * 2 + 0) * 3 + 2] as i32 + (error_b * 5 / 48);
    data[(width * 2 + 0) * 3 + 0] = if r > 0xff { 0xff } else { r as u8 };
    data[(width * 2 + 0) * 3 + 1] = if g > 0xff { 0xff } else { g as u8 };
    data[(width * 2 + 0) * 3 + 2] = if b > 0xff { 0xff } else { b as u8 };
    r = data[(width * 2 + 1) * 3 + 0] as i32 + (error_r * 3 / 48);
    g = data[(width * 2 + 1) * 3 + 1] as i32 + (error_g * 3 / 48);
    b = data[(width * 2 + 1) * 3 + 2] as i32 + (error_b * 3 / 48);
    data[(width * 2 + 1) * 3 + 0] = if r > 0xff { 0xff } else { r as u8 };
    data[(width * 2 + 1) * 3 + 1] = if g > 0xff { 0xff } else { g as u8 };
    data[(width * 2 + 1) * 3 + 2] = if b > 0xff { 0xff } else { b as u8 };
    r = data[(width * 2 + 2) * 3 + 0] as i32 + (error_r * 1 / 48);
    g = data[(width * 2 + 2) * 3 + 1] as i32 + (error_g * 1 / 48);
    b = data[(width * 2 + 2) * 3 + 2] as i32 + (error_b * 1 / 48);
    data[(width * 2 + 2) * 3 + 0] = if r > 0xff { 0xff } else { r as u8 };
    data[(width * 2 + 2) * 3 + 1] = if g > 0xff { 0xff } else { g as u8 };
    data[(width * 2 + 2) * 3 + 2] = if b > 0xff { 0xff } else { b as u8 };
}

/// Stucki's dithering
///
/// ```txt
///                  curr    8/48    4/48
///  2/48    4/48    8/48    4/48    2/48
///  1/48    2/48    4/48    2/48    1/48
/// ```
fn dither_stucki_15bpp(data: &mut [u8], width: i32) {
    let mut error_r = data[0] as i32 & 0x7;
    let mut error_g = data[1] as i32 & 0x7;
    let mut error_b = data[2] as i32 & 0x7;
    error_r += 4;
    error_g += 4;
    error_b += 4;
    let width = width as usize;

    let mut r = data[(width * 0 + 1) * 3 + 0] as i32 + (error_r * 8 / 48);
    let mut g = data[(width * 0 + 1) * 3 + 1] as i32 + (error_g * 8 / 48);
    let mut b = data[(width * 0 + 1) * 3 + 2] as i32 + (error_b * 8 / 48);
    data[(width * 0 + 1) * 3 + 0] = if r > 0xff { 0xff } else { r as u8 };
    data[(width * 0 + 1) * 3 + 1] = if g > 0xff { 0xff } else { g as u8 };
    data[(width * 0 + 1) * 3 + 2] = if b > 0xff { 0xff } else { b as u8 };
    r = data[(width * 0 + 2) * 3 + 0] as i32 + (error_r * 4 / 48);
    g = data[(width * 0 + 2) * 3 + 1] as i32 + (error_g * 4 / 48);
    b = data[(width * 0 + 2) * 3 + 2] as i32 + (error_b * 4 / 48);
    data[(width * 0 + 2) * 3 + 0] = if r > 0xff { 0xff } else { r as u8 };
    data[(width * 0 + 2) * 3 + 1] = if g > 0xff { 0xff } else { g as u8 };
    data[(width * 0 + 2) * 3 + 2] = if b > 0xff { 0xff } else { b as u8 };
    r = data[(width * 1 - 2) * 3 + 0] as i32 + (error_r * 2 / 48);
    g = data[(width * 1 - 2) * 3 + 1] as i32 + (error_g * 2 / 48);
    b = data[(width * 1 - 2) * 3 + 2] as i32 + (error_b * 2 / 48);
    data[(width * 1 - 2) * 3 + 0] = if r > 0xff { 0xff } else { r as u8 };
    data[(width * 1 - 2) * 3 + 1] = if g > 0xff { 0xff } else { g as u8 };
    data[(width * 1 - 2) * 3 + 2] = if b > 0xff { 0xff } else { b as u8 };
    r = data[(width * 1 - 1) * 3 + 0] as i32 + (error_r * 4 / 48);
    g = data[(width * 1 - 1) * 3 + 1] as i32 + (error_g * 4 / 48);
    b = data[(width * 1 - 1) * 3 + 2] as i32 + (error_b * 4 / 48);
    data[(width * 1 - 1) * 3 + 0] = if r > 0xff { 0xff } else { r as u8 };
    data[(width * 1 - 1) * 3 + 1] = if g > 0xff { 0xff } else { g as u8 };
    data[(width * 1 - 1) * 3 + 2] = if b > 0xff { 0xff } else { b as u8 };
    r = data[(width * 1 + 0) * 3 + 0] as i32 + (error_r * 8 / 48);
    g = data[(width * 1 + 0) * 3 + 1] as i32 + (error_g * 8 / 48);
    b = data[(width * 1 + 0) * 3 + 2] as i32 + (error_b * 8 / 48);
    data[(width * 1 + 0) * 3 + 0] = if r > 0xff { 0xff } else { r as u8 };
    data[(width * 1 + 0) * 3 + 1] = if g > 0xff { 0xff } else { g as u8 };
    data[(width * 1 + 0) * 3 + 2] = if b > 0xff { 0xff } else { b as u8 };
    r = data[(width * 1 + 1) * 3 + 0] as i32 + (error_r * 4 / 48);
    g = data[(width * 1 + 1) * 3 + 1] as i32 + (error_g * 4 / 48);
    b = data[(width * 1 + 1) * 3 + 2] as i32 + (error_b * 4 / 48);
    data[(width * 1 + 1) * 3 + 0] = if r > 0xff { 0xff } else { r as u8 };
    data[(width * 1 + 1) * 3 + 1] = if g > 0xff { 0xff } else { g as u8 };
    data[(width * 1 + 1) * 3 + 2] = if b > 0xff { 0xff } else { b as u8 };
    r = data[(width * 1 + 2) * 3 + 0] as i32 + (error_r * 2 / 48);
    g = data[(width * 1 + 2) * 3 + 1] as i32 + (error_g * 2 / 48);
    b = data[(width * 1 + 2) * 3 + 2] as i32 + (error_b * 2 / 48);
    data[(width * 1 + 2) * 3 + 0] = if r > 0xff { 0xff } else { r as u8 };
    data[(width * 1 + 2) * 3 + 1] = if g > 0xff { 0xff } else { g as u8 };
    data[(width * 1 + 2) * 3 + 2] = if b > 0xff { 0xff } else { b as u8 };
    r = data[(width * 2 - 2) * 3 + 0] as i32 + (error_r * 1 / 48);
    g = data[(width * 2 - 2) * 3 + 1] as i32 + (error_g * 1 / 48);
    b = data[(width * 2 - 2) * 3 + 2] as i32 + (error_b * 1 / 48);
    data[(width * 2 - 2) * 3 + 0] = if r > 0xff { 0xff } else { r as u8 };
    data[(width * 2 - 2) * 3 + 1] = if g > 0xff { 0xff } else { g as u8 };
    data[(width * 2 - 2) * 3 + 2] = if b > 0xff { 0xff } else { b as u8 };
    r = data[(width * 2 - 1) * 3 + 0] as i32 + (error_r * 2 / 48);
    g = data[(width * 2 - 1) * 3 + 1] as i32 + (error_g * 2 / 48);
    b = data[(width * 2 - 1) * 3 + 2] as i32 + (error_b * 2 / 48);
    data[(width * 2 - 1) * 3 + 0] = if r > 0xff { 0xff } else { r as u8 };
    data[(width * 2 - 1) * 3 + 1] = if g > 0xff { 0xff } else { g as u8 };
    data[(width * 2 - 1) * 3 + 2] = if b > 0xff { 0xff } else { b as u8 };
    r = data[(width * 2 + 0) * 3 + 0] as i32 + (error_r * 4 / 48);
    g = data[(width * 2 + 0) * 3 + 1] as i32 + (error_g * 4 / 48);
    b = data[(width * 2 + 0) * 3 + 2] as i32 + (error_b * 4 / 48);
    data[(width * 2 + 0) * 3 + 0] = if r > 0xff { 0xff } else { r as u8 };
    data[(width * 2 + 0) * 3 + 1] = if g > 0xff { 0xff } else { g as u8 };
    data[(width * 2 + 0) * 3 + 2] = if b > 0xff { 0xff } else { b as u8 };
    r = data[(width * 2 + 1) * 3 + 0] as i32 + (error_r * 2 / 48);
    g = data[(width * 2 + 1) * 3 + 1] as i32 + (error_g * 2 / 48);
    b = data[(width * 2 + 1) * 3 + 2] as i32 + (error_b * 2 / 48);
    data[(width * 2 + 1) * 3 + 0] = if r > 0xff { 0xff } else { r as u8 };
    data[(width * 2 + 1) * 3 + 1] = if g > 0xff { 0xff } else { g as u8 };
    data[(width * 2 + 1) * 3 + 2] = if b > 0xff { 0xff } else { b as u8 };
    r = data[(width * 2 + 2) * 3 + 0] as i32 + (error_r * 1 / 48);
    g = data[(width * 2 + 2) * 3 + 1] as i32 + (error_g * 1 / 48);
    b = data[(width * 2 + 2) * 3 + 2] as i32 + (error_b * 1 / 48);
    data[(width * 2 + 2) * 3 + 0] = if r > 0xff { 0xff } else { r as u8 };
    data[(width * 2 + 2) * 3 + 1] = if g > 0xff { 0xff } else { g as u8 };
    data[(width * 2 + 2) * 3 + 2] = if b > 0xff { 0xff } else { b as u8 };
}

/// Burkes' Method
///
/// ```txt
///                  curr    4/16    2/16
///  1/16    2/16    4/16    2/16    1/16
/// ```
fn dither_burkes_15bpp(data: &mut [u8], width: i32) {
    let mut error_r = data[0] as i32 & 0x7;
    let mut error_g = data[1] as i32 & 0x7;
    let mut error_b = data[2] as i32 & 0x7;
    error_r += 2;
    error_g += 2;
    error_b += 2;
    let width = width as usize;

    let mut r = data[(width * 0 + 1) * 3 + 0] as i32 + (error_r * 4 / 16);
    let mut g = data[(width * 0 + 1) * 3 + 1] as i32 + (error_g * 4 / 16);
    let mut b = data[(width * 0 + 1) * 3 + 2] as i32 + (error_b * 4 / 16);
    data[(width * 0 + 1) * 3 + 0] = if r > 0xff { 0xff } else { r as u8 };
    data[(width * 0 + 1) * 3 + 1] = if g > 0xff { 0xff } else { g as u8 };
    data[(width * 0 + 1) * 3 + 2] = if b > 0xff { 0xff } else { b as u8 };
    r = data[(width * 0 + 2) * 3 + 0] as i32 + (error_r * 2 / 16);
    g = data[(width * 0 + 2) * 3 + 1] as i32 + (error_g * 2 / 16);
    b = data[(width * 0 + 2) * 3 + 2] as i32 + (error_b * 2 / 16);
    data[(width * 0 + 2) * 3 + 0] = if r > 0xff { 0xff } else { r as u8 };
    data[(width * 0 + 2) * 3 + 1] = if g > 0xff { 0xff } else { g as u8 };
    data[(width * 0 + 2) * 3 + 2] = if b > 0xff { 0xff } else { b as u8 };
    r = data[(width * 1 - 2) * 3 + 0] as i32 + (error_r * 1 / 16);
    g = data[(width * 1 - 2) * 3 + 1] as i32 + (error_g * 1 / 16);
    b = data[(width * 1 - 2) * 3 + 2] as i32 + (error_b * 1 / 16);
    data[(width * 1 - 2) * 3 + 0] = if r > 0xff { 0xff } else { r as u8 };
    data[(width * 1 - 2) * 3 + 1] = if g > 0xff { 0xff } else { g as u8 };
    data[(width * 1 - 2) * 3 + 2] = if b > 0xff { 0xff } else { b as u8 };
    r = data[(width * 1 - 1) * 3 + 0] as i32 + (error_r * 2 / 16);
    g = data[(width * 1 - 1) * 3 + 1] as i32 + (error_g * 2 / 16);
    b = data[(width * 1 - 1) * 3 + 2] as i32 + (error_b * 2 / 16);
    data[(width * 1 - 1) * 3 + 0] = if r > 0xff { 0xff } else { r as u8 };
    data[(width * 1 - 1) * 3 + 1] = if g > 0xff { 0xff } else { g as u8 };
    data[(width * 1 - 1) * 3 + 2] = if b > 0xff { 0xff } else { b as u8 };
    r = data[(width * 1 + 0) * 3 + 0] as i32 + (error_r * 4 / 16);
    g = data[(width * 1 + 0) * 3 + 1] as i32 + (error_g * 4 / 16);
    b = data[(width * 1 + 0) * 3 + 2] as i32 + (error_b * 4 / 16);
    data[(width * 1 + 0) * 3 + 0] = if r > 0xff { 0xff } else { r as u8 };
    data[(width * 1 + 0) * 3 + 1] = if g > 0xff { 0xff } else { g as u8 };
    data[(width * 1 + 0) * 3 + 2] = if b > 0xff { 0xff } else { b as u8 };
    r = data[(width * 1 + 1) * 3 + 0] as i32 + (error_r * 2 / 16);
    g = data[(width * 1 + 1) * 3 + 1] as i32 + (error_g * 2 / 16);
    b = data[(width * 1 + 1) * 3 + 2] as i32 + (error_b * 2 / 16);
    data[(width * 1 + 1) * 3 + 0] = if r > 0xff { 0xff } else { r as u8 };
    data[(width * 1 + 1) * 3 + 1] = if g > 0xff { 0xff } else { g as u8 };
    data[(width * 1 + 1) * 3 + 2] = if b > 0xff { 0xff } else { b as u8 };
    r = data[(width * 1 + 2) * 3 + 0] as i32 + (error_r * 1 / 16);
    g = data[(width * 1 + 2) * 3 + 1] as i32 + (error_g * 1 / 16);
    b = data[(width * 1 + 2) * 3 + 2] as i32 + (error_b * 1 / 16);
    data[(width * 1 + 2) * 3 + 0] = if r > 0xff { 0xff } else { r as u8 };
    data[(width * 1 + 2) * 3 + 1] = if g > 0xff { 0xff } else { g as u8 };
    data[(width * 1 + 2) * 3 + 2] = if b > 0xff { 0xff } else { b as u8 };
}

/// Applies an arithmetic dithering effect to a pixel.
///
/// - Generates structured noise using a multiplicative mask.
/// - Does not propagate error to neighbors.
/// - Produces a positionally stable dithering effect.
fn dither_a_dither_15bpp(data: &mut [u8], _width: i32, x: i32, y: i32) {
    for c in 0..3 {
        let mask = (((x + c * 17) + y * 236) * 119) & 255;
        let mask = (mask - 128) / 256;
        let value = data[c as usize] as i32 + mask;
        data[c as usize] = value.clamp(0, 255) as u8;
    }
}

/// Applies an XOR-based dithering effect to a pixel.
///
/// - Uses an XOR operation for generating noise.
/// - Produces a high-frequency, structured dithering pattern.
/// - Positionally stable, without error diffusion.
fn dither_x_dither_15bpp(data: &mut [u8], _width: i32, x: i32, y: i32) {
    for c in 0..3 {
        let mask = ((((x + c * 17) ^ y) * 236) * 1234) & 511;
        let mask = (mask - 128) / 512;
        let value = data[c as usize] as i32 + mask;
        data[c as usize] = value.clamp(0, 255) as u8;
    }
}
