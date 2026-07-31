// devela/src/media/font/format/bdf/_parse/syntax.rs
//
//! Defines `BdfReader`, `BdfLine`, `BdfFields`.
//

use super::{BdfNumber, BdfResult, bdf_try};
use crate::{BdfError as E, Slice, TextScanner, is, unwrap, whilst};

#[derive(Clone, Debug)]
pub(crate) struct BdfReader<'a> {
    scanner: TextScanner<'a>,
    next_line: u32,
}
impl<'a> BdfReader<'a> {
    pub(super) const fn new(bytes: &'a [u8]) -> Self {
        Self {
            scanner: TextScanner::from_bytes(bytes),
            next_line: 1,
        }
    }
    pub(super) const fn pos(&self) -> usize {
        self.scanner.pos().as_usize()
    }
    pub(super) const fn line_number(&self) -> u32 {
        self.next_line
    }
    pub(super) const fn source(&self, start: usize, end: usize) -> &'a [u8] {
        Slice::range(self.scanner.bytes(), start, end)
    }
    pub(super) const fn next(&mut self) -> BdfResult<Option<BdfLine<'a>>> {
        let range = unwrap![some_or self.scanner.next_line(), return Ok(None)];
        let number = self.next_line;
        self.next_line = self.next_line.saturating_add(1);
        unwrap![ok_map BdfLine::read(self.scanner.slice(range), number), |v| Some(v)]
    }
    pub(super) const fn required(&mut self) -> BdfResult<BdfLine<'a>> {
        unwrap![some_ok_or bdf_try!(self.next()), E::unexpected_eof(self.next_line)]
    }
    /// Reads the next non-`COMMENT` line.
    pub(super) const fn required_data(&mut self) -> BdfResult<BdfLine<'a>> {
        loop {
            let line = bdf_try!(self.required());
            is! { !line.is(b"COMMENT"), return Ok(line) }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct BdfLine<'a> {
    pub(super) keyword: &'a [u8],
    pub(super) value: &'a [u8],
    pub(super) number: u32,
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
    /// Returns the sole token on this line.
    pub(super) const fn bare(self) -> BdfResult<&'a [u8]> {
        is! { self.value.is_empty(), Ok(self.keyword), Err(E::invalid_value(self.number)) }
    }
    pub(super) const fn is(self, keyword: &[u8]) -> bool {
        bytes_eq(self.keyword, keyword)
    }
    pub(super) const fn expect(self, keyword: &[u8]) -> BdfResult<Self> {
        is! { self.is(keyword), Ok(self), Err(E::unexpected_directive(self.number)) }
    }
    pub(super) const fn fields(self) -> BdfFields<'a> {
        BdfFields {
            scanner: TextScanner::from_bytes(self.value),
            line: self.number,
        }
    }
    pub(super) const fn trimmed_value(self) -> &'a [u8] {
        let mut end = self.value.len();
        while end != 0 && matches!(self.value[end - 1], b' ' | b'\t') {
            end -= 1;
        }
        Slice::range_to(self.value, end)
    }
}

#[derive(Clone, Debug)]
pub(crate) struct BdfFields<'a> {
    scanner: TextScanner<'a>,
    line: u32,
}
impl<'a> BdfFields<'a> {
    /// Returns whether no non-horizontal-whitespace fields remain.
    pub(super) const fn is_empty(&mut self) -> bool {
        self.scanner.skip_ascii_hws();
        self.scanner.is_eof()
    }
    pub(super) const fn token(&mut self) -> BdfResult<&'a [u8]> {
        self.scanner.skip_ascii_hws();
        is! { self.scanner.is_eof(), return Err(E::invalid_value(self.line)) }
        let range = self.scanner.take_until_any2(b' ', b'\t');
        let token = self.scanner.slice(range);
        is! { token.is_empty(), Err(E::invalid_value(self.line)), Ok(token) }
    }
    pub(super) const fn finish(&mut self) -> BdfResult<()> {
        self.scanner.skip_ascii_hws();
        is! { self.scanner.is_eof(), Ok(()), Err(E::invalid_value(self.line)) }
    }
    pub(super) const fn u64(&mut self) -> BdfResult<u64> {
        let token = bdf_try!(self.token());
        let mut scanner = TextScanner::from_bytes(token);
        let value = unwrap![ok_err_map? scanner.expect_ascii_u64(),
            |__| E::invalid_value(self.line)];
        is! { scanner.is_eof(), Ok(value), Err(E::invalid_value(self.line)) }
    }
    pub(super) const fn usize(&mut self) -> BdfResult<usize> {
        let token = bdf_try!(self.token());
        let mut scanner = TextScanner::from_bytes(token);
        let value = unwrap![ok_err_map? scanner.expect_ascii_usize(),
            |__| E::invalid_value(self.line)];
        is! { scanner.is_eof(), Ok(value), Err(E::invalid_value(self.line)) }
    }
    pub(super) const fn i64(&mut self) -> BdfResult<i64> {
        let token = bdf_try!(self.token());
        let mut scanner = TextScanner::from_bytes(token);
        let value = unwrap![ok_err_map? scanner.expect_ascii_i64(),
            |__| E::invalid_value(self.line)];
        is! { scanner.is_eof(), Ok(value), Err(E::invalid_value(self.line)) }
    }
    pub(super) const fn u32(&mut self) -> BdfResult<u32> {
        let value = bdf_try!(self.u64());
        is! { value > u32::MAX as u64, Err(E::invalid_value(self.line)), Ok(value as u32) }
    }
    pub(super) const fn i32(&mut self) -> BdfResult<i32> {
        let value = bdf_try!(self.i64());
        if value < i32::MIN as i64 || value > i32::MAX as i64 {
            Err(E::invalid_value(self.line))
        } else {
            Ok(value as i32)
        }
    }
    pub(super) const fn number(&mut self) -> BdfResult<BdfNumber> {
        let token = bdf_try!(self.token());
        BdfNumber::parse(token, self.line)
    }
}

/* helpers */

const fn bytes_eq(a: &[u8], b: &[u8]) -> bool {
    is! { a.len() != b.len(), return false }
    whilst! { i in 0..a.len(); {
        is! { a[i] != b[i], return false }
    }}
    true
}
