// devela::sys::io::no_std_define::write
//
//! Defines the [`IoWrite`] trait.
//

use crate::{IoError, IoErrorKind, IoResult, Mem};
use ::core::{cmp, fmt};

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

/* impls */

impl<W: IoWrite + ?Sized> IoWrite for &mut W {
    fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
        (**self).write(buf)
    }
    fn flush(&mut self) -> IoResult<()> {
        (**self).flush()
    }
    fn write_all(&mut self, buf: &[u8]) -> IoResult<()> {
        (**self).write_all(buf)
    }
    fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> IoResult<()> {
        (**self).write_fmt(fmt)
    }
}

/// Write is implemented for `Vec<u8>` by appending to the vector.
/// The vector will grow as needed.
#[cfg(feature = "alloc")]
impl IoWrite for crate::Vec<u8> {
    fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
        self.extend_from_slice(buf);
        Ok(buf.len())
    }
    fn write_all(&mut self, buf: &[u8]) -> IoResult<()> {
        self.extend_from_slice(buf);
        Ok(())
    }
    fn flush(&mut self) -> IoResult<()> {
        Ok(())
    }
}

/// Write is implemented for `&mut [u8]` by copying into the slice, overwriting
/// its data.
///
/// Note that writing updates the slice to point to the yet unwritten part.
/// The slice will be empty when it has been completely overwritten.
impl IoWrite for &mut [u8] {
    fn write(&mut self, data: &[u8]) -> IoResult<usize> {
        let amt = cmp::min(data.len(), self.len());
        let (a, b) = Mem::take(self).split_at_mut(amt);
        a.copy_from_slice(&data[..amt]);
        *self = b;
        Ok(amt)
    }

    fn write_all(&mut self, data: &[u8]) -> IoResult<()> {
        if self.write(data)? == data.len() {
            Ok(())
        } else {
            Err(IoError::new(IoErrorKind::WriteZero, "failed to write whole buffer"))
        }
    }

    fn flush(&mut self) -> IoResult<()> {
        Ok(())
    }
}
