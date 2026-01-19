// devela::work::process::output
//
//! Defines the [`OutputExt`] trait.
//

use crate::text::InvalidUtf8;
use crate::work::process::{ExitStatusError, Output};

/// Marker trait to prevent downstream implementations of the [`OutputExt`] trait.
trait Sealed {}
impl Sealed for Output {}

#[doc = crate::_tags!(platform concurrency namespace)]
/// Extension trait providing convenience methods interpreting process [`Output`].
#[doc = crate::_doc_location!("work/process")]
///
/// This trait provides ergonomic helpers for common patterns such as
/// decoding stdout as text or asserting successful termination.
#[rustfmt::skip]
#[cfg_attr(nightly_doc, doc(notable_trait))]
#[expect(private_bounds, reason = "Sealed")]
pub trait OutputExt: Sealed {
    /// Returns stdout as a UTF-8 string.
    ///
    /// This performs strict UTF-8 decoding.
    ///
    /// # Errors
    /// Returns [`InvalidUtf8`] if stdout is not valid UTF-8.
    fn stdout_string(self) -> Result<String, InvalidUtf8>; // FUTURE IMPROVE error type

    /// Returns stdout as a UTF-8 string, replacing invalid sequences.
    ///
    /// Invalid UTF-8 sequences are replaced with the Unicode replacement character (�).
    fn stdout_string_lossy(self) -> String;

    /// Ensures the process exited successfully.
    ///
    /// Returns `self` if the exit status indicates success.
    ///
    /// # Errors
    /// Returns [`ExitStatusError`] if the exit status indicates failure.
    fn exit_ok(self) -> Result<Self, ExitStatusError> where Self: Sized;
}
#[rustfmt::skip]
impl OutputExt for Output {
    fn stdout_string(self) -> Result<String, InvalidUtf8> {
        String::from_utf8(self.stdout).map_err(|e| e.utf8_error().into())
    }
    fn stdout_string_lossy(self) -> String {
        String::from_utf8_lossy(&self.stdout).into_owned()
    }
    fn exit_ok(self) -> Result<Self, ExitStatusError> {
        if self.status.success() { Ok(self) }
        else { Err(ExitStatusError { status: self.status }) }
    }
}
