// devela::sys::io::reimplement_no_std::fns
//
//! Defines [`io_copy`], [`IoEmpty`], [`IoRepeat`].
//
// TODO
// - https://doc.rust-lang.org/std/io/struct.IoSlice.html
// - https://doc.rust-lang.org/std/io/struct.IoSliceMut.html

use crate::{impl_trait, sf, IoErrorKind, IoRead, IoResult, IoSeek, IoSeekFrom, IoWrite};
#[cfg(feature = "alloc")]
use crate::{IoError, Vec};

/// Copies the entire contents of a reader into a writer.
///
/// # Features
/// Makes use of the `unsafe_array` feature to employ [`MaybeUninit`].
///
/// See <https://doc.rust-lang.org/std/io/fn.copy.html>.
pub fn io_copy<R, W, const S: usize>(reader: &mut R, writer: &mut W) -> IoResult<u64>
where
    R: ?Sized + IoRead,
    W: ?Sized + IoWrite,
{
    #[cfg(any(feature = "safe_io", not(feature = "unsafe_array")))]
    let mut buf = [0u8; S];
    #[cfg(all(not(feature = "safe_io"), feature = "unsafe_array"))]
    let mut buf = crate::MaybeUninit::<[u8; S]>::uninit();
    let mut written = 0;

    loop {
        let len = match reader.read({
            #[cfg(any(feature = "safe_io", not(feature = "unsafe_array")))]
            sf! { &mut buf }
            #[cfg(all(not(feature = "safe_io"), feature = "unsafe_array"))]
            // SAFETY: The read method will initialize the portion of the buffer it writes to.
            sf! { unsafe { &mut *buf.as_mut_ptr() } }
        }) {
            Ok(0) => return Ok(written),
            Ok(len) => len,
            Err(ref e) if e.kind() == IoErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        };

        #[cfg(any(feature = "safe_io", not(feature = "unsafe_array")))]
        writer.write_all(&buf[..len])?;
        #[cfg(all(not(feature = "safe_io"), feature = "unsafe_array"))]
        // SAFETY: `buf` has been initialized up to `len` bytes by the `read` operation.
        writer.write_all(unsafe { &buf.assume_init_ref()[..len] })?;

        written += len as u64;
    }
}

/// `Empty` ignores any data written via [`IoWrite`], and will always be empty
/// (returning zero bytes) when read via [`IoRead`].
///
/// See <https://doc.rust-lang.org/std/io/struct.Empty.html>.
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Default)]
pub struct IoEmpty;

sf! {
    impl IoRead for IoEmpty {
        fn read(&mut self, _buf: &mut [u8]) -> IoResult<usize> { Ok(0) }
    }
    impl IoWrite for IoEmpty {
        fn write(&mut self, buf: &[u8]) -> IoResult<usize> { Ok(buf.len()) }
        fn flush(&mut self) -> IoResult<()> { Ok(()) }
        // fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> IoResult<usize> {
        //     let total_len = bufs.iter().map(|b| b.len()).sum();
        //     Ok(total_len)
        // }
    }
    impl IoWrite for &IoEmpty {
        fn write(&mut self, buf: &[u8]) -> IoResult<usize> { Ok(buf.len()) }
        fn flush(&mut self) -> IoResult<()> { Ok(()) }
        // fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> IoResult<usize> {
        //     let total_len = bufs.iter().map(|b| b.len()).sum();
        //     Ok(total_len)
        // }
    }
    impl IoSeek for IoEmpty {
        fn seek(&mut self, _pos: IoSeekFrom) -> IoResult<u64> { Ok(0) }
    }
}

/// A reader which yields one byte over and over and over and over and over and...
///
/// See <https://doc.rust-lang.org/std/io/struct.Empty.html>.
pub struct IoRepeat {
    byte: u8,
}
impl_trait![fmt::Debug for IoRepeat |self, f| f.debug_struct("IoRepeat").finish_non_exhaustive()];

impl IoRead for IoRepeat {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        for slot in &mut *buf {
            *slot = self.byte;
        }
        Ok(buf.len())
    }
    /// This function is not supported by `IoRepeat`, because there's no end of its data
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
    fn read_to_end(&mut self, _: &mut Vec<u8>) -> IoResult<usize> {
        Err(IoError::from(IoErrorKind::OutOfMemory))
    }
    // /// This function is not supported by `IoRepeat`, because there's no end of its data
    // #[cfg(feature = "alloc")]
    // #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
    // fn read_to_string(&mut self, _: &mut String) -> IoResult<usize> {
    //     Err(IoError::from(IoErrorKind::OutOfMemory))
    // }
    // fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> IoResult<usize> {
    //     let mut nwritten = 0;
    //     for buf in bufs {
    //         nwritten += self.read(buf)?;
    //     }
    //     Ok(nwritten)
    // }
    // fn is_read_vectored(&self) -> bool { true }
}
