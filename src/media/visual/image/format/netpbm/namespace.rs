// devela/src/media/visual/image/format/netpbm/define.rs
//
//! Defines [`Pnm`].
//

use super::{PnmCursor, PnmFormat};
use crate::ImageError::{InsufficientBuffer, InvalidImageSize, InvalidPixel};
use crate::{Extent2, ImageResult, unwrap};
use PnmFormat::{P1, P2, P3, P4, P5, P6};

#[doc = crate::_tags!(image codec)]
/// Encoding and decoding for <abbr title="Portable anymap format">PNM</abbr> images.
#[doc = crate::_doc_meta!{location("media/visual/image")}]
///
/// PNM is the classic Netpbm family of simple bitmap formats:
/// - PBM: portable bitmap, monochrome.
/// - PGM: portable graymap, grayscale.
/// - PPM: portable pixmap, RGB color.
///
/// Each family has an ASCII form and a raw form:
///
/// | Magic | Family | Encoding | Decoded samples | Encoder |
/// | --- | --- | --- | --- | --- |
/// | `P1` | PBM | ASCII | unpacked `0`/`1` bytes | [`Pnm::encode_p1_bitmap`] |
/// | `P2` | PGM | ASCII | one grayscale byte per pixel | [`Pnm::encode_p2_gray8`] |
/// | `P3` | PPM | ASCII | interleaved RGB bytes | [`Pnm::encode_p3_rgb8`] |
/// | `P4` | PBM | raw | unpacked `0`/`1` bytes | [`Pnm::encode_p4_bitmap`] |
/// | `P5` | PGM | raw | one grayscale byte per pixel | [`Pnm::encode_p5_gray8`] |
/// | `P6` | PPM | raw | interleaved RGB bytes | [`Pnm::encode_p6_rgb8`] |
///
/// Decoding supports P1 through P6 into caller-provided `u8` storage.
/// Encoding supports all classic PNM forms from caller-provided `u8` samples.
///
/// Comments and ASCII whitespace are accepted in headers and ASCII rasters.
/// Raw P4 rows are byte-padded independently.
///
/// Current limitations:
/// - Decoding into `u8` supports only images with `max_value <= 255`.
/// - Sample values are preserved, not rescaled.
/// - P2, P3, P5 and P6 encoders always write `max_value = 255`.
#[derive(Debug)]
pub struct Pnm;

#[rustfmt::skip]
impl Pnm {
    /* queries */

    /// Returns the image extent declared by the PNM header.
    ///
    /// Supports P1 through P6.
    pub const fn extent(bytes: &[u8]) -> ImageResult<Extent2<usize>> {
        let h = unwrap![ok? Self::read_header(bytes)];
        Ok(Extent2::new([h.width, h.height]))
    }
    /// Returns the number of `u8` samples produced by [`Pnm::decode_u8`].
    ///
    /// This is `width * height * channels`, where PBM and PGM produce one
    /// sample per pixel and PPM produces three interleaved RGB samples per pixel.
    pub const fn decoded_len_u8(bytes: &[u8]) -> ImageResult<usize> {
        let h = unwrap![ok? Self::read_header(bytes)];
        h.sample_len()
    }

    /* decode */

    /// Decodes a PNM image into unpacked `u8` samples.
    ///
    /// Returns the decoded image extent and channel count.
    ///
    /// - P1/P4 produce one byte per pixel, either `0` or `1`.
    /// - P2/P5 produce one grayscale byte per pixel.
    /// - P3/P6 produce three interleaved RGB bytes per pixel.
    ///
    /// Sample values are preserved, not rescaled. For example, a PGM image
    /// with `max_value = 15` decodes samples in `0..=15`.
    ///
    /// This method currently supports only images with `max_value <= 255`.
    pub const fn decode_u8(bytes: &[u8], out: &mut [u8]) -> ImageResult<(Extent2<usize>, u8)> {
        let header = unwrap![ok? Self::read_header(bytes)];
        let needed = unwrap![ok? header.sample_len()];
        if out.len() < needed { return Err(InsufficientBuffer { needed, available: out.len() }); }
        if header.max_value > 255 { return Err(InvalidPixel); }
        match header.format {
            P1 | P2 | P3 => unwrap![ok? Self::decode_ascii_u8(bytes, header, out)],
            P4 => unwrap![ok? Self::decode_p4_u8(bytes, header, out)],
            P5 | P6 => unwrap![ok? Self::decode_raw_u8(bytes, header, out)],
        }
        Ok((header.extent(), header.format.channels()))
    }

    /* encode P1...P6 */

    /// Encodes unpacked bitmap samples as ASCII PBM P1 into caller-provided storage.
    ///
    /// The input must contain exactly `width * height` samples,
    /// each being either `0` or `1`.
    ///
    /// Returns the number of bytes written to `out`
    pub const fn encode_p1_bitmap(bits: &[u8], width: usize, height: usize, out: &mut [u8])
        -> ImageResult<usize> {
        let samples = unwrap![some_ok_or? width.checked_mul(height), InvalidImageSize(None)];
        if bits.len() != samples { return Err(InvalidImageSize(Some((width, height)))); }
        unwrap![ok? Self::validate_bitmap_u8(bits)];
        let needed = unwrap![ok? Self::encoded_len_p1_bitmap(width, height, samples)];
        if out.len() < needed { return Err(InsufficientBuffer { needed, available: out.len() }); }
        let mut wr = PnmCursor::writer(out);
        unwrap![ok? wr.header(*b"P1", width, height)];
        unwrap![ok? wr.byte(b'\n')];
        unwrap![ok? wr.raster_ascii_bitmap(bits, width)];
        Ok(wr.pos())
    }
    /// Encodes grayscale bytes as ASCII PGM P2 into caller-provided storage.
    ///
    /// The input must contain exactly `width * height` samples.
    /// The written image uses `max_value = 255`.
    ///
    /// Returns the number of bytes written to `out`.
    pub const fn encode_p2_gray8(gray: &[u8], width: usize, height: usize, out: &mut [u8])
        -> ImageResult<usize> {
        let samples = unwrap![some_ok_or? width.checked_mul(height), InvalidImageSize(None)];
        if gray.len() != samples { return Err(InvalidImageSize(Some((width, height)))); }
        let needed = unwrap![ok? Self::encoded_len_ascii_u8(gray, width, height)];
        if out.len() < needed { return Err(InsufficientBuffer { needed, available: out.len() }); }
        let mut wr = PnmCursor::writer(out);
        unwrap![ok? wr.header(*b"P2", width, height)];
        unwrap![ok? wr.max255()];
        unwrap![ok? wr.raster_ascii_u8(gray, width, 1)];
        Ok(wr.pos())
    }
    /// Encodes RGB bytes as ASCII PPM P3 into caller-provided storage.
    ///
    /// The input must contain exactly `width * height * 3` samples,
    /// stored as interleaved RGB bytes.
    /// The written image uses `max_value = 255`.
    ///
    /// Returns the number of bytes written to `out`.
    pub const fn encode_p3_rgb8(rgb: &[u8], width: usize, height: usize, out: &mut [u8])
        -> ImageResult<usize> {
        let pixels = unwrap![some_ok_or? width.checked_mul(height), InvalidImageSize(None)];
        let samples = unwrap![some_ok_or? pixels.checked_mul(3), InvalidImageSize(None)];
        if rgb.len() != samples { return Err(InvalidImageSize(Some((width, height)))); }
        let needed = unwrap![ok? Self::encoded_len_ascii_u8(rgb, width, height)];
        if out.len() < needed { return Err(InsufficientBuffer { needed, available: out.len() }); }
        let mut wr = PnmCursor::writer(out);
        unwrap![ok? wr.header(*b"P3", width, height)];
        unwrap![ok? wr.max255()];
        unwrap![ok? wr.raster_ascii_u8(rgb, width, 3)];
        Ok(wr.pos())
    }
    /// Encodes unpacked bitmap samples as raw PBM P4 into caller-provided storage.
    ///
    /// The input must contain exactly `width * height` samples,
    /// each being either `0` or `1`.
    ///
    /// Bits are packed most-significant-bit first, with each row padded to a byte boundary.
    ///
    /// Returns the number of bytes written to `out`.
    pub const fn encode_p4_bitmap(bits: &[u8], width: usize, height: usize, out: &mut [u8])
        -> ImageResult<usize> {
        let samples = unwrap![some_ok_or? width.checked_mul(height), InvalidImageSize(None)];
        if bits.len() != samples { return Err(InvalidImageSize(Some((width, height)))); }
        unwrap![ok? Self::validate_bitmap_u8(bits)];
        let needed = unwrap![ok? Self::encoded_len_p4_bitmap(width, height)];
        if out.len() < needed { return Err(InsufficientBuffer { needed, available: out.len() }); }
        let mut wr = PnmCursor::writer(out);
        unwrap![ok? wr.header(*b"P4", width, height)];
        unwrap![ok? wr.byte(b'\n')];
        unwrap![ok? wr.raster_p4_bitmap(bits, width, height)];
        Ok(wr.pos())
    }
    /// Encodes grayscale bytes as raw PGM P5 into caller-provided storage.
    ///
    /// The input must contain exactly `width * height` samples.
    /// The written image uses `max_value = 255`.
    ///
    /// Returns the number of bytes written to `out`.
    pub const fn encode_p5_gray8(gray: &[u8], width: usize, height: usize, out: &mut [u8])
        -> ImageResult<usize> {
        let samples = unwrap![some_ok_or? width.checked_mul(height), InvalidImageSize(None)];
        if gray.len() != samples { return Err(InvalidImageSize(Some((width, height)))); }
        let needed = unwrap![ok? Self::encoded_len_raw8(width, height, samples)];
        if out.len() < needed { return Err(InsufficientBuffer { needed, available: out.len() }); }
        let mut wr = PnmCursor::writer(out);
        unwrap![ok? wr.header(*b"P5", width, height)];
        unwrap![ok? wr.max255()];
        unwrap![ok? wr.bytes(gray)];
        Ok(wr.pos())
    }
    /// Encodes RGB bytes as raw PPM P6 into caller-provided storage.
    ///
    /// The input must contain exactly `width * height * 3` samples,
    /// stored as interleaved RGB bytes.
    /// The written image uses `max_value = 255`.
    ///
    /// Returns the number of bytes written to `out`.
    pub const fn encode_p6_rgb8(rgb: &[u8], width: usize, height: usize, out: &mut [u8])
        -> ImageResult<usize> {
        let pixels = unwrap![some_ok_or? width.checked_mul(height), InvalidImageSize(None)];
        let samples = unwrap![some_ok_or? pixels.checked_mul(3), InvalidImageSize(None)];
        if rgb.len() != samples { return Err(InvalidImageSize(Some((width, height)))); }
        let needed = unwrap![ok? Self::encoded_len_raw8(width, height, samples)];
        if out.len() < needed { return Err(InsufficientBuffer { needed, available: out.len() }); }
        let mut wr = PnmCursor::writer(out);
        unwrap![ok? wr.header(*b"P6", width, height)];
        unwrap![ok? wr.max255()];
        unwrap![ok? wr.bytes(rgb)];
        Ok(wr.pos())
    }
}
