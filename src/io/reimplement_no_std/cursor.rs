// devela::io::reimplement_no_std::cursor

use super::{BufRead, IoError, IoErrorKind, IoResult as Result, Read, Seek, SeekFrom, Write};
use core::cmp;

/// A `Cursor` wraps an in-memory buffer and provides it with a
/// [`Seek`] implementation.
///
/// See <https://doc.rust-lang.org/std/io/struct.Cursor.html>.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Cursor<T> {
    inner: T,
    pos: u64,
}

impl<T> Cursor<T> {
    /// Creates a new cursor wrapping the provided underlying in-memory buffer.
    ///
    /// See <https://doc.rust-lang.org/std/io/struct.Cursor.html#method.new>.
    pub fn new(inner: T) -> Cursor<T> {
        Cursor { pos: 0, inner }
    }

    /// Consumes this cursor, returning the underlying value.
    ///
    /// See <https://doc.rust-lang.org/std/io/struct.Cursor.html#method.into_inner>.
    pub fn into_inner(self) -> T {
        self.inner
    }

    /// Gets a reference to the underlying value in this cursor.
    ///
    /// See <https://doc.rust-lang.org/std/io/struct.Cursor.html#method.get_ref>.
    pub fn get_ref(&self) -> &T {
        &self.inner
    }

    /// Gets a mutable reference to the underlying value in this cursor.
    ///
    /// See <https://doc.rust-lang.org/std/io/struct.Cursor.html#method.get_mut>.
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    /// Returns the current position of this cursor.
    ///
    /// See <https://doc.rust-lang.org/std/io/struct.Cursor.html#method.position>.
    pub fn position(&self) -> u64 {
        self.pos
    }

    /// Sets the position of this cursor.
    ///
    /// See <https://doc.rust-lang.org/std/io/struct.Cursor.html#method.set_position>.
    pub fn set_position(&mut self, pos: u64) {
        self.pos = pos;
    }
}

impl<T> Seek for Cursor<T>
where
    T: AsRef<[u8]>,
{
    fn seek(&mut self, style: SeekFrom) -> Result<u64> {
        let (base_pos, offset) = match style {
            SeekFrom::Start(n) => {
                self.pos = n;
                return Ok(n);
            }
            SeekFrom::End(n) => (self.inner.as_ref().len() as u64, n),
            SeekFrom::Current(n) => (self.pos, n),
        };
        let new_pos = if offset >= 0 {
            base_pos.checked_add(offset as u64)
        } else {
            base_pos.checked_sub((offset.wrapping_neg()) as u64)
        };
        match new_pos {
            Some(n) => {
                self.pos = n;
                Ok(self.pos)
            }
            None => Err(IoError::new(
                IoErrorKind::InvalidInput,
                "invalid seek to a negative or overflowing position",
            )),
        }
    }
}

impl<T> Read for Cursor<T>
where
    T: AsRef<[u8]>,
{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let n = Read::read(&mut self.fill_buf()?, buf)?;
        self.pos += n as u64;
        Ok(n)
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        let n = buf.len();
        Read::read_exact(&mut self.fill_buf()?, buf)?;
        self.pos += n as u64;
        Ok(())
    }
}

impl<T> BufRead for Cursor<T>
where
    T: AsRef<[u8]>,
{
    fn fill_buf(&mut self) -> Result<&[u8]> {
        let amt = cmp::min(self.pos, self.inner.as_ref().len() as u64);
        Ok(&self.inner.as_ref()[(amt as usize)..])
    }
    fn consume(&mut self, amt: usize) {
        self.pos += amt as u64;
    }
}

// Non-resizing write implementation
#[inline]
fn slice_write(pos_mut: &mut u64, slice: &mut [u8], buf: &[u8]) -> Result<usize> {
    let pos = cmp::min(*pos_mut, slice.len() as u64);
    let amt = (&mut slice[(pos as usize)..]).write(buf)?;
    *pos_mut += amt as u64;
    Ok(amt)
}

impl Write for Cursor<&mut [u8]> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        slice_write(&mut self.pos, self.inner, buf)
    }

    #[inline]
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}
