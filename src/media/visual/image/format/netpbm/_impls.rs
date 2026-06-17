// devela/src/media/visual/image/format/netpbm/_impls.rs
//
//! `Pnm` private methods.
//

use super::{PnmCursor, PnmFormat, PnmHeader};
use crate::ImageError::{InvalidImageSize, InvalidPixel};
use crate::{ImageResult, Mem, Pnm, is, unwrap, whilst};

// Private PNM parsing, decoding and encoded-length helpers.
#[rustfmt::skip]
impl Pnm {
    /// Reads and normalizes a PNM header.
    ///
    /// Supports P1..P6. For P1/P4, `max_value` is normalized to `1`.
    ///
    /// Binary formats P4/P5/P6 use a strict single whitespace byte
    /// between the header and raster data.
    pub(crate) const fn read_header(bytes: &[u8]) -> ImageResult<PnmHeader> {
        let mut cursor = PnmCursor::reader(bytes);
        let magic = unwrap![ok? cursor.magic()];
        let format = unwrap![ok? PnmFormat::from_magic(magic)];
        let (width, height) = (unwrap![ok? cursor.number()], unwrap![ok? cursor.number()]);
        is! { width == 0 || height == 0, return Err(InvalidImageSize(Some((width, height)))) }
        let max_value = is![format.is_bitmap(), 1, unwrap![ok? cursor.number()]];
        if max_value == 0 || max_value > u16::MAX as usize {
            return Err(InvalidPixel);
        }
        let data_offset =
            is![format.is_ascii(), cursor.pos(), unwrap![ok? cursor.raw_data_offset()]];
        Ok(PnmHeader { format, width, height, max_value: max_value as u16, data_offset })
    }
    /// Decodes ASCII P1/P2/P3 raster samples into unpacked `u8` output.
    pub(crate) const fn decode_ascii_u8(bytes: &[u8], header: PnmHeader, out: &mut [u8])
        -> ImageResult<()> {
        let mut cursor = PnmCursor::reader_at(bytes, header.data_offset);
        whilst![sample in &mut out; {
            let value = unwrap![ok? cursor.number()];
            if value > header.max_value as usize { return Err(InvalidPixel); }
            if header.format.is_bitmap() && value > 1 { return Err(InvalidPixel); }
            *sample = value as u8;
        }];
        Ok(())
    }
    /// Decodes raw P5/P6 raster samples into `u8` output.
    pub(crate) const fn decode_raw_u8(bytes: &[u8], header: PnmHeader, out: &mut [u8])
        -> ImageResult<()> {
        let end = unwrap![some_ok_or?
            header.data_offset.checked_add(out.len()), InvalidImageSize(None)];
        if bytes.len() < end {
            return Err(InvalidImageSize(Some((header.width, header.height))));
        }
        whilst![i, sample in &mut out; {
            let value = bytes[header.data_offset + i];
            if value as u16 > header.max_value { return Err(InvalidPixel); }
            *sample = value;
        }];
        Ok(())
    }
    /// Unpacks raw PBM P4 bits into one `u8` sample per pixel.
    pub(crate) const fn decode_p4_u8(bytes: &[u8], header: PnmHeader, out: &mut [u8])
        -> ImageResult<()> {
        let packed_len = unwrap![ok? header.packed_bitmap_len()];
        let end = unwrap![some_ok_or?
            header.data_offset.checked_add(packed_len), InvalidImageSize(None)];
        if bytes.len() < end {
            return Err(InvalidImageSize(Some((header.width, header.height))));
        }
        let row_bytes = Mem::bytes_from_bits(header.width);
        whilst! { row in 0..header.height; {
            whilst! { col in 0..header.width; {
                let pixel_index = row * header.width + col;
                let byte_index = header.data_offset + row * row_bytes + col / 8;
                let bit = 1 << (7 - (col % 8));
                out[pixel_index] = if bytes[byte_index] & bit != 0 { 1 } else { 0 };
            }}
        }}
        Ok(())
    }
    /// Returns the encoded byte length for raw PGM/PPM with `max_value = 255`.
    pub(crate) const fn encoded_len_raw8(width: usize, height: usize, samples: usize)
        -> ImageResult<usize> {
        // "P[5|6]\n" + width + " " + height + "\n255\n" + samples
        let mut header = 3usize; // "P?\n"
        header = unwrap![some_ok_or?
            header.checked_add(Self::decimal_len_usize(width)), InvalidImageSize(None)];
        header = unwrap![some_ok_or? header.checked_add(1), InvalidImageSize(None)]; // " "
        header = unwrap![some_ok_or?
            header.checked_add(Self::decimal_len_usize(height)), InvalidImageSize(None)];
        header = unwrap![some_ok_or? header.checked_add(5), InvalidImageSize(None)]; // "\n255\n"
        unwrap![some_ok_or header.checked_add(samples), InvalidImageSize(None)]
    }
    /// Returns the encoded byte length for ASCII PGM/PPM with `max_value = 255`.
    pub(crate) const fn encoded_len_ascii_u8(samples: &[u8], width: usize, height: usize)
        -> ImageResult<usize> {
        // "P[2|3]\n" + width + " " + height + "\n255\n" + raster
        let mut total = 3usize;
        total = unwrap![some_ok_or?
            total.checked_add(Self::decimal_len_usize(width)), InvalidImageSize(None)];
        total = unwrap![some_ok_or? total.checked_add(1), InvalidImageSize(None)];
        total = unwrap![some_ok_or?
            total.checked_add(Self::decimal_len_usize(height)), InvalidImageSize(None)];
        total = unwrap![some_ok_or? total.checked_add(5), InvalidImageSize(None)];
        whilst!{ i in 0..samples.len(); {
            total = unwrap![some_ok_or?
                total.checked_add(Self::decimal_len_usize(samples[i] as usize)),
                InvalidImageSize(None)
            ];
            // Every sample is followed by exactly one separator: a space or the row-ending newline
            total = unwrap![some_ok_or? total.checked_add(1), InvalidImageSize(None)];
        }}
        Ok(total)
    }
    /// Returns the encoded byte length for ASCII PBM from unpacked bitmap samples.
    pub(crate) const fn encoded_len_p1_bitmap(width: usize, height: usize, samples: usize)
        -> ImageResult<usize> {
        // "P1\n" + width + " " + height + "\n" + raster
        let mut total = 3usize;
        total = unwrap![some_ok_or?
            total.checked_add(Self::decimal_len_usize(width)), InvalidImageSize(None)];
        total = unwrap![some_ok_or? total.checked_add(1), InvalidImageSize(None)];
        total = unwrap![some_ok_or?
            total.checked_add(Self::decimal_len_usize(height)), InvalidImageSize(None)];
        total = unwrap![some_ok_or? total.checked_add(1), InvalidImageSize(None)];
        // each sample contributes one digit plus one separator
        let raster = unwrap![some_ok_or? samples.checked_mul(2), InvalidImageSize(None)];
        unwrap![some_ok_or total.checked_add(raster), InvalidImageSize(None)]
    }
    /// Returns the encoded byte length for raw PBM from unpacked bitmap samples.
    pub(crate) const fn encoded_len_p4_bitmap(width: usize, height: usize) -> ImageResult<usize> {
        // "P4\n" + width + " " + height + "\n" + packed raster
        let mut total = 3usize;
        total = unwrap![some_ok_or?
            total.checked_add(Self::decimal_len_usize(width)), InvalidImageSize(None)];
        total = unwrap![some_ok_or? total.checked_add(1), InvalidImageSize(None)];
        total = unwrap![some_ok_or?
            total.checked_add(Self::decimal_len_usize(height)), InvalidImageSize(None)];
        total = unwrap![some_ok_or? total.checked_add(1), InvalidImageSize(None)];
        let packed = unwrap![some_ok_or?
            Mem::bytes_from_bits(width).checked_mul(height), InvalidImageSize(None)];
        unwrap![some_ok_or total.checked_add(packed), InvalidImageSize(None)]
    }
    /// Returns the number of decimal digits in `n`.
    const fn decimal_len_usize(mut n: usize) -> usize {
        let mut len = 1;
        while n >= 10 {
            n /= 10;
            len += 1;
        }
        len
    }
    /// Checks that every bitmap sample is either `0` or `1`.
    pub(crate) const fn validate_bitmap_u8(bits: &[u8]) -> ImageResult<()> {
        let mut i = 0;
        while i < bits.len() {
            if bits[i] > 1 {
                return Err(InvalidPixel);
            }
            i += 1;
        }
        Ok(())
    }
}
