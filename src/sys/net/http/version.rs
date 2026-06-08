// devela::sys::net::http::version
//
//! Defines [`HttpVersion`].
//

use crate::{HttpError, HttpRequestLine, HttpStatus, Version};

#[doc = crate::_tags!(network protocol)]
/// HTTP protocol version.
#[doc = crate::_doc_meta!{location("sys/net/http")}]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HttpVersion {
    /// `HTTP/1.0`.
    Http10,
    /// `HTTP/1.1`.
    Http11,
    /// `HTTP/2.0`.
    Http2,
    /// `HTTP/3.0`.
    Http3,
}
impl HttpVersion {
    /// Returns the canonical protocol label.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Http10 => "HTTP/1.0",
            Self::Http11 => "HTTP/1.1",
            Self::Http2 => "HTTP/2",
            Self::Http3 => "HTTP/3",
        }
    }
    /// Returns the major and minor version components.
    #[must_use]
    pub const fn parts(self) -> (u8, u8) {
        (self.major(), self.minor())
    }
    /// Returns the major version component.
    #[must_use]
    pub const fn major(self) -> u8 {
        match self {
            Self::Http10 | Self::Http11 => 1,
            Self::Http2 => 2,
            Self::Http3 => 3,
        }
    }
    /// Returns the minor version component.
    #[must_use]
    pub const fn minor(self) -> u8 {
        match self {
            Self::Http11 => 1,
            Self::Http10 | Self::Http2 | Self::Http3 => 0,
        }
    }
    /// Returns whether this is an HTTP/1 version.
    #[must_use]
    pub const fn is_http1(self) -> bool {
        matches!(self, Self::Http10 | Self::Http11)
    }
    /// Returns the HTTP/1 request-line token, when applicable.
    #[must_use]
    pub const fn http1_token(self) -> Option<&'static str> {
        match self {
            Self::Http10 => Some("HTTP/1.0"),
            Self::Http11 => Some("HTTP/1.1"),
            Self::Http2 | Self::Http3 => None,
        }
    }
    /// Parses an HTTP/1 request-line version token.
    #[must_use]
    pub const fn parse_http1_token(token: &[u8]) -> Option<Self> {
        if token.len() != 8 {
            return None;
        }
        if token[0] != b'H'
            || token[1] != b'T'
            || token[2] != b'T'
            || token[3] != b'P'
            || token[4] != b'/'
            || token[5] != b'1'
            || token[6] != b'.'
        {
            return None;
        }
        match token[7] {
            b'0' => Some(Self::Http10),
            b'1' => Some(Self::Http11),
            _ => None,
        }
    }
}
