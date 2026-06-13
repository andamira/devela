// devela/src/sys/net/http/response.rs
//
//! Defines [`HttpResponseHead`].
//

use crate::{Digits, Str, is, unwrap, whilst, write_at};
use crate::{HttpError, HttpMethod, HttpStatus, HttpVersion, TextScanner};

#[doc = crate::_tags!(network protocol)]
/// HTTP response head.
#[doc = crate::_doc_meta!{location("sys/net/http")}]
///
/// Represents an HTTP status line and basic representation headers.
///
/// The current encoder writes the textual HTTP/1 wire format.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct HttpResponseHead<'a> {
    version: HttpVersion,
    status: HttpStatus,
    content_type: Option<&'a str>,
    content_length: Option<usize>,
}
impl<'a> HttpResponseHead<'a> {
    /// Creates an HTTP/1.1 response head with the given status.
    #[must_use]
    pub const fn new(status: HttpStatus) -> Self {
        Self {
            version: HttpVersion::Http11,
            status,
            content_type: None,
            content_length: None,
        }
    }
    /// Creates a `200 OK` HTTP/1.1 response head.
    #[must_use]
    pub const fn ok() -> Self {
        Self::new(HttpStatus::OK)
    }
    /// Creates a `404 Not Found` HTTP/1.1 response head.
    #[must_use]
    pub const fn not_found() -> Self {
        Self::new(HttpStatus::NOT_FOUND)
    }
    /// Sets the HTTP version.
    #[must_use]
    pub const fn with_version(mut self, version: HttpVersion) -> Self {
        self.version = version;
        self
    }
    /// Sets the `Content-Type` header.
    ///
    /// The value is validated when the response head is encoded.
    #[must_use]
    pub const fn with_content_type(mut self, content_type: &'a str) -> Self {
        self.content_type = Some(content_type);
        self
    }
    /// Sets the `Content-Length` header.
    #[must_use]
    pub const fn with_content_length(mut self, len: usize) -> Self {
        self.content_length = Some(len);
        self
    }
    /// Returns the HTTP version.
    #[must_use]
    pub const fn version(self) -> HttpVersion {
        self.version
    }
    /// Returns the response status.
    #[must_use]
    pub const fn status(self) -> HttpStatus {
        self.status
    }
    /// Returns the `Content-Type` value, when set.
    #[must_use]
    pub const fn content_type(self) -> Option<&'a str> {
        self.content_type
    }
    /// Returns the `Content-Length` value, when set.
    #[must_use]
    pub const fn content_length(self) -> Option<usize> {
        self.content_length
    }
    /// Returns the encoded HTTP/1 response-head length.
    ///
    /// Returns an error when the configured version has no HTTP/1 textual
    /// representation or a configured header value is invalid.
    pub const fn http1_encoded_len(self) -> Result<usize, HttpError> {
        let Some(version) = self.version.http1_token() else {
            return Err(HttpError::InvalidVersion);
        };
        let reason = match self.status.reason() {
            Some(reason) => reason,
            None => "",
        };
        let mut len = version.len()
            + 1 // SP
            + 3 // status code
            + 1 // SP
            + reason.len()
            + 2; // CRLF
        if let Some(content_type) = self.content_type {
            if !Self::is_valid_header_value(content_type.as_bytes()) {
                return Err(HttpError::InvalidHeaderValue);
            }
            len += b"Content-Type: ".len() + content_type.len() + 2; // CRLF
        }
        if let Some(content_length) = self.content_length {
            len += b"Content-Length: ".len() + Digits(content_length).count_digits10() as usize + 2; // CRLF
        }
        Ok(len + 2) // final CRLF
    }
    /// Writes the textual HTTP/1 response head into `dst`.
    ///
    /// Returns the number of bytes written.
    ///
    /// The operation is atomic: if an error is returned, `dst` is unchanged.
    pub const fn write_http1_into(self, dst: &mut [u8]) -> Result<usize, HttpError> {
        let needed = match self.http1_encoded_len() {
            Ok(needed) => needed,
            Err(error) => return Err(error),
        };
        if dst.len() < needed {
            return Err(HttpError::NotEnoughSpace(needed));
        }
        let Some(version) = self.version.http1_token() else {
            return Err(HttpError::InvalidVersion);
        };
        let reason = match self.status.reason() {
            Some(reason) => reason,
            None => "",
        };
        let mut pos = 0;
        /* status line */
        write_at!(dst, +=pos, @version.as_bytes(), b' ');
        pos += Digits(self.status.code()).write_digits10(dst, pos);
        write_at!(dst, +=pos, b' ', @reason.as_bytes(), @b"\r\n");
        /* headers */
        if let Some(content_type) = self.content_type {
            write_at!(dst, +=pos, @b"Content-Type: ", @content_type.as_bytes(), @b"\r\n");
        }
        if let Some(content_length) = self.content_length {
            write_at!(dst, +=pos, @b"Content-Length: ");
            pos += Digits(content_length).write_digits10(dst, pos);
            write_at!(dst, +=pos, @b"\r\n");
        }
        /* end of head */
        write_at!(dst, +=pos, @b"\r\n");
        Ok(pos)
    }
    /// Returns whether `bytes` form a safe HTTP field value.
    const fn is_valid_header_value(bytes: &[u8]) -> bool {
        whilst! { i in 0..bytes.len(); {
            let byte = bytes[i];
            // Permit HTAB, visible ASCII and non-ASCII opaque bytes.
            // Reject CR, LF, DEL and the remaining control bytes.
            if (byte < b' ' && byte != b'\t') || byte == 0x7f { return false; }
        }}
        true
    }
}
