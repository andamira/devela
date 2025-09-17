// devela::text::fmt::buf
//
//! Defines [`FmtWriter`] and [`format_buf!`].
//

use crate::{Compare, FmtResult, FmtWrite, Slice, Str, is};
crate::_use! {compat::from_utf8}

#[doc = crate::_TAG_FMT!()]
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
/// let mut buf = [0u8; 12];
/// let s = format_buf![&mut buf, "Test: {} {}", "foo", 4];
/// assert_eq!(Ok("Test: foo 4"), s);
///
/// // use the `?` arm to unwrap the result even if it's truncated
/// let s = format_buf![? &mut buf, "Test: {} {}", "foo", 400_000];
/// assert_eq!("Test: foo 40", s);
/// ```
/// # Features
/// Makes use of the `unsafe_str` feature if enabled.
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! format_buf {
    ($buf:expr, $($arg:tt)*) => {
        $crate::Fmt::format_buf($buf, $crate::format_args![$($arg)*])
    };
    (? $buf:expr, $($arg:tt)*) => {
        $crate::unwrap![ok_err $crate::Fmt::format_buf($buf, $crate::format_args![$($arg)*])]
    };
}
#[doc(inline)]
pub use format_buf;

/// A specialized formatter that works with a fixed byte buffer and has truncation detection.
///
/// It should be faster at runtime than the default formatter.
#[derive(Debug)]
pub struct FmtWriter<'a> {
    buf: &'a mut [u8],
    /// The number of bytes actually written.
    len: usize,
    /// Set to true if any call to write_str did not write the complete input.
    truncated: bool,
}

#[rustfmt::skip]
impl<'a> FmtWriter<'a> {
    /// Creates a new writer for the given buffer.
    pub const fn new(buf: &'a mut [u8]) -> Self { FmtWriter { buf, len: 0, truncated: false } }

    #[inline(always)]
    /// Returns true if truncation occurred.
    pub const fn is_truncated(&self) -> bool { self.truncated }

    #[inline(always)]
    /// Returns the number of bytes written.
    pub const fn written_len(&self) -> usize { self.len }

    /// Writes a string slice to the buffer.
    ///
    /// This is a `const` function that can be used in compile-time evaluation.
    /// It copies as much of the string as will fit into the remaining buffer space.
    ///
    /// # Example
    /// ```
    /// # use devela::FmtWriter;
    /// const fn create_message(buf: &mut [u8]) -> &str {
    ///     let mut writer = FmtWriter::new(buf);
    ///     writer.write_str_truncate("Hello");
    ///     writer.write_str_truncate(" world");
    ///     writer.as_str_const()
    /// }
    /// ```
    pub const fn write_str_truncate(&mut self, s: &str) -> FmtResult<()> {
        let available = self.buf.len().saturating_sub(self.len);
        let s_bytes = s.as_bytes();
        let n = Compare(s_bytes.len()).min(available);
        if n > 0 {
            Slice::range_mut(self.buf, self.len, self.len + n)
                .copy_from_slice(Slice::range_to(&s_bytes, n));
        }
        is![n < s_bytes.len(); self.truncated = true];
        self.len += n;
        Ok(())
    }

    #[must_use]
    /// Returns the written content as a valid UTF‑8 string.
    ///
    /// If the final write ended in the middle of a multi‑byte codepoint,
    /// only the valid prefix is returned.
    ///
    /// # Features
    /// Makes use of the `dep_simd_utf8` feature to accelerate validation,
    /// and of the `unsafe_str` feature to avoid double validation.
    pub fn as_str(self) -> &'a str {
        match from_utf8(&self.buf[..self.len]) { // == &(*self.buf).index(..self.len)
            Ok(valid_str) => valid_str,
            Err(e) => {
                let valid_len = e.valid_up_to();
                let valid_range = &self.buf[..valid_len];
                #[cfg(any(feature = "safe_text", not(feature = "unsafe_str")))]
                { from_utf8(valid_range).unwrap() }
                #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
                // SAFETY: we only convert the confirmed valid utf-8 length
                { unsafe { Str::from_utf8_unchecked(valid_range) } }
            }
        }
    }

    #[must_use]
    /// Compile-time friendly version of [`as_str`][Self::str].
    ///
    /// If the final write ended in the middle of a multi‑byte codepoint,
    /// only the valid prefix is returned.
    ///
    /// # Features
    /// Makes use of the `unsafe_str` feature to avoid double validation.
    pub const fn as_str_const(self) -> &'a str {
        match Str::from_utf8(Slice::range_to(self.buf, self.len)) {
            Ok(valid_str) => valid_str,
            Err(e) => {
                let valid_len = e.valid_up_to;
                let valid_range = Slice::range_to(self.buf, valid_len);
                #[cfg(any(feature = "safe_text", not(feature = "unsafe_str")))]
                { crate::unwrap![ok Str::from_utf8(valid_range)] }
                #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
                // SAFETY: we only convert the confirmed valid utf-8 length
                { unsafe { Str::from_utf8_unchecked(valid_range) } }
            }
        }
    }
}

impl FmtWrite for FmtWriter<'_> {
    #[inline(always)]
    fn write_str(&mut self, s: &str) -> FmtResult<()> {
        self.write_str_truncate(s)
    }
}
