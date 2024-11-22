// devela::sys::io::reimplement_no_std::buffered
//
//! Buffering wrappers for I/O traits

use super::{
    IoBufRead, IoError, IoErrorKind, IoRead, IoResult as Result, IoSeek, IoSeekFrom, IoWrite,
};
use core::{cmp, fmt};

#[cfg(feature = "dep_memchr")]
use memchr::memrchr;
#[rustfmt::skip]
#[cfg(not(feature = "dep_memchr"))]
fn memrchr(needle: u8, haystack: &[u8]) -> Option<usize> {
    haystack.iter().enumerate().rev().find_map(
        |(index, &byte)| crate::iif![byte == needle; Some(index); None])
}

/// The `IoBufReader<R, S>` struct adds buffering to any reader.
///
/// See <https://doc.rust-lang.org/std/io/struct.BufReader.html>.
pub struct IoBufReader<R, const S: usize> {
    inner: R,
    buf: [u8; S],
    pos: usize,
    cap: usize,
}

impl<R: IoRead, const S: usize> IoBufReader<R, S> {
    /// Creates a new `IoBufReader<R, S>` with a default buffer capacity.
    ///
    /// See <https://doc.rust-lang.org/std/io/struct.BufReader.html#method.new>.
    pub fn new(inner: R) -> IoBufReader<R, S> {
        IoBufReader { inner, buf: [0; S], pos: 0, cap: 0 }
    }
}

impl<R, const S: usize> IoBufReader<R, S> {
    /// Gets a reference to the underlying reader.
    ///
    /// See <https://doc.rust-lang.org/std/io/struct.BufReader.html#method.get_ref>.
    pub fn get_ref(&self) -> &R {
        &self.inner
    }

    /// Gets a mutable reference to the underlying reader.
    ///
    /// See <https://doc.rust-lang.org/std/io/struct.BufReader.html#method.get_mut>.
    pub fn get_mut(&mut self) -> &mut R {
        &mut self.inner
    }

    /// Returns a reference to the internally buffered data.
    ///
    /// See <https://doc.rust-lang.org/std/io/struct.BufReader.html#method.buffer>.
    pub fn buffer(&self) -> &[u8] {
        &self.buf[self.pos..self.cap]
    }

    /// Returns the number of bytes the internal buffer can hold at once.
    ///
    /// See <https://doc.rust-lang.org/std/io/struct.BufReader.html#method.capacity>.
    pub fn capacity(&self) -> usize {
        S
    }

    /// Unwraps this `IoBufReader<R, S>`, returning the underlying reader.
    ///
    /// See <https://doc.rust-lang.org/std/io/struct.BufReader.html#method.into_inner>.
    pub fn into_inner(self) -> R {
        self.inner
    }

    /// Invalidates all data in the internal buffer.
    fn discard_buffer(&mut self) {
        self.pos = 0;
        self.cap = 0;
    }
}

impl<R: IoRead, const S: usize> IoRead for IoBufReader<R, S> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        // If we don't have any buffered data and we're doing a massive read
        // (larger than our internal buffer), bypass our internal buffer
        // entirely.
        if self.pos == self.cap && buf.len() >= S {
            self.discard_buffer();
            return self.inner.read(buf);
        }
        let nread = {
            let mut rem = self.fill_buf()?;
            rem.read(buf)?
        };
        self.consume(nread);
        Ok(nread)
    }
}

impl<R: IoRead, const S: usize> IoBufRead for IoBufReader<R, S> {
    fn fill_buf(&mut self) -> Result<&[u8]> {
        // If we've reached the end of our internal buffer then we need to fetch
        // some more data from the underlying reader.
        // Branch using `>=` instead of the more correct `==`
        // to tell the compiler that the pos..cap slice is always valid.
        if self.pos >= self.cap {
            debug_assert!(self.pos == self.cap);
            self.cap = self.inner.read(&mut self.buf)?;
            self.pos = 0;
        }
        Ok(&self.buf[self.pos..self.cap])
    }

    fn consume(&mut self, amt: usize) {
        self.pos = cmp::min(self.pos + amt, self.cap);
    }
}

impl<R, const S: usize> fmt::Debug for IoBufReader<R, S>
where
    R: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("IoBufReader")
            .field("reader", &self.inner)
            .field("buffer", &format_args!("{}/{}", self.cap - self.pos, S))
            .finish()
    }
}

impl<R: IoSeek, const S: usize> IoSeek for IoBufReader<R, S> {
    /// Seek to an offset, in bytes, in the underlying reader.
    ///
    /// See <https://doc.rust-lang.org/std/io/struct.BufReader.html#method.seek>.
    fn seek(&mut self, pos: IoSeekFrom) -> Result<u64> {
        let result: u64;
        if let IoSeekFrom::Current(n) = pos {
            let remainder = (self.cap - self.pos) as i64;
            // it should be safe to assume that remainder fits within an i64 as the alternative
            // means we managed to allocate 8 exbibytes and that's absurd.
            // But it's not out of the realm of possibility for some weird underlying reader to
            // support seeking by i64::MIN so we need to handle underflow when subtracting
            // remainder.
            if let Some(offset) = n.checked_sub(remainder) {
                result = self.inner.seek(IoSeekFrom::Current(offset))?;
            } else {
                // seek backwards by our remainder, and then by the offset
                self.inner.seek(IoSeekFrom::Current(-remainder))?;
                self.discard_buffer();
                result = self.inner.seek(IoSeekFrom::Current(n))?;
            }
        } else {
            // Seeking with Start/End doesn't care about our buffer length.
            result = self.inner.seek(pos)?;
        }
        self.discard_buffer();
        Ok(result)
    }
}

/// Wraps a writer and buffers its output.
///
/// See <https://doc.rust-lang.org/std/io/struct.BufWriter.html>.
pub struct IoBufWriter<W: IoWrite, const S: usize> {
    inner: Option<W>,
    buf: [u8; S],
    len: usize,
    // #30888: If the inner writer panics in a call to write, we don't want to
    // write the buffered data a second time in IoBufWriter's destructor. This
    // flag tells the Drop impl if it should skip the flush.
    panicked: bool,
}

/// An error returned by [`IoBufWriter::into_inner`] which combines an error that
/// happened while writing out the buffer, and the buffered writer object
/// which may be used to recover from the condition.
///
/// See <https://doc.rust-lang.org/std/io/struct.IntoInnerError.html>.
#[derive(Debug)]
pub struct IntoInnerError<W>(W, IoError);

impl<W, const S: usize> IoBufWriter<W, S>
where
    W: IoWrite,
{
    /// Creates a new `IoBufWriter<W>` with a default buffer capacity. The default is currently 8 KB,
    /// but may change in the future.
    ///
    /// See <https://doc.rust-lang.org/std/io/struct.BufWriter.html#method.new>.
    pub fn new(inner: W) -> IoBufWriter<W, S> {
        IoBufWriter {
            inner: Some(inner),
            buf: [0; S],
            len: 0,
            panicked: false,
        }
    }

    /// Send data in our local buffer into the inner writer, looping as
    /// necessary until either it's all been sent or an error occurs.
    ///
    /// See <https://doc.rust-lang.org/std/io/struct.BufWriter.html#method.flush_buf>.
    fn flush_buf(&mut self) -> Result<()> {
        /// Helper struct to ensure the buffer is updated after all the writes
        /// are complete. It tracks the number of written bytes and drains them
        /// all from the front of the buffer when dropped.
        struct BufGuard<'a, const S: usize> {
            buffer: &'a mut [u8; S],
            written: usize,
        }

        impl<'a, const S: usize> BufGuard<'a, S> {
            fn new(buffer: &'a mut [u8; S]) -> Self {
                Self { buffer, written: 0 }
            }

            /// The unwritten part of the buffer
            fn remaining(&self) -> &[u8] {
                &self.buffer[self.written..]
            }

            /// Flag some bytes as removed from the front of the buffer
            fn consume(&mut self, amt: usize) {
                self.written += amt;
            }

            /// true if all of the bytes have been written
            fn done(&self) -> bool {
                self.written >= self.buffer.len()
            }
        }

        impl<const S: usize> Drop for BufGuard<'_, S> {
            fn drop(&mut self) {
                if self.written > 0 {
                    let mut new_buf = [0; S];
                    new_buf.copy_from_slice(&self.buffer[self.written..]);
                    *self.buffer = new_buf;
                }
            }
        }

        let mut guard = BufGuard::new(&mut self.buf);
        let inner = self.inner.as_mut().unwrap();
        while !guard.done() {
            self.panicked = true;
            let r = inner.write(guard.remaining());
            self.panicked = false;

            match r {
                Ok(0) => {
                    return Err(IoError::new(
                        IoErrorKind::WriteZero,
                        "failed to write the buffered data",
                    ));
                }
                Ok(n) => guard.consume(n),
                Err(ref e) if e.kind() == IoErrorKind::Interrupted => {}
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    /// Buffer some data without flushing it, regardless of the size of the
    /// data. Writes as much as possible without exceeding capacity. Returns
    /// the number of bytes written.
    fn write_to_buf(&mut self, buf: &[u8]) -> usize {
        let available = S - self.len;
        let amt_to_buffer = available.min(buf.len());
        self.buf[available..].copy_from_slice(&buf[..amt_to_buffer]);
        self.len += amt_to_buffer;
        amt_to_buffer
    }

    /// Gets a reference to the underlying writer.
    ///
    /// See <https://doc.rust-lang.org/std/io/struct.BufWriter.html#method.get_ref>.
    pub fn get_ref(&self) -> &W {
        self.inner.as_ref().unwrap()
    }

    /// Gets a mutable reference to the underlying writer.
    ///
    /// See <https://doc.rust-lang.org/std/io/struct.BufWriter.html#method.get_mut>.
    pub fn get_mut(&mut self) -> &mut W {
        self.inner.as_mut().unwrap()
    }

    /// Returns a reference to the internally buffered data.
    ///
    /// See <https://doc.rust-lang.org/std/io/struct.BufWriter.html#method.buffer>.
    pub fn buffer(&self) -> &[u8] {
        &self.buf
    }

    /// Returns the number of bytes the internal buffer can hold without flushing.
    ///
    /// See <https://doc.rust-lang.org/std/io/struct.BufWriter.html#method.capacity>.
    pub fn capacity(&self) -> usize {
        S
    }

    /// Unwraps this `IoBufWriter<W>`, returning the underlying writer.
    ///
    /// See <https://doc.rust-lang.org/std/io/struct.BufWriter.html#method.into_inner>.
    pub fn into_inner(mut self) -> core::result::Result<W, IntoInnerError<IoBufWriter<W, S>>> {
        match self.flush_buf() {
            Err(e) => Err(IntoInnerError(self, e)),
            Ok(()) => Ok(self.inner.take().unwrap()),
        }
    }
}

impl<W: IoWrite, const S: usize> IoWrite for IoBufWriter<W, S> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        if self.len + buf.len() > S {
            self.flush_buf()?;
        }
        // MAYBE:FIXME? https://github.com/rust-lang/rust/issues/72919
        // Why no len > capacity? Why not buffer len == capacity?
        if buf.len() >= S {
            self.panicked = true;
            let r = self.get_mut().write(buf);
            self.panicked = false;
            r
        } else {
            self.buf.copy_from_slice(buf);
            Ok(buf.len())
        }
    }

    fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        // Normally, `write_all` just calls `write` in a loop. We can do better
        // by calling `self.get_mut().write_all()` directly, which avoids
        // round trips through the buffer in the event of a series of partial
        // writes in some circumstances.
        if self.len + buf.len() > S {
            self.flush_buf()?;
        }
        // MAYBE:FIXME? https://github.com/rust-lang/rust/issues/72919
        if buf.len() >= S {
            self.panicked = true;
            let r = self.get_mut().write_all(buf);
            self.panicked = false;
            r
        } else {
            self.buf.copy_from_slice(buf);
            Ok(())
        }
    }

    fn flush(&mut self) -> Result<()> {
        self.flush_buf().and_then(|()| self.get_mut().flush())
    }
}

impl<W: IoWrite, const S: usize> fmt::Debug for IoBufWriter<W, S>
where
    W: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("IoBufWriter")
            .field("writer", &self.inner.as_ref().unwrap())
            .field("buffer", &format_args!("{}/{}", self.buf.len(), S))
            .finish()
    }
}

impl<W: IoWrite + IoSeek, const S: usize> IoSeek for IoBufWriter<W, S> {
    /// Seek to the offset, in bytes, in the underlying writer.
    ///
    /// Seeking always writes out the internal buffer before seeking.
    fn seek(&mut self, pos: IoSeekFrom) -> Result<u64> {
        self.flush_buf()?;
        self.get_mut().seek(pos)
    }
}

impl<W: IoWrite, const S: usize> Drop for IoBufWriter<W, S> {
    fn drop(&mut self) {
        if self.inner.is_some() && !self.panicked {
            // dtors should not panic, so we ignore a failed flush
            let _r = self.flush_buf();
        }
    }
}

impl<W> IntoInnerError<W> {
    /// Returns the error which caused the call to [`IoBufWriter::into_inner()`]
    /// to fail.
    ///
    /// See <https://doc.rust-lang.org/std/io/struct.IntoInnerError.html#method.error>.
    pub fn error(&self) -> &IoError {
        &self.1
    }

    /// Returns the buffered writer instance which generated the error.
    ///
    /// See <https://doc.rust-lang.org/std/io/struct.IntoInnerError.html#method.into_inner>.
    pub fn into_inner(self) -> W {
        self.0
    }
}

impl<W> From<IntoInnerError<W>> for IoError {
    fn from(iie: IntoInnerError<W>) -> IoError {
        iie.1
    }
}

impl<W> fmt::Display for IntoInnerError<W> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.error().fmt(f)
    }
}

/// Private helper struct for implementing the line-buffered writing logic.
/// This shim temporarily wraps an `IoBufWriter`, and uses its internals to
/// implement a line-buffered writer (specifically by using the internal
/// methods like write_to_buf and flush_buf). In this way, a more
/// efficient abstraction can be created than one that only had access to
/// `write` and `flush`, without needlessly duplicating a lot of the
/// implementation details of `IoBufWriter`. This also allows existing
/// `BufWriters` to be temporarily given line-buffering logic; this is what
/// enables Stdout to be alternately in line-buffered or block-buffered mode.
#[derive(Debug)]
pub(super) struct LineWriterShim<'a, W: IoWrite, const S: usize> {
    buffer: &'a mut IoBufWriter<W, S>,
}

impl<'a, W: IoWrite, const S: usize> LineWriterShim<'a, W, S> {
    pub fn new(buffer: &'a mut IoBufWriter<W, S>) -> Self {
        Self { buffer }
    }

    /// Get a mutable reference to the inner writer (that is, the writer
    /// wrapped by the `IoBufWriter`). Be careful with this writer, as writes to
    /// it will bypass the buffer.
    fn inner_mut(&mut self) -> &mut W {
        self.buffer.get_mut()
    }

    /// Get the content currently buffered in self.buffer
    fn buffered(&self) -> &[u8] {
        self.buffer.buffer()
    }

    /// Flush the buffer iff the last byte is a newline (indicating that an
    /// earlier write only succeeded partially, and we want to retry flushing
    /// the buffered line before continuing with a subsequent write)
    fn flush_if_completed_line(&mut self) -> Result<()> {
        match self.buffered().last().copied() {
            Some(b'\n') => self.buffer.flush_buf(),
            _ => Ok(()),
        }
    }
}

impl<W: IoWrite, const S: usize> IoWrite for LineWriterShim<'_, W, S> {
    /// Write some data into this `IoBufReader` with line buffering. This means
    /// that, if any newlines are present in the data, the data up to the last
    /// newline is sent directly to the underlying writer, and data after it
    /// is buffered. Returns the number of bytes written.
    ///
    /// This function operates on a "best effort basis"; in keeping with the
    /// convention of `Write::write`, it makes at most one attempt to write
    /// new data to the underlying writer. If that write only reports a partial
    /// success, the remaining data will be buffered.
    ///
    /// Because this function attempts to send completed lines to the underlying
    /// writer, it will also flush the existing buffer if it ends with a
    /// newline, even if the incoming data does not contain any newlines.
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let newline_idx = match memrchr(b'\n', buf) {
            // If there are no new newlines (that is, if this write is less than
            // one line), just do a regular buffered write (which may flush if
            // we exceed the inner buffer's size)
            None => {
                self.flush_if_completed_line()?;
                return self.buffer.write(buf);
            }
            // Otherwise, arrange for the lines to be written directly to the
            // inner writer.
            Some(newline_idx) => newline_idx + 1,
        };

        // Flush existing content to prepare for our write. We have to do this
        // before attempting to write `buf` in order to maintain consistency;
        // if we add `buf` to the buffer then try to flush it all at once,
        // we're obligated to return Ok(), which would mean suppressing any
        // errors that occur during flush.
        self.buffer.flush_buf()?;

        // This is what we're going to try to write directly to the inner
        // writer. The rest will be buffered, if nothing goes wrong.
        let lines = &buf[..newline_idx];

        // Write `lines` directly to the inner writer. In keeping with the
        // `write` convention, make at most one attempt to add new (unbuffered)
        // data. Because this write doesn't touch the IoBufWriter state directly,
        // and the buffer is known to be empty, we don't need to worry about
        // self.buffer.panicked here.
        let flushed = self.inner_mut().write(lines)?;

        // If buffer returns Ok(0), propagate that to the caller without
        // doing additional buffering; otherwise we're just guaranteeing
        // an "IoErrorKind::WriteZero" later.
        if flushed == 0 {
            return Ok(0);
        }

        // Now that the write has succeeded, buffer the rest (or as much of
        // the rest as possible). If there were any unwritten newlines, we
        // only buffer out to the last unwritten newline that fits in the
        // buffer; this helps prevent flushing partial lines on subsequent
        // calls to LineWriterShim::write.

        // Handle the cases in order of most-common to least-common, under
        // the presumption that most writes succeed in totality, and that most
        // writes are smaller than the buffer.
        // - Is this a partial line (ie, no newlines left in the unwritten tail)
        // - If not, does the data out to the last unwritten newline fit in
        //   the buffer?
        // - If not, scan for the last newline that *does* fit in the buffer
        let tail = if flushed >= newline_idx {
            &buf[flushed..]
        } else if newline_idx - flushed <= self.buffer.capacity() {
            &buf[flushed..newline_idx]
        } else {
            let scan_area = &buf[flushed..];
            let scan_area = &scan_area[..self.buffer.capacity()];
            match memrchr(b'\n', scan_area) {
                Some(newline_idx) => &scan_area[..newline_idx + 1],
                None => scan_area,
            }
        };

        let buffered = self.buffer.write_to_buf(tail);
        Ok(flushed + buffered)
    }

    fn flush(&mut self) -> Result<()> {
        self.buffer.flush()
    }

    /// Write some data into this `IoBufReader` with line buffering. This means
    /// that, if any newlines are present in the data, the data up to the last
    /// newline is sent directly to the underlying writer, and data after it
    /// is buffered.
    ///
    /// Because this function attempts to send completed lines to the underlying
    /// writer, it will also flush the existing buffer if it contains any
    /// newlines, even if the incoming data does not contain any newlines.
    fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        match memrchr(b'\n', buf) {
            // If there are no new newlines (that is, if this write is less than
            // one line), just do a regular buffered write (which may flush if
            // we exceed the inner buffer's size)
            None => {
                self.flush_if_completed_line()?;
                self.buffer.write_all(buf)
            }
            Some(newline_idx) => {
                let (lines, tail) = buf.split_at(newline_idx + 1);

                if self.buffered().is_empty() {
                    self.inner_mut().write_all(lines)?;
                } else {
                    // If there is any buffered data, we add the incoming lines
                    // to that buffer before flushing, which saves us at least
                    // one write call. We can't really do this with `write`,
                    // since we can't do this *and* not suppress errors *and*
                    // report a consistent state to the caller in a return
                    // value, but here in write_all it's fine.
                    self.buffer.write_all(lines)?;
                    self.buffer.flush_buf()?;
                }

                self.buffer.write_all(tail)
            }
        }
    }
}

/// Wraps a writer and buffers output to it, flushing whenever a newline
/// (`0x0a`, `'\n'`) is detected.
///
/// See <https://doc.rust-lang.org/std/io/struct.LineWriter.html>.
pub struct IoLineWriter<W: IoWrite, const S: usize> {
    inner: IoBufWriter<W, S>,
}

impl<W: IoWrite, const S: usize> IoLineWriter<W, S> {
    /// Creates a new `IoLineWriter`.
    ///
    /// See <https://doc.rust-lang.org/std/io/struct.LineWriter.html#method.new>.
    pub fn new(inner: W) -> IoLineWriter<W, S> {
        IoLineWriter { inner: IoBufWriter::new(inner) }
    }

    /// Gets a reference to the underlying writer.
    ///
    /// See <https://doc.rust-lang.org/std/io/struct.LineWriter.html#method.get_ref>.
    pub fn get_ref(&self) -> &W {
        self.inner.get_ref()
    }

    /// Gets a mutable reference to the underlying writer.
    ///
    /// See <https://doc.rust-lang.org/std/io/struct.LineWriter.html#method.get_mut>.
    pub fn get_mut(&mut self) -> &mut W {
        self.inner.get_mut()
    }

    /// Unwraps this `IoLineWriter`, returning the underlying writer.
    ///
    /// See <https://doc.rust-lang.org/std/io/struct.LineWriter.html#method.into_inner>.
    pub fn into_inner(self) -> core::result::Result<W, IntoInnerError<IoLineWriter<W, S>>> {
        self.inner
            .into_inner()
            .map_err(|IntoInnerError(buf, e)| IntoInnerError(IoLineWriter { inner: buf }, e))
    }
}

impl<W: IoWrite, const S: usize> IoWrite for IoLineWriter<W, S> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        LineWriterShim::new(&mut self.inner).write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.inner.flush()
    }

    fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        LineWriterShim::new(&mut self.inner).write_all(buf)
    }

    fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> Result<()> {
        LineWriterShim::new(&mut self.inner).write_fmt(fmt)
    }
}

impl<W: IoWrite, const S: usize> fmt::Debug for IoLineWriter<W, S>
where
    W: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("IoLineWriter")
            .field("writer", &self.inner.inner)
            .field("buffer", &format_args!("{}/{}", self.inner.len, S))
            .finish()
    }
}
