// devela::sys::io::reimplement_no_std::cursor
//
//! Defines [`IoCursor`], [`IoSeek`], [`IoSeekFrom`].
//
// TOC
// - trait IoSeek
// - enum IoSeekFrom
// - struct IoCursor

use crate::{IoBufRead, IoError, IoErrorKind, IoRead, IoResult, IoWrite};
use ::core::cmp;

/// The `IoSeek` trait provides a cursor which can be moved within a stream of
/// bytes.
///
/// This struct is generally created by calling [`bytes`][IoRead::bytes] on a reader.
///
/// See <https://doc.rust-lang.org/std/io/trait.Seek.html>.
pub trait IoSeek {
    /// Seek to an offset, in bytes, in a stream.
    fn seek(&mut self, pos: IoSeekFrom) -> IoResult<u64>;
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

/// Wraps an in-memory buffer and provides it with an [`IoSeek`] implementation.
///
/// See <https://doc.rust-lang.org/std/io/struct.Cursor.html>.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct IoCursor<T> {
    inner: T,
    pos: u64,
}
#[rustfmt::skip]
impl<T> IoCursor<T> {
    /// Creates a new cursor wrapping the provided underlying in-memory buffer.
    pub fn new(inner: T) -> IoCursor<T> { IoCursor { pos: 0, inner } }
    /// Consumes this cursor, returning the underlying value.
    pub fn into_inner(self) -> T { self.inner }
    /// Gets a reference to the underlying value in this cursor.
    pub fn get_ref(&self) -> &T { &self.inner }
    /// Gets a mutable reference to the underlying value in this cursor.
    pub fn get_mut(&mut self) -> &mut T { &mut self.inner }
    /// Returns the current position of this cursor.
    pub fn position(&self) -> u64 { self.pos }
    /// Sets the position of this cursor.
    pub fn set_position(&mut self, pos: u64) { self.pos = pos; }
}
#[rustfmt::skip]
impl<T: AsRef<[u8]>> IoSeek for IoCursor<T> {
    fn seek(&mut self, style: IoSeekFrom) -> IoResult<u64> {
        let (base_pos, offset) = match style {
            IoSeekFrom::Start(n) => { self.pos = n; return Ok(n); }
            IoSeekFrom::End(n) => (self.inner.as_ref().len() as u64, n),
            IoSeekFrom::Current(n) => (self.pos, n),
        };
        let new_pos = if offset >= 0 { base_pos.checked_add(offset as u64) }
            else { base_pos.checked_sub((offset.wrapping_neg()) as u64) };
        match new_pos {
            Some(n) => { self.pos = n; Ok(self.pos) }
            None => Err(IoError::new(
                IoErrorKind::InvalidInput, "invalid seek to a negative or overflowing position",
            )),
        }
    }
}

impl<T: AsRef<[u8]>> IoRead for IoCursor<T> {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        let n = IoRead::read(&mut self.fill_buf()?, buf)?;
        self.pos += n as u64;
        Ok(n)
    }
    fn read_exact(&mut self, buf: &mut [u8]) -> IoResult<()> {
        let n = buf.len();
        IoRead::read_exact(&mut self.fill_buf()?, buf)?;
        self.pos += n as u64;
        Ok(())
    }
}

impl<T: AsRef<[u8]>> IoBufRead for IoCursor<T> {
    fn fill_buf(&mut self) -> IoResult<&[u8]> {
        let amt = cmp::min(self.pos, self.inner.as_ref().len() as u64);
        Ok(&self.inner.as_ref()[(amt as usize)..])
    }
    fn consume(&mut self, amt: usize) {
        self.pos += amt as u64;
    }
}

// Non-resizing write implementation
fn slice_write(pos_mut: &mut u64, slice: &mut [u8], buf: &[u8]) -> IoResult<usize> {
    let pos = cmp::min(*pos_mut, slice.len() as u64);
    let amt = (&mut slice[(pos as usize)..]).write(buf)?;
    *pos_mut += amt as u64;
    Ok(amt)
}

impl IoWrite for IoCursor<&mut [u8]> {
    fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
        slice_write(&mut self.pos, self.inner, buf)
    }
    fn flush(&mut self) -> IoResult<()> {
        Ok(())
    }
}
