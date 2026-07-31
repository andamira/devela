// devela/src/media/font/format/dvbf/_test.rs
//
//! Tests the DVBF binary contract independently of external font assets.

use crate::{Dvbf, DvbfError, FontBitmapView, Version};

const HEADER_LEN: usize = Dvbf::HEADER_BYTES as usize;
const MINIMAL_LEN: usize = HEADER_LEN + 4 + 1;
const TWO_GLYPH_LEN: usize = HEADER_LEN + 8 + 2;

const fn write_u16<const N: usize>(dst: &mut [u8; N], offset: usize, value: u16) {
    let bytes = value.to_le_bytes();
    dst[offset] = bytes[0];
    dst[offset + 1] = bytes[1];
}
const fn write_u32<const N: usize>(dst: &mut [u8; N], offset: usize, value: u32) {
    let bytes = value.to_le_bytes();
    dst[offset] = bytes[0];
    dst[offset + 1] = bytes[1];
    dst[offset + 2] = bytes[2];
    dst[offset + 3] = bytes[3];
}

/// Creates the common header for tightly packed 1×1 glyphs.
const fn make_header<const N: usize>(glyph_count: u32, bitmaps_offset: u32) -> [u8; N] {
    let mut bytes = [0_u8; N];
    bytes[0] = b'D';
    bytes[1] = b'V';
    bytes[2] = b'B';
    bytes[3] = b'F';
    write_u16(&mut bytes, 4, Dvbf::VERSION.major);
    write_u16(&mut bytes, 6, Dvbf::VERSION.minor);
    write_u16(&mut bytes, 8, Dvbf::VERSION.patch);
    write_u16(&mut bytes, 10, Dvbf::HEADER_BYTES);
    // flags
    write_u32(&mut bytes, 12, 0);
    write_u32(&mut bytes, 16, N as u32);
    write_u32(&mut bytes, 20, glyph_count);
    write_u32(&mut bytes, 24, HEADER_LEN as u32);
    write_u32(&mut bytes, 28, bitmaps_offset);
    // One byte per 1×1 glyph.
    write_u32(&mut bytes, 32, 1);
    write_u16(&mut bytes, 36, 1); // width
    write_u16(&mut bytes, 38, 1); // height
    write_u16(&mut bytes, 40, 1); // row stride
    bytes[42] = 1; // bit depth
    bytes[43] = 0; // reserved
    write_u16(&mut bytes, 44, 0); // bounds x
    write_u16(&mut bytes, 46, 0); // bounds y
    write_u16(&mut bytes, 48, 1); // advance x
    write_u16(&mut bytes, 50, 1); // line advance
    write_u16(&mut bytes, 52, 1); // ascent
    write_u16(&mut bytes, 54, 0); // descent
    write_u32(&mut bytes, 56, 0x41); // default: 'A'
    write_u32(&mut bytes, 60, 0); // reserved
    bytes
}

const fn make_minimal() -> [u8; MINIMAL_LEN] {
    let mut bytes = make_header::<MINIMAL_LEN>(1, 68);
    write_u32(&mut bytes, 64, 0x41);
    bytes[68] = 0x80;
    bytes
}

const fn make_two_glyphs() -> [u8; TWO_GLYPH_LEN] {
    let mut bytes = make_header::<TWO_GLYPH_LEN>(2, 72);
    write_u32(&mut bytes, 64, 0x41);
    write_u32(&mut bytes, 68, 0x42);
    bytes[72] = 0x80;
    bytes[73] = 0x40;
    bytes
}

const MINIMAL_DVBF: [u8; MINIMAL_LEN] = make_minimal();
const TWO_GLYPH_DVBF: [u8; TWO_GLYPH_LEN] = make_two_glyphs();

/// Also verifies that the decoder remains const-evaluable.
///
/// This can be removed if const usability does not need a dedicated regression
/// check.
const MINIMAL_VIEW: FontBitmapView<'static> = match Dvbf::read(&MINIMAL_DVBF) {
    Ok(view) => view,
    Err(_) => panic!("invalid minimal DVBF fixture"),
};
fn assert_error(bytes: &[u8], expected: DvbfError) {
    assert_eq!(Dvbf::read(bytes), Err(expected));
}

/* valid inputs */

#[test]
fn reads_minimal_font() {
    let font = Dvbf::read(&MINIMAL_DVBF).unwrap();
    assert_eq!(font.glyph_count(), 1);
    assert_eq!((font.width(), font.height()), (1, 1));
    assert_eq!(font.row_stride(), 1);
    assert_eq!(font.glyph_stride(), 1);
    assert_eq!(font.advance_x(), 1);
    assert_eq!(font.line_advance(), 1);
    assert_eq!((font.ascent(), font.descent()), (1, 0));
    assert_eq!(font.default_scalar(), Some(0x41));
    let glyph = font.glyph('A').unwrap();
    assert_eq!(glyph.character(), 'A');
    assert_eq!(glyph.bitmap(), &[0x80]);
    assert_eq!(glyph.is_set(0, 0), Some(true));
}
#[test]
fn reads_two_sorted_glyphs() {
    let font = Dvbf::read(&TWO_GLYPH_DVBF).unwrap();
    assert_eq!(font.glyph_count(), 2);
    assert_eq!(font.scalar_at(0), Some(0x41));
    assert_eq!(font.scalar_at(1), Some(0x42));
    assert!(font.has_glyph('A'));
    assert!(font.has_glyph('B'));
}
#[test]
fn const_decoding_works() {
    assert_eq!(MINIMAL_VIEW.glyph_count(), 1);
    assert_eq!(MINIMAL_VIEW.default_scalar(), Some(0x41));
}
#[test]
fn accepts_no_default_scalar() {
    let mut bytes = MINIMAL_DVBF;
    write_u32(&mut bytes, 56, Dvbf::NO_SCALAR);
    let font = Dvbf::read(&bytes).unwrap();
    assert_eq!(font.default_scalar(), None);
    assert_eq!(font.glyph_or_default('B'), None);
}
#[test]
fn accepts_null_as_a_real_scalar_and_default() {
    let mut bytes = MINIMAL_DVBF;
    write_u32(&mut bytes, 64, 0);
    write_u32(&mut bytes, 56, 0);
    let font = Dvbf::read(&bytes).unwrap();
    assert_eq!(font.default_scalar(), Some(0));
    assert!(font.has_glyph('\0'));
}

/* header */

#[test]
fn rejects_short_header() {
    assert_error(&MINIMAL_DVBF[..HEADER_LEN - 1], DvbfError::TooShort);
}
#[test]
fn rejects_invalid_magic() {
    let mut bytes = MINIMAL_DVBF;
    bytes[0] = b'X';
    assert_error(&bytes, DvbfError::InvalidMagic);
}
#[test]
fn rejects_unsupported_version() {
    let mut bytes = MINIMAL_DVBF;
    write_u16(&mut bytes, 4, 1);
    assert_error(&bytes, DvbfError::UnsupportedVersion(Version::new(1, 1, 0)));
}
#[test]
fn rejects_invalid_header_size() {
    let mut bytes = MINIMAL_DVBF;
    write_u16(&mut bytes, 10, Dvbf::HEADER_BYTES - 1);
    assert_error(&bytes, DvbfError::InvalidHeader);
}
#[test]
fn rejects_nonzero_reserved_byte() {
    let mut bytes = MINIMAL_DVBF;
    bytes[43] = 1;
    assert_error(&bytes, DvbfError::InvalidHeader);
}
#[test]
fn rejects_nonzero_reserved_word() {
    let mut bytes = MINIMAL_DVBF;
    write_u32(&mut bytes, 60, 1);
    assert_error(&bytes, DvbfError::InvalidHeader);
}
#[test]
fn rejects_unsupported_flags() {
    let mut bytes = MINIMAL_DVBF;
    write_u32(&mut bytes, 12, 0x8000_0001);
    assert_error(&bytes, DvbfError::UnsupportedFlags(0x8000_0001));
}
#[test]
fn rejects_unsupported_bit_depth() {
    let mut bytes = MINIMAL_DVBF;
    bytes[42] = 8;
    assert_error(&bytes, DvbfError::UnsupportedBitDepth(8));
}
#[test]
fn rejects_incorrect_declared_file_size() {
    let mut bytes = MINIMAL_DVBF;
    write_u32(&mut bytes, 16, (MINIMAL_LEN + 1) as u32);
    assert_error(
        &bytes,
        DvbfError::InvalidFileSize {
            declared: (MINIMAL_LEN + 1) as u32,
            actual: MINIMAL_LEN,
        },
    );
}
#[test]
fn rejects_trailing_bytes() {
    let mut bytes = MINIMAL_DVBF.to_vec();
    bytes.push(0);
    assert_error(
        &bytes,
        DvbfError::InvalidFileSize {
            declared: MINIMAL_LEN as u32,
            actual: MINIMAL_LEN + 1,
        },
    );
}

/* metrics */

#[test]
fn rejects_zero_glyph_count() {
    let mut bytes = MINIMAL_DVBF;
    write_u32(&mut bytes, 20, 0);
    assert_error(&bytes, DvbfError::InvalidMetrics);
}
#[test]
fn rejects_zero_width() {
    let mut bytes = MINIMAL_DVBF;
    write_u16(&mut bytes, 36, 0);
    assert_error(&bytes, DvbfError::InvalidMetrics);
}
#[test]
fn rejects_zero_height() {
    let mut bytes = MINIMAL_DVBF;
    write_u16(&mut bytes, 38, 0);
    assert_error(&bytes, DvbfError::InvalidMetrics);
}
#[test]
fn rejects_zero_horizontal_advance() {
    let mut bytes = MINIMAL_DVBF;
    write_u16(&mut bytes, 48, 0);
    assert_error(&bytes, DvbfError::InvalidMetrics);
}
#[test]
fn rejects_zero_line_advance() {
    let mut bytes = MINIMAL_DVBF;
    write_u16(&mut bytes, 50, 0);
    assert_error(&bytes, DvbfError::InvalidMetrics);
}
#[test]
fn rejects_line_advance_below_font_height() {
    let mut bytes = MINIMAL_DVBF;
    write_u16(&mut bytes, 52, 1);
    write_u16(&mut bytes, 54, 1);
    write_u16(&mut bytes, 50, 1);
    assert_error(&bytes, DvbfError::InvalidMetrics);
}
#[test]
fn rejects_overflowing_ascent_and_descent() {
    let mut bytes = MINIMAL_DVBF;
    write_u16(&mut bytes, 52, u16::MAX);
    write_u16(&mut bytes, 54, 1);
    assert_error(&bytes, DvbfError::InvalidMetrics);
}

/* layout */

#[test]
fn rejects_noncanonical_scalar_offset() {
    let mut bytes = MINIMAL_DVBF;
    write_u32(&mut bytes, 24, 63);
    assert_error(&bytes, DvbfError::InvalidLayout);
}
#[test]
fn rejects_noncanonical_bitmap_offset() {
    let mut bytes = MINIMAL_DVBF;
    write_u32(&mut bytes, 28, 69);
    assert_error(&bytes, DvbfError::InvalidLayout);
}
#[test]
fn rejects_incorrect_row_stride() {
    let mut bytes = MINIMAL_DVBF;
    write_u16(&mut bytes, 40, 2);
    assert_error(&bytes, DvbfError::InvalidLayout);
}
#[test]
fn rejects_incorrect_glyph_stride() {
    let mut bytes = MINIMAL_DVBF;
    write_u32(&mut bytes, 32, 2);
    assert_error(&bytes, DvbfError::InvalidLayout);
}
#[test]
fn rejects_overflowing_scalar_table_size() {
    let mut bytes = MINIMAL_DVBF;
    write_u32(&mut bytes, 20, u32::MAX);
    assert_error(&bytes, DvbfError::InvalidLayout);
}

/* scalar mapping */

#[test]
fn rejects_surrogate_scalar() {
    let mut bytes = MINIMAL_DVBF;
    write_u32(&mut bytes, 64, 0xD800);
    assert_error(&bytes, DvbfError::InvalidScalar { index: 0, scalar: 0xD800 });
}
#[test]
fn rejects_scalar_above_unicode_range() {
    let mut bytes = MINIMAL_DVBF;
    write_u32(&mut bytes, 64, 0x11_0000);
    assert_error(&bytes, DvbfError::InvalidScalar { index: 0, scalar: 0x11_0000 });
}
#[test]
fn rejects_duplicate_scalars() {
    let mut bytes = TWO_GLYPH_DVBF;
    write_u32(&mut bytes, 68, 0x41);
    assert_error(&bytes, DvbfError::UnsortedScalars { index: 1 });
}
#[test]
fn rejects_descending_scalars() {
    let mut bytes = TWO_GLYPH_DVBF;
    write_u32(&mut bytes, 64, 0x42);
    write_u32(&mut bytes, 68, 0x41);
    assert_error(&bytes, DvbfError::UnsortedScalars { index: 1 });
}
#[test]
fn rejects_invalid_default_scalar() {
    let mut bytes = MINIMAL_DVBF;
    write_u32(&mut bytes, 56, 0xD800);
    assert_error(&bytes, DvbfError::InvalidDefaultScalar(0xD800));
}
#[test]
fn rejects_unmapped_default_scalar() {
    let mut bytes = MINIMAL_DVBF;
    write_u32(&mut bytes, 56, 0x42);
    assert_error(&bytes, DvbfError::MissingDefaultGlyph(0x42));
}
