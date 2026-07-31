// devela/src/media/font/format/bdf/_test.rs

use crate::{Bdf, BdfError, Version};

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
