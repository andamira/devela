// devela::text::fmt::buf
//
//! Non-allocating formatting backed by a buffer.
//

use crate::{fmt_write, iif, FmtArguments, FmtError, FmtResult, TextWrite, _core::cmp::min};
#[allow(unused_imports, reason = "±unsafe")]
use ::core::str::{from_utf8, from_utf8_unchecked};

/// Returns a formatted [`str`] slice backed by a buffer, non-allocating.
///
/// It calls the [`format_buf_args`][format_buf_args()] function with the
/// [`format_args`] macro.
///
/// # Example
/// ```
/// # use devela::format_buf;
/// let mut buf = [0u8; 64];
/// let s = format_buf![&mut buf, "Test: {} {}", "foo", 42];
/// assert_eq!(Ok("Test: foo 42"), s);
/// ```
///
/// # Features
/// Makes use of the `unsafe_str` feature if enabled.
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! format_buf {
    ($buf:expr, $($args:tt)*) => { $crate::format_buf_args($buf, format_args![$($args)*]) };
}
#[doc(inline)]
pub use format_buf;

/// Returns a formatted [`str`] slice backed by a buffer, non-allocating.
///
/// See also the slightly more convenient to use [`format_buf!`][crate::format_buf!] macro.
///
/// # Example
/// ```
/// # use devela::text::format_buf_args;
/// let mut buf = [0u8; 64];
/// let s = format_buf_args(&mut buf, format_args!["Test: {} {}", "foo", 42]);
/// assert_eq!(Ok("Test: foo 42"), s);
/// ```
///
/// # Features
/// Makes use of the `unsafe_str` feature if enabled.
pub fn format_buf_args<'a>(buf: &'a mut [u8], arg: FmtArguments) -> Result<&'a str, FmtError> {
    let mut w = WriteTo::new(buf);
    fmt_write(&mut w, arg)?;
    w.as_str()
}

#[doc = crate::doc_private!()]
#[derive(Debug)]
struct WriteTo<'a> {
    buf: &'a mut [u8],
    len: usize,
}
#[rustfmt::skip]
impl<'a> WriteTo<'a> {
    fn new(buf: &'a mut [u8]) -> Self { WriteTo { buf, len: 0 } }
    fn as_str(self) -> Result<&'a str, FmtError> {
        if self.len <= self.buf.len() {
            #[cfg(any(feature = "safe_text", not(feature = "unsafe_str")))]
            { from_utf8(&self.buf[..self.len]).map_err(|_| FmtError) }
            #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
            // SAFETY: the buffer is always filled from a previous &str
            { Ok(unsafe { from_utf8_unchecked(&self.buf[..self.len]) }) }
        } else {
            Err(FmtError)
        }
    }
}
impl TextWrite for WriteTo<'_> {
    fn write_str(&mut self, s: &str) -> FmtResult<()> {
        // IMPROVE?
        let rem = &mut self.buf[self.len..];
        let raw_s = s.as_bytes();
        let num = min(raw_s.len(), rem.len());

        rem[..num].copy_from_slice(&raw_s[..num]);
        self.len += num;

        iif![num < raw_s.len(); Err(FmtError); Ok(())]
    }
}
