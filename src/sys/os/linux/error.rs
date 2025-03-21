// devela::sys::os::linux::error
//
//! Defines [`LinuxError`] and [`LinuxResult`].
//

#[doc = crate::TAG_ERROR_COMPOSITE!()]
/// Represents a Linux-related error.
///
/// Encapsulates errors that can occur when interacting with Linux syscalls or
/// performing Linux-specific operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinuxError {
    /// A syscall error, wrapping the [`LINUX_ERRNO`][crate::LINUX_ERRNO] code.
    Sys(isize),
    /// No input was available (e.g., when reading from stdin).
    NoInput,
    /// The input was not a valid UTF-8 sequence.
    InvalidUtf8,
    // /// A custom error with a static string message.
    // Other(&'static str),
}

#[doc = crate::TAG_RESULT!()]
/// The return type for Linux-related functions that can fail.
pub type LinuxResult<T> = crate::Result<T, LinuxError>;
