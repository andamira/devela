// devela/src/media/font/format/bdf/_parse/glyph.rs
//
//! Private BDF glyph-record parsing.
//

use super::{BdfHeader, BdfLine, BdfMetrics, BdfReader, BdfResult};
use super::{bdf_try, read_i32_pair, read_number_pair};
use crate::{BdfError as E, Region2, is, unwrap, whilst};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct BdfEncoding {
    /// Primary encoding, or `-1` for an unencoded glyph.
    pub(crate) primary: i64,
    /// Optional non-standard encoding accompanying a primary value of `-1`.
    pub(crate) alternate: Option<i64>,
}
impl BdfEncoding {
    const fn read(line: BdfLine<'_>) -> BdfResult<Self> {
        let mut fields = line.fields();
        let primary = bdf_try!(fields.i64());
        let alternate = is! { fields.is_empty(), None, Some(bdf_try!(fields.i64())) };
        bdf_try!(fields.finish());
        match (primary, alternate) {
            // In practice, encoding zero is used and should remain valid.
            (primary, None) if primary >= 0 => Ok(Self { primary, alternate: None }),
            (-1, None) => Ok(Self { primary: -1, alternate: None }),
            (-1, Some(alternate)) if alternate >= 0 => {
                Ok(Self { primary: -1, alternate: Some(alternate) })
            }
            _ => Err(E::invalid_value(line.number)),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct BdfBitmap<'a> {
    /// Original validated bitmap rows, including their line endings.
    pub(crate) source: &'a [u8],
    /// One-based number of the first bitmap row.
    pub(super) first_line: u32,
    /// Number of bytes encoded by each row.
    pub(crate) row_bytes: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct BdfGlyph<'a> {
    pub(crate) name: &'a [u8],
    pub(crate) encoding: BdfEncoding,
    pub(super) metrics: BdfMetrics,
    pub(crate) bounds: Region2<i32, u32>,
    pub(crate) bitmap: BdfBitmap<'a>,
}
impl<'a> BdfGlyph<'a> {
    pub(super) const fn read(
        reader: &mut BdfReader<'a>,
        header: &BdfHeader<'a>,
    ) -> BdfResult<Self> {
        let start_line = bdf_try!(reader.required_data());
        let start_line = bdf_try!(start_line.expect(b"STARTCHAR"));
        let name = start_line.trimmed_value();
        is! { name.is_empty(), return Err(E::invalid_value(start_line.number)) }
        let encoding_line = bdf_try!(reader.required_data());
        let encoding_line = bdf_try!(encoding_line.expect(b"ENCODING"));
        let encoding = bdf_try!(BdfEncoding::read(encoding_line));
        let mut metrics = BdfMetrics::EMPTY;
        let mut bounds = None;
        loop {
            let line = bdf_try!(reader.required_data());
            if line.is(b"SWIDTH") {
                is! { metrics.swidth.is_some(), return Err(E::unexpected_directive(line.number)) }
                metrics.swidth = Some(bdf_try!(read_number_pair(line)));
                continue;
            }
            if line.is(b"DWIDTH") {
                is! { metrics.dwidth.is_some(), return Err(E::unexpected_directive(line.number)) }
                metrics.dwidth = Some(bdf_try!(read_i32_pair(line)));
                continue;
            }
            if line.is(b"SWIDTH1") {
                is! { !header.supports_vertical_metrics() || metrics.swidth1.is_some(),
                return Err(E::unexpected_directive(line.number)) }
                metrics.swidth1 = Some(bdf_try!(read_number_pair(line)));
                continue;
            }
            if line.is(b"DWIDTH1") {
                is! { !header.supports_vertical_metrics() || metrics.dwidth1.is_some(),
                return Err(E::unexpected_directive(line.number)) }
                metrics.dwidth1 = Some(bdf_try!(read_i32_pair(line)));
                continue;
            }
            if line.is(b"VVECTOR") {
                is! { !header.supports_vertical_metrics() || metrics.vvector.is_some(),
                return Err(E::unexpected_directive(line.number)) }
                metrics.vvector = Some(bdf_try!(read_i32_pair(line)));
                continue;
            }
            if line.is(b"BBX") {
                is! { bounds.is_some(), return Err(E::unexpected_directive(line.number)) }
                bounds = Some(bdf_try!(read_bounds(line)));
                continue;
            }
            if line.is(b"BITMAP") {
                let mut fields = line.fields();
                bdf_try!(fields.finish());
                let bounds = unwrap![some_or bounds,
                    return Err(E::unexpected_directive(line.number))];
                let metrics = bdf_try!(header.resolve_metrics(metrics, start_line.number));
                let bitmap = bdf_try!(read_bitmap(reader, bounds));
                return Ok(Self { name, encoding, metrics, bounds, bitmap });
            }
            return Err(E::unexpected_directive(line.number));
        }
    }
}

/* helpers */

const fn read_bounds(line: BdfLine<'_>) -> BdfResult<Region2<i32, u32>> {
    let mut fields = line.fields();
    let width = bdf_try!(fields.u32());
    let height = bdf_try!(fields.u32());
    let x = bdf_try!(fields.i32());
    let y = bdf_try!(fields.i32());
    bdf_try!(fields.finish());
    Ok(Region2::from_xy_wh(x, y, width, height)) // zero dimensions not required
}
const fn read_bitmap<'a>(
    reader: &mut BdfReader<'a>,
    bounds: Region2<i32, u32>,
) -> BdfResult<BdfBitmap<'a>> {
    let width = bounds.ext.dim[0];
    let height = bounds.ext.dim[1];
    let row_bytes = width as usize / 8 + is![width.is_multiple_of(8), 0, 1];
    let first_line = reader.line_number();
    let start = reader.pos();
    whilst! { _row in 0..height as usize; {
        // Do not use `required_data`: COMMENT is not special inside BITMAP.
        let line = bdf_try!(reader.required());
        let token = bdf_try!(line.bare());
        bdf_try!(validate_bitmap_row(token, width, row_bytes, line.number));
    }}
    let end = reader.pos();
    let source = reader.source(start, end);
    let end_line = bdf_try!(reader.required_data());
    let end_line = bdf_try!(end_line.expect(b"ENDCHAR"));
    let mut fields = end_line.fields();
    bdf_try!(fields.finish());
    Ok(BdfBitmap { source, first_line, row_bytes })
}
const fn validate_bitmap_row(row: &[u8], width: u32, row_bytes: usize, line: u32) -> BdfResult<()> {
    let expected_len = unwrap![some_or row_bytes.checked_mul(2),
    return Err(E::invalid_value(line))];
    is! { row.len() != expected_len, return Err(E::invalid_value(line)) }
    whilst! { i in 0..row.len(); {
        is! { hex_value(row[i]).is_none(), return Err(E::invalid_value(line)) }
    }}
    // Rows are MSB-first and padded on the right to a complete byte.
    let remainder = width % 8;
    if remainder != 0 {
        let hi = unwrap![some_or hex_value(row[row.len() - 2]), return Err(E::invalid_value(line))];
        let lo = unwrap![some_or hex_value(row[row.len() - 1]), return Err(E::invalid_value(line))];
        let last_byte = (hi << 4) | lo;
        let padding = 8 - remainder;
        let padding_mask = (1_u8 << padding) - 1;
        is! { last_byte & padding_mask != 0, return Err(E::invalid_value(line)) }
    }
    Ok(())
}
const fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}
