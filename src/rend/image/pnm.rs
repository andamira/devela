// devela::rend::image::pnm
//
//! PNM is the portable anymap format Netpbm image format family (PBM, PGM, PPM)
//

// The PBM (Portable Bitmap), PGM (Portable Graymap), and PPM (Portable Pixmap)
// formats are part of the Netpbm format family. These formats are simple and
// straightforward for representing grayscale (PGM), black and white (PBM), and
// color (PPM) images in either ASCII or binary modes. Here's a brief overview:
//
// - PBM (Portable Bitmap Format): Used for black and white images. It supports
// both ASCII (P1) and binary (P4) representations.
//
// - PGM (Portable Graymap Format): Used for grayscale images. It also supports
// ASCII (P2) and binary (P5) representations, with the gray value typically
// ranging from 0 (black) to 255 (white) for 8-bit images.
//
// - PPM (Portable Pixmap Format): Used for color images. Similar to PGM,
// it supports ASCII (P3) and binary (P6) formats. Each pixel is represented by
// three values (red, green, and blue), each in the range of 0 to 255 for 8-bit images.

// use crate::rend::color:: ;
use crate::rend::{RendError as E, RendResult as Result};

#[cfg(feature = "alloc")]
use crate::_liballoc::string::String;
use crate::code::Mismatch;
use core::fmt::{self, Write};

/// A collection of methods for encoding and decoding
/// <abbr title="Portable anymap format">PNM</abbr> bitmap formats.
///
/// - <https://en.wikipedia.org/wiki/Netpbm>
pub struct Pnm;

impl Pnm {
    /// Converts a `bitmap` of 1-bit bytes into PBM ASCII P1 representation.
    ///
    /// Expects a slice of width * height bytes equal to `0` or `1`.
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
    pub fn p1_encode_bytes(bitmap: &[u8], width: usize, height: usize) -> Result<String> {
        if bitmap.len() != width * height {
            return Err(E::InvalidImageSize(Some((width, height))));
        }
        let mut result = String::new();
        writeln!(result, "P1\n{} {}", width, height)?;

        // Convert each byte in the bitmap to '0' (white) or '1' (black) ASCII
        for (i, &pixel) in bitmap.iter().enumerate() {
            // Add a space before each pixel except the first in a row.
            if i % width != 0 {
                result.push(' ');
            }
            match pixel {
                0 => result.push('0'),
                1 => result.push('1'),
                _ => return Err(E::InvalidPixel),
            }
            // Add a newline at the end of each row.
            if (i + 1) % width == 0 {
                result.push('\n');
            }
        }
        Ok(result)
    }

    /// Converts a `bitmap` of 1-bit bits into PBM ASCII P1 representation.
    ///
    /// Each byte in `bitmap` represents 8 pixels, with the most significant bit (MSB)
    /// of each byte representing the leftmost pixel.
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
    pub fn p1_encode_bits(bitmap: &[u8], width: usize, height: usize) -> Result<String> {
        // Calculate the expected number of bytes in the bitmap
        let expected_bytes = (width * height + 7) / 8;
        if bitmap.len() != expected_bytes {
            return Err(E::InvalidImageSize(Some((width, height))));
        }

        let mut result = String::new();
        writeln!(result, "P1\n{} {}", width, height)?;

        for row in 0..height {
            for col in 0..width {
                let byte_index = (row * width + col) / 8;
                let bit_index = 7 - (col % 8); // Calculate bit position within the byte
                let pixel = (bitmap[byte_index] >> bit_index) & 1;

                // Write pixel value
                if col % width != 0 {
                    result.push(' ');
                }
                result.push(if pixel == 1 { '1' } else { '0' });
            }
            result.push('\n'); // End of row
        }
        Ok(result)
    }
}
