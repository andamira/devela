// devela::sys::net::http::request
//
//! Defines [`HttpRequestLine`].
//

use crate::{Digits, Str, is, unwrap, whilst, write_at};
use crate::{HttpError, HttpMethod, HttpStatus, HttpVersion, TextScanner};

#[doc = crate::_tags!(network protocol parser lifetime)]
/// Borrowed HTTP request line.
#[doc = crate::_doc_meta!{location("sys/net/http")}]
///
/// Represents `METHOD SP request-target SP HTTP-version`.
///
/// This type does not include headers or a message body.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct HttpRequestLine<'a> {
    method: HttpMethod<'a>,
    target: &'a str,
    version: HttpVersion,
}
impl<'a> HttpRequestLine<'a> {
    /// Parses request-line content without its trailing CRLF.
    pub const fn parse(line: &'a [u8]) -> Result<Self, HttpError> {
        is! { line.is_empty(), return Err(HttpError::EmptyRequestLine) }
        let mut scan = TextScanner::from_bytes(line);
        /* method */
        let method_range = scan.take_until_byte(b' ');
        is! { method_range.is_empty(), return Err(HttpError::MissingMethod) }
        is! { scan.expect_byte(b' ').is_err(), return Err(HttpError::MissingTarget) }
        /* request target */
        let target_range = scan.take_until_byte(b' ');
        is! { target_range.is_empty(), return Err(HttpError::MissingTarget) }
        is! { scan.expect_byte(b' ').is_err(), return Err(HttpError::MissingVersion) }
        /* version */
        let version_range = scan.take_until_byte(b' ');
        is! { version_range.is_empty(), return Err(HttpError::MissingVersion) }
        is! { !scan.is_eof(), return Err(HttpError::TrailingData); }
        /* validate and construct */
        let Some(method_str) = scan.slice_str(method_range) else {
            return Err(HttpError::InvalidMethod);
        };
        let method = match HttpMethod::parse(method_str) {
            Ok(method) => method,
            Err(_) => return Err(HttpError::InvalidMethod),
        };
        let target_bytes = scan.slice(target_range);
        is! { !Self::is_valid_target(target_bytes), return Err(HttpError::InvalidTarget) }
        let Some(target) = scan.slice_str(target_range) else {
            return Err(HttpError::InvalidTarget);
        };
        let Some(version) = HttpVersion::parse_http1_token(scan.slice(version_range)) else {
            return Err(HttpError::InvalidVersion);
        };
        Ok(Self { method, target, version })
    }
    /// Returns the request method.
    #[must_use]
    pub const fn method(self) -> HttpMethod<'a> {
        self.method
    }
    /// Returns the raw request target.
    #[must_use]
    pub const fn target(self) -> &'a str {
        self.target
    }
    /// Returns the HTTP version.
    #[must_use]
    pub const fn version(self) -> HttpVersion {
        self.version
    }
    /// Returns the origin-form path without its query string.
    ///
    /// Returns `None` when the request target is not origin-form.
    #[must_use]
    pub const fn origin_path(self) -> Option<&'a str> {
        let bytes = self.target.as_bytes();
        is! { bytes.is_empty() || bytes[0] != b'/', return None }
        whilst! { i in 1..bytes.len(); {
            if bytes[i] == b'?' { return Some(Str::range_to(self.target, i)); }
        }}
        Some(self.target)
    }
    /// Returns whether a request target contains no forbidden whitespace
    /// or control bytes.
    const fn is_valid_target(bytes: &[u8]) -> bool {
        is! { bytes.is_empty(), return false }
        whilst! { i in 0..bytes.len(); {
            let byte = bytes[i];
            // Reject SP, control bytes and DEL.
            if byte <= b' ' || byte == 0x7f { return false; }
        }}
        true
    }
}
