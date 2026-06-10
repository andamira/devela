// devela::sys::os::term::render::core_io
//
// TOC
// - impl core state
// - impl I/O
// - impl traits

use crate::{TermRenderer, TermSize};
use core::fmt;

/* helpers */

const fn term_size(cols: u16, rows: u16) -> TermSize {
    TermSize { cols, rows, x: cols, y: rows }
}

#[crate::macro_apply(crate::_std_not_linux_syscall)]
pub(super) fn present_bytes(bytes: &[u8]) -> crate::IoResult<()> {
    use crate::IoWrite;
    let mut out = crate::Io::stdout();
    out.write_all(bytes)?;
    out.flush()
}
#[crate::macro_apply(crate::_linux_syscall)]
pub(super) fn present_bytes(bytes: &[u8]) -> crate::IoResult<()> {
    crate::Linux::print_bytes(bytes).map_err(crate::LinuxError::to_io)
}

/* impls */

/// # Core state
impl<B> TermRenderer<B> {
    /// Creates a renderer from byte-frame storage and terminal dimensions.
    ///
    /// The active frame starts empty.
    /// The renderer uses `buf.as_ref().len()` as writable capacity.
    pub const fn from_buf(buf: B, cols: u16, rows: u16) -> Self {
        Self {
            buf,
            len: 0,
            size: term_size(cols, rows),
            bytes_written: 0,
            frames_presented: 0,
        }
    }
    /// Consumes the renderer and returns its byte-frame storage.
    pub fn into_buf(self) -> B {
        self.buf
    }

    /// Returns the byte-frame storage.
    #[must_use]
    pub const fn buf(&self) -> &B {
        &self.buf
    }
    /// Returns the byte-frame storage mutably.
    ///
    /// Mutating the active prefix manually can desynchronize semantic contents,
    /// but never the renderer's active length.
    #[must_use]
    pub const fn buf_mut(&mut self) -> &mut B {
        &mut self.buf
    }

    /// Replaces the byte-frame storage and clears the active frame.
    ///
    /// Renderer metrics and terminal size are preserved. Active bytes are not copied.
    pub fn replace_buf<C>(self, buf: C) -> TermRenderer<C> {
        TermRenderer {
            buf,
            len: 0,
            size: self.size,
            bytes_written: self.bytes_written,
            frames_presented: self.frames_presented,
        }
    }

    /* terminal extent */

    /// Returns the terminal size known by the renderer.
    pub const fn size(&self) -> TermSize {
        self.size
    }
    /// Sets the terminal size known by the renderer.
    pub const fn set_size(&mut self, cols: u16, rows: u16) {
        self.size = term_size(cols, rows);
    }

    /// Returns the terminal columns known by the renderer.
    #[must_use]
    pub const fn cols(&self) -> u16 {
        self.size.cols
    }
    /// Returns the terminal rows known by the renderer.
    #[must_use]
    pub const fn rows(&self) -> u16 {
        self.size.rows
    }

    /// Returns the total bytes successfully flushed.
    #[must_use]
    pub const fn bytes_written(&self) -> u64 {
        self.bytes_written
    }
    /// Returns the total frames successfully presented.
    #[must_use]
    pub const fn frames_presented(&self) -> u64 {
        self.frames_presented
    }
}

/// # I/O
#[crate::macro_apply(crate::_std_or_linux_syscall)]
impl<B: AsRef<[u8]>> TermRenderer<B> {
    /// Presents the active byte frame to the terminal.
    ///
    /// Writes the active buffered bytes, clears the active frame,
    /// and updates byte/frame metrics.
    ///
    /// This is the only terminal I/O method on the renderer.
    /// It does not append reset, cursor, or screen-control sequences.
    /// Add those explicitly before calling this method.
    pub fn present(&mut self) -> crate::IoResult<()> {
        if self.len != 0 {
            let len = self.len;
            present_bytes(self.buffered())?;
            self.bytes_written = self.bytes_written.saturating_add(len as u64);
            self.len = 0;
            self.frames_presented = self.frames_presented.saturating_add(1);
        }
        Ok(())
    }
}

/* impl traits */

impl<B: AsRef<[u8]> + AsMut<[u8]>> fmt::Write for TermRenderer<B> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.try_push_str(s).map_err(|_| fmt::Error)?;
        Ok(())
    }
}
impl<B: AsRef<[u8]>> fmt::Debug for TermRenderer<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TermRenderer")
            .field("size", &self.size)
            .field("buffered_len", &self.len)
            .field("capacity", &self.capacity())
            .field("bytes_written", &self.bytes_written)
            .field("frames_presented", &self.frames_presented)
            .finish_non_exhaustive()
    }
}
