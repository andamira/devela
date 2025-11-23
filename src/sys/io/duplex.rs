// devela::sys::io::duplex
//
//! Defines [`IoDuplex`].
//

use crate::{IoRead, IoResult, IoWrite, is};

impl<T: IoRead + IoWrite> IoDuplex for T {}

/// A duplex byte I/O primitive.
///
/// This trait is blanket-implemented for any type that implements both
/// [`IoRead`] and [`IoWrite`]. Duplex capability does not imply simultaneous
/// readiness for reading and writing.
///
/// It provides fundamental duplex semantics without imposing buffering,
/// allocation, or higher-level protocols.
///
/// Convenience methods offer safe, low-overhead patterns for partial I/O,
/// full-buffer writes, and direct stream-to-stream copying.
pub trait IoDuplex: IoRead + IoWrite {
    /// Reads until at least `min` bytes are read into `buf`, unless EOF occurs first.
    fn read_at_least(&mut self, buf: &mut [u8], min: usize) -> IoResult<usize> {
        let mut filled = 0;
        while filled < min {
            let n = self.read(&mut buf[filled..])?;
            is![n == 0; break];
            filled += n;
        }
        Ok(filled)
    }

    /// Copies all available data from this duplex into the given writer.
    fn copy_to<W: IoWrite>(&mut self, out: &mut W) -> IoResult<u64> {
        const BUF: usize = 8 * 1024;
        let mut tmp = [0u8; BUF];
        let mut total = 0u64;
        loop {
            let n = self.read(&mut tmp)?;
            is![n == 0; break];
            out.write_all(&tmp[..n])?;
            total += n as u64;
        }
        Ok(total)
    }

    /// Reads bytes into `out` until `delim` is found or no more space remains.
    ///
    /// The delimiter is included in the output when encountered. Stops early on EOF.
    /// This method performs no allocation.
    fn read_until_byte(&mut self, delim: u8, out: &mut [u8]) -> IoResult<usize> {
        let mut count = 0;
        for slot in out.iter_mut() {
            let mut b = [0];
            let n = self.read(&mut b)?;
            is![n == 0; break];
            *slot = b[0];
            count += 1;
            is![b[0] == delim; break];
        }
        Ok(count)
    }
}
