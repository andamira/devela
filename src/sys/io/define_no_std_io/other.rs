// devela::sys::io::reimplement_no_std::traits

use super::error::{IoError as Error, IoErrorKind, IoResult as Result};
#[allow(unused_imports, reason = "±unsafe")]
use crate::sf;
use crate::OptRes;
#[cfg(feature = "alloc")]
use crate::Vec;
use core::{cmp, fmt, slice};

#[cfg(feature = "alloc")]
mod alloc_impls {
    use super::*;
    use crate::vec_ as vec;

    /// Reads all bytes from a reader into the given buffer, adapting the buffer size as needed.
    ///
    /// This function uses an adaptive system to efficiently handle varying amounts of data.
    /// It avoids allocating excessive memory for small reads, while still efficiently handling
    /// larger data sizes. The default reservation size is set to 32 bytes, balancing memory
    /// usage with performance for different read sizes.
    ///
    /// For safety, this function ensures that any allocated but uninitialized part of the buffer
    /// is truncated in case of a panic, preventing exposure of uninitialized data.
    ///
    pub(super) fn read_to_end<R: IoRead + ?Sized>(r: &mut R, buf: &mut Vec<u8>) -> Result<usize> {
        read_to_end_with_reservation(r, buf, |_| 32)
    }

    #[cfg(any(feature = "safe_sys", not(feature = "unsafe_slice")))]
    pub(super) fn read_to_end_with_reservation<R, F>(
        r: &mut R,
        buf: &mut Vec<u8>,
        mut reservation_size: F,
    ) -> Result<usize>
    where
        R: IoRead + ?Sized,
        F: FnMut(&R) -> usize,
    {
        const PROBE_SIZE: usize = 32;

        fn small_probe_read<R: IoRead + ?Sized>(
            r: &mut R,
            buf: &mut Vec<u8>,
            probe_size: usize,
        ) -> Result<usize> {
            let mut probe = vec![0u8; probe_size];
            match r.read(&mut probe) {
                Ok(n) => {
                    probe.truncate(n);
                    buf.extend_from_slice(&probe);
                    Ok(n)
                }
                Err(ref e) if e.kind() == IoErrorKind::Interrupted => Ok(0),
                Err(e) => Err(e),
            }
        }
        let start_len = buf.len();
        if (reservation_size(r) == 0 || buf.capacity() == buf.len())
            && buf.capacity() - buf.len() < PROBE_SIZE
        {
            let read = small_probe_read(r, buf, PROBE_SIZE)?;
            if read == 0 {
                return Ok(0);
            }
        }
        loop {
            if buf.len() == buf.capacity() {
                buf.reserve(cmp::min(reservation_size(r), PROBE_SIZE));
            }
            let buf_len = buf.len(); // Capture the length of the buffer
            buf.resize(buf.capacity(), 0); // Safely extend the buffer with zeroed bytes
            let read_result = {
                let spare = &mut buf[buf_len..];
                r.read(spare)
            };
            match read_result {
                Ok(0) => {
                    buf.truncate(buf_len); // Safely shrink the buffer to original size
                    return Ok(buf_len - start_len);
                }
                Ok(read_bytes) => {
                    buf.truncate(buf_len + read_bytes); // Adjust the length correctly
                }
                Err(e) => {
                    buf.truncate(buf_len); // Revert to the original length on error
                    return Err(e);
                }
            }
        }
    }

    #[cfg(all(not(feature = "safe_sys"), feature = "unsafe_slice"))]
    pub(super) fn read_to_end_with_reservation<R, F>(
        r: &mut R,
        buf: &mut Vec<u8>,
        mut reservation_size: F,
    ) -> Result<usize>
    where
        R: IoRead + ?Sized,
        F: FnMut(&R) -> usize,
    {
        const PROBE_SIZE: usize = 32;

        fn small_probe_read<R: IoRead + ?Sized>(
            r: &mut R,
            buf: &mut Vec<u8>,
            _probe_size: usize,
        ) -> Result<usize> {
            let mut probe = [0u8; PROBE_SIZE];
            loop {
                match r.read(&mut probe) {
                    Ok(n) => {
                        buf.extend_from_slice(&probe[..n]);
                        return Ok(n);
                    }
                    Err(ref e) if e.kind() == IoErrorKind::Interrupted => {}
                    Err(e) => return Err(e),
                }
            }
        }

        let start_len = buf.len();

        if (reservation_size(r) == 0 || buf.capacity() == buf.len())
            && buf.capacity() - buf.len() < PROBE_SIZE
        {
            let read = small_probe_read(r, buf, PROBE_SIZE)?;
            if read == 0 {
                return Ok(0);
            }
        }

        loop {
            if buf.len() == buf.capacity() {
                buf.reserve(cmp::min(reservation_size(r), PROBE_SIZE));
            }

            let buf_len = buf.len(); // capture the length of the buffer.

            // SAFETY: We ensure that the buffer length is set to its capacity.
            // This avoids zeroing out uninitialized memory.
            sf! { unsafe { buf.set_len(buf.capacity()); }}

            let read_result = {
                let spare = &mut buf[buf_len..];
                r.read(spare)
            };

            match read_result {
                Ok(0) => {
                    // SAFETY: Revert the buffer length to its previous value,
                    // ensuring no uninitialized memory is exposed.
                    sf! { unsafe { buf.set_len(buf_len); }}
                    return Ok(buf_len - start_len);
                }
                // SAFETY: Adjust the buffer length to account for the bytes read,
                // ensuring that we do not include any uninitialized memory.
                Ok(read_bytes) => sf! { unsafe { buf.set_len(buf_len + read_bytes); }},
                Err(e) => {
                    // SAFETY: Revert the buffer length to its previous value on error,
                    // ensuring no uninitialized memory is exposed.
                    sf! { unsafe { buf.set_len(buf_len); }}
                    return Err(e);
                }
            }

            if buf.len() < buf.capacity() {
                // If we haven't filled the buffer yet, continue reading.
                continue;
            }

            // If the buffer is full, we'll extend it on the next iteration.
        }
    }
}

/// The `IoRead` trait allows for reading bytes from a source.
///
/// See <https://doc.rust-lang.org/std/io/trait.Read.html>.
pub trait IoRead {
    /// Pull some bytes from this source into the specified buffer, returning
    /// how many bytes were read.
    ///
    /// See <https://doc.rust-lang.org/std/io/trait.Read.html#method.read>.
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;

    /// Read all bytes until EOF in this source, placing them into `buf`.
    ///
    /// See <https://doc.rust-lang.org/std/io/trait.Read.html#method.read_to_end>.
    #[cfg(feature = "alloc")]
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize> {
        alloc_impls::read_to_end(self, buf)
    }

    /// Read the exact number of bytes required to fill `buf`.
    ///
    /// See <https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact>.
    fn read_exact(&mut self, mut buf: &mut [u8]) -> Result<()> {
        while !buf.is_empty() {
            match self.read(buf) {
                Ok(0) => break,
                Ok(n) => {
                    let tmp = buf;
                    buf = &mut tmp[n..];
                }
                Err(ref e) if e.kind() == IoErrorKind::Interrupted => {}
                Err(e) => return Err(e),
            }
        }
        if !buf.is_empty() {
            Err(Error::new(IoErrorKind::UnexpectedEof, "failed to fill whole buffer"))
        } else {
            Ok(())
        }
    }

    /// Creates a "by reference" adaptor for this instance of `IoRead`.
    ///
    /// See <https://doc.rust-lang.org/std/io/trait.Read.html#method.by_ref>.
    fn by_ref(&mut self) -> &mut Self
    where
        Self: Sized,
    {
        self
    }

    /// Transforms this `IoRead` instance to an [`Iterator`] over its bytes.
    ///
    /// See <https://doc.rust-lang.org/std/io/trait.Read.html#method.bytes>.
    fn bytes(self) -> IoBytes<Self>
    where
        Self: Sized,
    {
        IoBytes { inner: self }
    }

    /// Creates an adaptor which will chain this stream with another.
    ///
    /// See <https://doc.rust-lang.org/std/io/trait.Read.html#method.chain>.
    fn chain<R: IoRead>(self, next: R) -> IoChain<Self, R>
    where
        Self: Sized,
    {
        IoChain { first: self, second: next, done_first: false }
    }

    /// Creates an adaptor which will read at most `limit` bytes from it.
    ///
    fn take(self, limit: u64) -> IoTake<Self>
    where
        Self: Sized,
    {
        IoTake { inner: self, limit }
    }
}

/// A trait for objects which are byte-oriented sinks.
///
/// See <https://doc.rust-lang.org/std/io/trait.Write.html>.
pub trait IoWrite {
    /// Write a buffer into this writer, returning how many bytes were written.
    ///
    /// See <https://doc.rust-lang.org/std/io/trait.Write.html#method.write>.
    fn write(&mut self, buf: &[u8]) -> Result<usize>;

    /// Flush this output stream, ensuring that all intermediately buffered
    /// contents reach their destination.
    ///
    /// See <https://doc.rust-lang.org/std/io/trait.Write.html#method.flush>.
    fn flush(&mut self) -> Result<()>;

    /// Attempts to write an entire buffer into this writer.
    ///
    /// See <https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all>.
    fn write_all(&mut self, mut buf: &[u8]) -> Result<()> {
        while !buf.is_empty() {
            match self.write(buf) {
                Ok(0) => {
                    return Err(Error::new(IoErrorKind::WriteZero, "failed to write whole buffer"));
                }
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
    fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> Result<()> {
        // Create a shim which translates an IoWrite to a fmt::Write and saves
        // off I/O errors. instead of discarding them
        struct Adaptor<'a, T: ?Sized + 'a> {
            inner: &'a mut T,
            error: Result<()>,
        }

        impl<T: IoWrite + ?Sized> fmt::Write for Adaptor<'_, T> {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                match self.inner.write_all(s.as_bytes()) {
                    Ok(()) => Ok(()),
                    Err(e) => {
                        self.error = Err(e);
                        Err(fmt::Error)
                    }
                }
            }
        }

        let mut output = Adaptor { inner: self, error: Ok(()) };
        match fmt::write(&mut output, fmt) {
            Ok(()) => Ok(()),
            Err(..) => {
                // check if the error came from the underlying `IoWrite` or not
                if output.error.is_err() {
                    output.error
                } else {
                    Err(Error::new(IoErrorKind::Other, "formatter error"))
                }
            }
        }
    }

    /// Creates a "by reference" adaptor for this instance of `IoWrite`.
    ///
    /// See <https://doc.rust-lang.org/std/io/trait.Write.html#method.by_ref>.
    fn by_ref(&mut self) -> &mut Self
    where
        Self: Sized,
    {
        self
    }
}

/// The `IoSeek` trait provides a cursor which can be moved within a stream of
/// bytes.
///
/// This struct is generally created by calling [`bytes`][IoRead::bytes] on a reader.
pub trait IoSeek {
    /// Seek to an offset, in bytes, in a stream.
    ///
    /// See <https://doc.rust-lang.org/std/io/trait.Seek.html#method.seek>.
    fn seek(&mut self, pos: IoSeekFrom) -> Result<u64>;
}

/// Enumeration of possible methods to seek within an I/O object.
///
/// It is used by the [`IoSeek`] trait.
#[derive(Copy, PartialEq, Eq, Clone, Debug)]
pub enum IoSeekFrom {
    /// Sets the offset to the provided number of bytes.
    Start(u64),

    /// Sets the offset to the size of this object plus the specified number of
    /// bytes.
    ///
    /// It is possible to seek beyond the end of an object, but it's an error to
    /// seek before byte 0.
    End(i64),

    /// Sets the offset to the current position plus the specified number of
    /// bytes.
    ///
    /// It is possible to seek beyond the end of an object, but it's an error to
    /// seek before byte 0.
    Current(i64),
}

/// An iterator over `u8` values of a reader.
///
/// See <https://doc.rust-lang.org/std/io/trait.Bytes.html>.
#[derive(Debug)]
pub struct IoBytes<R> {
    inner: R,
}

impl<R: IoRead> Iterator for IoBytes<R> {
    type Item = Result<u8>;

    fn next(&mut self) -> OptRes<u8, Error> {
        let mut byte = 0;
        loop {
            return match self.inner.read(slice::from_mut(&mut byte)) {
                Ok(0) => None,
                Ok(..) => Some(Ok(byte)),
                Err(ref e) if e.kind() == IoErrorKind::Interrupted => continue,
                Err(e) => Some(Err(e)),
            };
        }
    }
}

/// An `IoBufRead` is a type of `IoRead`er which has an internal buffer, allowing it
/// to perform extra ways of reading.
///
/// See <https://doc.rust-lang.org/std/io/trait.BufRead.html>.
pub trait IoBufRead: IoRead {
    /// Returns the contents of the internal buffer, filling it with more data
    /// from the inner reader if it is empty.
    ///
    /// See <https://doc.rust-lang.org/std/io/trait.BufRead.html#method.fill_buf>.
    fn fill_buf(&mut self) -> Result<&[u8]>;

    /// Tells this buffer that `amt` bytes have been consumed from the buffer,
    /// so they should no longer be returned in calls to `read`.
    ///
    /// See <https://doc.rust-lang.org/std/io/trait.BufRead.html#method.consume>.
    fn consume(&mut self, amt: usize);
}

/// Adaptor to chain together two readers.
///
/// This struct is generally created by calling [`chain`][IoRead::chain] on a reader.
pub struct IoChain<T, U> {
    first: T,
    second: U,
    done_first: bool,
}

impl<T, U> IoChain<T, U> {
    /// Consumes the `Chain`, returning the wrapped readers.
    ///
    /// See <https://doc.rust-lang.org/std/io/trait.Chain.html#method.into_inner>.
    pub fn into_inner(self) -> (T, U) {
        (self.first, self.second)
    }

    /// Gets references to the underlying readers in this `Chain`.
    ///
    /// See <https://doc.rust-lang.org/std/io/trait.Chain.html#method.get_ref>.
    pub fn get_ref(&self) -> (&T, &U) {
        (&self.first, &self.second)
    }

    /// Gets mutable references to the underlying readers in this `Chain`.
    ///
    /// See <https://doc.rust-lang.org/std/io/trait.Chain.html#method.get_mut>.
    pub fn get_mut(&mut self) -> (&mut T, &mut U) {
        (&mut self.first, &mut self.second)
    }
}

impl<T: fmt::Debug, U: fmt::Debug> fmt::Debug for IoChain<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IoChain").field("t", &self.first).field("u", &self.second).finish()
    }
}

impl<T: IoRead, U: IoRead> IoRead for IoChain<T, U> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        if !self.done_first {
            match self.first.read(buf)? {
                0 if !buf.is_empty() => self.done_first = true,
                n => return Ok(n),
            }
        }
        self.second.read(buf)
    }
}

impl<T: IoBufRead, U: IoBufRead> IoBufRead for IoChain<T, U> {
    fn fill_buf(&mut self) -> Result<&[u8]> {
        if !self.done_first {
            match self.first.fill_buf()? {
                [] => {
                    self.done_first = true;
                }
                buf => return Ok(buf),
            }
        }
        self.second.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        if !self.done_first {
            self.first.consume(amt);
        } else {
            self.second.consume(amt);
        }
    }
}

/// Reader adaptor which limits the bytes read from an underlying reader.
///
/// This struct is generally created by calling [`take`][IoRead::take] on a reader.
#[derive(Debug)]
pub struct IoTake<T> {
    inner: T,
    limit: u64,
}

impl<T> IoTake<T> {
    /// Returns the number of bytes that can be read before this instance will
    /// return EOF.
    ///
    /// See <https://doc.rust-lang.org/std/io/trait.Take.html#method.limit>.
    pub fn limit(&self) -> u64 {
        self.limit
    }

    /// Sets the number of bytes that can be read before this instance will
    /// return EOF.
    ///
    /// See <https://doc.rust-lang.org/std/io/trait.Take.html#method.set_limit>.
    pub fn set_limit(&mut self, limit: u64) {
        self.limit = limit;
    }

    /// Consumes the `IoTake`, returning the wrapped reader.
    ///
    /// See <https://doc.rust-lang.org/std/io/trait.Take.html#method.into_inner>.
    pub fn into_inner(self) -> T {
        self.inner
    }

    /// Gets a reference to the underlying reader.
    ///
    /// See <https://doc.rust-lang.org/std/io/trait.Take.html#method.get_ref>.
    pub fn get_ref(&self) -> &T {
        &self.inner
    }

    /// Gets a mutable reference to the underlying reader.
    ///
    /// See <https://doc.rust-lang.org/std/io/trait.Take.html#method.get_mut>.
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T: IoRead> IoRead for IoTake<T> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        // Don't call into inner reader at all at EOF because it may still block
        if self.limit == 0 {
            return Ok(0);
        }

        let max = cmp::min(buf.len() as u64, self.limit) as usize;
        let n = self.inner.read(&mut buf[..max])?;
        self.limit -= n as u64;
        Ok(n)
    }

    #[cfg(feature = "alloc")]
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize> {
        // Pass in a reservation_size closure that respects the current value
        // of limit for each read. If we hit the read limit, this prevents the
        // final zero-byte read from allocating again.
        alloc_impls::read_to_end_with_reservation(self, buf, |self_| {
            cmp::min(self_.limit, 32) as usize
        })
    }
}

impl<T: IoBufRead> IoBufRead for IoTake<T> {
    fn fill_buf(&mut self) -> Result<&[u8]> {
        // Don't call into inner reader at all at EOF because it may still block
        if self.limit == 0 {
            return Ok(&[]);
        }

        let buf = self.inner.fill_buf()?;
        let cap = cmp::min(buf.len() as u64, self.limit) as usize;
        Ok(&buf[..cap])
    }

    fn consume(&mut self, amt: usize) {
        // Don't let callers reset the limit by passing an overlarge value
        let amt = cmp::min(amt as u64, self.limit) as usize;
        self.limit -= amt as u64;
        self.inner.consume(amt);
    }
}
