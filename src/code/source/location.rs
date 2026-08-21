// devela/src/code/source/location.rs
//
//! Defines [`CodeLocation`].
//

use crate::{Display, FmtResult, Formatter, Slice};

#[doc = crate::_tags!(code)]
/// A precise location in the source code.
#[doc = crate::_doc_meta!{
    location("code/source", struct CodeLocation),
    #[cfg(target_pointer_width = "32")]
    test_size_of(CodeLocation = 24|192),
    #[cfg(target_pointer_width = "64")]
    test_size_of(CodeLocation = 40|320),
}]
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

    /* misc. */

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
