// devela::text::fmt::misc
//
//! Miscellaneous formatting functionality.
//
// DOCS
// - https://doc.rust-lang.org/core/fmt/trait.Write.html
// - https://doc.rust-lang.org/nightly/core/macro.format_args.html
// - https://doc.rust-lang.org/nightly/core/fmt/struct.Arguments.html
// - https://stackoverflow.com/questions/50200268/how-can-i-use-the-format-macro-in-a-no-std-environment

#[allow(unused_imports)]
use core::{
    cmp::min,
    fmt,
    str::{from_utf8, from_utf8_unchecked},
};

/// Returns a formatted [`str`] slice backed by a buffer, `no_std` compatible.
///
/// See also the slightly more convenient to use [`format_buf!`][crate::format_buf!] macro.
///
/// # Examples
/// ```
/// use devela::text::format_buf_args;
///
/// let mut buf = [0u8; 64];
/// let s = format_buf_args(&mut buf, format_args!["Test: {} {}", "foo", 42]);
///
/// assert_eq!(Ok("Test: foo 42"), s);
/// ```
pub fn format_buf_args<'a>(buf: &'a mut [u8], arg: fmt::Arguments) -> Result<&'a str, fmt::Error> {
    let mut w = WriteTo::new(buf);
    fmt::write(&mut w, arg)?;
    w.as_str().ok_or(fmt::Error)
}

/// Returns a formatted [`str`] slice backed by a buffer, `no_std` compatible.
///
/// It calls the [`format_buf_args`][format_buf_args()] function with the
/// [`format_args`] macro.
///
/// # Examples
/// ```
/// use devela::format_buf;
///
/// let mut buf = [0u8; 64];
/// let s = format_buf![&mut buf, "Test: {} {}", "foo", 42];
///
/// assert_eq!(Ok("Test: foo 42"), s);
/// ```
#[macro_export]
macro_rules! format_buf {
    ($buf:expr, $($args:tt)*) => {
        $crate::text::format_buf_args($buf, format_args![$($args)*])
    };
}
pub use format_buf;

#[derive(Debug)]
struct WriteTo<'a> {
    buf: &'a mut [u8],
    len: usize,
}

impl<'a> fmt::Write for WriteTo<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if self.len > self.buf.len() {
            return Err(fmt::Error);
        }

        let rem = &mut self.buf[self.len..];
        let raw_s = s.as_bytes();
        let num = min(raw_s.len(), rem.len());

        rem[..num].copy_from_slice(&raw_s[..num]);
        self.len += raw_s.len();

        if num < raw_s.len() {
            Err(fmt::Error)
        } else {
            Ok(())
        }
    }
}

impl<'a> WriteTo<'a> {
    fn new(buf: &'a mut [u8]) -> Self {
        WriteTo { buf, len: 0 }
    }

    #[cfg(any(feature = "safe_text", not(feature = "unsafe_str")))]
    fn as_str(self) -> Option<&'a str> {
        if self.len <= self.buf.len() {
            Some(from_utf8(&self.buf[..self.len]).expect("invalid utf-8"))
        } else {
            None
        }
    }
    /// # Features
    /// Makes use of the `unsafe_str` feature if enabled.
    #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
    fn as_str(self) -> Option<&'a str> {
        if self.len <= self.buf.len() {
            // SAFETY: the buffer is always filled from a previous &str
            Some(unsafe { from_utf8_unchecked(&self.buf[..self.len]) })
        } else {
            None
        }
    }
}
