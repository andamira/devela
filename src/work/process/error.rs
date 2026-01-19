// devela::work::process::error
//
//! Defines [`ExitStatusError`].
//

use crate::{ExitStatus, impl_trait};

#[doc = crate::_tags!(platform runtime error)]
/// Indicates that a process terminated unsuccessfully.
#[doc = crate::_doc_location!("work/process")]
/// This type is a stable placeholder for the unstable standard library's
/// [`ExitStatusError`](https://doc.rust-lang.org/std/process/struct.ExitStatusError.html)
// WAIT: [ExitStatusError](https://github.com/rust-lang/rust/issues/84908)
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ExitStatusError {
    pub(crate) status: ExitStatus,
}

impl_trait!(fmt::Display+Error for ExitStatusError |self, f| {
    write!(f, "process exited unsuccessfully: {}", self.status)
});

impl ExitStatusError {
    /// Reports the exit code, if applicable, from an ExitStatusError.
    pub fn code(&self) -> Option<i32> {
        self.status.code()
    }

    /// Converts an `ExitStatusError` (back) to an `ExitStatus`.
    pub const fn into_status(&self) -> ExitStatus {
        self.status
    }
}
