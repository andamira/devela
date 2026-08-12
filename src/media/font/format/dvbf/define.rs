// devela/src/media/font/format/dvbf/define.rs
//
//! Defines [`Dvbf`].
//

use crate::DvbfError::{self, self as E};
use crate::{Debug, FontBitmapView, Version};
use crate::{Slice, is, read_at, slice, unwrap, whilst};

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
        is![bytes.len() < Self::HEADER_BYTES as usize, return Err(E::TooShort)];
        let mut off = Self::MAGIC.len();
        is![off == Self::HEADER_BYTES as usize, return Err(E::InvalidMagic)];
        is![!Slice::<u8>::eq(slice![bytes, ..off], b"DVBF"), return Err(E::InvalidMagic)];
        let version = Version::new(
            u16::from_le_bytes(read_at![bytes, +=off, @2]),
            u16::from_le_bytes(read_at![bytes, +=off, @2]),
            u16::from_le_bytes(read_at![bytes, +=off, @2]),
        );
        if version.major != Self::VERSION.major
            || version.minor != Self::VERSION.minor
            || version.patch != Self::VERSION.patch
        {
            return Err(E::UnsupportedVersion(version));
        }
        let header_bytes = u16::from_le_bytes(read_at![bytes, +=off, @2]);
        let flags = u32::from_le_bytes(read_at![bytes, +=off, @4]);
        let file_bytes = u32::from_le_bytes(read_at![bytes, +=off, @4]);
        let glyph_count = u32::from_le_bytes(read_at![bytes, +=off, @4]);
        let scalars_offset = u32::from_le_bytes(read_at![bytes, +=off, @4]);
        let bitmaps_offset = u32::from_le_bytes(read_at![bytes, +=off, @4]);
        let glyph_stride = u32::from_le_bytes(read_at![bytes, +=off, @4]);
        let width = u16::from_le_bytes(read_at![bytes, +=off, @2]);
        let height = u16::from_le_bytes(read_at![bytes, +=off, @2]);
        let row_stride = u16::from_le_bytes(read_at![bytes, +=off, @2]);
        let bit_depth = read_at![bytes, +=off, @1][0];
        let reserved0 = read_at![bytes, +=off, @1][0];
        let bounds_x = i16::from_le_bytes(read_at![bytes, +=off, @2]);
        let bounds_y = i16::from_le_bytes(read_at![bytes, +=off, @2]);
        let advance_x = u16::from_le_bytes(read_at![bytes, +=off, @2]);
        let line_advance = u16::from_le_bytes(read_at![bytes, +=off, @2]);
        let ascent = u16::from_le_bytes(read_at![bytes, +=off, @2]);
        let descent = u16::from_le_bytes(read_at![bytes, +=off, @2]);
        let default_scalar = u32::from_le_bytes(read_at![bytes, +=off, @4]);
        let reserved1 = u32::from_le_bytes(read_at![bytes, off, @4]);
        if header_bytes != Self::HEADER_BYTES || reserved0 != 0 || reserved1 != 0 {
            return Err(E::InvalidHeader);
        }
        is! { flags != 0, return Err(E::UnsupportedFlags(flags)) }
        if file_bytes as usize != bytes.len() {
            return Err(E::InvalidFileSize { declared: file_bytes, actual: bytes.len() });
        }
        is! { bit_depth != 1, return Err(E::UnsupportedBitDepth(bit_depth)) }
        if glyph_count == 0 || width == 0 || height == 0 || advance_x == 0 || line_advance == 0 {
            return Err(E::InvalidMetrics);
        }
        let expected_row_stride = (width as u32).div_ceil(8) as u16;
        let expected_glyph_stride =
            unwrap![some_ok_or?(row_stride as u32).checked_mul(height as u32), E::InvalidMetrics];
        let metric_height = unwrap![some_ok_or? ascent.checked_add(descent), E::InvalidMetrics];
        if row_stride != expected_row_stride || glyph_stride != expected_glyph_stride {
            return Err(E::InvalidLayout);
        }
        is! { line_advance < metric_height, return Err(E::InvalidMetrics) }
        let scalar_bytes = unwrap![some_ok_or? glyph_count.checked_mul(4), E::InvalidLayout];
        let bitmap_bytes = unwrap![some_ok_or? glyph_count.checked_mul(glyph_stride),
            E::InvalidLayout];
        let expected_bitmaps_offset = unwrap![some_ok_or? scalars_offset.checked_add(scalar_bytes),
            E::InvalidLayout];
        let expected_file_bytes = unwrap![some_ok_or? bitmaps_offset.checked_add(bitmap_bytes),
            E::InvalidLayout];
        if scalars_offset != header_bytes as u32
            || bitmaps_offset != expected_bitmaps_offset
            || file_bytes != expected_file_bytes
        {
            return Err(E::InvalidLayout);
        }
        let (scalar_start, scalar_end) = (scalars_offset as usize, bitmaps_offset as usize);
        let bm_end = file_bytes as usize;
        if scalar_start > scalar_end || scalar_end > bm_end || bm_end > bytes.len() {
            return Err(E::InvalidLayout);
        }
        let scalars_le = Slice::range(bytes, scalar_start, scalar_end);
        let bitmaps = Slice::range(bytes, scalar_end, bm_end);
        let mut previous = 0u32;
        whilst! { i in 0..glyph_count; {
            let scalar = u32::from_le_bytes(read_at![scalars_le, (i as usize) * 4, @4]);
            is![char::from_u32(scalar).is_none(), return Err(E::invalid_scalar(i, scalar))];
            is![i != 0 && scalar <= previous, return Err(E::UnsortedScalars { index: i })];
            previous = scalar;
        }}
        let default_character = if default_scalar == Self::NO_SCALAR {
            None
        } else {
            unwrap![=some_or char::from_u32(default_scalar),
            return Err(E::InvalidDefaultScalar(default_scalar))]
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
                Err(E::MissingDefaultGlyph(character as u32))
            }
            _ => Ok(view),
        }
    }
}
