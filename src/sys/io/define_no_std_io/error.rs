// devela::sys::io::error

use crate::{impl_trait, Debug, From, Result};

#[doc = crate::TAG_RESULT!()]
/// A specialized [`Result`] type for I/O operations.
///
/// See <https://doc.rust-lang.org/std/io/struct.Result.html>.
pub type IoResult<T> = Result<T, IoError>;

#[doc = crate::TAG_ERROR_COMPOSITE!()]
/// Error type for I/O operations of [`Read`], [`Write`], [`Seek`], and
/// associated traits.
///
/// See <https://doc.rust-lang.org/std/io/struct.Error.html>.
// #[derive(Clone, Copy)] // std::io::Error derives no Clone, Copy or PartialEq…
pub struct IoError {
    repr: Repr,
}
impl crate::Error for IoError {}
impl_trait![fmt::Debug for IoError |self, f| Debug::fmt(&self.repr, f)];
impl_trait![fmt::Display for IoError |self, f|
    match self.repr {
        Repr::Custom(ref c) => Debug::fmt(&c, f),
        Repr::Simple(kind) => write!(f, "{}", kind.as_str()),
    }
];

#[derive(Clone, Copy, PartialEq)]
enum Repr {
    Custom(Custom),
    Simple(IoErrorKind),
}
impl_trait![fmt::Debug for Repr |self, f|
    match *self {
        Repr::Custom(ref c) => Debug::fmt(&c, f),
        Repr::Simple(kind) => f.debug_tuple("Kind").field(&kind).finish(),
    }
];

#[derive(Clone, Copy, Debug, PartialEq)]
struct Custom {
    kind: IoErrorKind,
    error: &'static str,
}

#[doc = crate::TAG_ERROR_COMPOSITE!()]
/// A list specifying general categories of I/O error.
///
/// This list is intended to grow over time and it is not recommended to
/// exhaustively match against it.
///
/// It is used with the [`IoError`] type.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum IoErrorKind {
    /// An entity was not found, often a file.
    NotFound,
    /// The operation lacked the necessary privileges to complete.
    PermissionDenied,
    /// The connection was refused by the remote server.
    ConnectionRefused,
    /// The connection was reset by the remote server.
    ConnectionReset,
    /// The remote host is not reachable.
    HostUnreachable,
    /// The network containing the remote host is not reachable.
    NetworkUnreachable,
    /// The connection was aborted (terminated) by the remote server.
    ConnectionAborted,
    /// The network operation failed because it was not connected yet.
    NotConnected,
    /// A socket address could not be bound because the address is already in
    /// use elsewhere.
    AddrInUse,
    /// A nonexistent interface was requested or the requested address was not local.
    AddrNotAvailable,
    /// The system's networking is down.
    NetworkDown,
    /// The operation failed because a pipe was closed.
    BrokenPipe,
    /// An entity already exists, often a file.
    AlreadyExists,
    /// The operation needs to block to complete, but the blocking operation was
    /// requested to not occur.
    WouldBlock,
    /// A filesystem object is, unexpectedly, not a directory.
    ///
    /// For example, a filesystem path was specified where one of the intermediate directory
    /// components was, in fact, a plain file.
    NotADirectory,
    /// The filesystem object is, unexpectedly, a directory.
    ///
    /// A directory was specified when a non-directory was expected.
    IsADirectory,
    /// A non-empty directory was specified where an empty directory was expected.
    DirectoryNotEmpty,
    /// The filesystem or storage medium is read-only, but a write operation was attempted.
    ReadOnlyFilesystem,
    /// Stale network file handle.
    ///
    /// With some network filesystems, notably NFS, an open file (or directory) can be invalidated
    /// by problems with the network or server.
    StaleNetworkFileHandle,
    /// A parameter was incorrect.
    InvalidInput,
    /// Data not valid for the operation were encountered.
    ///
    /// Unlike [`InvalidInput`], this typically means that the operation
    /// parameters were valid, however the error was caused by malformed
    /// input data.
    ///
    /// For example, a function that reads a file into a string will error with
    /// `InvalidData` if the file's contents are not valid UTF-8.
    ///
    /// [`InvalidInput`]: IoErrorKind::InvalidInput
    InvalidData,
    /// The I/O operation's timeout expired, causing it to be canceled.
    TimedOut,
    /// An error returned when an operation could not be completed because a
    /// call to [`write`] returned [`Ok(0)`].
    ///
    /// This typically means that an operation could only succeed if it wrote a
    /// particular number of bytes but only a smaller number of bytes could be
    /// written.
    ///
    /// [`write`]: crate::sys::io::IoWrite::write
    /// [`Ok(0)`]: Ok
    WriteZero,
    /// The underlying storage (typically, a filesystem) is full.
    ///
    /// This does not include out of quota errors.
    StorageFull,
    /// Seek on unseekable file.
    ///
    /// Seeking was attempted on an open file handle which is not suitable for seeking - for
    /// example, on Unix, a named pipe opened with `File::open`.
    NotSeekable,
    /// File larger than allowed or supported.
    ///
    /// This might arise from a hard limit of the underlying filesystem or file access API, or from
    /// an administratively imposed resource limitation.  Simple disk full, and out of quota, have
    /// their own errors.
    FileTooLarge,
    /// Resource is busy.
    ResourceBusy,
    /// Executable file is busy.
    ///
    /// An attempt was made to write to a file which is also in use as a running program.  (Not all
    /// operating systems detect this situation.)
    ExecutableFileBusy,
    /// Deadlock (avoided).
    ///
    /// A file locking operation would result in deadlock.  This situation is typically detected, if
    /// at all, on a best-effort basis.
    Deadlock,
    /// Too many (hard) links to the same filesystem object.
    ///
    /// The filesystem does not support making so many hardlinks to the same file.
    TooManyLinks,
    /// Program argument list too long.
    ///
    /// When trying to run an external program, a system or process limit on the size of the
    /// arguments would have been exceeded.
    ArgumentListTooLong,
    /// This operation was interrupted.
    ///
    /// Interrupted operations can typically be retried.
    Interrupted,
    /// Any I/O error not part of this list.
    ///
    /// Errors that are `Other` now may move to a different or a new
    /// [`IoErrorKind`] variant in the future. It is not recommended to match
    /// an error against `Other` and to expect any additional characteristics,
    /// e.g., a specific [`Error::raw_os_error`] return value.
    Other,

    /// An error returned when an operation could not be completed because an
    /// "end of file" was reached prematurely.
    ///
    /// This typically means that an operation could only succeed if it read a
    /// particular number of bytes but only a smaller number of bytes could be
    /// read.
    UnexpectedEof,

    /// Any I/O error from the standard library that's not part of this list.
    ///
    /// Errors that are `Uncategorized` now may move to a different or a new
    /// [`IoErrorKind`] variant in the future. It is not recommended to match
    /// an error against `Uncategorized`; use a wildcard match (`_`) instead.
    #[doc(hidden)]
    Uncategorized,
}

impl IoErrorKind {
    pub(crate) fn as_str(self) -> &'static str {
        use IoErrorKind as E;
        match self {
            E::NotFound => "entity not found",
            E::PermissionDenied => "permission denied",
            E::ConnectionRefused => "connection refused",
            E::ConnectionReset => "connection reset",
            E::HostUnreachable => "remote host unreachable",
            E::NetworkUnreachable => "network unreachable",
            E::ConnectionAborted => "connection aborted",
            E::NotConnected => "not connected",
            E::AddrInUse => "address in use",
            E::AddrNotAvailable => "address not available",
            E::NetworkDown => "network is down",
            E::BrokenPipe => "broken pipe",
            E::AlreadyExists => "entity already exists",
            E::WouldBlock => "operation would block",
            E::NotADirectory => "filesystem object is not a directory",
            E::IsADirectory => "filesystem object is a directory",
            E::DirectoryNotEmpty => "directory is not empty",
            E::ReadOnlyFilesystem => "read-only filesystem",
            E::StaleNetworkFileHandle => "stale network file handle",
            E::InvalidInput => "invalid input parameter",
            E::InvalidData => "invalid data",
            E::TimedOut => "timed out",
            E::WriteZero => "write zero",
            E::StorageFull => "storage full",
            E::NotSeekable => "file not seekable",
            E::FileTooLarge => "file too large",
            E::ResourceBusy => "resource busy",
            E::ExecutableFileBusy => "executable file busy",
            E::Deadlock => "deadlock (avoided)",
            E::TooManyLinks => "too many hard links",
            E::ArgumentListTooLong => "argument list too long",
            E::Interrupted => "operation interrupted",
            E::Other => "other os error",
            E::UnexpectedEof => "unexpected end of file",
            E::Uncategorized => "uncategorized",
        }
    }
}

/// Intended for use for errors not exposed to the user, where allocating onto
/// the heap (for normal construction via Error::new) is too costly.
impl From<IoErrorKind> for IoError {
    /// Converts an [`IoErrorKind`] into an [`Error`].
    ///
    /// See <https://doc.rust-lang.org/std/io/struct.Error.html#impl-From%3CErrorKind%3E-for-Error>.
    fn from(kind: IoErrorKind) -> IoError {
        IoError { repr: Repr::Simple(kind) }
    }
}

impl IoError {
    /// Creates a new I/O error from a known kind of error as well as an arbitrary error payload.
    pub fn new(kind: IoErrorKind, error: &'static str) -> IoError {
        IoError { repr: Repr::Custom(Custom { kind, error }) }
    }

    /// Returns a reference to the inner error wrapped by this error (if any).
    pub fn get_ref(&self) -> Option<&&'static str> {
        match self.repr {
            Repr::Custom(ref c) => Some(&c.error),
            Repr::Simple(..) => None,
        }
    }

    /// Consumes the `IoError`, returning its inner error (if any).
    pub fn into_inner(self) -> Option<&'static str> {
        match self.repr {
            Repr::Custom(c) => Some(c.error),
            Repr::Simple(..) => None,
        }
    }

    /// Returns the corresponding [`IoErrorKind`] for this error.
    pub fn kind(&self) -> IoErrorKind {
        match self.repr {
            Repr::Custom(ref c) => c.kind,
            Repr::Simple(kind) => kind,
        }
    }
}

// IMPROVE
fn _assert_error_is_sync_send() {
    fn _is_sync_send<T: Sync + Send>() {}
    _is_sync_send::<IoError>();
}
