// devela/src/media/font/format/bdf/namespace.rs
//
//! Defines [`Bdf`].
//

use super::_parse::{BdfHeader, BdfParser, BdfVersion};
use super::BdfError;
use crate::{Debug, Region2, Version};

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
        match BdfParser::new(bytes) {
            Ok(parser) => parser.finish(),
            Err(error) => Err(error),
        }
    }
    /// Reads the format version from the opening `STARTFONT` directive.
    pub const fn version(bytes: &[u8]) -> Result<Version, BdfError> {
        match BdfVersion::read(bytes) {
            Ok(version) => Ok(version.to_version()),
            Err(error) => Err(error),
        }
    }
    /// Reads the declared font-wide bitmap bounding region.
    ///
    /// This parses the global header through the `CHARS` directive but does
    /// not validate the glyph records that follow.
    pub const fn bounds(bytes: &[u8]) -> Result<Region2<i32, u32>, BdfError> {
        match BdfHeader::read(bytes) {
            Ok(header) => Ok(header.bounds),
            Err(error) => Err(error),
        }
    }
    /// Reads the glyph count declared by `CHARS`.
    ///
    /// This does not yet verify that the declared number of glyph records
    /// actually follows.
    pub const fn glyph_count(bytes: &[u8]) -> Result<usize, BdfError> {
        match BdfHeader::read(bytes) {
            Ok(header) => Ok(header.glyph_count),
            Err(error) => Err(error),
        }
    }
}
