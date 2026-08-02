// devela/src/media/font/format/bdf/_parse/header.rs
//
//! Defines `BdfVersion`, `BdfSection`, `BdfHeader`.
//

#![allow(dead_code, reason = "staged BDF parser implementation")]

use super::{BdfLine, BdfNumber, BdfReader, BdfResult, bdf_try};
use crate::{BdfError as E, Region2, TextScanner, Version, is, unwrap, whilst};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum BdfVersion {
    V2_1,
    V2_2,
}
impl BdfVersion {
    pub(crate) const fn read(bytes: &[u8]) -> BdfResult<Self> {
        let mut reader = BdfReader::new(bytes);
        read_startfont(&mut reader)
    }
    pub(crate) const fn to_version(self) -> Version {
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
    const fn supports_global_metrics(self) -> bool {
        matches![self, Self::V2_2]
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub(crate) struct BdfMetrics {
    pub(super) swidth: Option<[BdfNumber; 2]>,
    pub(super) dwidth: Option<[i32; 2]>,
    pub(super) swidth1: Option<[BdfNumber; 2]>,
    pub(super) dwidth1: Option<[i32; 2]>,
    pub(super) vvector: Option<[i32; 2]>,
}
impl BdfMetrics {
    pub(super) const EMPTY: Self = Self {
        swidth: None,
        dwidth: None,
        swidth1: None,
        dwidth1: None,
        vvector: None,
    };

    /// Fills absent local metrics from the global font metrics.
    const fn with_defaults(self, defaults: Self) -> Self {
        Self {
            swidth: unwrap![=some_or self.swidth, defaults.swidth],
            dwidth: unwrap![=some_or self.dwidth, defaults.dwidth],
            swidth1: unwrap![=some_or self.swidth1, defaults.swidth1],
            dwidth1: unwrap![=some_or self.dwidth1, defaults.dwidth1],
            vvector: unwrap![=some_or self.vvector, defaults.vvector],
        }
    }
    /// Validates the effective metrics for the declared writing directions.
    const fn validate(self, metrics_set: u8, line: u32) -> BdfResult<Self> {
        let valid = match metrics_set {
            // Writing direction 0
            0 => {
                self.swidth.is_some()
                    && self.dwidth.is_some()
                    && self.swidth1.is_none()
                    && self.dwidth1.is_none()
            }
            // Writing direction 1. Direction-0 metrics are optional
            1 => self.swidth1.is_some() && self.dwidth1.is_some() && self.vvector.is_some(),
            // Both writing directions
            2 => {
                self.swidth.is_some()
                    && self.dwidth.is_some()
                    && self.swidth1.is_some()
                    && self.dwidth1.is_some()
                    && self.vvector.is_some()
            }
            _ => false,
        };
        is! { valid, Ok(self), Err(E::invalid_value(line)) }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct BdfSection {
    start: usize,
    end: usize,
    count: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct BdfHeader<'a> {
    version: BdfVersion,
    name: &'a [u8],
    point_size: BdfNumber,
    resolution: [u32; 2],
    pub(crate) bounds: Region2<i32, u32>,
    content_version: Option<i64>,
    metrics_set: u8,
    global_metrics: BdfMetrics,
    properties: Option<BdfSection>,
    pub(crate) glyph_count: usize,
    glyphs_offset: usize,
}
impl<'a> BdfHeader<'a> {
    pub(super) const fn resolve_metrics(
        &self,
        local: BdfMetrics,
        line: u32,
    ) -> BdfResult<BdfMetrics> {
        local.with_defaults(self.global_metrics).validate(self.metrics_set, line)
    }
    pub(super) const fn supports_vertical_metrics(&self) -> bool {
        matches!(self.version, BdfVersion::V2_2)
    }
    pub(crate) const fn read(bytes: &'a [u8]) -> BdfResult<Self> {
        let mut reader = BdfReader::new(bytes);
        Self::read_from(&mut reader)
    }
    pub(super) const fn read_from(reader: &mut BdfReader<'a>) -> BdfResult<Self> {
        let version = bdf_try!(read_startfont(reader));
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
                is! { properties.is_some(), return Err(E::unexpected_directive(line.number)) }
                properties = Some(bdf_try!(read_properties_section(reader, line)));
                continue;
            }
            if line.is(b"METRICSSET") {
                is! { !version.supports_global_metrics() || has_metrics_set,
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
                is! { !version.supports_global_metrics() || global_metrics.swidth.is_some(),
                return Err(E::unexpected_directive(line.number)) }
                global_metrics.swidth = Some(bdf_try!(read_number_pair(line)));
                continue;
            }
            if line.is(b"DWIDTH") {
                is! { !version.supports_global_metrics() || global_metrics.dwidth.is_some(),
                return Err(E::unexpected_directive(line.number)) }
                global_metrics.dwidth = Some(bdf_try!(read_i32_pair(line)));
                continue;
            }
            if line.is(b"SWIDTH1") {
                is! { !version.supports_global_metrics() || global_metrics.swidth1.is_some(),
                return Err(E::unexpected_directive(line.number)) }
                global_metrics.swidth1 = Some(bdf_try!(read_number_pair(line)));
                continue;
            }
            if line.is(b"DWIDTH1") {
                is! { !version.supports_global_metrics() || global_metrics.dwidth1.is_some(),
                return Err(E::unexpected_directive(line.number)) }
                global_metrics.dwidth1 = Some(bdf_try!(read_i32_pair(line)));
                continue;
            }
            if line.is(b"VVECTOR") {
                is! { !version.supports_global_metrics() || global_metrics.vvector.is_some(),
                return Err(E::unexpected_directive(line.number)) }
                global_metrics.vvector = Some(bdf_try!(read_i32_pair(line)));
                continue;
            }
            return Err(E::unexpected_directive(line.number));
        }
    }
}

/* helpers */

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
pub(crate) const fn read_i32_pair(line: BdfLine<'_>) -> BdfResult<[i32; 2]> {
    let mut fields = line.fields();
    let x = bdf_try!(fields.i32());
    let y = bdf_try!(fields.i32());
    bdf_try!(fields.finish());
    Ok([x, y])
}
pub(crate) const fn read_number_pair(line: BdfLine<'_>) -> BdfResult<[BdfNumber; 2]> {
    let mut fields = line.fields();
    let x = bdf_try!(fields.number());
    let y = bdf_try!(fields.number());
    bdf_try!(fields.finish());
    Ok([x, y])
}
