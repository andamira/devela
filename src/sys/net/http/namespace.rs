// devela/src/sys/net/http/namespace.rs
//
//! Defines [`Http`], [`HttpMethod`].
//

use crate::{HttpError, HttpRequestLine, HttpStatus};
use crate::{is, whilst};

#[doc = crate::_tags!(network protocol)]
/// HTTP protocol operations.
#[doc = crate::_doc_meta!{location("sys/net/http")}]
#[derive(Debug)]
pub struct Http;
impl Http {
    /// Parses an HTTP request line from borrowed bytes.
    pub const fn parse_request_line(bytes: &[u8]) -> Result<HttpRequestLine<'_>, HttpError> {
        HttpRequestLine::parse(bytes)
    }
    /// Returns the registered reason phrase for `status`, when known.
    pub const fn reason_phrase(status: HttpStatus) -> Option<&'static str> {
        status.reason()
    }
}

#[doc = crate::_tags!(network protocol)]
/// HTTP request method.
#[doc = crate::_doc_meta!{location("sys/net/http")}]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum HttpMethod<'a> {
    /// `GET`.
    Get,
    /// `HEAD`.
    Head,
    /// `POST`.
    Post,
    /// `PUT`.
    Put,
    /// `DELETE`.
    Delete,
    /// `CONNECT`.
    Connect,
    /// `OPTIONS`.
    Options,
    /// `TRACE`.
    Trace,
    /// `PATCH`.
    Patch,
    /// Any other valid method token.
    Other(&'a str),
}
impl<'a> HttpMethod<'a> {
    /// Parses a method token.
    pub const fn parse(token: &'a str) -> Result<Self, HttpError> {
        let bytes = token.as_bytes();
        is! { bytes.is_empty(), return Err(HttpError::InvalidMethod) }
        whilst! { i in 0..bytes.len(); {
            if !Self::is_token_byte(bytes[i]) { return Err(HttpError::InvalidMethod); }
        }}
        let method = if Self::bytes_eq(bytes, b"GET") {
            Self::Get
        } else if Self::bytes_eq(bytes, b"HEAD") {
            Self::Head
        } else if Self::bytes_eq(bytes, b"POST") {
            Self::Post
        } else if Self::bytes_eq(bytes, b"PUT") {
            Self::Put
        } else if Self::bytes_eq(bytes, b"DELETE") {
            Self::Delete
        } else if Self::bytes_eq(bytes, b"CONNECT") {
            Self::Connect
        } else if Self::bytes_eq(bytes, b"OPTIONS") {
            Self::Options
        } else if Self::bytes_eq(bytes, b"TRACE") {
            Self::Trace
        } else if Self::bytes_eq(bytes, b"PATCH") {
            Self::Patch
        } else {
            Self::Other(token)
        };
        Ok(method)
    }

    /// Returns the method as a string slice.
    #[must_use]
    pub const fn as_str(self) -> &'a str {
        match self {
            Self::Get => "GET",
            Self::Head => "HEAD",
            Self::Post => "POST",
            Self::Put => "PUT",
            Self::Delete => "DELETE",
            Self::Connect => "CONNECT",
            Self::Options => "OPTIONS",
            Self::Trace => "TRACE",
            Self::Patch => "PATCH",
            Self::Other(token) => token,
        }
    }

    /// Returns whether this is the `HEAD` method.
    #[must_use]
    pub const fn is_head(self) -> bool {
        matches!(self, Self::Head)
    }

    /// Returns whether `byte` is valid in an HTTP token.
    const fn is_token_byte(byte: u8) -> bool {
        matches!(
            byte,
            b'0'..=b'9'
                | b'A'..=b'Z'
                | b'a'..=b'z'
                | b'!'
                | b'#'
                | b'$'
                | b'%'
                | b'&'
                | b'\''
                | b'*'
                | b'+'
                | b'-'
                | b'.'
                | b'^'
                | b'_'
                | b'`'
                | b'|'
                | b'~'
        )
    }

    /// Compares two byte slices in const contexts.
    const fn bytes_eq(a: &[u8], b: &[u8]) -> bool {
        is! { a.len() != b.len(), return false }
        whilst! { i in 0..a.len(); {
            is! { a[i] != b[i], return false }
        }}
        true
    }
}
