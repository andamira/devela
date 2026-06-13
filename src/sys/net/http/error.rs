// devela/src/sys/net/http/error.rs
//
//! Defines [`HttpError`].
//

#[doc = crate::_tags!(network protocol error)]
/// HTTP parsing and formatting error.
#[doc = crate::_doc_meta!{location("sys/net/http")}]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum HttpError {
    /// The request line is empty.
    EmptyRequestLine,

    /// The request line is missing the method token.
    MissingMethod,

    /// The request line is missing the request-target token.
    MissingTarget,

    /// The request line is missing the HTTP-version token.
    MissingVersion,

    /// The request line contains extra data after the HTTP-version token.
    TrailingData,

    /// A response header value contains a forbidden control byte.
    InvalidHeaderValue,

    /// The method is not a valid HTTP token.
    InvalidMethod,

    /// The request target is malformed.
    InvalidTarget,

    /// The HTTP version token is invalid or unsupported.
    InvalidVersion,

    /// A required CRLF line ending was not found.
    MissingLineEnding,

    /// The destination buffer is too small.
    ///
    /// Contains the minimum required length in bytes.
    NotEnoughSpace(usize),
}
impl HttpError {
    /// Returns the minimum required buffer length, when known.
    #[must_use]
    pub const fn needed_space(self) -> Option<usize> {
        match self {
            Self::NotEnoughSpace(needed) => Some(needed),
            _ => None,
        }
    }
}
crate::_impl_init! { Self::EmptyRequestLine => HttpError }

crate::impl_trait! {
    fmt::Display+Error for HttpError |self, f| match self {
        Self::EmptyRequestLine => {
            f.write_str("The HTTP request line is empty.")
        }
        Self::MissingMethod => {
            f.write_str("The HTTP request line is missing the method.")
        }
        Self::MissingTarget => {
            f.write_str("The HTTP request line is missing the request target.")
        }
        Self::MissingVersion => {
            f.write_str("The HTTP request line is missing the protocol version.")
        }
        Self::TrailingData => {
            f.write_str("The HTTP request line contains trailing data.")
        }
        Self::InvalidHeaderValue => {
            f.write_str("The HTTP header value contains a forbidden control byte.")
        }
        Self::InvalidMethod => {
            f.write_str("The HTTP method token is invalid.")
        }
        Self::InvalidTarget => {
            f.write_str("The HTTP request target is invalid.")
        }
        Self::InvalidVersion => {
            f.write_str("The HTTP version is invalid or unsupported.")
        }
        Self::MissingLineEnding => {
            f.write_str("The HTTP message is missing a required CRLF line ending.")
        }
        Self::NotEnoughSpace(needed) => {
            write!(
                f,
                "The destination buffer is too small. It needs at least {needed} bytes."
            )
        }
    }
}
