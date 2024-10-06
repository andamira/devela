// devela::sys::io::reimplement_no_std::fns

use crate::{
    mem::MaybeUninit,
    sys::{IoErrorKind, IoRead, IoResult, IoWrite},
};

/// Copies the entire contents of a reader into a writer.
///
/// # Features
/// Makes use of the `unsafe_array` feature to employ [`MaybeUninit`].
pub fn io_copy<R, W, const S: usize>(reader: &mut R, writer: &mut W) -> IoResult<u64>
where
    R: ?Sized + IoRead,
    W: ?Sized + IoWrite,
{
    #[cfg(not(feature = "unsafe_array"))]
    let mut buf = [0u8; S];
    #[cfg(feature = "unsafe_array")]
    let mut buf = MaybeUninit::<[u8; S]>::uninit();
    let mut written = 0;

    loop {
        let len = match reader.read({
            #[cfg(not(feature = "unsafe_array"))]
            {
                &mut buf
            }
            #[cfg(feature = "unsafe_array")]
            // SAFETY: The read method will initialize the portion of the buffer it writes to.
            unsafe {
                &mut *buf.as_mut_ptr()
            }
        }) {
            Ok(0) => return Ok(written),
            Ok(len) => len,
            Err(ref e) if e.kind() == IoErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        };

        #[cfg(not(feature = "unsafe_array"))]
        writer.write_all(&buf[..len])?;
        #[cfg(feature = "unsafe_array")]
        // SAFETY: `buf` has been initialized up to `len` bytes by the `read` operation.
        writer.write_all(unsafe { &buf.assume_init_ref()[..len] })?;

        written += len as u64;
    }
}
