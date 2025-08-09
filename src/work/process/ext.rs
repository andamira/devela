// devela::work::process::ext
//
//! Defines the [`ExtProcess`] trait.
//

use crate::{OsStr, Process, ProcessCommand};
use std::process::{abort, exit, id};

/// Marker trait to prevent downstream implementations of the [`ExtThread`] trait.
trait Sealed {}
impl Sealed for Process {}

#[doc = crate::TAG_NAMESPACE!()]
/// Extension trait providing additional methods for [`Process`]es.
///
/// It offers the standalone functions in `std::process` as associated methods.
#[rustfmt::skip]
#[cfg_attr(nightly_doc, doc(notable_trait))]
#[expect(private_bounds, reason = "Sealed")]
pub trait ExtProcess: Sealed {
    /// Constructs a new `ProcessCommand` for launching the `program`.
    ///
    /// See `ProcessCommand::`[new][ProcessCommand::new].
    fn command<S: AsRef<OsStr>>(program: S) -> ProcessCommand {
        ProcessCommand::new(program)
    }

    /// Terminates the current process in an abnormal fashion.
    ///
    /// See `std::process::`[abort].
    #[rustfmt::skip]
    fn abort() -> ! { abort() }

    /// Terminates the current process with the specified exit code.
    ///
    /// See `std::process::`[exit].
    #[rustfmt::skip]
    fn exit(code: i32) -> ! { exit(code) }


    /// Returns the OS-assigned process identifier associated with this process.
    ///
    /// See `std::process::`[id].
    #[must_use] #[rustfmt::skip]
    fn id() -> u32 { id() }
}
impl ExtProcess for Process {}
