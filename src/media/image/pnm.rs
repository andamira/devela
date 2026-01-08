// devela::media::image::pnm
//
//! PNM is the portable anymap format Netpbm image format family (PBM, PGM, PPM)

#![allow(unused, reason = "WIP")]

#[cfg(doc)]
use crate::ImageError::FmtError;
#[cfg(feature = "alloc")]
use crate::text::String;
use crate::{
    FmtWrite, ImageError,
    ImageError::{InvalidImageSize, InvalidPixel},
    ImageResult as Result, Mem,
};

// Helper function: Returns `InvalidPixel` as the cold path.
#[cold] #[rustfmt::skip]
const fn invalid_pixel<T>() -> crate::Result<T, ImageError> { Err(InvalidPixel) }

#[doc = crate::_tags!(image namespace)]
/// A collection of methods for encoding and decoding
/// <abbr title="Portable anymap format">PNM</abbr> bitmap formats.
#[doc = crate::_doc_location!("media/image")]
///
/// - <https://en.wikipedia.org/wiki/Netpbm>
///
/// The PBM (Portable Bitmap), PGM (Portable Graymap), and PPM (Portable Pixmap)
/// formats are part of the Netpbm format family. These formats are simple and
/// straightforward for representing grayscale (PGM), black and white (PBM), and
/// color (PPM) images in either ASCII or binary modes. Here's a brief overview:
///
/// - PBM (Portable Bitmap Format): Used for black and white images. It supports
/// both ASCII (P1) and binary (P4) representations.
///
/// - PGM (Portable Graymap Format): Used for grayscale images. It also supports
/// ASCII (P2) and binary (P5) representations, with the gray value typically
/// ranging from 0 (black) to 255 (white) for 8-bit images.
///
/// - PPM (Portable Pixmap Format): Used for color images. Similar to PGM,
/// it supports ASCII (P3) and binary (P6) formats. Each pixel is represented by
/// three values (red, green, and blue), each in the range of 0 to 255 for 8-bit images.
#[derive(Debug)]
pub struct Pnm;

impl Pnm {
    /// Converts a `bitmap` of 1-bit bytes into PBM ASCII P1 representation.
    ///
    /// Expects a slice of width * height bytes equal to `0` or `1`.
    ///
    /// # Errors
    /// Returns [`InvalidImageSize`] if the `bitmap` slice doesn't contain exactly
    /// `width` * `height` elements, [`InvalidPixel`] if any pixel value is invalid
    /// and [`FmtError`] if the string writing fails.
    #[cfg(feature = "alloc")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
    pub fn p1_encode_bytes(bitmap: &[u8], width: usize, height: usize) -> Result<String> {
        if bitmap.len() != width * height {
            return Err(InvalidImageSize(Some((width, height))));
        }
        let mut result = String::new();
        writeln!(result, "P1\n{width} {height}")?;

        // Convert each byte in `bitmap` to '0' (white) or '1' (black) ASCII
        for row in 0..height {
            let first_pixel = bitmap[row * width];
            result.push(match first_pixel {
                0 => '0',
                1 => '1',
                _ => return invalid_pixel(),
            });

            for col in 1..width {
                let pixel = bitmap[row * width + col];
                result.push(' '); // leading space on non-first-in-row pixels
                match pixel {
                    0 => result.push('0'),
                    1 => result.push('1'),
                    _ => return invalid_pixel(),
                }
            }
            result.push('\n'); // End of row
        }
        Ok(result)
    }

    /// Converts a `bitmap` of 1-bit bits into PBM ASCII P1 representation.
    ///
    /// Each byte in `bitmap` represents 8 pixels, with the most significant bit (MSB)
    /// of each byte representing the leftmost pixel.
    ///
    /// # Errors
    /// Returns [`InvalidImageSize`] if the `bitmap` slice doesn't contain exactly
    /// the number of expected bytes `width` * `height` elements
    /// and [`FmtError`] if the string writing fails.
    #[cfg(feature = "alloc")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
    pub fn p1_encode_bits(bitmap: &[u8], width: usize, height: usize) -> Result<String> {
        if bitmap.len() != Mem::bytes_from_bits(width * height) {
            return Err(InvalidImageSize(Some((width, height))));
        }
        let mut result = String::new();
        writeln!(result, "P1\n{width} {height}")?;

        // Convert each bit in `bitmap` to '0' (white) or '1' (black) ASCII
        for row in 0..height {
            let first_col = 0;
            let byte_index = (row * width + first_col) / 8;
            let bit = 1 << (7 - (first_col % 8));
            let first_pixel = (bitmap[byte_index] & bit) != 0;
            result.push(if first_pixel { '1' } else { '0' });

            for col in 1..width {
                let byte_index = (row * width + col) / 8;
                let bit = 1 << (7 - (col % 8));
                let pixel = (bitmap[byte_index] & bit) != 0;
                result.push(' '); // leading space on non-first-in-row pixels
                result.push(if pixel { '1' } else { '0' });
            }
            result.push('\n'); // end of row
        }
        Ok(result)
    }
}

#[cfg(all(test, feature = "alloc"))]
mod tests_alloc {
    use super::*;
    use crate::_dep::_alloc::vec;

    #[test]
    fn p1_encode_bytes() {
        let bitmap = vec![
            1, 0, 1, 0, 1, 0, 1, 0, // First row
            0, 1, 0, 1, 0, 1, 0, 1, // Second row
        ];
        let (w, h) = (8, 2);
        let expected_output = "P1\n8 2\n1 0 1 0 1 0 1 0\n0 1 0 1 0 1 0 1\n";
        let result = Pnm::p1_encode_bytes(&bitmap, w, h).expect("PNM P1 encoded");
        assert_eq!(result, expected_output);
    }
    #[test]
    fn p1_encode_bytes_invalid_size() {
        let bitmap = vec![1, 0, 1, 0]; // Incorrect size for 2x2 image
        let (w, h) = (3, 3);
        let result = Pnm::p1_encode_bytes(&bitmap, w, h);
        assert_eq!(result, Err(InvalidImageSize(Some((3, 3)))));
    }
    #[test]
    fn p1_encode_bytes_invalid_pixel() {
        let bitmap = vec![1, 0, 2, 0]; // Invalid pixel value (2)
        let (w, h) = (2, 2);
        let result = Pnm::p1_encode_bytes(&bitmap, w, h);
        assert_eq!(result, Err(InvalidPixel));
    }

    #[test]
    fn p1_encode_bits() {
        let bitmap = vec![0b10101010, 0b01010101]; // Packed bits for 8x2 image
        let (w, h) = (8, 2);
        let expected_output = "P1\n8 2\n1 0 1 0 1 0 1 0\n0 1 0 1 0 1 0 1\n";
        let result = Pnm::p1_encode_bits(&bitmap, w, h).expect("PNM P1 encoded");
        assert_eq!(result, expected_output);
    }
    #[test]
    fn p1_encode_bits_invalid_size() {
        let bitmap = vec![0b10101010]; // Incorrect size for 8x2 image
        let (w, h) = (8, 2);
        let result = Pnm::p1_encode_bits(&bitmap, w, h);
        assert_eq!(result, Err(InvalidImageSize(Some((8, 2)))));
    }
}
