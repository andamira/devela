// devela::sys::io::io
//
//! Defines the [`Io`] namespace.
//

#[cfg(not(feature = "std"))]
use super::{io_copy as copy, io_empty as empty, io_repeat as repeat};
#[cfg(feature = "std")]
use crate::{
    _dep::_std::io::{copy, empty, pipe, read_to_string, repeat, sink, stderr, stdin, stdout},
    IoPipeReader, IoPipeWriter, IoSink, Stderr, Stdin, Stdout, String,
};
use crate::{IoEmpty, IoRead, IoRepeat, IoResult, IoWrite};

#[doc = crate::_tags!(io namespace)]
/// I/O-related operations.
#[derive(Debug)]
pub struct Io;

/// # Methods available in `no_std`.
#[rustfmt::skip]
impl Io {
    #[cfg(not(feature = "std"))]
    /// Copies the entire contents of a reader into a writer.
    ///
    /// See <https://doc.rust-lang.org/std/io/fn.copy.html>.
    ///
    /// # Features
    /// Makes use of the `unsafe_array` feature to employ [`MaybeUninit`].
    pub fn copy<R, W, const LEN: usize>(reader: &mut R, writer: &mut W) -> IoResult<u64>
    where R: IoRead + ?Sized, W: IoWrite + ?Sized { copy::<R, W, LEN>(reader, writer) }
    #[cfg(feature = "std")]
    /// Copies the entire contents of a reader into a writer.
    ///
    /// See `std::io::`[`copy`].
    pub fn copy<R, W>(reader: &mut R, writer: &mut W) -> IoResult<u64>
    where R: IoRead + ?Sized, W: IoWrite + ?Sized { copy(reader, writer) }

    /// Creates a value that is always at EOF for reads, and ignores all data written.
    ///
    /// See `std::io::`[`empty`].
    ///
    /// [`empty`]: ::std::io::empty
    #[must_use]
    pub const fn empty() -> IoEmpty { empty() }

    /// Creates an instance of a reader that infinitely repeats one byte.
    ///
    /// See `std::io::`[`repeat`].
    ///
    /// [`repeat`]: ::std::io::repeat
    #[must_use]
    pub const fn repeat(byte: u8) -> IoRepeat { repeat(byte) }
}

/// # Methods only available in `std`.
#[rustfmt::skip]
#[cfg(feature = "std")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
impl Io {
    /// Creates an anonymous pipe.
    ///
    /// See `std::io::`[`pipe`].
    pub fn pipe() -> IoResult<(IoPipeReader, IoPipeWriter)> { pipe() }

    /// Reads all bytes from a reader into a new String.
    ///
    /// See `std::io::`[`read_to_string`].
    pub fn read_to_string<R: IoRead>(reader: R) -> IoResult<String> { read_to_string(reader) }

    /// Creates an instance of a writer which will successfully consume all data.
    ///
    /// See `std::io::`[`sink`].
    pub const fn sink() -> IoSink { sink() }

    /// Constructs a new handle to the standard error of the current process.
    ///
    /// See `std::io::`[`stderr`].
    pub fn stderr() -> Stderr { stderr() }

    /// Constructs a new handle to the standard input of the current process.
    ///
    /// See `std::io::`[`stdin`].
    pub fn stdin() -> Stdin { stdin() }

    /// Constructs a new handle to the standard output of the current process.
    ///
    /// See `std::io::`[`stdout`].
    pub fn stdout() -> Stdout { stdout() }
}
