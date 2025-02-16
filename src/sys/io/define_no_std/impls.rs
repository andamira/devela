// devela::sys::io::reimplement_no_std::impls

use crate::{IoBufRead, IoError, IoErrorKind, IoRead, IoResult, IoSeek, IoSeekFrom, IoWrite};
use ::core::{cmp, fmt, mem};

// =============================================================================
// Forwarding implementations

impl<R: IoRead + ?Sized> IoRead for &mut R {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        (**self).read(buf)
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> IoResult<()> {
        (**self).read_exact(buf)
    }
}

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

impl<S: IoSeek + ?Sized> IoSeek for &mut S {
    fn seek(&mut self, pos: IoSeekFrom) -> IoResult<u64> {
        (**self).seek(pos)
    }
}

impl<B: IoBufRead + ?Sized> IoBufRead for &mut B {
    fn fill_buf(&mut self) -> IoResult<&[u8]> {
        (**self).fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        (**self).consume(amt);
    }
}

// =============================================================================
// In-memory buffer implementations

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
        if amt == 1 {
            buf[0] = a[0];
        } else {
            buf[..amt].copy_from_slice(a);
        }

        *self = b;
        Ok(amt)
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> IoResult<()> {
        if buf.len() > self.len() {
            return Err(IoError::new(IoErrorKind::UnexpectedEof, "failed to fill whole buffer"));
        }
        let (a, b) = self.split_at(buf.len());

        // First check if the amount of bytes we want to read is small:
        // `copy_from_slice` will generally expand to a call to `memcpy`, and
        // for a single byte the overhead is significant.
        if buf.len() == 1 {
            buf[0] = a[0];
        } else {
            buf.copy_from_slice(a);
        }

        *self = b;
        Ok(())
    }
}

impl IoBufRead for &[u8] {
    fn fill_buf(&mut self) -> IoResult<&[u8]> {
        Ok(*self)
    }

    fn consume(&mut self, amt: usize) {
        *self = &self[amt..];
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
        let (a, b) = mem::take(self).split_at_mut(amt);
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

/// Write is implemented for `Vec<u8>` by appending to the vector.
/// The vector will grow as needed.
#[cfg(feature = "alloc")]
impl IoWrite for crate::data::Vec<u8> {
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
