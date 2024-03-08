// devela::text::nonul
//
//! Non-nul `String` backed by an array.
//
// TOC
// - definitions
// - trait impls

use crate::{
    result::unwrap,
    text::{helpers::impl_sized_alias, TextError, TextResult as Result},
};
use core::{fmt, ops::Deref, str::Chars};
use TextError::{InvalidNul, InvalidUtf8, NotEnoughCapacity, NotEnoughElements, OutOfBounds};

#[cfg(feature = "alloc")]
use crate::_deps::alloc::{ffi::CString, string::ToString};

use super::char::*;

/* definitions */

/// The nul character.
const NUL_CHAR: char = '\0';

/// A UTF-8-encoded string, backed by an array with [`u8::MAX`] bytes of capacity.
/// Can't contain nul chars.
///
/// Internally, the first 0 byte in the array indicates the end of the string.
///
/// ## Methods
///
/// - Construct:
///   [`new`][Self::new],
///   [`from_char`][Self::from_char]*(
///     [`7`](Self::from_char7),
///     [`8`](Self::from_char8),
///     [`16`](Self::from_char16),
///     [`24`](Self::from_char24),
///     [`32`](Self::from_char32)
///   )*.
/// - Deconstruct:
///   [`into_array`][Self::into_array],
///   [`as_array`][Self::as_array],
///   [`as_bytes`][Self::as_bytes]
///     *([mut][Self::as_bytes_mut]<sup title="unsafe function">⚠</sup>)*,
///   [`as_str`][Self::as_str]
///     *([mut][Self::as_mut_str]<sup title="unsafe function">⚠</sup>)*,
///   [`chars`][Self::chars],
///   [`to_cstring`][Self::to_cstring](`alloc`).
/// - Query:
///   [`len`][Self::len],
///   [`is_empty`][Self::is_empty],
///   [`is_full`][Self::is_full],
///   [`capacity`][Self::capacity],
///   [`remaining_capacity`][Self::remaining_capacity].
/// - Operations:
///   [`clear`][Self::clear], [`reset`][Self::reset],
///   [`pop`][Self::pop]*([try][Self::try_pop])*,
///   [`push`][Self::push]*([try][Self::try_push])*.
///   [`push_str`][Self::push]*([try][Self::try_push_str])*,
///   [`try_push_str_complete`][Self::try_push_str_complete].
#[must_use]
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct StringNonul<const CAP: usize> {
    arr: [u8; CAP],
}

impl<const CAP: usize> StringNonul<CAP> {
    /// Creates a new empty `StringNonul`.
    ///
    /// # Errors
    /// Returns [`OutOfBounds`] if `CAP` > 255.
    #[inline]
    pub const fn new() -> Result<Self> {
        if CAP <= u8::MAX as usize {
            Ok(Self { arr: [0; CAP] })
        } else {
            Err(OutOfBounds(Some(CAP)))
        }
    }

    /* query */

    /// Returns the total capacity in bytes.
    #[inline] #[must_use] #[rustfmt::skip]
    pub const fn capacity() -> usize { CAP }

    /// Returns the remaining capacity.
    #[inline] #[must_use] #[rustfmt::skip]
    pub fn remaining_capacity(&self) -> usize { CAP - self.len() }

    /// Returns the current length.
    #[inline] #[must_use] #[rustfmt::skip]
    pub fn len(&self) -> usize {
        self.arr.iter().position(|&x| x == 0).unwrap_or(self.arr.len())
    }

    /// Returns `true` if the current length is 0.
    #[inline] #[must_use] #[rustfmt::skip]
    pub fn is_empty(&self) -> bool { self.len() == 0 }

    /// Returns `true` if the current remaining capacity is 0.
    #[inline] #[must_use] #[rustfmt::skip]
    pub fn is_full(&self) -> bool { self.len() == CAP }

    /* deconstruct */

    /// Returns the inner array with the full contents.
    ///
    /// The array contains all the bytes, including those outside the current length.
    #[inline] #[must_use] #[rustfmt::skip]
    pub const fn into_array(self) -> [u8; CAP] { self.arr }

    /// Returns a copy of the inner array with the full contents.
    ///
    /// The array contains all the bytes, including those outside the current length.
    #[inline] #[must_use] #[rustfmt::skip]
    pub const fn as_array(&self) -> [u8; CAP] { self.arr }

    /// Returns a byte slice of the inner string slice.
    ///
    /// # Features
    /// Makes use of the `unsafe_slice` feature if enabled.
    #[inline] #[must_use] #[rustfmt::skip]
    pub fn as_bytes(&self) -> &[u8] {
        #[cfg(any(feature = "safe_text", not(feature = "unsafe_slice")))]
        return self.arr.get(0..self.len()).unwrap();

        #[cfg(all(not(feature = "safe_text"), feature = "unsafe_slice"))]
        // SAFETY: self.len() is always <= arr.len()
        return unsafe { self.arr.get_unchecked(0..self.len()) };
    }

    /// Returns a mutable byte slice of the inner string slice.
    ///
    /// # Safety
    /// The caller must ensure that the content of the slice is valid UTF-8
    /// before the borrow ends and the underlying `str` is used.
    ///
    /// Use of a `str` whose contents are not valid UTF-8 is undefined behavior.
    #[inline]
    #[must_use]
    #[cfg(all(not(feature = "safe_text"), feature = "unsafe_slice"))]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_slice")))]
    pub unsafe fn as_bytes_mut(&mut self) -> &mut [u8] {
        let len = self.len();
        self.arr.get_unchecked_mut(0..len)
    }

    /// Returns the inner string slice.
    /// # Features
    /// Makes use of the `unsafe_slice` feature if enabled.
    #[inline]
    #[must_use]
    pub fn as_str(&self) -> &str {
        #[cfg(any(feature = "safe_text", not(feature = "unsafe_slice")))]
        return core::str::from_utf8(self.arr.get(0..self.len()).unwrap())
            .expect("must be valid utf-8");

        #[cfg(all(not(feature = "safe_text"), feature = "unsafe_slice"))]
        // SAFETY: we ensure to contain only valid UTF-8
        unsafe {
            return core::str::from_utf8_unchecked(self.arr.get_unchecked(0..self.len()));
        }
    }

    /// Returns the mutable inner string slice.
    ///
    /// # Safety
    /// The caller must ensure that the content of the slice is valid UTF-8
    /// and that it doesn't contain any `NUL` characters before the borrow
    /// ends and the underlying `str` is used.
    #[inline]
    #[must_use]
    #[cfg(all(not(feature = "safe_text"), feature = "unsafe_slice"))]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_slice")))]
    pub unsafe fn as_mut_str(&mut self) -> &mut str {
        &mut *(self.as_bytes_mut() as *mut [u8] as *mut str)
    }

    /// Returns an iterator over the `chars` of this grapheme cluster.
    #[inline]
    pub fn chars(&self) -> Chars {
        self.as_str().chars()
    }

    /// Returns a new allocated C-compatible, nul-terminanted string.
    #[inline]
    #[must_use]
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
    pub fn to_cstring(&self) -> CString {
        CString::new(self.to_string()).unwrap()
    }

    /* operations */

    /// Sets the length to 0, by resetting all bytes to 0.
    #[inline] #[rustfmt::skip]
    pub fn clear(&mut self) { self.arr = [0; CAP]; }

    /// Removes the last character and returns it, or `None` if
    /// the string is empty.
    #[inline]
    #[must_use]
    pub fn pop(&mut self) -> Option<char> {
        if self.is_empty() {
            None
        } else {
            Some(self.pop_unchecked())
        }
    }

    /// Tries to remove the last character and return it.
    ///
    /// # Errors
    /// Returns [`NotEnoughElements`] if the string is empty.
    #[inline]
    pub fn try_pop(&mut self) -> Result<char> {
        if self.is_empty() {
            Err(NotEnoughElements(1))
        } else {
            Ok(self.pop_unchecked())
        }
    }

    /// Removes the last character and returns it.
    ///
    /// # Panics
    /// Panics if the string is empty.
    #[inline]
    #[must_use]
    pub fn pop_unchecked(&mut self) -> char {
        let len = self.len();
        let mut idx_last_char = len - 1;
        while idx_last_char > 0 && !self.as_str().is_char_boundary(idx_last_char) {
            idx_last_char -= 1;
        }
        let last_char = self.as_str()[idx_last_char..len].chars().next().unwrap();
        for i in idx_last_char..len {
            self.arr[i] = 0;
        }
        last_char
    }

    /// Appends to the end of the string the given `character`.
    ///
    /// Returns the number of bytes written.
    ///
    /// It will return 0 bytes if the given `character` doesn't fit in
    /// the remaining capacity, or if it is the nul character.
    pub fn push(&mut self, character: char) -> usize {
        let char_len = character.len_utf8();

        if character != NUL_CHAR && self.remaining_capacity() >= char_len {
            let len = self.len();
            let new_len = len + char_len;

            let _ = character.encode_utf8(&mut self.arr[len..new_len]);
            char_len
        } else {
            0
        }
    }

    /// Tries to append to the end of the string the given `character`.
    ///
    /// Returns the number of bytes written.
    ///
    /// Trying to push a nul character does nothing and returns 0 bytes.
    ///
    /// # Errors
    /// Returns [`NotEnoughCapacity`]
    /// if the capacity is not enough to hold the given character.
    pub fn try_push(&mut self, character: char) -> Result<usize> {
        let char_len = character.len_utf8();

        if character == NUL_CHAR {
            Ok(0)
        } else if self.remaining_capacity() >= char_len {
            let len = self.len();
            let new_len = len + char_len;

            let _ = character.encode_utf8(&mut self.arr[len..new_len]);
            Ok(char_len)
        } else {
            Err(NotEnoughCapacity(char_len))
        }
    }

    /// Appends to the end the fitting characters from the given `string` slice.
    ///
    /// Nul characters will be stripped out.
    ///
    /// Returns the number of bytes written, which will be 0
    /// if not even the first non-nul character can fit.
    pub fn push_str(&mut self, string: &str) -> usize {
        let mut rem_cap = self.remaining_capacity();
        let mut bytes_written = 0;

        for character in string.chars() {
            if character != NUL_CHAR {
                let char_len = character.len_utf8();

                if char_len <= rem_cap {
                    self.push(character);
                    rem_cap -= char_len;
                    bytes_written += char_len;
                } else {
                    break;
                }
            }
        }
        bytes_written
    }

    /// Tries to append to the end the fitting characters from the given `string`
    /// slice.
    ///
    /// Nul characters will be stripped out.
    ///
    /// Returns the number of bytes written.
    ///
    /// # Errors
    /// Returns [`NotEnoughCapacity`] if the capacity is not enough
    /// to hold even the first non-nul character.
    pub fn try_push_str(&mut self, string: &str) -> Result<usize> {
        let first_char_len = string
            .chars()
            .find(|&c| c != NUL_CHAR)
            .map(|c| c.len_utf8())
            .unwrap_or(0);
        if self.remaining_capacity() < first_char_len {
            Err(NotEnoughCapacity(first_char_len))
        } else {
            Ok(self.push_str(string))
        }
    }

    /// Tries to append the complete `string` slice to the end.
    ///
    /// Returns the number of bytes written in success.
    ///
    /// Nul characters will not be taken into account.
    ///
    /// # Errors
    /// Returns [`NotEnoughCapacity`] if the slice wont completely fit.
    #[inline]
    pub fn try_push_str_complete(&mut self, string: &str) -> Result<usize> {
        let non_nul_len = string.as_bytes().iter().filter(|x| **x != 0).count();

        if self.remaining_capacity() >= non_nul_len {
            Ok(self.push_str(string))
        } else {
            Err(NotEnoughCapacity(non_nul_len))
        }
    }

    /* from char */

    /// Creates a new `StringNonul` from a `char`.
    ///
    /// If `c` is NUL an empty string will be returned.
    ///
    /// # Errors
    /// Returns [`OutOfBounds`] if `CAP` > 255,
    /// or [`NotEnoughCapacity`] if `!c.is_nul()`
    /// and `CAP` < `c.`[`len_utf8()`][Char32#method.len_utf8].
    ///
    /// Will always succeed if `CAP` >= 4.
    #[inline]
    pub const fn from_char(c: char) -> Result<Self> {
        let mut new = unwrap![ok? Self::new()];

        if c as u32 != 0 {
            let bytes = char_to_utf8_bytes(c);
            let len = char_utf8_4bytes_len(bytes);

            new.arr[0] = bytes[0];
            if len > 1 {
                new.arr[1] = bytes[1];
            }
            if len > 2 {
                new.arr[2] = bytes[2];
            }
            if len > 3 {
                new.arr[3] = bytes[3];
            }
        }
        Ok(new)
    }

    /// Creates a new `StringNonul` from a `Char7`.
    ///
    /// If `c`.[`is_nul()`][Char7#method.is_nul] an empty string will be returned.
    ///
    /// # Errors
    /// Returns [`OutOfBounds`] if `CAP` > 255,
    /// or [`NotEnoughCapacity`] if `!c.is_nul()` and `CAP` < 1.
    ///
    /// Will always succeed if `CAP` >= 1.
    #[inline]
    #[cfg(feature = "text")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "text")))]
    pub const fn from_char7(c: Char7) -> Result<Self> {
        let mut new = unwrap![ok? Self::new()];
        if !c.is_nul() {
            new.arr[0] = c.to_utf8_bytes()[0];
        }
        Ok(new)
    }

    /// Creates a new `StringNonul` from a `Char8`.
    ///
    /// If `c`.[`is_nul()`][Char8#method.is_nul] an empty string will be returned.
    ///
    /// # Errors
    /// Returns [`OutOfBounds`] if `CAP` > 255,
    /// or [`NotEnoughCapacity`] if `!c.is_nul()`
    /// and `CAP` < `c.`[`len_utf8()`][Char8#method.len_utf8].
    ///
    /// Will always succeed if `CAP` >= 2.
    #[inline]
    #[cfg(feature = "text")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "text")))]
    pub const fn from_char8(c: Char8) -> Result<Self> {
        let mut new = unwrap![ok? Self::new()];
        if !c.is_nul() {
            let bytes = c.to_utf8_bytes();
            let len = char_utf8_2bytes_len(bytes);

            new.arr[0] = bytes[0];
            if len > 1 {
                new.arr[1] = bytes[1];
            }
        }
        Ok(new)
    }

    /// Creates a new `StringNonul` from a `Char16`.
    ///
    /// If `c`.[`is_nul()`][Char16#method.is_nul] an empty string will be returned.
    ///
    /// # Errors
    /// Returns [`OutOfBounds`] if `CAP` > 255,
    /// or [`NotEnoughCapacity`] if `!c.is_nul()`
    /// and `CAP` < `c.`[`len_utf8()`][Char16#method.len_utf8].
    ///
    /// Will always succeed if `CAP` >= 3.
    #[inline]
    #[cfg(feature = "text")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "text")))]
    pub const fn from_char16(c: Char16) -> Result<Self> {
        let mut new = unwrap![ok? Self::new()];
        if !c.is_nul() {
            let bytes = c.to_utf8_bytes();
            let len = char_utf8_3bytes_len(bytes);

            new.arr[0] = bytes[0];
            if len > 1 {
                new.arr[1] = bytes[1];
            }
            if len > 2 {
                new.arr[2] = bytes[2];
            }
        }
        Ok(new)
    }

    /// Creates a new `StringNonul` from a `Char24`.
    ///
    /// If `c`.[`is_nul()`][Char24#method.is_nul] an empty string will be returned.
    ///
    /// # Errors
    /// Returns [`OutOfBounds`] if `CAP` > 255,
    /// or [`NotEnoughCapacity`] if `!c.is_nul()`
    /// and `CAP` < `c.`[`len_utf8()`][Char24#method.len_utf8].
    ///
    /// Will always succeed if `CAP` >= 4.
    #[inline]
    #[cfg(feature = "text")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "text")))]
    pub const fn from_char24(c: Char24) -> Result<Self> {
        let mut new = unwrap![ok? Self::new()];
        if !c.is_nul() {
            let bytes = c.to_utf8_bytes();
            let len = char_utf8_4bytes_len(bytes);

            new.arr[0] = bytes[0];
            if len > 1 {
                new.arr[1] = bytes[1];
            }
            if len > 2 {
                new.arr[2] = bytes[2];
            }
            if len > 3 {
                new.arr[3] = bytes[3];
            }
        }
        Ok(new)
    }

    /// Creates a new `StringNonul` from a `Char32`.
    ///
    /// If `c`.[`is_nul()`][UnicodeScalar#method.is_nul] an empty string will be returned.
    ///
    /// # Errors
    /// Returns [`OutOfBounds`] if `CAP` > 255,
    /// or [`NotEnoughCapacity`] if `!c.is_nul()`
    /// and `CAP` < `c.`[`len_utf8()`][UnicodeScalar#method.len_utf8].
    ///
    /// Will always succeed if `CAP` >= 4.
    #[inline]
    #[cfg(feature = "text")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "text")))]
    pub const fn from_char32(c: Char32) -> Result<Self> {
        Ok(unwrap![ok? Self::from_char(c.0)])
    }

    /* from bytes */

    /// Returns a string from an array of `bytes`.
    ///
    /// # Errors
    /// Returns [`InvalidUtf8`] if the bytes are not valid UTF-8,
    /// and [`InvalidNul`] if the bytes contains a NUL character.
    #[inline]
    pub const fn from_bytes(bytes: [u8; CAP]) -> Result<Self> {
        match core::str::from_utf8(&bytes) {
            Ok(_) => {
                let mut index = 0;
                while index < CAP {
                    if bytes[index] == 0 {
                        return Err(InvalidNul);
                    }
                    index += 1;
                }
                Ok(Self { arr: bytes })
            }
            Err(e) => Err(InvalidUtf8(Some(e))),
        }
    }

    /// Returns a string from an array of `bytes` that must be valid UTF-8.
    ///
    /// # Safety
    /// The caller must ensure that the content of the slice is valid UTF-8,
    /// and that it doesn't contain nul characters.
    ///
    /// Use of a `str` whose contents are not valid UTF-8 is undefined behavior.
    #[inline]
    #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_str")))]
    pub const unsafe fn from_bytes_unchecked(bytes: [u8; CAP]) -> Self {
        Self { arr: bytes }
    }
}

/* traits implementations */

impl<const CAP: usize> Default for StringNonul<CAP> {
    /// Returns an empty string.
    ///
    /// # Panics
    /// Panics if `CAP > 255`.
    #[inline]
    fn default() -> Self {
        Self::new().unwrap()
    }
}

impl<const CAP: usize> fmt::Display for StringNonul<CAP> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl<const CAP: usize> fmt::Debug for StringNonul<CAP> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.as_str())
    }
}

impl<const CAP: usize> Deref for StringNonul<CAP> {
    type Target = str;
    #[inline]
    #[must_use]
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl<const CAP: usize> AsRef<str> for StringNonul<CAP> {
    #[inline]
    #[must_use]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<const CAP: usize> AsRef<[u8]> for StringNonul<CAP> {
    #[inline]
    #[must_use]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl<const CAP: usize> TryFrom<&str> for StringNonul<CAP> {
    type Error = TextError;

    /// Tries to create a new `StringNonul` from the given string slice.
    ///
    /// # Errors
    /// Returns [`OutOfBounds`] if `CAP > `[`u8::MAX`],
    /// or [`NotEnoughCapacity`] if `CAP < str.len()`.
    fn try_from(string: &str) -> Result<Self> {
        let non_nul_len = string.as_bytes().iter().filter(|x| **x != 0).count();
        if CAP < non_nul_len {
            Err(NotEnoughCapacity(non_nul_len))
        } else {
            let mut new_string = Self::new()?;
            let copied_bytes = new_string.push_str(string);
            debug_assert_eq![non_nul_len, copied_bytes];
            Ok(new_string)
        }
    }
}

impl<const CAP: usize> TryFrom<&[u8]> for StringNonul<CAP> {
    type Error = TextError;

    /// Tries to create a new `StringNonul` from the given slice of `bytes`.
    ///
    /// # Errors
    /// Returns [`OutOfBounds`] if `CAP > `[u8::MAX`],
    /// [`NotEnoughCapacity`] if `CAP < bytes.len()`
    /// or [`InvalidUtf8`] if the `bytes` are not valid UTF-8.
    fn try_from(bytes: &[u8]) -> Result<Self> {
        match core::str::from_utf8(bytes) {
            Ok(_) => {
                let mut arr = [0; CAP];
                let mut idx = 0;

                for &byte in bytes.iter() {
                    if byte != 0 {
                        if idx >= CAP {
                            return Err(NotEnoughCapacity(bytes.len()));
                        }
                        arr[idx] = byte;
                        idx += 1;
                    }
                }

                Ok(Self { arr })
            }
            Err(e) => Err(InvalidUtf8(Some(e))),
        }
    }
}

impl_sized_alias![
    StringNonul, StringNonul,
    "UTF-8–encoded string, backed by an array of ",
    ". Can't contain nul chars.":
    "An" 8, 1 "";
    "A" 16, 2 "s";
    "A" 32, 4 "s";
    "A" 64, 8 "s";
    "A" 128, 16 "s";
];
