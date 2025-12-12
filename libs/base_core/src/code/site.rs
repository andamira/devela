// devela_base_core::code::site
//
//! Defines [`CodeLocation`], [`CodeSpan`].
//

use crate::{Display, FmtResult, Formatter, Slice};

/// A precise location in the source code.
///
/// Captures the module path, file name, line, and column of a specific
/// invocation site. Intended as a lightweight, zero-cost provenance
/// primitive usable across diagnostics, logging, errors, and profiling.
///
/// Instances are typically created via [`CodeLocation::here`], which
/// records the current call site using compile-time macros.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CodeLocation {
    /// The module path.
    pub module: &'static str,
    /// The file name.
    pub file: &'static str,
    /// The line number.
    pub line: u32,
    /// The column number.
    pub column: u32,
}

impl Display for CodeLocation {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
        write!(f, "{} ({}:{}:{})", self.module, self.file, self.line, self.column)
    }
}

impl CodeLocation {
    /* constructors */

    /// Returns the code location of the current invocation site.
    #[inline(always)]
    pub const fn here() -> Self {
        CodeLocation {
            module: crate::code_module!(),
            file: crate::code_file!(),
            line: crate::code_line!(),
            column: crate::code_column!(),
        }
    }

    /* deconstructors */

    /// Returns the file name and line number.
    pub const fn file_line(&self) -> (&'static str, u32) {
        (self.file, self.line)
    }

    /// Returns the file name, and line and column numbers.
    pub const fn file_line_column(&self) -> (&'static str, u32, u32) {
        (self.file, self.line, self.column)
    }

    /* */

    /// Compile-time equality comparison.
    pub const fn eq(&self, other: &Self) -> bool {
        self.line == other.line
            && self.column == other.column
            && Slice::<&str>::eq(self.file, other.file)
            && Slice::<&str>::eq(self.module, other.module)
    }

    /// Short display helper.
    pub fn fmt_short(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
        write!(f, "{}:{}:{}", self.file, self.line, self.column)
    }
}

/// A contiguous span between two code locations.
///
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
            write!(
                f,
                "{}:{}:{} – {}:{}:{}",
                self.start.file,
                self.start.line,
                self.start.column,
                self.end.line,
                self.end.column,
                self.end.module
            )
        }
    }
}
