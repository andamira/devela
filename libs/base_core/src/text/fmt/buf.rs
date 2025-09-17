// devela_base_core::text::fmt::buf
//
//! Defines [`FmtWriter`] and [`format_buf!`].
//
// TOC
// - macro format_buf
// - struct FmtWriter
// - doc CONSTs
// - impl methods
// - impl FmtWrite

use crate::{CONST, Compare, FmtArguments, FmtResult, FmtWrite, Slice, Str, is};
crate::_use! {compat::from_utf8}

#[doc = crate::_TAG_FMT!()]
/// Returns a formatted [`str`] slice backed by a buffer, non-allocating.
///
/// Underneath it calls [`FmtWriter::format`] and [`format_args!`][crate::format_args].
///
/// # Example
/// ```
/// # use devela_base_core::format_buf;
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
        $crate::FmtWriter::format($buf, $crate::format_args![$($arg)*])
    };
    (? $buf:expr, $($arg:tt)*) => {
        $crate::unwrap![ok_err $crate::FmtWriter::format($buf, $crate::format_args![$($arg)*])]
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

CONST! {
    _DOC_AS_INTO_STR = "Returns the written content as a valid UTF‑8 string.\n\n
If the final write ended in the middle of a multi‑byte codepoint only the valid prefix is returned.
# Features
Makes use of the `dep_simd_utf8` feature to accelerate validation,
and of the `unsafe_str` feature to avoid double validation.";
    _DOC_AS_INTO_STR_CONST = "Returns the written content as a valid UTF‑8 string.\n\n
If the final write ended in the middle of a multi‑byte codepoint only the valid prefix is returned.
Compile-time friendly version using basic validation.
# Features
Makes use of the `unsafe_str` feature to avoid double validation.";
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

    /// Writes formatted output into the given byte buffer.
    ///
    /// Returns:
    /// - `Ok(&str)` if all the formatted data fits into `buf`.
    /// - `Err(&str)` containing the valid partial result if truncation occurred.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::FmtWriter;
    /// let mut buf = [0u8; 32]; // Big enough to fit everything
    /// let s = FmtWriter::format(&mut buf, format_args!["Test: {} {}", "foo", 42]);
    /// assert_eq!(Ok("Test: foo 42"), s);
    ///
    /// let mut buf = [0u8; 9]; // Can't fit everything
    /// let s = FmtWriter::format(&mut buf, format_args!["Test: {} {}", "foo", 42]);
    /// assert_eq!(Err("Test: foo"), s);
    /// ```
    pub fn format(buf: &'a mut [u8], args: FmtArguments) -> Result<&'a str, &'a str> {
        let mut w = Self::new(buf);
        let _ = ::core::fmt::write(&mut w, args); // MAYBE re-export as fmt_write() ?
        is![w.is_truncated(); Err(w.into_str()); Ok(w.into_str())]
    }

    /// Writes a string slice to the buffer.
    ///
    /// It copies as much of the string as will fit into the remaining buffer space.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::FmtWriter;
    /// const fn create_message(buf: &mut [u8]) -> &str {
    ///     let mut writer = FmtWriter::new(buf);
    ///     writer.write_str_truncate("Hello");
    ///     writer.write_str_truncate(" world");
    ///     writer.into_str_const()
    /// }
    /// ```
    pub const fn write_str_truncate(&mut self, s: &str) {
        let available = self.buf.len().saturating_sub(self.len);
        let s_bytes = s.as_bytes();
        let n = Compare(s_bytes.len()).min(available);
        if n > 0 {
            Slice::range_mut(self.buf, self.len, self.len + n)
                .copy_from_slice(Slice::range_to(&s_bytes, n));
        }
        is![n < s_bytes.len(); self.truncated = true];
        self.len += n;
    }
    
    /// Writes a string slice, returning the actually written part.
    ///
    /// Returns `Err` if truncation occurred, with the truncated result.
    pub const fn write_str_truncate_checked(&mut self, s: &str) -> Result<&str, &str> {
        self.write_str_truncate(s);
        let written = self.as_str_const();
        is![self.is_truncated(); Err(written); Ok(written)]
    }
    /// Writes a string slice, returning the length of the actually written part.
    pub const fn write_str_truncate_checked_len(&mut self, s: &str) -> Result<usize, usize> {
        self.write_str_truncate(s);
        is![self.is_truncated(); Err(self.len); Ok(self.len)]
    }

    #[must_use]
    #[doc = _DOC_AS_INTO_STR!()]
    pub fn as_str(&self) -> &str {
        match from_utf8(&self.buf[..self.len]) { // == &(*self.buf).index(..self.len)
            Ok(valid_str) => valid_str,
            Err(e) => Self::get_str_from_slice(self.buf, e.valid_up_to()),
        }
    }
    #[must_use]
    #[doc = _DOC_AS_INTO_STR!()]
    pub fn into_str(self) -> &'a str {
        match from_utf8(&self.buf[..self.len]) {
            Ok(valid_str) => valid_str,
            Err(e) => Self::get_str_from_slice(self.buf, e.valid_up_to()),
        }
    }

    #[must_use]
    #[doc = _DOC_AS_INTO_STR_CONST!()]
    pub const fn as_str_const(&'a self) -> &'a str {
        match Str::from_utf8(Slice::range_to(self.buf, self.len)) {
            Ok(valid_str) => valid_str,
            Err(e) => Self::get_str_from_slice_const(self.buf, e.valid_up_to),
        }
    }
    #[must_use]
    #[doc = _DOC_AS_INTO_STR_CONST!()]
    pub const fn into_str_const(self) -> &'a str {
        match Str::from_utf8(Slice::range_to(self.buf, self.len)) {
            Ok(valid_str) => valid_str,
            Err(e) => Self::get_str_from_slice_const(self.buf, e.valid_up_to)
        }
    }

    /* helper methods */

    #[inline(always)]
    fn get_str_from_slice(slice: &[u8], valid_len: usize) -> &str {
        let valid_range = &slice[..valid_len]; // could be faster in debug builds (non-const)
        #[cfg(any(feature = "safe_text", not(feature = "unsafe_str")))]
        { from_utf8(valid_range).unwrap() } // could use dep_simdutf8 (non-const)
        #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
        // SAFETY: we only convert the confirmed valid utf-8 length
        { unsafe { Str::from_utf8_unchecked(valid_range) } }
    }

    #[inline(always)]
    const fn get_str_from_slice_const(slice: &[u8], valid_len: usize) -> &str {
        let valid_range = Slice::range_to(slice, valid_len);
        #[cfg(any(feature = "safe_text", not(feature = "unsafe_str")))]
        { crate::unwrap![ok Str::from_utf8(valid_range)] }
        #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
        // SAFETY: we only convert the confirmed valid utf-8 length
        { unsafe { Str::from_utf8_unchecked(valid_range) } }
    }
}

impl FmtWrite for FmtWriter<'_> {
    #[inline(always)]
    fn write_str(&mut self, s: &str) -> FmtResult<()> {
        self.write_str_truncate(s);
        Ok(())
    }
}
