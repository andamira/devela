// devela::sys::os::term::render::owned

use crate::{TermRenderer, Vec, is, vec_};

impl TermRenderer<Vec<u8>> {
    /// Creates an allocated renderer with `bytes` initialized storage.
    ///
    /// This uses the vector length as renderer capacity. It does not rely on
    /// spare vector capacity.
    pub fn with_buf_len(cols: u16, rows: u16, bytes: usize) -> Self {
        let buf = vec_![0; bytes];
        Self::from_buf(buf, cols, rows)
    }
}
impl<B> TermRenderer<B> {
    /// Replaces the byte-frame storage with an initialized vector and clears the frame.
    pub fn replace_with_vec_len(self, bytes: usize) -> TermRenderer<Vec<u8>> {
        let buf = vec_![0; bytes];
        self.replace_buf(buf)
    }
}
impl<B: AsRef<[u8]>> TermRenderer<B> {
    /// Replaces the byte-frame storage with an initialized vector and copies the active frame.
    ///
    /// Returns `self` if `bytes` is smaller than the current active frame.
    pub fn try_replace_with_vec_len_copy(
        self,
        bytes: usize,
    ) -> Result<TermRenderer<Vec<u8>>, Self> {
        is! { bytes < self.len, return Err(self) }
        let mut buf = vec_![0; bytes];
        let Self {
            buf: old_buf,
            len,
            size,
            bytes_written,
            frames_presented,
        } = self;
        buf[..len].copy_from_slice(&old_buf.as_ref()[..len]);
        Ok(TermRenderer { buf, len, size, bytes_written, frames_presented })
    }
}
