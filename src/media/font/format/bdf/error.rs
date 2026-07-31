// devela/src/media/font/format/bdf/error.rs
//
//! Defines [`BdfError`].
//

use crate::{Debug, Display, Error, FmtResult, Formatter, Version, write};

#[doc = crate::_tags!(font error_composite)]
/// An error encountered while parsing BDF data.
#[doc = crate::_doc_meta!{location("media/font")}]
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BdfError {
    /// The input ended before a required BDF directive.
    UnexpectedEof {
        /// One-based line at which the directive was expected.
        line: u32,
    },

    /// A directive was absent, misplaced, duplicated, or not recognized.
    UnexpectedDirective {
        /// One-based source line.
        line: u32,
    },

    /// A directive contained a malformed or out-of-range value.
    InvalidValue {
        /// One-based source line.
        line: u32,
    },

    /// The declared BDF version is syntactically valid but unsupported.
    UnsupportedVersion(Version),
}
impl Error for BdfError {}
impl Display for BdfError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
        match *self {
            Self::UnexpectedEof { line } => {
                write!(f, "unexpected end of BDF data at line {line}")
            }
            Self::UnexpectedDirective { line } => {
                write!(f, "unexpected BDF directive at line {line}")
            }
            Self::InvalidValue { line } => {
                write!(f, "invalid BDF value at line {line}")
            }
            Self::UnsupportedVersion(version) => {
                write!(f, "unsupported BDF version {version}")
            }
        }
    }
}
impl BdfError {
    pub(crate) const fn invalid_value(line: u32) -> Self {
        Self::InvalidValue { line }
    }
    pub(crate) const fn unexpected_directive(line: u32) -> Self {
        Self::UnexpectedDirective { line }
    }
    pub(crate) const fn unexpected_eof(line: u32) -> Self {
        Self::UnexpectedEof { line }
    }
}
