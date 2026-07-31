// devela/src/media/font/format/bdf/_parse/parser.rs
//
//! Stateful traversal of a complete BDF font.
//

#![expect(dead_code)]

use super::{BdfGlyph, BdfHeader, BdfReader, BdfResult, bdf_try};
use crate::{BdfError as E, is};

#[derive(Clone, Debug)]
pub(crate) struct BdfParser<'a> {
    reader: BdfReader<'a>,
    header: BdfHeader<'a>,
    glyph_index: usize,
}
impl<'a> BdfParser<'a> {
    pub(crate) const fn new(bytes: &'a [u8]) -> BdfResult<Self> {
        let mut reader = BdfReader::new(bytes);
        let header = bdf_try!(BdfHeader::read_from(&mut reader));
        Ok(Self { reader, header, glyph_index: 0 })
    }
    pub(super) const fn header(&self) -> &BdfHeader<'a> {
        &self.header
    }
    pub(super) const fn glyph_index(&self) -> usize {
        self.glyph_index
    }
    pub(crate) const fn remaining(&self) -> usize {
        self.header.glyph_count - self.glyph_index
    }
    pub(crate) const fn next_glyph(&mut self) -> BdfResult<Option<BdfGlyph<'a>>> {
        is! { self.glyph_index == self.header.glyph_count, return Ok(None) }
        let glyph = bdf_try!(BdfGlyph::read(&mut self.reader, &self.header));
        self.glyph_index += 1;
        Ok(Some(glyph))
    }
    /// Parses all remaining glyphs and validates the final file boundary.
    pub(crate) const fn finish(mut self) -> BdfResult<()> {
        while self.glyph_index < self.header.glyph_count {
            match bdf_try!(self.next_glyph()) {
                Some(_) => {}
                None => {
                    return Err(E::unexpected_eof(self.reader.line_number()));
                }
            }
        }
        let line = bdf_try!(self.reader.required_data());
        let line = bdf_try!(line.expect(b"ENDFONT"));
        let mut fields = line.fields();
        bdf_try!(fields.finish());
        match bdf_try!(self.reader.next()) {
            None => Ok(()),
            Some(line) => Err(E::unexpected_directive(line.number)),
        }
    }
}
