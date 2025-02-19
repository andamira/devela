// devela::sys::io::reimplement_no_std::read
//
//! Defines the [`IoRead`] trait.
//
// TOC
// - trait IoRead
// - trait IoBufRead
// - struct IoBytes
// - struct IoChain
// - struct IoLines (TODO)
// - struct IoTake
// - impls
// - mod alloc_impls
//
// TODO:
// - https://doc.rust-lang.org/std/io/trait.Read.html#method.read_to_string

#[cfg(feature = "alloc")]
use crate::Vec;
use crate::{IoError, IoErrorKind, IoResult, _core::fmt, iif, sf, OptRes, Slice};
use ::core::cmp;

/// The `IoRead` trait allows for reading bytes from a source.
///
/// See <https://doc.rust-lang.org/std/io/trait.Read.html>.
#[rustfmt::skip]
pub trait IoRead {
    /// Pull some bytes from this source into the specified buffer, returning
    /// how many bytes were read.
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize>;

    /* provided */

    /// Read all bytes until EOF in this source, placing them into `buf`.
    ///
    /// # Features
    /// Makes use of the `unsafe_slice` feature if enabled.
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> IoResult<usize> {
        alloc_impls::read_to_end(self, buf)
    }
    /// Read the exact number of bytes required to fill `buf`.
    fn read_exact(&mut self, mut buf: &mut [u8]) -> IoResult<()> {
        while !buf.is_empty() {
            match self.read(buf) {
                Ok(0) => break,
                Ok(n) => { let tmp = buf; buf = &mut tmp[n..]; }
                Err(ref e) if e.kind() == IoErrorKind::Interrupted => {}
                Err(e) => return Err(e),
            } }
        if !buf.is_empty() {
            Err(IoError::new(IoErrorKind::UnexpectedEof, "failed to fill whole buffer"))
        } else { Ok(()) }
    }
    /// Creates a "by reference" adaptor for this instance of `IoRead`.
    fn by_ref(&mut self) -> &mut Self where Self: Sized { self }
    /// Transforms this `IoRead` instance to an [`Iterator`] over its bytes.
    fn bytes(self) -> IoBytes<Self> where Self: Sized { IoBytes { inner: self } }
    /// Creates an adaptor which will chain this stream with another.
    fn chain<R: IoRead>(self, next: R) -> IoChain<Self, R> where Self: Sized {
        IoChain::new(self, next, false) }
    /// Creates an adaptor which will read at most `limit` bytes from it.
    fn take(self, limit: u64) -> IoTake<Self> where Self: Sized { IoTake::new(self, limit) }
}

/// A type of `IoRead`er which has an internal buffer, allowing it
/// to perform extra ways of reading.
///
/// See <https://doc.rust-lang.org/std/io/trait.BufRead.html>.
pub trait IoBufRead: IoRead {
    /// Returns the contents of the internal buffer, filling it with more data
    /// from the inner reader if it is empty.
    fn fill_buf(&mut self) -> IoResult<&[u8]>;

    /// Tells this buffer that `amt` bytes have been consumed from the buffer,
    /// so they should no longer be returned in calls to `read`.
    fn consume(&mut self, amt: usize);

    // TODO:IO_LINES
    // #[cfg(feature = "alloc")]
    // #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
    // fn read_line(&mut self, buf: &mut String) -> IoResult<usize> {
    //     // Note that we are not calling the `.read_until` method here, but
    //     // rather our hardcoded implementation. For more details as to why, see
    //     // the comments in `read_to_end`.
    //     unsafe { append_to_string(buf, |b| read_until(self, b'\n', b)) }
    // }
    //
    // /// Returns an iterator over the lines of this reader.
    // #[cfg(feature = "alloc")]
    // #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
    // fn lines(self) -> IoLines<Self>
    // where
    //     Self: Sized,
    // {
    //     IoLines { buf: self }
    // }
}

/// An iterator over `u8` values of a reader.
///
/// See <https://doc.rust-lang.org/std/io/trait.Bytes.html>.
#[derive(Debug)]
pub struct IoBytes<R> {
    pub(super) inner: R,
}
impl<R: IoRead> Iterator for IoBytes<R> {
    type Item = IoResult<u8>;
    fn next(&mut self) -> OptRes<u8, IoError> {
        let mut byte = 0;
        loop {
            return match self.inner.read(Slice::from_mut(&mut byte)) {
                Ok(0) => None,
                Ok(..) => Some(Ok(byte)),
                Err(ref e) if e.kind() == IoErrorKind::Interrupted => continue,
                Err(e) => Some(Err(e)),
            };
        }
    }
}

/// Adaptor to chain together two readers.
///
/// This struct is generally created by calling [`chain`][IoRead::chain] on a reader.
///
/// See <https://doc.rust-lang.org/std/io/trait.Chain.html>.
pub struct IoChain<T, U> {
    first: T,
    second: U,
    done_first: bool,
}
#[rustfmt::skip]
impl<T, U> IoChain<T, U> {
    pub(super) fn new(first: T, second: U, done_first: bool) -> Self {
        IoChain { first, second, done_first }
    }
    /// Consumes the `Chain`, returning the wrapped readers.
    pub fn into_inner(self) -> (T, U) { (self.first, self.second) }
    /// Gets references to the underlying readers in this `Chain`.
    pub fn get_ref(&self) -> (&T, &U) { (&self.first, &self.second) }
    /// Gets mutable references to the underlying readers in this `Chain`.
    pub fn get_mut(&mut self) -> (&mut T, &mut U) { (&mut self.first, &mut self.second) }
}

impl<T: fmt::Debug, U: fmt::Debug> fmt::Debug for IoChain<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IoChain").field("t", &self.first).field("u", &self.second).finish()
    }
}
impl<T: IoRead, U: IoRead> IoRead for IoChain<T, U> {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
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
    fn fill_buf(&mut self) -> IoResult<&[u8]> {
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
        iif![!self.done_first; self.first.consume(amt); self.second.consume(amt)];
    }
}

// TODO:IO_LINES
// /// An iterator over the lines of an instance of [`IoBufRead`].
// ///
// /// This struct is generally created by calling [`lines`] on an `IoBufRead`.
// /// Please see the documentation of [`lines`] for more details.
// ///
// /// See: <https://doc.rust-lang.org/std/io/struct.Lines.html>
// ///
// /// [`lines`]: IoBufRead::lines
// #[derive(Debug)]
// #[cfg(feature = "alloc")]
// #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
// pub struct IoLines<B> {
//     buf: B,
// }
//
// #[cfg(feature = "alloc")]
// #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
// impl<B: IoBufRead> Iterator for IoLines<B> {
//     type Item = IoResult<String>;
//
//     fn next(&mut self) -> Option<IoResult<String>> {
//         let mut buf = String::new();
//         match self.buf.read_line(&mut buf) {
//             Ok(0) => None,
//             Ok(_n) => {
//                 if buf.ends_with('\n') {
//                     buf.pop();
//                     if buf.ends_with('\r') {
//                         buf.pop();
//                     }
//                 }
//                 Some(Ok(buf))
//             }
//             Err(e) => Some(Err(e)),
//         }
//     }
// }

/// Reader adaptor which limits the bytes read from an underlying reader.
///
/// This struct is generally created by calling [`take`][IoRead::take] on a reader.
///
/// See <https://doc.rust-lang.org/std/io/trait.Take.html>.
#[derive(Debug)]
pub struct IoTake<T> {
    inner: T,
    limit: u64,
}
#[rustfmt::skip]
impl<T> IoTake<T> {
    pub(super) fn new(inner: T, limit: u64) -> Self { IoTake { inner, limit } }
    /// Returns the number of bytes that can be read before this instance will return EOF.
    pub fn limit(&self) -> u64 { self.limit }
    /// Sets the number of bytes that can be read before this instance will return EOF.
    pub fn set_limit(&mut self, limit: u64) { self.limit = limit; }
    /// Consumes the `IoTake`, returning the wrapped reader.
    pub fn into_inner(self) -> T { self.inner }
    /// Gets a reference to the underlying reader.
    pub fn get_ref(&self) -> &T { &self.inner }
    /// Gets a mutable reference to the underlying reader.
    pub fn get_mut(&mut self) -> &mut T { &mut self.inner }
}
impl<T: IoRead> IoRead for IoTake<T> {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        // Don't call into inner reader at all at EOF because it may still block
        iif![self.limit == 0; return Ok(0)];
        let max = cmp::min(buf.len() as u64, self.limit) as usize;
        let n = self.inner.read(&mut buf[..max])?;
        self.limit -= n as u64;
        Ok(n)
    }
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> IoResult<usize> {
        // Pass in a reservation_size closure that respects the current value
        // of limit for each read. If we hit the read limit, this prevents the
        // final zero-byte read from allocating again.
        alloc_impls::read_to_end_with_reservation(self, buf, |self_| {
            cmp::min(self_.limit, 32) as usize
        })
    }
}
impl<T: IoBufRead> IoBufRead for IoTake<T> {
    fn fill_buf(&mut self) -> IoResult<&[u8]> {
        // Don't call into inner reader at all at EOF because it may still block
        iif![self.limit == 0; return Ok(&[])];
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

/* impls */

sf! {
    impl<R: IoRead + ?Sized> IoRead for &mut R {
        fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> { (**self).read(buf) }
        fn read_exact(&mut self, buf: &mut [u8]) -> IoResult<()> { (**self).read_exact(buf) }
    }
    impl<B: IoBufRead + ?Sized> IoBufRead for &mut B {
        fn fill_buf(&mut self) -> IoResult<&[u8]> { (**self).fill_buf() }
        fn consume(&mut self, amt: usize) { (**self).consume(amt); }
    }
}

/// `IoRead` is implemented for `&[u8]` by copying from the slice.
///
/// Note that reading updates the slice to point to the yet unread part.
/// The slice will be empty when EOF is reached.
impl IoRead for &[u8] {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        let amt = cmp::min(buf.len(), self.len());
        let (a, b) = self.split_at(amt);
        // First check if the amount of bytes we want to read is small:
        // `copy_from_slice` will generally expand to a call to `memcpy`, and
        // for a single byte the overhead is significant.
        iif![amt == 1; buf[0] = a[0]; buf[..amt].copy_from_slice(a)];
        *self = b;
        Ok(amt)
    }
    fn read_exact(&mut self, buf: &mut [u8]) -> IoResult<()> {
        if buf.len() > self.len() {
            return Err(IoError::new(IoErrorKind::UnexpectedEof, "failed to fill whole buffer"));
        }
        let (a, b) = self.split_at(buf.len());
        // See equivalent comment in `read` method.
        iif![buf.len() == 1; buf[0] = a[0]; buf.copy_from_slice(a)];
        *self = b;
        Ok(())
    }
}
#[rustfmt::skip]
impl IoBufRead for &[u8] {
    fn fill_buf(&mut self) -> IoResult<&[u8]> { Ok(*self) }
    fn consume(&mut self, amt: usize) { *self = &self[amt..]; }
}

#[cfg(feature = "alloc")]
mod alloc_impls {
    use super::*;

    /// Reads all bytes from a reader into the given buffer, adapting the buffer size as needed.
    ///
    /// This function uses an adaptive system to efficiently handle varying amounts of data.
    /// It avoids allocating excessive memory for small reads, while still efficiently handling
    /// larger data sizes. The default reservation size is set to 32 bytes, balancing memory
    /// usage with performance for different read sizes.
    ///
    /// For safety, this function ensures that any allocated but uninitialized part of the buffer
    /// is truncated in case of a panic, preventing exposure of uninitialized data.
    pub(super) fn read_to_end<R: IoRead + ?Sized>(r: &mut R, buf: &mut Vec<u8>) -> IoResult<usize> {
        read_to_end_with_reservation(r, buf, |_| 32)
    }

    /// SAFE version
    #[cfg(any(feature = "safe_sys", not(feature = "unsafe_slice")))]
    pub(super) fn read_to_end_with_reservation<R, F>(
        r: &mut R,
        buf: &mut Vec<u8>,
        mut reservation_size: F,
    ) -> IoResult<usize>
    where
        R: IoRead + ?Sized,
        F: FnMut(&R) -> usize,
    {
        const PROBE_SIZE: usize = 32;
        fn small_probe_read<R: IoRead + ?Sized>(
            r: &mut R,
            buf: &mut Vec<u8>,
            probe_size: usize,
        ) -> IoResult<usize> {
            let mut probe = crate::vec_![0u8; probe_size];
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
            iif![read == 0; return Ok(0)];
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
    /// UNSAFE version
    #[cfg(all(not(feature = "safe_sys"), feature = "unsafe_slice"))]
    pub(super) fn read_to_end_with_reservation<R, F>(
        r: &mut R,
        buf: &mut Vec<u8>,
        mut reservation_size: F,
    ) -> IoResult<usize>
    where
        R: IoRead + ?Sized,
        F: FnMut(&R) -> usize,
    {
        const PROBE_SIZE: usize = 32;
        fn small_probe_read<R: IoRead + ?Sized>(
            r: &mut R,
            buf: &mut Vec<u8>,
            _probe_size: usize,
        ) -> IoResult<usize> {
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
            iif![read == 0; return Ok(0)];
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
