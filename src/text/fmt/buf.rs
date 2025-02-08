// devela::text::fmt::buf
//
//! Non-allocating formatting backed by a buffer.
//

use crate::{iif, FmtResult, FmtWrite, _core::cmp::min};

#[cfg(not(feature = "dep_simdutf8"))]
use ::core::str::from_utf8;
#[cfg(feature = "dep_simdutf8")]
use ::simdutf8::compat::from_utf8;

/// Returns a formatted [`str`] slice backed by a buffer, non-allocating.
///
/// Underneath it calls [`Fmt::format_buf`][crate::Fmt::format_buf] and [`format_args!`].
///
/// See also the allocating [`format!`] macro.
///
/// [`format!`]: crate::format
///
/// # Example
/// ```
/// # use devela::format_buf;
/// let mut buf = [0u8; 64];
/// let s = format_buf![&mut buf, "Test: {} {}", "foo", 42];
/// assert_eq!(Ok("Test: foo 42"), s);
/// ```
/// # Features
/// Makes use of the `unsafe_str` feature if enabled.
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! format_buf {
    ($buf:expr, $($arg:tt)*) => {
        $crate::Fmt::format_buf($buf, $crate::format_args![$($arg)*])
    };
}
#[doc(inline)]
pub use format_buf;

/// A helper type that writes formatted text into a fixed byte buffer.
#[derive(Debug)]
pub(super) struct WriteTo<'a> {
    buf: &'a mut [u8],
    /// The number of bytes actually written.
    len: usize,
    /// Set to true if any call to write_str did not write the complete input.
    pub(super) truncated: bool,
}
impl<'a> WriteTo<'a> {
    pub(super) const fn new(buf: &'a mut [u8]) -> Self {
        WriteTo { buf, len: 0, truncated: false }
    }

    /// Returns the written bytes as a valid UTF‑8 string.
    ///
    /// If the final write ended in the middle of a multi‑byte codepoint,
    /// only the valid prefix is returned.
    pub(super) fn as_str(self) -> &'a str {
        match from_utf8(&self.buf[..self.len]) {
            Ok(valid_str) => valid_str,
            Err(e) => {
                let valid_len = e.valid_up_to();
                #[cfg(any(feature = "safe_text", not(feature = "unsafe_str")))]
                {
                    from_utf8(&self.buf[..valid_len]).unwrap()
                }
                #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
                {
                    // SAFETY: we only convert the confirmed valid utf-8 length
                    unsafe { ::core::str::from_utf8_unchecked(&self.buf[..valid_len]) }
                }
            }
        }
    }
}
impl FmtWrite for WriteTo<'_> {
    fn write_str(&mut self, s: &str) -> FmtResult<()> {
        let available = self.buf.len().saturating_sub(self.len);
        let s_bytes = s.as_bytes();
        let n = min(s_bytes.len(), available);
        iif![n > 0; self.buf[self.len..self.len + n].copy_from_slice(&s_bytes[..n])];
        iif![n < s_bytes.len(); self.truncated = true];
        self.len += n;
        Ok(())
    }
}
