// devela::sys::io::reimplement_no_std::write
//
//! Defines the [`IoWrite`] trait.
//

use crate::{IoError, IoErrorKind, IoResult, _core::fmt};

/// A trait for objects which are byte-oriented sinks.
///
/// See <https://doc.rust-lang.org/std/io/trait.Write.html>.
#[rustfmt::skip]
pub trait IoWrite {
    /// Write a buffer into this writer, returning how many bytes were written.
    ///
    /// See <https://doc.rust-lang.org/std/io/trait.Write.html#method.write>.
    fn write(&mut self, buf: &[u8]) -> IoResult<usize>;

    /// Flush this output stream, ensuring that all intermediately buffered
    /// contents reach their destination.
    ///
    /// See <https://doc.rust-lang.org/std/io/trait.Write.html#method.flush>.
    fn flush(&mut self) -> IoResult<()>;

    /// Attempts to write an entire buffer into this writer.
    ///
    /// See <https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all>.
    fn write_all(&mut self, mut buf: &[u8]) -> IoResult<()> {
        while !buf.is_empty() {
            match self.write(buf) {
                Ok(0) => { return Err(IoError::new(
                    IoErrorKind::WriteZero, "failed to write whole buffer")); }
                Ok(n) => buf = &buf[n..],
                Err(ref e) if e.kind() == IoErrorKind::Interrupted => {}
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    /// Writes a formatted string into this writer, returning any error encountered.
    ///
    /// See <https://doc.rust-lang.org/std/io/trait.Write.html#method.write_fmt>.
    fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> IoResult<()> {
        // Create a shim which translates an IoWrite to a fmt::Write and saves
        // off I/O errors. instead of discarding them
        struct Adaptor<'a, T: ?Sized + 'a> {
            inner: &'a mut T,
            error: IoResult<()>,
        }

        impl<T: IoWrite + ?Sized> fmt::Write for Adaptor<'_, T> {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                match self.inner.write_all(s.as_bytes()) {
                    Ok(()) => Ok(()),
                    Err(e) => { self.error = Err(e); Err(fmt::Error) }
                }
            }
        }

        let mut output = Adaptor { inner: self, error: Ok(()) };
        match fmt::write(&mut output, fmt) {
            Ok(()) => Ok(()),
            Err(..) => {
                // check if the error came from the underlying `IoWrite` or not
                if output.error.is_err() { output.error }
                else { Err(IoError::new(IoErrorKind::Other, "formatter error")) }
            }
        }
    }

    /// Creates a "by reference" adaptor for this instance of `IoWrite`.
    ///
    /// See <https://doc.rust-lang.org/std/io/trait.Write.html#method.by_ref>.
    fn by_ref(&mut self) -> &mut Self where Self: Sized { self }
}
