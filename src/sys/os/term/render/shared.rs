// devela::sys::os::term::render::shared

use crate::TermRenderer;

/// # Shared byte-frame access
impl<B: AsRef<[u8]>> TermRenderer<B> {
    /// Returns the byte-frame capacity.
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.buf.as_ref().len()
    }

    /// Returns the active buffered bytes.
    #[must_use]
    pub fn buffered(&self) -> &[u8] {
        &self.buf.as_ref()[..self.len]
    }
    /// Returns the active buffered byte length.
    #[must_use]
    pub const fn buffered_len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the active frame is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns `true` if the active frame fills the available byte storage.
    #[must_use]
    pub fn is_full(&self) -> bool {
        self.len == self.capacity()
    }

    /// Replaces the byte-frame storage and copies the active frame if it fits.
    ///
    /// Returns `(self, buf)` when the new storage is smaller than the active frame,
    /// preserving both values.
    pub fn try_replace_buf_copy<C>(self, mut buf: C) -> Result<TermRenderer<C>, (Self, C)>
    where
        C: AsRef<[u8]> + AsMut<[u8]>,
    {
        if buf.as_ref().len() < self.len {
            return Err((self, buf));
        }
        let Self {
            buf: old_buf,
            len,
            size,
            bytes_written,
            frames_presented,
        } = self;
        buf.as_mut()[..len].copy_from_slice(&old_buf.as_ref()[..len]);
        Ok(TermRenderer { buf, len, size, bytes_written, frames_presented })
    }
}
