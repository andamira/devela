// devela/src/media/font/format/dvbf/define.rs
//
//! Defines [`Dvbf`].
//

use crate::{Debug, DvbfError, FontBitmapView, Fonts, Slice, Version, is, unwrap, whilst};

/// A DVBF decoding result.
type DvbfResult<T> = crate::Result<T, DvbfError>;

#[doc = crate::_tags!(font codec)]
/// Devela Bitmap Font format operations and constants.
#[doc = crate::_doc_meta!{location("media/font")}]
///
/// DVBF stores one fixed-metric monochrome bitmap-font strike in a compact,
/// directly addressable binary representation.
///
/// Version 0.1 contains:
/// - a fixed-size little-endian header;
/// - a strictly increasing table of Unicode scalar values;
/// - one fixed-size one-bit bitmap record for every scalar;
/// - shared glyph dimensions, bounds and advances;
/// - an optional mapped default character.
///
/// [`Dvbf::read`] validates the complete input before returning a borrowed
/// [`FontBitmapView`]. It performs no allocation or copying.
///
/// DVBF 0.1 is deliberately limited to fixed-metric one-bit bitmap strikes.
/// Variable metrics, multiple strikes, color data, layers and font-family
/// selection belong to other representations or higher-level composition.
#[derive(Debug)]
pub struct Dvbf;

impl Dvbf {
    /// The four-byte ASCII DVBF file signature.
    pub const MAGIC: [u8; 4] = *b"DVBF";

    /// The DVBF format version supported by this implementation.
    pub const VERSION: Version = Version::new(0, 1, 0);

    /// The byte length of a DVBF 0.1 header.
    pub const HEADER_BYTES: u16 = 64;

    /// The header sentinel indicating that no default glyph is declared.
    pub const NO_SCALAR: u32 = u32::MAX;

    /// Reads and validates a DVBF font from `bytes`.
    ///
    /// The returned view borrows its scalar table
    /// and bitmap records directly from `bytes`.
    ///
    /// Validation covers the signature, exact supported version, header and
    /// reserved fields, file length, fixed metrics, section layout, scalar
    /// validity and ordering, and the optional default-glyph mapping.
    ///
    /// # Errors
    ///
    /// Returns [`DvbfError`] when any part of the encoded representation
    /// is unsupported, malformed or internally inconsistent.
    pub const fn read(bytes: &[u8]) -> DvbfResult<FontBitmapView<'_>> {
        is! { bytes.len() < Self::HEADER_BYTES as usize, return Err(DvbfError::TooShort) }
        if bytes[0] != b'D' || bytes[1] != b'V' || bytes[2] != b'B' || bytes[3] != b'F' {
            return Err(DvbfError::InvalidMagic);
        }
        let version = Version::new(
            Fonts::read_u16(bytes, 4),
            Fonts::read_u16(bytes, 6),
            Fonts::read_u16(bytes, 8),
        );
        if version.major != Self::VERSION.major
            || version.minor != Self::VERSION.minor
            || version.patch != Self::VERSION.patch
        {
            return Err(DvbfError::UnsupportedVersion(version));
        }
        let header_bytes = Fonts::read_u16(bytes, 10);
        let flags = Fonts::read_u32(bytes, 12);
        let file_bytes = Fonts::read_u32(bytes, 16);
        let glyph_count = Fonts::read_u32(bytes, 20);
        let scalars_offset = Fonts::read_u32(bytes, 24);
        let bitmaps_offset = Fonts::read_u32(bytes, 28);
        let glyph_stride = Fonts::read_u32(bytes, 32);
        let width = Fonts::read_u16(bytes, 36);
        let height = Fonts::read_u16(bytes, 38);
        let row_stride = Fonts::read_u16(bytes, 40);
        let bit_depth = bytes[42];
        let reserved0 = bytes[43];
        let bounds_x = Fonts::read_i16(bytes, 44);
        let bounds_y = Fonts::read_i16(bytes, 46);
        let advance_x = Fonts::read_u16(bytes, 48);
        let line_advance = Fonts::read_u16(bytes, 50);
        let ascent = Fonts::read_u16(bytes, 52);
        let descent = Fonts::read_u16(bytes, 54);
        let default_scalar = Fonts::read_u32(bytes, 56);
        let reserved1 = Fonts::read_u32(bytes, 60);
        if header_bytes != Self::HEADER_BYTES || reserved0 != 0 || reserved1 != 0 {
            return Err(DvbfError::InvalidHeader);
        }
        is! { flags != 0, return Err(DvbfError::UnsupportedFlags(flags)) }
        if file_bytes as usize != bytes.len() {
            return Err(DvbfError::InvalidFileSize { declared: file_bytes, actual: bytes.len() });
        }
        is! { bit_depth != 1, return Err(DvbfError::UnsupportedBitDepth(bit_depth)) }
        if glyph_count == 0 || width == 0 || height == 0 || advance_x == 0 || line_advance == 0 {
            return Err(DvbfError::InvalidMetrics);
        }
        let expected_row_stride = (width as u32).div_ceil(8) as u16;
        let Some(expected_glyph_stride) = (row_stride as u32).checked_mul(height as u32) else {
            return Err(DvbfError::InvalidMetrics);
        };
        let Some(metric_height) = ascent.checked_add(descent) else {
            return Err(DvbfError::InvalidMetrics);
        };
        if row_stride != expected_row_stride || glyph_stride != expected_glyph_stride {
            return Err(DvbfError::InvalidLayout);
        }
        is! { line_advance < metric_height, return Err(DvbfError::InvalidMetrics) }

        let Some(scalar_bytes) = glyph_count.checked_mul(4) else {
            return Err(DvbfError::InvalidLayout);
        };
        let Some(bitmap_bytes) = glyph_count.checked_mul(glyph_stride) else {
            return Err(DvbfError::InvalidLayout);
        };
        let Some(expected_bitmaps_offset) = scalars_offset.checked_add(scalar_bytes) else {
            return Err(DvbfError::InvalidLayout);
        };
        let Some(expected_file_bytes) = bitmaps_offset.checked_add(bitmap_bytes) else {
            return Err(DvbfError::InvalidLayout);
        };
        if scalars_offset != header_bytes as u32
            || bitmaps_offset != expected_bitmaps_offset
            || file_bytes != expected_file_bytes
        {
            return Err(DvbfError::InvalidLayout);
        }
        let scalar_start = scalars_offset as usize;
        let scalar_end = bitmaps_offset as usize;
        let bm_end = file_bytes as usize;
        if scalar_start > scalar_end || scalar_end > bm_end || bm_end > bytes.len() {
            return Err(DvbfError::InvalidLayout);
        }
        let scalars_le = Slice::range(bytes, scalar_start, scalar_end);
        let bitmaps = Slice::range(bytes, scalar_end, bm_end);

        let mut previous = 0u32;
        whilst! { i in 0..glyph_count; {
            let scalar = Fonts::read_u32(scalars_le, (i as usize) * 4);
            if !Fonts::valid_scalar(scalar) {
                return Err(DvbfError::InvalidScalar { index: i, scalar });
            }
            is![i != 0 && scalar <= previous, return Err(DvbfError::UnsortedScalars { index: i })];
            previous = scalar;
        }}
        let default_character = if default_scalar == Self::NO_SCALAR {
            None
        } else {
            unwrap![=some_or char::from_u32(default_scalar),
                return Err(DvbfError::InvalidDefaultScalar(default_scalar))]
        };
        let view = FontBitmapView {
            scalars_le,
            bitmaps,
            glyph_stride,
            width,
            height,
            row_stride,
            bounds_x,
            bounds_y,
            advance_x,
            line_advance,
            ascent,
            descent,
            default_character,
        };
        match default_character {
            Some(character) if !view.has_glyph(character) => {
                Err(DvbfError::MissingDefaultGlyph(character as u32))
            }
            _ => Ok(view),
        }
    }
}
