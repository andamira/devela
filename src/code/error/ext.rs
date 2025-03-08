// devela::code::error::ext
//
//! Defines the `ExtError` trait.
//

use crate::Error;

/// Extension trait providing additional methods for `T:`[`Error`].
// #[cfg_attr(nightly_doc, doc(notable_trait))]
pub trait ExtError: Error {
    /// Represents the specific kind of error, if applicable.
    ///
    /// Types implementing this trait may use `Kind` to categorize
    /// errors into meaningful groups for comparison or analysis.
    // WAIT: [associated_type_defaults](https://github.com/rust-lang/rust/issues/29661)
    type Kind; // = ()

    /// Checks if two errors are equivalent based on their kind or other criteria.
    #[must_use]
    fn error_eq(&self, other: &Self) -> bool;

    /// Returns the kind of the error, if applicable.
    #[must_use]
    fn error_kind(&self) -> Self::Kind; // {}
}
