// devela/src/text/str/buf.rs
//
//! Defines [`StrBuf`].
//

use crate::{Char, Str, slice, whilst};
use crate::{FmtArguments, FmtError, FmtResult, FmtWrite};
use crate::{InvalidText, MismatchedCapacity, NotEnoughSpace};

#[doc = crate::_tags!(string)]
/// A borrowed UTF-8 string buffer over caller-provided bytes.
#[doc = crate::_doc_meta!{location("text/str")}]
///
/// It stores a valid UTF-8 prefix inside a mutable byte slice,
/// leaving the remaining bytes as spare capacity.
///
/// This is the borrowed counterpart to fixed-capacity owned strings.
pub struct StrBuf<'a> {
    buf: &'a mut [u8],
    len: usize,
}

// private helpers
#[rustfmt::skip]
impl<'a> StrBuf<'a> {
    /// Copies `bytes[..n]` into the spare tail.
    const fn _push_bytes_prefix(&mut self, bytes: &[u8], n: usize) {
        whilst! { i in 0..n; { self.buf[self.len + i] = bytes[i]; }}
        self.len += n;
    }
    /// Returns `buf[..len]` as `str`.
    const fn _str_from_buf_len(buf: &[u8], len: usize) -> &str {
        cfg_select! { all(feature = "unsafe_str", not(feature = "safe_text")) => {
            // SAFETY: `StrBuf` preserves the invariant that `buf[..len]` is valid UTF-8.
            unsafe { ::core::str::from_utf8_unchecked(slice![&buf, ..len]) }
        } _ => {
            match Str::from_utf8(slice![&buf, ..len]) {
                Ok(s) => s,
                Err(_) => panic!("invalid StrBuf invariant"),
            }
        }}
    }
    /// Returns `buf[..len]` as `mut str`.
    const fn _str_from_buf_len_mut(buf: &mut [u8], len: usize) -> &mut str {
        cfg_select! { all(feature = "unsafe_str", not(feature = "safe_text")) => {
            // SAFETY: `StrBuf` preserves the invariant that `buf[..len]` is valid UTF-8.
            unsafe { ::core::str::from_utf8_unchecked_mut(slice![mut buf, ..len]) }
        } _=> {
            match Str::from_utf8_mut(slice![mut buf, ..len]) {
                Ok(s) => s,
                Err(_) => panic!("invalid StrBuf invariant"),
            }
        }}
    }
}
impl<'a> StrBuf<'a> {
    /* constructors */

    /// Creates an empty buffer over `buf`.
    #[inline(always)]
    pub const fn new(buf: &'a mut [u8]) -> Self {
        Self { buf, len: 0 }
    }
    /// Creates a buffer from the first `len` bytes of `buf`.
    ///
    /// # Errors
    /// Returns [`InvalidText::MismatchedCapacity`] if `len > buf.len()`,
    /// or [`InvalidText::Utf8`] if `buf[..len]` is not valid UTF-8.
    pub const fn from_bytes(buf: &'a mut [u8], len: usize) -> Result<Self, InvalidText> {
        if len > buf.len() {
            let err = MismatchedCapacity::too_small(buf.len(), len);
            Err(InvalidText::from_mismatched_capacity(err))
        } else {
            match Str::from_utf8(slice![&buf, ..len]) {
                Ok(_) => Ok(Self { buf, len }),
                Err(e) => Err(InvalidText::from_invalid_utf8(e)),
            }
        }
    }
    /// Creates a buffer from the first `len` bytes of `buf` without validation.
    ///
    /// # Safety
    /// `len` must be `<= buf.len()` and `buf[..len]` must be valid UTF-8.
    #[cfg(all(feature = "unsafe_str", not(feature = "safe_text")))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_str")))]
    #[inline(always)]
    pub const unsafe fn from_bytes_unchecked(buf: &'a mut [u8], len: usize) -> Self {
        Self { buf, len }
    }

    /* views */

    /// Returns the initialized UTF-8 string.
    #[must_use]
    #[inline(always)]
    pub const fn as_str(&self) -> &str {
        Self::_str_from_buf_len(self.buf, self.len)
    }
    /// Returns the initialized UTF-8 string mutably.
    #[must_use]
    #[inline(always)]
    pub const fn as_mut_str(&mut self) -> &mut str {
        Self::_str_from_buf_len_mut(self.buf, self.len)
    }

    /// Returns the initialized bytes.
    #[must_use]
    #[inline(always)]
    pub const fn as_bytes(&self) -> &[u8] {
        slice![&self.buf, ..self.len]
    }
    /// Returns the initialized bytes mutably.
    ///
    /// # Safety
    /// The caller must preserve valid UTF-8 in `self.as_bytes()`.
    #[cfg(all(feature = "unsafe_str", not(feature = "safe_text")))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_str")))]
    #[inline(always)]
    pub const unsafe fn as_bytes_mut(&mut self) -> &mut [u8] {
        slice![mut &mut self.buf, .. self.len]
    }

    /// Returns the spare byte capacity.
    ///
    /// Writing here does not change the string length. Use
    /// [`set_len_checked`][Self::set_len_checked] afterwards to extend safely.
    #[must_use]
    #[inline(always)]
    pub const fn spare_capacity_mut(&mut self) -> &mut [u8] {
        slice![mut &mut self.buf, self.len, ..]
    }

    /// Consumes the buffer and returns the initialized UTF-8 string.
    #[must_use]
    #[inline(always)]
    pub const fn into_str(self) -> &'a str {
        Self::_str_from_buf_len(self.buf, self.len)
    }

    /* queries */

    /// Returns the initialized length in bytes.
    #[must_use]
    #[inline(always)]
    pub const fn len(&self) -> usize {
        self.len
    }
    /// Returns the total byte capacity.
    #[must_use]
    #[inline(always)]
    pub const fn capacity(&self) -> usize {
        self.buf.len()
    }
    /// Returns the remaining byte capacity.
    #[must_use]
    #[inline(always)]
    pub const fn remaining_capacity(&self) -> usize {
        self.buf.len().saturating_sub(self.len)
    }
    /// Returns `true` if no bytes are initialized.
    #[must_use]
    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }
    /// Returns `true` if no spare capacity remains.
    #[must_use]
    #[inline(always)]
    pub const fn is_full(&self) -> bool {
        self.len == self.buf.len()
    }

    /* setters */

    /// Clears the string.
    #[inline(always)]
    pub const fn clear(&mut self) {
        self.len = 0;
    }

    /// Truncates the string to `new_len` bytes.
    ///
    /// Does nothing if `new_len >= self.len()`.
    ///
    /// # Errors
    /// Returns [`InvalidText::Utf8`] if `new_len` is not a UTF-8 boundary.
    pub const fn truncate(&mut self, new_len: usize) -> Result<(), InvalidText> {
        if new_len >= self.len { Ok(()) } else { self.set_len_checked(new_len) }
    }

    /// Sets the initialized length after validating `buf[..len]`.
    ///
    /// # Errors
    /// Returns [`InvalidText::MismatchedCapacity`] if `len > self.capacity()`,
    /// or [`InvalidText::Utf8`] if `buf[..len]` is not valid UTF-8.
    pub const fn set_len_checked(&mut self, len: usize) -> Result<(), InvalidText> {
        if len > self.buf.len() {
            let err = MismatchedCapacity::too_small(self.buf.len(), len);
            Err(InvalidText::from_mismatched_capacity(err))
        } else {
            match Str::from_utf8(slice![&self.buf, ..len]) {
                Ok(_) => {
                    self.len = len;
                    Ok(())
                }
                Err(e) => Err(InvalidText::from_invalid_utf8(e)),
            }
        }
    }
    /// Sets the initialized length without validation.
    ///
    /// # Safety
    /// `len` must be `<= self.capacity()` and `self.buf[..len]` must be valid UTF-8.
    #[cfg(all(feature = "unsafe_str", not(feature = "safe_text")))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_str")))]
    #[inline(always)]
    pub const unsafe fn set_len_unchecked(&mut self, len: usize) {
        self.len = len;
    }

    /* push */

    /// Appends `ch` if it fits, returning the number of bytes written.
    ///
    /// Returns `0` if there is not enough remaining capacity.
    pub const fn push(&mut self, ch: char) -> usize {
        let need = ch.len_utf8();
        if need > self.remaining_capacity() {
            0
        } else {
            let bytes = Char(ch).to_utf8_bytes();
            self._push_bytes_prefix(&bytes, need);
            need
        }
    }
    /// Appends `ch`, returning its UTF-8 length.
    ///
    /// # Errors
    /// Returns [`NotEnoughSpace`] if the character does not fit.
    pub const fn try_push(&mut self, ch: char) -> Result<usize, NotEnoughSpace> {
        let need = ch.len_utf8();
        if need > self.remaining_capacity() {
            Err(NotEnoughSpace(Some(need)))
        } else {
            Ok(self.push(ch))
        }
    }
    /// Appends the largest UTF-8 prefix of `s` that fits.
    ///
    /// Returns the number of bytes written.
    pub const fn push_str(&mut self, s: &str) -> usize {
        let n = Char(s.as_bytes()).floor_utf8_boundary(self.remaining_capacity());
        self._push_bytes_prefix(s.as_bytes(), n);
        n
    }
    /// Appends the largest UTF-8 prefix of `s` that fits.
    ///
    /// Returns `Ok(written)` if the complete string fit, otherwise
    /// `Err(written)` with the valid prefix length that was written.
    pub const fn try_push_str(&mut self, s: &str) -> Result<usize, usize> {
        let n = self.push_str(s);
        if n == s.len() { Ok(n) } else { Err(n) }
    }
    /// Appends `s` only if it fits completely.
    ///
    /// # Errors
    /// Returns [`NotEnoughSpace`] without modifying the buffer if `s` does not fit.
    pub const fn try_push_str_complete(&mut self, s: &str) -> Result<usize, NotEnoughSpace> {
        let need = s.len();
        if need > self.remaining_capacity() {
            Err(NotEnoughSpace(Some(need)))
        } else {
            self._push_bytes_prefix(s.as_bytes(), need);
            Ok(need)
        }
    }

    /* formatting */

    /// Writes formatted output into `buf`.
    ///
    /// Returns:
    /// - `Ok(&str)` if all formatting writes completed.
    /// - `Err(&str)` with the valid partial result if formatting stopped early.
    ///
    /// Unlike [`FmtWriter`][crate::FmtWriter], this uses [`StrBuf`]'s
    /// `FmtWrite` implementation: every `write_str` chunk is all-or-nothing.
    pub fn format(buf: &'a mut [u8], args: FmtArguments<'_>) -> Result<&'a str, &'a str> {
        let mut w = Self::new(buf);
        let result = ::core::fmt::write(&mut w, args);
        let s = w.into_str();
        if result.is_ok() { Ok(s) } else { Err(s) }
    }
}

impl FmtWrite for StrBuf<'_> {
    fn write_str(&mut self, s: &str) -> FmtResult<()> {
        self.try_push_str_complete(s).map(|_| ()).map_err(|_| FmtError)
    }
}

#[cfg(test)]
mod tests {
    use crate::{FmtWrite, StrBuf};

    #[test]
    fn new_and_queries() {
        let mut bytes = [0u8; 8];
        let s = StrBuf::new(&mut bytes);
        assert_eq!(s.len(), 0);
        assert_eq!(s.capacity(), 8);
        assert_eq!(s.remaining_capacity(), 8);
        assert!(s.is_empty());
        assert!(!s.is_full());
        assert_eq!(s.as_str(), "");
    }
    #[test]
    fn from_bytes() {
        let mut bytes = *b"abc\0\0";
        let s = StrBuf::from_bytes(&mut bytes, 3).unwrap();
        assert_eq!(s.as_str(), "abc");
        assert_eq!(s.len(), 3);
        assert_eq!(s.remaining_capacity(), 2);
    }
    #[test]
    fn from_bytes_rejects_invalid_utf8() {
        let mut bytes = [0xFFu8; 4];
        assert!(StrBuf::from_bytes(&mut bytes, 1).is_err());
    }
    #[test]
    fn push_char() {
        let mut bytes = [0u8; 3];
        let mut s = StrBuf::new(&mut bytes);
        assert_eq!(s.try_push('ñ'), Ok(2));
        assert_eq!(s.as_str(), "ñ");
        assert!(s.try_push('ñ').is_err());
        assert_eq!(s.as_str(), "ñ");
        assert_eq!(s.try_push('a'), Ok(1));
        assert_eq!(s.as_str(), "ña");
        assert!(s.is_full());
    }
    #[test]
    fn push_str_truncates_on_utf8_boundary() {
        let mut bytes = [0u8; 4];
        let mut s = StrBuf::new(&mut bytes);
        assert_eq!(s.push_str("café"), 3);
        assert_eq!(s.as_str(), "caf");
    }
    #[test]
    fn try_push_str_writes_prefix_on_error() {
        let mut bytes = [0u8; 4];
        let mut s = StrBuf::new(&mut bytes);
        assert_eq!(s.try_push_str("café"), Err(3));
        assert_eq!(s.as_str(), "caf");
    }
    #[test]
    fn try_push_str_complete_is_all_or_nothing() {
        let mut bytes = [0u8; 4];
        let mut s = StrBuf::new(&mut bytes);
        assert!(s.try_push_str_complete("café").is_err());
        assert_eq!(s.as_str(), "");
        assert_eq!(s.try_push_str_complete("cafe"), Ok(4));
        assert_eq!(s.as_str(), "cafe");
    }
    #[test]
    fn truncate_checks_utf8_boundary() {
        let mut bytes = [0u8; 8];
        let mut s = StrBuf::new(&mut bytes);
        assert_eq!(s.push_str("a€b"), 5);
        assert!(s.truncate(2).is_err()); // inside `€`
        assert_eq!(s.as_str(), "a€b");
        assert!(s.truncate(4).is_ok());
        assert_eq!(s.as_str(), "a€");
    }
    #[test]
    fn spare_capacity_then_set_len_checked() {
        let mut bytes = [0u8; 8];
        let mut s = StrBuf::new(&mut bytes);
        s.push_str("ab");
        s.spare_capacity_mut()[0] = b'c';
        s.spare_capacity_mut()[1] = b'd';
        assert!(s.set_len_checked(4).is_ok());
        assert_eq!(s.as_str(), "abcd");
    }
    #[test]
    fn fmt_write_is_all_or_nothing_per_chunk() {
        let mut bytes = [0u8; 5];
        let mut s = StrBuf::new(&mut bytes);
        assert!(s.write_str("hello").is_ok());
        assert!(s.write_str("!").is_err());
        assert_eq!(s.as_str(), "hello");
    }
    #[test]
    fn format() {
        let mut bytes = [0u8; 32];
        let s = StrBuf::format(&mut bytes, format_args!("Test: {} {}", "foo", 42));
        assert_eq!(s, Ok("Test: foo 42"));
        let mut bytes = [0u8; 9]; // strict, non-truncating
        let s = StrBuf::format(&mut bytes, format_args!("Test: {} {}", "foo", 42));
        assert_eq!(s, Err(""));
    }
}
