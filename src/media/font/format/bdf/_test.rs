// devela/src/media/font/format/bdf/_test.rs

use super::_parse::BdfParser;
use crate::{Bdf, BdfError, Version};

const FONT_1: &[u8] = b"\
STARTFONT 2.1
FONT test
SIZE 8 72 72
FONTBOUNDINGBOX 8 2 0 0
CHARS 1
STARTCHAR A
ENCODING 65
SWIDTH 500 0
DWIDTH 8 0
BBX 8 2 0 0
BITMAP
81
7E
ENDCHAR
ENDFONT
";

#[test]
fn validates_complete_font() {
    assert_eq!(Bdf::validate(FONT_1), Ok(()));
}
#[test]
fn streams_glyphs() {
    let mut parser = BdfParser::new(FONT_1).unwrap();
    assert_eq!(parser.remaining(), 1);
    let glyph = parser.next_glyph().unwrap().unwrap();
    assert_eq!(glyph.name, b"A");
    assert_eq!(glyph.encoding.primary, 65);
    assert_eq!(glyph.encoding.alternate, None);
    assert_eq!(glyph.bounds.pos.dim, [0, 0]);
    assert_eq!(glyph.bounds.ext.dim, [8, 2]);
    assert_eq!(glyph.bitmap.row_bytes, 1);
    assert_eq!(glyph.bitmap.source, b"81\n7E\n");
    assert!(parser.next_glyph().unwrap().is_none());
    assert_eq!(parser.finish(), Ok(()));
}
#[test]
fn inherits_global_metrics() {
    const FONT: &[u8] = b"\
STARTFONT 2.2
FONT test
SIZE 8 72 72
FONTBOUNDINGBOX 8 1 0 0
SWIDTH 500 0
DWIDTH 8 0
CHARS 1
STARTCHAR A
ENCODING 65
BBX 8 1 0 0
BITMAP
80
ENDCHAR
ENDFONT
";
    assert_eq!(Bdf::validate(FONT), Ok(()));
}
#[test]
fn reads_alternate_encoding() {
    const FONT: &[u8] = b"\
STARTFONT 2.1
FONT test
SIZE 8 72 72
FONTBOUNDINGBOX 8 1 0 0
CHARS 1
STARTCHAR custom
ENCODING -1 1234
SWIDTH 500 0
DWIDTH 8 0
BBX 8 1 0 0
BITMAP
80
ENDCHAR
ENDFONT
";
    let mut parser = BdfParser::new(FONT).unwrap();
    let glyph = parser.next_glyph().unwrap().unwrap();
    assert_eq!(glyph.encoding.primary, -1);
    assert_eq!(glyph.encoding.alternate, Some(1234));
    assert_eq!(parser.finish(), Ok(()));
}
#[test]
fn rejects_wrong_declared_count() {
    const TOO_MANY: &[u8] = b"\
STARTFONT 2.1
FONT test
SIZE 8 72 72
FONTBOUNDINGBOX 8 1 0 0
CHARS 2
STARTCHAR A
ENCODING 65
SWIDTH 500 0
DWIDTH 8 0
BBX 8 1 0 0
BITMAP
80
ENDCHAR
ENDFONT
";
    assert!(Bdf::validate(TOO_MANY).is_err());
}
#[test]
fn rejects_invalid_bitmap_row() {
    const INVALID: &[u8] = b"\
STARTFONT 2.1
FONT test
SIZE 8 72 72
FONTBOUNDINGBOX 8 1 0 0
CHARS 1
STARTCHAR A
ENCODING 65
SWIDTH 500 0
DWIDTH 8 0
BBX 8 1 0 0
BITMAP
8Z
ENDCHAR
ENDFONT
";
    assert!(matches!(Bdf::validate(INVALID), Err(BdfError::InvalidValue { line: 12 })));
}

const HEADER_21: &[u8] = b"\
STARTFONT 2.1
COMMENT synthetic fixed-metric font
CONTENTVERSION 3
FONT -devela-test-medium-r-normal--16
SIZE 16 72 72
FONTBOUNDINGBOX 8 16 0 -4
STARTPROPERTIES 3
FONT_ASCENT 12
FONT_DESCENT 4
DEFAULT_CHAR 65533
ENDPROPERTIES
CHARS 1356
";
const HEADER_22: &[u8] = b"\
STARTFONT 2.2
FONT test
SIZE 16.0 72 72
FONTBOUNDINGBOX 8 16 0 -4
METRICSSET 0
SWIDTH 500 0
DWIDTH 8 0
CHARS 1
";

#[test]
fn reads_version() {
    assert_eq!(Bdf::version(HEADER_21), Ok(Version::new(2, 1, 0)));
    assert_eq!(Bdf::version(HEADER_22), Ok(Version::new(2, 2, 0)));
}
#[test]
fn reads_declared_bounds_and_glyph_count() {
    let bounds = Bdf::bounds(HEADER_21).unwrap();
    assert_eq!(bounds.pos.dim, [0, -4]);
    assert_eq!(bounds.ext.dim, [8, 16]);
    assert_eq!(Bdf::glyph_count(HEADER_21), Ok(1356));
}
#[test]
fn accepts_bdf_22_global_metrics() {
    assert_eq!(Bdf::glyph_count(HEADER_22), Ok(1));
}
#[test]
fn rejects_unsupported_version() {
    assert_eq!(
        Bdf::version(b"STARTFONT 3.0\n"),
        Err(BdfError::UnsupportedVersion(Version::new(3, 0, 0)))
    );
}
#[test]
fn rejects_missing_required_header_data() {
    let error = Bdf::bounds(b"STARTFONT 2.1\nFONT test\n").unwrap_err();
    assert!(matches!(error, BdfError::UnexpectedEof { .. }));
}
#[test]
fn rejects_global_metrics_in_bdf_21() {
    const INVALID: &[u8] = b"\
STARTFONT 2.1
FONT test
SIZE 16 72 72
FONTBOUNDINGBOX 8 16 0 -4
DWIDTH 8 0
CHARS 0
";
    assert!(matches!(Bdf::glyph_count(INVALID), Err(BdfError::UnexpectedDirective { line: 5 })));
}
#[test]
fn validates_property_count_boundary() {
    const INVALID: &[u8] = b"\
STARTFONT 2.1
FONT test
SIZE 16 72 72
FONTBOUNDINGBOX 8 16 0 -4
STARTPROPERTIES 2
FONT_ASCENT 12
ENDPROPERTIES
CHARS 0
";
    assert!(Bdf::glyph_count(INVALID).is_err());
}
