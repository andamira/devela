// devela::media::image::sixel::output::pixel_format
//
// TOC
// - LegacySixelPixelFormat
//   - bits_per_pixel
//   - bytes_per_pixel
//   - required_bytes
//   - normalize
// - get_rgb
// - sixel_helper_compute_depth
// - expand_rgb
// - expand_palette

#![allow(clippy::identity_op, reason = "symmetry")]

use crate::{ConstInit, LegacySixelError, LegacySixelResult};

/// Pixel format type of input image.
///
/// # Adaptation
/// Derived from `pixelFormat` enum in the `libsixel` C library.
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub enum LegacySixelPixelFormat {
    /// RGB color 15bpp.
    RGB555 = 1,
    /// RGB color 16bpp.
    RGB565 = 2,
    /// RGB color 24bpp. (Default)
    #[default]
    RGB888 = 3, // for compatibility, the value must be 3.
    /// BGR color 15bpp.
    BGR555 = 4,
    /// BGR color 16bpp.
    BGR565 = 5,
    /// BGR color 24bpp.
    BGR888 = 6,
    /// ARGB color 32bpp.
    ARGB8888 = 0x10,
    /// RGBA color 32bpp.
    RGBA8888 = 0x11,
    /// ABGR color 32bpp.
    ABGR8888 = 0x12,
    /// BGRA color 32bpp.
    BGRA8888 = 0x13,
    /// Grayscale 1bpp.
    G1 = (1 << 6),
    /// Grayscale 2bpp.
    G2 = (1 << 6) | 0x01,
    /// Grayscale 4bpp.
    G4 = (1 << 6) | 0x02,
    /// Grayscale 8bpp.
    G8 = (1 << 6) | 0x03,
    /// AG grayscale 16bpp.
    AG88 = (1 << 6) | 0x13,
    /// GA grayscale 16bpp.
    GA88 = (1 << 6) | 0x23,
    /// Palette 1bpp.
    PAL1 = (1 << 7),
    /// Palette 2bpp.
    PAL2 = (1 << 7) | 0x01,
    /// Palette 4bpp.
    PAL4 = (1 << 7) | 0x02,
    /// Palette 8bpp.
    PAL8 = (1 << 7) | 0x03,
}
#[rustfmt::skip]
impl ConstInit for LegacySixelPixelFormat { const INIT: Self = Self::RGB888; }

impl LegacySixelPixelFormat {
    /// Returns the bits per pixel of the current format.
    #[rustfmt::skip]
    pub const fn bits_per_pixel(self) -> usize {
        match self {
            LegacySixelPixelFormat::RGB555
            | LegacySixelPixelFormat::BGR555 => 15,
            LegacySixelPixelFormat::RGB565
            | LegacySixelPixelFormat::BGR565
            | LegacySixelPixelFormat::AG88
            | LegacySixelPixelFormat::GA88 => 16,
            LegacySixelPixelFormat::RGB888
            | LegacySixelPixelFormat::BGR888
            | LegacySixelPixelFormat::G8
            | LegacySixelPixelFormat::PAL8 => 24,
            LegacySixelPixelFormat::ARGB8888
            | LegacySixelPixelFormat::RGBA8888
            | LegacySixelPixelFormat::ABGR8888
            | LegacySixelPixelFormat::BGRA8888 => 32,
            LegacySixelPixelFormat::G1 | LegacySixelPixelFormat::PAL1 => 1,
            LegacySixelPixelFormat::G2 | LegacySixelPixelFormat::PAL2 => 2,
            LegacySixelPixelFormat::G4 | LegacySixelPixelFormat::PAL4 => 4,
        }
    }

    /// Returns the bytes per pixel of the current format.
    pub const fn bytes_per_pixel(self) -> usize {
        match self {
            LegacySixelPixelFormat::ARGB8888
            | LegacySixelPixelFormat::RGBA8888
            | LegacySixelPixelFormat::ABGR8888
            | LegacySixelPixelFormat::BGRA8888 => 4,
            LegacySixelPixelFormat::RGB888 | LegacySixelPixelFormat::BGR888 => 3,
            LegacySixelPixelFormat::RGB555
            | LegacySixelPixelFormat::RGB565
            | LegacySixelPixelFormat::BGR555
            | LegacySixelPixelFormat::BGR565
            | LegacySixelPixelFormat::AG88
            | LegacySixelPixelFormat::GA88 => 2,
            LegacySixelPixelFormat::G1
            | LegacySixelPixelFormat::G2
            | LegacySixelPixelFormat::G4
            | LegacySixelPixelFormat::G8
            | LegacySixelPixelFormat::PAL1
            | LegacySixelPixelFormat::PAL2
            | LegacySixelPixelFormat::PAL4
            | LegacySixelPixelFormat::PAL8 => 1,
        }
    }

    /// Returns the number of bytes required to store an image of the given dimensions,
    /// using the current pixel format.
    pub const fn required_bytes(self, width: i32, height: i32) -> usize {
        let total_bits = width as usize * height as usize * self.bits_per_pixel();
        crate::Mem::bytes_from_bits(total_bits)
    }

    /// returns dst_pixelformat: LegacySixelPixelFormat,
    pub(crate) fn normalize(
        self,
        dst: &mut [u8],
        src: &[u8],
        width: i32,
        height: i32,
    ) -> LegacySixelResult<LegacySixelPixelFormat> /* height of source image */ {
        match self {
            LegacySixelPixelFormat::G8 => {
                expand_rgb(dst, src, width, height, self, 1);
                Ok(LegacySixelPixelFormat::RGB888)
            }

            LegacySixelPixelFormat::RGB565
            | LegacySixelPixelFormat::RGB555
            | LegacySixelPixelFormat::BGR565
            | LegacySixelPixelFormat::BGR555
            | LegacySixelPixelFormat::GA88
            | LegacySixelPixelFormat::AG88 => {
                expand_rgb(dst, src, width, height, self, 2);
                Ok(LegacySixelPixelFormat::RGB888)
            }

            LegacySixelPixelFormat::RGB888 | LegacySixelPixelFormat::BGR888 => {
                expand_rgb(dst, src, width, height, self, 3);
                Ok(LegacySixelPixelFormat::RGB888)
            }

            LegacySixelPixelFormat::RGBA8888
            | LegacySixelPixelFormat::ARGB8888
            | LegacySixelPixelFormat::BGRA8888
            | LegacySixelPixelFormat::ABGR8888 => {
                expand_rgb(dst, src, width, height, self, 4);
                Ok(LegacySixelPixelFormat::RGB888)
            }

            LegacySixelPixelFormat::PAL1
            | LegacySixelPixelFormat::PAL2
            | LegacySixelPixelFormat::PAL4 => {
                expand_palette(dst, src, width, height, self)?;
                Ok(LegacySixelPixelFormat::PAL8)
            }

            LegacySixelPixelFormat::G1
            | LegacySixelPixelFormat::G2
            | LegacySixelPixelFormat::G4 => {
                expand_palette(dst, src, width, height, self)?;
                Ok(LegacySixelPixelFormat::G8)
            }
            LegacySixelPixelFormat::PAL8 => {
                dst[..((width * height) as usize)]
                    .copy_from_slice(&src[..((width * height) as usize)]);
                Ok(self)
            }
        }
    }
}

/// TODO
#[allow(clippy::identity_op, reason = "symmetry")]
fn get_rgb(data: &[u8], pixelformat: LegacySixelPixelFormat, depth: usize) -> (u8, u8, u8) {
    let mut count = 0;
    let mut pixels: u32 = 0;
    while count < depth {
        pixels = data[count] as u32 | (pixels << 8);
        count += 1;
    }
    /*
        /* TODO: we should swap bytes (only necessary on LSByte first hardware?) */
    #if SWAP_BYTES
        if (depth == 2) {
            low    = pixels & 0xff;
            high   = (pixels >> 8) & 0xff;
            pixels = (low << 8) | high;
        }
    #endif*/
    let (r, g, b) = match pixelformat {
        LegacySixelPixelFormat::RGB555 => {
            (((pixels >> 10) & 0x1f) << 3, ((pixels >> 5) & 0x1f) << 3, ((pixels >> 0) & 0x1f) << 3)
        }
        LegacySixelPixelFormat::RGB565 => {
            (((pixels >> 11) & 0x1f) << 3, ((pixels >> 5) & 0x3f) << 2, ((pixels >> 0) & 0x1f) << 3)
        }
        LegacySixelPixelFormat::RGB888 => {
            ((pixels >> 16) & 0xff, (pixels >> 8) & 0xff, (pixels >> 0) & 0xff)
        }
        LegacySixelPixelFormat::BGR555 => {
            (((pixels >> 0) & 0x1f) << 3, ((pixels >> 5) & 0x1f) << 3, ((pixels >> 10) & 0x1f) << 3)
        }
        LegacySixelPixelFormat::BGR565 => {
            (((pixels >> 0) & 0x1f) << 3, ((pixels >> 5) & 0x3f) << 2, ((pixels >> 11) & 0x1f) << 3)
        }
        LegacySixelPixelFormat::BGR888 => {
            ((pixels >> 0) & 0xff, (pixels >> 8) & 0xff, (pixels >> 16) & 0xff)
        }
        LegacySixelPixelFormat::ARGB8888 => {
            ((pixels >> 16) & 0xff, (pixels >> 8) & 0xff, (pixels >> 0) & 0xff)
        }
        LegacySixelPixelFormat::RGBA8888 => {
            ((pixels >> 24) & 0xff, (pixels >> 16) & 0xff, (pixels >> 8) & 0xff)
        }
        LegacySixelPixelFormat::ABGR8888 => {
            ((pixels >> 0) & 0xff, (pixels >> 8) & 0xff, (pixels >> 16) & 0xff)
        }
        LegacySixelPixelFormat::BGRA8888 => {
            ((pixels >> 8) & 0xff, (pixels >> 16) & 0xff, (pixels >> 24) & 0xff)
        }
        LegacySixelPixelFormat::G8 | LegacySixelPixelFormat::AG88 => {
            (pixels & 0xff, pixels & 0xff, pixels & 0xff)
        }
        LegacySixelPixelFormat::GA88 => {
            ((pixels >> 8) & 0xff, (pixels >> 8) & 0xff, (pixels >> 8) & 0xff)
        }
        _ => (0, 0, 0),
    };
    (r as u8, g as u8, b as u8)
}

/// TODO
fn expand_rgb(
    dst: &mut [u8],
    src: &[u8],
    width: i32,
    height: i32,
    pixelformat: LegacySixelPixelFormat,
    depth: usize,
) {
    for y in 0..height {
        for x in 0..width {
            let src_offset = depth * (y * width + x) as usize;
            let dst_offset: usize = 3 * (y * width + x) as usize;
            let (r, g, b) = get_rgb(&src[src_offset..], pixelformat, depth);

            dst[dst_offset + 0] = r;
            dst[dst_offset + 1] = g;
            dst[dst_offset + 2] = b;
        }
    }
}

/// TODO
fn expand_palette(
    dst: &mut [u8],
    src: &[u8],
    width: i32,
    height: i32,
    pixelformat: LegacySixelPixelFormat,
) -> LegacySixelResult<()> {
    let bpp = match pixelformat {
        LegacySixelPixelFormat::PAL1 | LegacySixelPixelFormat::G1 => 1,

        LegacySixelPixelFormat::PAL2 | LegacySixelPixelFormat::G2 => 2,

        LegacySixelPixelFormat::PAL4 | LegacySixelPixelFormat::G4 => 4,

        LegacySixelPixelFormat::PAL8 | LegacySixelPixelFormat::G8 => {
            dst[..((width * height) as usize)].copy_from_slice(&src[..((width * height) as usize)]);
            return Ok(());
        }

        //          sixel_helper_set_additional_message(    "expand_palette: invalid pixelformat.");
        _ => return Err(LegacySixelError::BadArgument),
    };
    let mut dst_offset = 0;
    let mut src_offset = 0;

    let max_x = width * bpp / 8;
    for _y in 0..height {
        for _x in 0..max_x {
            for i in 0..8 / bpp {
                let shift = ((8 / bpp) - 1 - i) * (bpp & (1 << (bpp - 1)));
                dst[dst_offset] = ((src[src_offset] as i32) >> shift) as u8;
                dst_offset += 1;
            }
            src_offset += 1;
        }

        let x = width - max_x * 8 / bpp;
        if x > 0 {
            for i in 0..x {
                dst[dst_offset] = src[src_offset] >> ((8 - (i + 1) * bpp) & ((1 << bpp) - 1));
                dst_offset += 1;
            }
            src_offset += 1;
        }
    }
    Ok(())
}
