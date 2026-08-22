// devela/src/code/source/location.rs
//
//! Defines [`CodeSpan`].
//

use crate::{CodeLocation, Display, FmtResult, Formatter};

#[doc = crate::_tags!(code)]
/// A contiguous span between two code locations.
#[doc = crate::_doc_meta!{
    location("code/source", struct CodeSpan),
    #[cfg(target_pointer_width = "32")]
    test_size_of(CodeSpan = 48|384; niche Option),
    #[cfg(target_pointer_width = "64")]
    test_size_of(CodeSpan = 80|640; niche Option),
}]
/// Represents a range in the source code, typically describing where a construct,
/// operation, or effect originates. The span is inclusive of both endpoints
/// and carries no semantic meaning beyond positional ordering.
///
/// `CodeSpan` is purely structural and does not assume ordering across different files or modules.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CodeSpan {
    /// The start code location.
    pub start: CodeLocation,
    /// The end code location.
    pub end: CodeLocation,
}

impl CodeSpan {
    /// Creates a span from two code locations.
    pub const fn new(start: CodeLocation, end: CodeLocation) -> Self {
        Self { start, end }
    }
    /// Creates a zero-length span at the current invocation site.
    pub const fn here() -> Self {
        let loc = CodeLocation::here();
        Self { start: loc, end: loc }
    }
    /// Returns true if the span represents a single point.
    pub const fn is_point(&self) -> bool {
        self.start.eq(&self.end)
    }
}

impl Display for CodeSpan {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
        if self.is_point() {
            write!(f, "{}", self.start)
        } else {
            write!(f, "{} – {}", self.start, self.end)
        }
    }
}
