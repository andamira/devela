// devela/src/media/font/format/bdf/_parse.rs
//
//! Private BDF grammar and header parsing.
//

#![allow(dead_code, reason = "staged BDF parser implementation")]

use super::BdfError as E;
use crate::{Region2, Slice, TextScanner, Version, is, lets, unwrap, whilst};

/* local result propagation */

macro_rules! bdf_try {
    ($result:expr) => {
        match $result {
            Ok(value) => value,
            Err(error) => return Err(error),
        }
    };
}

type BdfResult<T> = Result<T, E>;

/* version */

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) enum BdfVersion {
    V2_1,
    V2_2,
}
impl BdfVersion {
    pub(super) const fn read(bytes: &[u8]) -> BdfResult<Self> {
        let mut reader = BdfReader::new(bytes);
        read_startfont(&mut reader)
    }
    pub(super) const fn to_version(self) -> Version {
        match self {
            Self::V2_1 => Version::new(2, 1, 0),
            Self::V2_2 => Version::new(2, 2, 0),
        }
    }
    const fn from_version(version: Version) -> BdfResult<Self> {
        match version {
            Version { major: 2, minor: 1, patch: 0 } => Ok(Self::V2_1),
            Version { major: 2, minor: 2, patch: 0 } => Ok(Self::V2_2),
            version => Err(E::UnsupportedVersion(version)),
        }
    }
    const fn is_v2(self) -> bool {
        matches![self, Self::V2_1]
    }
}

/* exact decimal */

/// Exact normalized BDF `number`.
///
/// Its value is `coefficient × 10⁻ˢᶜᵃˡᵉ`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
struct BdfNumber {
    coefficient: i64,
    scale: u8,
}

impl BdfNumber {
    const ZERO: Self = Self { coefficient: 0, scale: 0 };

    const fn new(mut coefficient: i64, mut scale: u8) -> Self {
        while scale != 0 && coefficient % 10 == 0 {
            coefficient /= 10;
            scale -= 1;
        }
        Self { coefficient, scale }
    }
    const fn is_positive(self) -> bool {
        self.coefficient > 0
    }
    const fn parse(bytes: &[u8], line: u32) -> BdfResult<Self> {
        is! { bytes.is_empty(), return Err(E::invalid_value(line)) }
        lets! { negative = bytes[0] == b'-', mut i = is! { negative, 1, 0 } }
        is! { i == bytes.len(), return Err(E::invalid_value(line)) }
        lets! { mut magnitude = 0_u64, mut scale = 0_u8, mut digits = 0_usize, mut decimal = false }
        while i < bytes.len() {
            let byte = bytes[i];
            if byte == b'.' {
                is! { decimal, return Err(E::invalid_value(line)) }
                decimal = true;
            } else if byte >= b'0' && byte <= b'9' {
                let digit = (byte - b'0') as u64;
                magnitude = unwrap![some_or magnitude.checked_mul(10),
                    return Err(E::invalid_value(line))];
                magnitude = unwrap![some_or magnitude.checked_add(digit),
                    return Err(E::invalid_value(line))];
                digits += 1;
                if decimal {
                    scale = unwrap![some_or scale.checked_add(1),
                        return Err(E::invalid_value(line))];
                }
            } else {
                return Err(E::invalid_value(line));
            }
            i += 1;
        }
        is! { digits == 0, return Err(E::invalid_value(line)) }
        let coefficient = if negative {
            const MIN_MAGNITUDE: u64 = i64::MAX as u64 + 1;
            if magnitude > MIN_MAGNITUDE {
                return Err(E::invalid_value(line));
            } else if magnitude == MIN_MAGNITUDE {
                i64::MIN
            } else {
                -(magnitude as i64)
            }
        } else {
            is! { magnitude > i64::MAX as u64, return Err(E::invalid_value(line)) }
            magnitude as i64
        };
        Ok(Self::new(coefficient, scale))
    }
}

/* line reader */

#[derive(Clone, Debug)]
struct BdfReader<'a> {
    scanner: TextScanner<'a>,
    next_line: u32,
}
impl<'a> BdfReader<'a> {
    const fn new(bytes: &'a [u8]) -> Self {
        Self {
            scanner: TextScanner::from_bytes(bytes),
            next_line: 1,
        }
    }
    const fn pos(&self) -> usize {
        self.scanner.pos().as_usize()
    }
    const fn next(&mut self) -> BdfResult<Option<BdfLine<'a>>> {
        let range = unwrap![some_or self.scanner.next_line(), return Ok(None)];
        let number = self.next_line;
        self.next_line = self.next_line.saturating_add(1);
        unwrap![ok_map BdfLine::read(self.scanner.slice(range), number), |v| Some(v)]
    }
    const fn required(&mut self) -> BdfResult<BdfLine<'a>> {
        unwrap![some_ok_or bdf_try!(self.next()), E::unexpected_eof(self.next_line)]
    }
    /// Reads the next non-`COMMENT` line.
    const fn required_data(&mut self) -> BdfResult<BdfLine<'a>> {
        loop {
            let line = bdf_try!(self.required());
            is! { !line.is(b"COMMENT"), return Ok(line) }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct BdfLine<'a> {
    keyword: &'a [u8],
    value: &'a [u8],
    number: u32,
}
impl<'a> BdfLine<'a> {
    const fn read(bytes: &'a [u8], number: u32) -> BdfResult<Self> {
        if bytes.is_empty() || matches!(bytes[0], b' ' | b'\t') {
            return Err(E::unexpected_directive(number));
        }
        let mut scanner = TextScanner::from_bytes(bytes);
        let keyword_range = scanner.take_until_any2(b' ', b'\t');
        let keyword = scanner.slice(keyword_range);
        is! { keyword.is_empty(), return Err(E::unexpected_directive(number)) }
        scanner.skip_ascii_hws();
        Ok(Self { keyword, value: scanner.rest(), number })
    }
    const fn is(self, keyword: &[u8]) -> bool {
        bytes_eq(self.keyword, keyword)
    }
    const fn expect(self, keyword: &[u8]) -> BdfResult<Self> {
        is! { self.is(keyword), Ok(self), Err(E::unexpected_directive(self.number)) }
    }
    const fn fields(self) -> BdfFields<'a> {
        BdfFields {
            scanner: TextScanner::from_bytes(self.value),
            line: self.number,
        }
    }
    const fn trimmed_value(self) -> &'a [u8] {
        let mut end = self.value.len();
        while end != 0 && matches!(self.value[end - 1], b' ' | b'\t') {
            end -= 1;
        }
        Slice::range_to(self.value, end)
    }
}

/* fields */

#[derive(Clone, Debug)]
struct BdfFields<'a> {
    scanner: TextScanner<'a>,
    line: u32,
}
impl<'a> BdfFields<'a> {
    const fn token(&mut self) -> BdfResult<&'a [u8]> {
        self.scanner.skip_ascii_hws();
        is! { self.scanner.is_eof(), return Err(E::invalid_value(self.line)) }
        let range = self.scanner.take_until_any2(b' ', b'\t');
        let token = self.scanner.slice(range);
        is! { token.is_empty(), Err(E::invalid_value(self.line)), Ok(token) }
    }
    const fn finish(&mut self) -> BdfResult<()> {
        self.scanner.skip_ascii_hws();
        is! { self.scanner.is_eof(), Ok(()), Err(E::invalid_value(self.line)) }
    }
    const fn u64(&mut self) -> BdfResult<u64> {
        let token = bdf_try!(self.token());
        let mut scanner = TextScanner::from_bytes(token);
        let value = unwrap![ok_err_map? scanner.expect_ascii_u64(),
            |__| E::invalid_value(self.line)];
        is! { scanner.is_eof(), Ok(value), Err(E::invalid_value(self.line)) }
    }
    const fn usize(&mut self) -> BdfResult<usize> {
        let token = bdf_try!(self.token());
        let mut scanner = TextScanner::from_bytes(token);
        let value = unwrap![ok_err_map? scanner.expect_ascii_usize(),
            |__| E::invalid_value(self.line)];
        is! { scanner.is_eof(), Ok(value), Err(E::invalid_value(self.line)) }
    }
    const fn i64(&mut self) -> BdfResult<i64> {
        let token = bdf_try!(self.token());
        let mut scanner = TextScanner::from_bytes(token);
        let value = unwrap![ok_err_map? scanner.expect_ascii_i64(),
            |__| E::invalid_value(self.line)];
        is! { scanner.is_eof(), Ok(value), Err(E::invalid_value(self.line)) }
    }
    const fn u32(&mut self) -> BdfResult<u32> {
        let value = bdf_try!(self.u64());
        is! { value > u32::MAX as u64, Err(E::invalid_value(self.line)), Ok(value as u32) }
    }
    const fn i32(&mut self) -> BdfResult<i32> {
        let value = bdf_try!(self.i64());
        if value < i32::MIN as i64 || value > i32::MAX as i64 {
            Err(E::invalid_value(self.line))
        } else {
            Ok(value as i32)
        }
    }
    const fn number(&mut self) -> BdfResult<BdfNumber> {
        let token = bdf_try!(self.token());
        BdfNumber::parse(token, self.line)
    }
}

/* private header records */

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
struct BdfMetrics {
    swidth: Option<[BdfNumber; 2]>,
    dwidth: Option<[i32; 2]>,
    swidth1: Option<[BdfNumber; 2]>,
    dwidth1: Option<[i32; 2]>,
    vvector: Option<[i32; 2]>,
}
impl BdfMetrics {
    const EMPTY: Self = Self {
        swidth: None,
        dwidth: None,
        swidth1: None,
        dwidth1: None,
        vvector: None,
    };
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct BdfSection {
    start: usize,
    end: usize,
    count: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) struct BdfHeader<'a> {
    version: BdfVersion,
    name: &'a [u8],
    point_size: BdfNumber,
    resolution: [u32; 2],
    pub(super) bounds: Region2<i32, u32>,
    content_version: Option<i64>,
    metrics_set: u8,
    global_metrics: BdfMetrics,
    properties: Option<BdfSection>,
    pub(super) glyph_count: usize,
    glyphs_offset: usize,
}
impl<'a> BdfHeader<'a> {
    pub(super) const fn read(bytes: &'a [u8]) -> BdfResult<Self> {
        let mut reader = BdfReader::new(bytes);
        let version = bdf_try!(read_startfont(&mut reader));
        let mut line = bdf_try!(reader.required_data());
        let content_version = if line.is(b"CONTENTVERSION") {
            let mut fields = line.fields();
            let value = bdf_try!(fields.i64());
            bdf_try!(fields.finish());
            line = bdf_try!(reader.required_data());
            Some(value)
        } else {
            None
        };
        line = bdf_try!(line.expect(b"FONT"));
        let name = line.trimmed_value();
        is! { name.is_empty(), return Err(E::invalid_value(line.number)) }
        line = bdf_try!(reader.required_data());
        line = bdf_try!(line.expect(b"SIZE"));
        let mut fields = line.fields();
        let point_size = bdf_try!(fields.number());
        let resolution_x = bdf_try!(fields.u32());
        let resolution_y = bdf_try!(fields.u32());
        bdf_try!(fields.finish());
        if !point_size.is_positive() || resolution_x == 0 || resolution_y == 0 {
            return Err(E::invalid_value(line.number));
        }
        line = bdf_try!(reader.required_data());
        line = bdf_try!(line.expect(b"FONTBOUNDINGBOX"));
        let mut fields = line.fields();
        let width = bdf_try!(fields.u32());
        let height = bdf_try!(fields.u32());
        let x = bdf_try!(fields.i32());
        let y = bdf_try!(fields.i32());
        bdf_try!(fields.finish());
        is! { width == 0 || height == 0, return Err(E::invalid_value(line.number)) }
        let bounds = Region2::from_xy_wh(x, y, width, height);
        let mut metrics_set = 0_u8;
        let mut has_metrics_set = false;
        let mut global_metrics = BdfMetrics::EMPTY;
        let mut properties = None;
        loop {
            line = bdf_try!(reader.required_data());
            if line.is(b"CHARS") {
                let mut fields = line.fields();
                let glyph_count = bdf_try!(fields.usize());
                bdf_try!(fields.finish());
                return Ok(Self {
                    version,
                    name,
                    point_size,
                    resolution: [resolution_x, resolution_y],
                    bounds,
                    content_version,
                    metrics_set,
                    global_metrics,
                    properties,
                    glyph_count,
                    glyphs_offset: reader.pos(),
                });
            }
            if line.is(b"STARTPROPERTIES") {
                is! { properties.is_some(), return Err(E::unexpected_directive(line.number)); }
                properties = Some(bdf_try!(read_properties_section(&mut reader, line)));
                continue;
            }
            if line.is(b"METRICSSET") {
                is! { version.is_v2() || has_metrics_set,
                return Err(E::unexpected_directive(line.number)) }
                let mut fields = line.fields();
                let value = bdf_try!(fields.u32());
                bdf_try!(fields.finish());
                is! { value > 2, return Err(E::invalid_value(line.number)) }
                metrics_set = value as u8;
                has_metrics_set = true;
                continue;
            }
            if line.is(b"SWIDTH") {
                is! { version.is_v2() || global_metrics.swidth.is_some(),
                return Err(E::unexpected_directive(line.number)) }
                global_metrics.swidth = Some(bdf_try!(read_number_pair(line)));
                continue;
            }
            if line.is(b"DWIDTH") {
                is! { version.is_v2() || global_metrics.dwidth.is_some(),
                return Err(E::unexpected_directive(line.number)) }
                global_metrics.dwidth = Some(bdf_try!(read_i32_pair(line)));
                continue;
            }
            if line.is(b"SWIDTH1") {
                is! { version.is_v2() || global_metrics.swidth1.is_some(),
                return Err(E::unexpected_directive(line.number)) }
                global_metrics.swidth1 = Some(bdf_try!(read_number_pair(line)));
                continue;
            }
            if line.is(b"DWIDTH1") {
                is! { version.is_v2() || global_metrics.dwidth1.is_some(),
                return Err(E::unexpected_directive(line.number)) }
                global_metrics.dwidth1 = Some(bdf_try!(read_i32_pair(line)));
                continue;
            }
            if line.is(b"VVECTOR") {
                is! { version.is_v2() || global_metrics.vvector.is_some(),
                return Err(E::unexpected_directive(line.number)) }
                global_metrics.vvector = Some(bdf_try!(read_i32_pair(line)));
                continue;
            }
            return Err(E::unexpected_directive(line.number));
        }
    }
}

/* parsing helpers */

const fn read_startfont(reader: &mut BdfReader<'_>) -> BdfResult<BdfVersion> {
    let line = bdf_try!(reader.required());
    let line = bdf_try!(line.expect(b"STARTFONT"));
    let mut fields = line.fields();
    let token = bdf_try!(fields.token());
    bdf_try!(fields.finish());
    let version = bdf_try!(parse_version(token, line.number));
    BdfVersion::from_version(version)
}
const fn parse_version(token: &[u8], line: u32) -> BdfResult<Version> {
    let mut scanner = TextScanner::from_bytes(token);
    let major = unwrap![ok_err_map? scanner.expect_ascii_u64(), |__|E::invalid_value(line)];
    is! { !scanner.eat_byte(b'.'), return Err(E::invalid_value(line)) }
    let minor = unwrap![ok_err_map? scanner.expect_ascii_u64(), |__|E::invalid_value(line)];
    if !scanner.is_eof() || major > u16::MAX as u64 || minor > u16::MAX as u64 {
        return Err(E::invalid_value(line));
    }
    Ok(Version::new(major as u16, minor as u16, 0))
}
const fn read_properties_section(
    reader: &mut BdfReader<'_>,
    start_line: BdfLine<'_>,
) -> BdfResult<BdfSection> {
    let mut fields = start_line.fields();
    let count = bdf_try!(fields.usize());
    bdf_try!(fields.finish());
    let start = reader.pos();
    whilst! { index in 0..count; {
        let _property = bdf_try!(reader.required());
    }}
    let end = reader.pos();
    let end_line = bdf_try!(reader.required());
    let end_line = bdf_try!(end_line.expect(b"ENDPROPERTIES"));
    let mut fields = end_line.fields();
    bdf_try!(fields.finish());
    Ok(BdfSection { start, end, count })
}
const fn read_i32_pair(line: BdfLine<'_>) -> BdfResult<[i32; 2]> {
    let mut fields = line.fields();
    let x = bdf_try!(fields.i32());
    let y = bdf_try!(fields.i32());
    bdf_try!(fields.finish());
    Ok([x, y])
}
const fn read_number_pair(line: BdfLine<'_>) -> BdfResult<[BdfNumber; 2]> {
    let mut fields = line.fields();
    let x = bdf_try!(fields.number());
    let y = bdf_try!(fields.number());
    bdf_try!(fields.finish());
    Ok([x, y])
}
const fn bytes_eq(a: &[u8], b: &[u8]) -> bool {
    is! { a.len() != b.len(), return false }
    whilst! { i in 0..a.len(); {
        is! { a[i] != b[i], return false }
    }}
    true
}
