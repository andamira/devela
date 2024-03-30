// devela::io::reimplement_no_std::error

use core::{convert::From, fmt, result};

/// A specialized [`Result`] type for I/O operations.
///
/// See <https://doc.rust-lang.org/std/io/struct.Result.html>.
pub type IoResult<T> = result::Result<T, IoError>;

/// The error type for I/O operations of [`Read`], [`Write`], [`Seek`], and
/// associated traits.
///
/// See <https://doc.rust-lang.org/std/io/struct.Error.html>.
pub struct IoError {
    repr: Repr,
}

impl crate::code::Error for IoError {}

impl fmt::Debug for IoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.repr, f)
    }
}

enum Repr {
    Simple(IoErrorKind),
    Custom(Custom),
}

#[derive(Debug)]
struct Custom {
    kind: IoErrorKind,
    error: &'static str,
}

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
    /// The connection was aborted (terminated) by the remote server.
    ConnectionAborted,
    /// The network operation failed because it was not connected yet.
    NotConnected,
    /// A socket address could not be bound because the address is already in
    /// use elsewhere.
    AddrInUse,
    /// A nonexistent interface was requested or the requested address was not
    /// local.
    AddrNotAvailable,
    /// The operation failed because a pipe was closed.
    BrokenPipe,
    /// An entity already exists, often a file.
    AlreadyExists,
    /// The operation needs to block to complete, but the blocking operation was
    /// requested to not occur.
    WouldBlock,
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
    /// [`write`]: crate::io::Write::write
    /// [`Ok(0)`]: Ok
    WriteZero,
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
    pub(crate) fn as_str(&self) -> &'static str {
        match *self {
            IoErrorKind::NotFound => "entity not found",
            IoErrorKind::PermissionDenied => "permission denied",
            IoErrorKind::ConnectionRefused => "connection refused",
            IoErrorKind::ConnectionReset => "connection reset",
            IoErrorKind::ConnectionAborted => "connection aborted",
            IoErrorKind::NotConnected => "not connected",
            IoErrorKind::AddrInUse => "address in use",
            IoErrorKind::AddrNotAvailable => "address not available",
            IoErrorKind::BrokenPipe => "broken pipe",
            IoErrorKind::AlreadyExists => "entity already exists",
            IoErrorKind::WouldBlock => "operation would block",
            IoErrorKind::InvalidInput => "invalid input parameter",
            IoErrorKind::InvalidData => "invalid data",
            IoErrorKind::TimedOut => "timed out",
            IoErrorKind::WriteZero => "write zero",
            IoErrorKind::Interrupted => "operation interrupted",
            IoErrorKind::Other => "other os error",
            IoErrorKind::UnexpectedEof => "unexpected end of file",
            IoErrorKind::Uncategorized => "uncategorized",
        }
    }
}

/// Intended for use for errors not exposed to the user, where allocating onto
/// the heap (for normal construction via Error::new) is too costly.
impl From<IoErrorKind> for IoError {
    /// Converts an [`IoErrorKind`] into an [`Error`].
    ///
    /// See <https://doc.rust-lang.org/std/io/struct.Error.html#impl-From%3CErrorKind%3E-for-Error>.
    #[inline]
    fn from(kind: IoErrorKind) -> IoError {
        IoError {
            repr: Repr::Simple(kind),
        }
    }
}

impl IoError {
    /// Creates a new I/O error from a known kind of error as well as an
    /// arbitrary error payload.
    ///
    pub fn new(kind: IoErrorKind, error: &'static str) -> IoError {
        Self::_new(kind, error)
    }

    fn _new(kind: IoErrorKind, error: &'static str) -> IoError {
        IoError {
            repr: Repr::Custom(Custom { kind, error }),
        }
    }

    /// Returns a reference to the inner error wrapped by this error (if any).
    ///
    pub fn get_ref(&self) -> Option<&&'static str> {
        match self.repr {
            Repr::Simple(..) => None,
            Repr::Custom(ref c) => Some(&c.error),
        }
    }

    /// Consumes the `IoError`, returning its inner error (if any).
    ///
    pub fn into_inner(self) -> Option<&'static str> {
        match self.repr {
            Repr::Simple(..) => None,
            Repr::Custom(c) => Some(c.error),
        }
    }

    /// Returns the corresponding [`IoErrorKind`] for this error.
    ///
    pub fn kind(&self) -> IoErrorKind {
        match self.repr {
            Repr::Custom(ref c) => c.kind,
            Repr::Simple(kind) => kind,
        }
    }
}

impl fmt::Debug for Repr {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Repr::Custom(ref c) => fmt::Debug::fmt(&c, fmt),
            Repr::Simple(kind) => fmt.debug_tuple("Kind").field(&kind).finish(),
        }
    }
}

impl fmt::Display for IoError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.repr {
            Repr::Custom(ref c) => c.error.fmt(fmt),
            Repr::Simple(kind) => write!(fmt, "{}", kind.as_str()),
        }
    }
}

// IMPROVE
fn _assert_error_is_sync_send() {
    fn _is_sync_send<T: Sync + Send>() {}
    _is_sync_send::<IoError>();
}
