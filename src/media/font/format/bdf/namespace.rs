// devela/src/media/font/format/bdf/namespace.rs
//
//! Defines [`Bdf`].
//

use super::_parse::{BdfHeader, BdfParser, BdfVersion};
use crate::{BdfError, Debug, Region2, Version, unwrap};

#[doc = crate::_tags!(font codec)]
/// Glyph Bitmap Distribution Format operations.
#[doc = crate::_doc_meta!{location("media/font")}]
///
/// BDF is a line-oriented textual format for one bitmap-font strike.
///
/// The initial implementation exposes allocation-free header inspection.
/// Complete validation and decoding are added by later parser stages.
#[derive(Debug)]
pub struct Bdf;

impl Bdf {
    /// Validates the complete structure and glyph data of a BDF font.
    pub const fn validate(bytes: &[u8]) -> Result<(), BdfError> {
        unwrap![ok_map_into? BdfParser::new(bytes), |parser| parser.finish()]
    }
    /// Reads the format version from the opening `STARTFONT` directive.
    pub const fn version(bytes: &[u8]) -> Result<Version, BdfError> {
        unwrap![ok_map? BdfVersion::read(bytes), |version| version.to_version()]
    }
    /// Reads the declared font-wide bitmap bounding region.
    ///
    /// This parses the global header through the `CHARS` directive but does
    /// not validate the glyph records that follow.
    pub const fn bounds(bytes: &[u8]) -> Result<Region2<i32, u32>, BdfError> {
        unwrap![ok_map? BdfHeader::read(bytes), |header| header.bounds]
    }
    /// Reads the glyph count declared by `CHARS`.
    ///
    /// This does not yet verify that the declared number of glyph records
    /// actually follows.
    pub const fn glyph_count(bytes: &[u8]) -> Result<usize, BdfError> {
        unwrap![ok_map? BdfHeader::read(bytes), |header| header.glyph_count]
    }
}
