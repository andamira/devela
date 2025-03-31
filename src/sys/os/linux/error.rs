// devela::sys::os::linux::error
//
//! Defines [`LinuxError`] and [`LinuxResult`].
//

use crate::{IoError, IoErrorKind, LINUX_ERRNO as ERRNO, LINUX_EXIT as EXIT};

#[doc = crate::TAG_RESULT!()]
/// The return type for Linux-related functions that can fail.
pub type LinuxResult<T> = crate::Result<T, LinuxError>;

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
macro_rules! match_linux_to_io {
    ($self:ident) => {
        match $self {
            LinuxError::Sys(errno) => {
                let kind = match errno {
                    ERRNO::EPERM => IoErrorKind::PermissionDenied,
                    ERRNO::ENOENT => IoErrorKind::NotFound,
                    ERRNO::EINTR => IoErrorKind::Interrupted,
                    ERRNO::EIO => IoErrorKind::Other,
                    ERRNO::ENXIO => IoErrorKind::NotFound,
                    ERRNO::EAGAIN => IoErrorKind::WouldBlock,
                    ERRNO::ENOMEM => IoErrorKind::OutOfMemory,
                    ERRNO::EACCES => IoErrorKind::PermissionDenied,
                    ERRNO::EFAULT => IoErrorKind::InvalidInput,
                    ERRNO::EBUSY => IoErrorKind::ResourceBusy,
                    ERRNO::EEXIST => IoErrorKind::AlreadyExists,
                    ERRNO::ENOTDIR => IoErrorKind::NotADirectory,
                    ERRNO::EISDIR => IoErrorKind::IsADirectory,
                    ERRNO::EINVAL => IoErrorKind::InvalidInput,
                    ERRNO::ENOSPC => IoErrorKind::StorageFull,
                    ERRNO::EROFS => IoErrorKind::ReadOnlyFilesystem,
                    ERRNO::EMLINK => IoErrorKind::TooManyLinks,
                    ERRNO::EPIPE => IoErrorKind::BrokenPipe,
                    ERRNO::EDOM => IoErrorKind::InvalidInput,
                    ERRNO::ERANGE => IoErrorKind::InvalidInput,
                    ERRNO::EDEADLK => IoErrorKind::Deadlock,
                    // WAIT:1.87 [io_error_more](https://github.com/rust-lang/rust/pull/134076)
                    // ERRNO::ENAMETOOLONG => IoErrorKind::InvalidFilename,
                    ERRNO::ENOLCK => IoErrorKind::ResourceBusy,
                    ERRNO::ENOSYS => IoErrorKind::Unsupported,
                    ERRNO::ENOTEMPTY => IoErrorKind::DirectoryNotEmpty,
                    // WAIT:1.?? [io_error_more](https://github.com/rust-lang/rust/issues/86442)
                    // ERRNO::ELOOP => IoErrorKind::FilesystemLoop,
                    ERRNO::ENODEV => IoErrorKind::NotFound,
                    ERRNO::ETIMEDOUT => IoErrorKind::TimedOut,
                    ERRNO::EXDEV => IoErrorKind::CrossesDevices,
                    ERRNO::ETXTBSY => IoErrorKind::ExecutableFileBusy,
                    _ => IoErrorKind::Other,
                };
                IoError::new(kind, "system call failed")
            }
            LinuxError::NoInput => IoError::new(IoErrorKind::UnexpectedEof, "no input available"),
            LinuxError::InvalidUtf8 => IoError::new(IoErrorKind::InvalidData, "invalid UTF-8 data"),
        }
    };
}
macro_rules! match_io_to_linux {
    ($err:ident) => {
        match $err.kind() {
            IoErrorKind::PermissionDenied => LinuxError::Sys(ERRNO::EACCES),
            IoErrorKind::NotFound => LinuxError::Sys(ERRNO::ENOENT),
            IoErrorKind::Interrupted => LinuxError::Sys(ERRNO::EINTR),
            IoErrorKind::WouldBlock => LinuxError::Sys(ERRNO::EAGAIN),
            IoErrorKind::OutOfMemory => LinuxError::Sys(ERRNO::ENOMEM),
            IoErrorKind::InvalidInput => LinuxError::Sys(ERRNO::EINVAL),
            IoErrorKind::StorageFull => LinuxError::Sys(ERRNO::ENOSPC),
            IoErrorKind::BrokenPipe => LinuxError::Sys(ERRNO::EPIPE),
            IoErrorKind::UnexpectedEof => LinuxError::NoInput,
            IoErrorKind::InvalidData => LinuxError::InvalidUtf8,
            _ => LinuxError::Sys(ERRNO::EIO), // Default to "I/O error"
        }
    };
}
#[rustfmt::skip]
impl LinuxError {
    /// Converts `LinuxError` to `IoError`.
    ///
    /// This will only be *const* if the `std` feature is **disabled**,
    /// because `std::io::Error::new` is not *const*.
    #[cfg(feature = "std")]
    pub fn to_io(self) -> IoError { match_linux_to_io!(self) }
    /// Converts `LinuxError` to `IoError`.
    #[cfg(not(feature = "std"))]
    pub const fn to_io(self) -> IoError { match_linux_to_io!(self) }

    /// Converts `IoError` to `LinuxError`.
    ///
    /// This will only be *const* if the `std` feature is **disabled**,
    /// because `std::io::Error::kind` is not *const*.
    #[cfg(feature = "std")]
    pub fn from_io(err: IoError) -> LinuxError { match_io_to_linux!(err) }
    /// Converts `IoError` to `LinuxError`.
    #[cfg(not(feature = "std"))]
    pub const fn from_io(err: IoError) -> LinuxError { match_io_to_linux!(err) }
}
impl From<LinuxError> for IoError {
    fn from(err: LinuxError) -> Self {
        err.to_io()
    }
}
impl From<IoError> for LinuxError {
    fn from(err: IoError) -> Self {
        LinuxError::from_io(err)
    }
}

impl LinuxError {
    /// Convert the error to [`LINUX_EXIT`][EXIT] with guaranteed valid value (0..=255).
    ///
    /// Invalid values are converted to `INTERNAL_ERROR`.
    pub const fn to_exit_code(self) -> i32 {
        let code = self.to_raw_exit_code();
        if code >= EXIT::SUCCESS && code <= EXIT::MAX { // >= 0 && <= 255
            code
        } else {
            EXIT::INTERNAL_ERROR // 254
        }
    }

    /// Convert the error to [`LINUX_EXIT`][EXIT] without validation.
    pub const fn to_raw_exit_code(self) -> i32 {
        match self {
            LinuxError::Sys(errno) => {
                match errno {
                    ERRNO::EPERM => EXIT::NOPERM,
                    ERRNO::ENOENT => EXIT::NOINPUT,
                    ERRNO::EACCES => EXIT::NOPERM,
                    ERRNO::EINVAL => EXIT::USAGE,
                    ERRNO::ENOSYS => EXIT::SOFTWARE,
                    ERRNO::ENOMEM => EXIT::OSERR,
                    ERRNO::EIO => EXIT::IOERR,
                    ERRNO::ENFILE | ERRNO::EMFILE => EXIT::OSFILE,
                    ERRNO::EEXIST => EXIT::CANTCREAT,
                    ERRNO::ENOTDIR => EXIT::DATAERR,
                    ERRNO::EISDIR => EXIT::DATAERR,
                    ERRNO::ETIMEDOUT => EXIT::TEMPFAIL,
                    // Add other specific mappings as needed
                    _ => EXIT::OSERR,
                }
            }
            LinuxError::NoInput => EXIT::NOINPUT,
            LinuxError::InvalidUtf8 => EXIT::DATAERR,
        }
    }
}
