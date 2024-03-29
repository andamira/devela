// devela::text::string_u
//
//! `String` backed by an array.
//
// TOC
// - generate_array_string!
//   - definitions
//   - trait impls
// - tests

use super::{
    char::{char_to_utf8_bytes, char_utf8_4bytes_len},
    helpers::impl_sized_alias,
    TextError, TextResult as Result,
};
use crate::{code::cfor, num::Compare, result::unwrap};
use core::{
    fmt,
    ops::Deref,
    str::{from_utf8, Chars},
};
use TextError::{InvalidUtf8, NotEnoughCapacity, NotEnoughElements, OutOfBounds};

#[cfg(feature = "text")]
use super::char::{
    char_utf8_2bytes_len, char_utf8_3bytes_len, Char16, Char24, Char32, Char7, Char8,
};
#[cfg(feature = "alloc")]
use crate::_deps::alloc::{ffi::CString, string::ToString};
#[cfg(feature = "unsafe_str")]
use core::str::from_utf8_unchecked;

macro_rules! generate_array_string {
    ($($t:ty),+ $(,)?) => {
        $( generate_array_string![@$t]; )+
    };
    (@$t:ty) => { $crate::code::paste! {

        /* definitions */

        #[doc = "A UTF-8–encoded string, backed by an array with [`" $t "::MAX`] bytes of capacity."]
        ///
        #[doc = "Internally, the current length is stored as a [`" u8 "`]."]
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
        pub struct [<String $t:camel>]<const CAP: usize> {
            // WAITING for when we can use CAP: u8 for panic-less const boundary check.
            arr: [u8; CAP],
            len: $t,
        }

        impl<const CAP: usize> [<String $t:camel>]<CAP> {
            /* construct */

            #[doc = "Creates a new empty `String" $t:camel "` with a capacity of `CAP` bytes."]
            ///
            /// # Errors
            #[doc = "Returns [`OutOfBounds`] if `CAP > `[`" $t "::MAX`]."]
            #[inline]
            pub const fn new() -> Result<Self> {
                if CAP <= $t::MAX as usize {
                    Ok(Self { arr: [0; CAP], len: 0 })
                } else {
                    Err(OutOfBounds(Some(CAP)))
                }
            }

            /* query */

            /// Returns the current string length in bytes.
            #[inline] #[must_use] #[rustfmt::skip]
            pub const fn len(&self) -> usize { self.len as usize }

            /// Returns `true` if the current length is 0.
            #[inline] #[must_use] #[rustfmt::skip]
            pub const fn is_empty(&self) -> bool { self.len == 0 }

            /// Returns `true` if the current remaining capacity is 0.
            #[inline] #[must_use] #[rustfmt::skip]
            pub const fn is_full(&self) -> bool { self.len == CAP as $t }

            /// Returns the total capacity in bytes.
            #[inline] #[must_use] #[rustfmt::skip]
            pub const fn capacity() -> usize { CAP }

            /// Returns the remaining capacity in bytes.
            #[inline] #[must_use] #[rustfmt::skip]
            pub const fn remaining_capacity(&self) -> usize { CAP - self.len as usize }

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
                return self.arr.get(0..self.len as usize).expect("len must be <= arr.len()");

                #[cfg(all(not(feature = "safe_text"), feature = "unsafe_slice"))]
                // SAFETY: we ensure len is always <= arr.len()
                unsafe { self.arr.get_unchecked(0..self.len as usize) }
            }
            /// Returns an exclusive byte slice of the inner string slice.
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
                self.arr.get_unchecked_mut(0..self.len as usize)
            }

            /// Returns the inner string slice.
            ///
            /// # Features
            /// Makes use of the `unsafe_str` feature if enabled.
            #[inline]
            #[must_use]
            pub fn as_str(&self) -> &str {
                #[cfg(any(feature = "safe_text", not(feature = "unsafe_str")))]
                return from_utf8(
                    self.arr
                        .get(0..self.len as usize)
                        .expect("len must be <= arr.len()"),
                )
                .expect("must be valid UTF-8");

                #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
                // SAFETY: we ensure to contain only valid UTF-8
                unsafe {
                    from_utf8_unchecked(
                        self.arr
                            .get(0..self.len as usize)
                            .expect("len must be <= arr.len()"),
                    )
                }
            }

            /// Returns the exclusive inner string slice.
            /// Makes use of the `unsafe_str` feature if enabled.
            #[must_use]
            #[cfg(all(not(feature = "safe_text"), feature = "unsafe_slice"))]
            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_slice")))]
            pub fn as_mut_str(&mut self) -> &mut str {
                unsafe { &mut *(self.as_bytes_mut() as *mut [u8] as *mut str) }
            }

            /// Returns an iterator over the `chars` of this grapheme cluster.
            #[inline] #[rustfmt::skip]
            pub fn chars(&self) -> Chars { self.as_str().chars() }

            /// Returns a new allocated C-compatible, nul-terminanted string.
            #[inline] #[must_use] #[rustfmt::skip]
            #[cfg(feature = "alloc")]
            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
            pub fn to_cstring(&self) -> CString { CString::new(self.to_string()).unwrap() }

            /* operations */

            /// Sets the length to 0.
            #[inline]
            pub fn clear(&mut self) {
                self.len = 0;
            }

            /// Sets the length to 0, and resets all the bytes to 0.
            #[inline]
            pub fn reset(&mut self) {
                self.arr = [0; CAP];
                self.len = 0;
            }

            /// Removes the last character and returns it, or `None` if
            /// the string is empty.
            #[inline]
            #[must_use]
            pub fn pop(&mut self) -> Option<char> {
                self.as_str().chars().last().map(|c| {
                    self.len -= c.len_utf8() as $t;
                    c
                })
            }

            /// Tries to remove the last character and returns it, or `None` if
            /// the string is empty.
            ///
            /// # Errors
            /// Returns a [`NotEnoughElements`] error
            /// if the capacity is not enough to hold the `character`.
            #[inline]
            pub fn try_pop(&mut self) -> Result<char> {
                self.as_str()
                    .chars()
                    .last()
                    .map(|c| {
                        self.len -= c.len_utf8() as $t;
                        c
                    })
                    .ok_or(NotEnoughElements(1))
            }

            /// Appends to the end of the string the given `character`.
            ///
            /// Returns the number of bytes written.
            ///
            /// It will return 0 bytes if the given `character` doesn't fit in
            /// the remaining capacity.
            pub fn push(&mut self, character: char) -> usize {
                let char_len = character.len_utf8();
                if self.remaining_capacity() >= char_len {
                    let beg = self.len as usize;
                    let end = beg + char_len;
                    let _ = character.encode_utf8(&mut self.arr[beg..end]);
                    self.len += char_len as $t;
                    char_len
                } else {
                    0
                }
            }

            /// Tries to append to the end of the string the given `character`.
            ///
            /// Returns the number of bytes written.
            ///
            /// # Errors
            /// Returns a [`NotEnoughCapacity`] error
            /// if the capacity is not enough to hold the `character`.
            pub fn try_push(&mut self, character: char) -> Result<usize> {
                let char_len = character.len_utf8();
                if self.remaining_capacity() >= char_len {
                    let beg = self.len as usize;
                    let end = beg + char_len;
                    let _ = character.encode_utf8(&mut self.arr[beg..end]);
                    self.len += char_len as $t;
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
                let mut bytes_written = 0;

                for character in string.chars() {
                    let char_len = character.len_utf8();
                    if self.len as usize + char_len <= CAP {
                        let start_pos = self.len as usize;
                        character.encode_utf8(&mut self.arr[start_pos..]);
                        self.len += char_len as $t;
                        bytes_written += char_len;
                    } else {
                        break;
                    }
                }
                bytes_written
            }

            /// Tries to append to the end the characters from the given `string` slice.
            ///
            /// Returns the number of bytes written.
            ///
            /// # Errors
            /// Returns [`NotEnoughCapacity`] if the capacity is not enough
            /// to hold even the first character.
            pub fn try_push_str(&mut self, string: &str) -> Result<usize> {
                if string.is_empty() {
                    return Ok(0);
                }
                let first_char_len = string.chars().next().unwrap().len_utf8();
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
            /// # Errors
            /// Returns [`NotEnoughCapacity`] if the slice wont completely fit.
            pub fn try_push_str_complete(&mut self, string: &str) -> Result<usize> {
                if self.remaining_capacity() >= string.len() {
                    Ok(self.push_str(string))
                } else {
                    Err(NotEnoughCapacity(string.len()))
                }
            }

            /* from char  */

            #[doc = "Creates a new `String" $t:camel "` from a `char`."]
            ///
            /// # Errors
            #[doc = "Returns [`OutOfBounds`] if `CAP > `[`" $t "::MAX`]."]
            /// or [`NotEnoughCapacity`] if
            /// `CAP < c.`[`len_utf8()`][UnicodeScalar#method.len_utf8].
            ///
            #[doc = "It will always succeed if `CAP >= 4 && CAP <= `[`" $t "::MAX`]."]
            #[inline]
            pub const fn from_char(c: char) -> Result<Self> {
                let mut new = unwrap![ok? Self::new()];

                let bytes = char_to_utf8_bytes(c);
                new.len = char_utf8_4bytes_len(bytes) as $t;

                new.arr[0] = bytes[0];
                if new.len > 1 {
                    new.arr[1] = bytes[1];
                }
                if new.len > 2 {
                    new.arr[2] = bytes[2];
                }
                if new.len > 3 {
                    new.arr[3] = bytes[3];
                }
                Ok(new)
            }

            #[doc = "Creates a new `String" $t:camel "` from a `Char7`."]
            ///
            /// # Errors
            #[doc = "Returns [`OutOfBounds`] if `CAP > `[`" $t "::MAX`]."]
            /// or [`NotEnoughCapacity`] if `CAP < 1.
            ///
            #[doc = "It will always succeed if `CAP >= 1 && CAP <= `[`" $t "::MAX`]."]
            #[inline]
            #[cfg(feature = "text")]
            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "text")))]
            pub const fn from_char7(c: Char7) -> Result<Self> {
                let mut new = unwrap![ok? Self::new()];
                new.arr[0] = c.to_utf8_bytes()[0];
                new.len = 1;
                Ok(new)
            }

            #[doc = "Creates a new `String" $t:camel "` from a `Char8`."]
            ///
            /// # Errors
            #[doc = "Returns [`OutOfBounds`] if `CAP > `[`" $t "::MAX`]."]
            /// or [`NotEnoughCapacity`] if `CAP < 2.
            ///
            #[doc = "It will always succeed if `CAP >= 2 && CAP <= `[`" $t "::MAX`]."]
            #[inline]
            #[cfg(feature = "text")]
            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "text")))]
            pub const fn from_char8(c: Char8) -> Result<Self> {
                let mut new = unwrap![ok? Self::new()];

                let bytes = c.to_utf8_bytes();
                new.len = char_utf8_2bytes_len(bytes) as $t;

                new.arr[0] = bytes[0];
                if new.len > 1 {
                    new.arr[1] = bytes[1];
                }
                Ok(new)
            }

            #[doc = "Creates a new `String" $t:camel "` from a `Char16`."]
            ///
            /// # Panics
            #[doc = "Panics if `CAP > `[`" $t
                "::MAX`]` || CAP < c.`[`len_utf8()`][Char16#method.len_utf8]."]
            ///
            #[doc = "Will never panic if `CAP >= 3 && CAP <= `[`" $t "::MAX`]."]
            #[inline]
            #[cfg(feature = "text")]
            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "text")))]
            pub const fn from_char16(c: Char16) -> Result<Self> {
                let mut new = unwrap![ok? Self::new()];

                let bytes = c.to_utf8_bytes();
                new.len = char_utf8_3bytes_len(bytes) as $t;

                new.arr[0] = bytes[0];
                if new.len > 1 {
                    new.arr[1] = bytes[1];
                }
                if new.len > 2 {
                    new.arr[2] = bytes[2];
                }
                Ok(new)
            }

            #[doc = "Creates a new `String" $t:camel "` from a `Char24`."]
            ///
            /// # Panics
            #[doc = "Panics if `CAP > `[`" $t
                "::MAX`]` || CAP < c.`[`len_utf8()`][Char24#method.len_utf8]."]
            ///
            #[doc = "Will never panic if `CAP >= 4 && CAP <= `[`" $t "::MAX`]."]
            #[inline]
            #[cfg(feature = "text")]
            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "text")))]
            pub const fn from_char24(c: Char24) -> Result<Self> {
                let mut new = unwrap![ok? Self::new()];

                let bytes = c.to_utf8_bytes();
                new.len = char_utf8_4bytes_len(bytes) as $t;

                new.arr[0] = bytes[0];
                if new.len > 1 {
                    new.arr[1] = bytes[1];
                }
                if new.len > 2 {
                    new.arr[2] = bytes[2];
                }
                if new.len > 3 {
                    new.arr[3] = bytes[3];
                }
                Ok(new)
            }

            #[doc = "Creates a new `String" $t:camel "` from a `Char32`."]
            ///
            /// # Panics
            #[doc = "Panics if `CAP > `[`" $t
                "::MAX`]` || CAP < c.`[`len_utf8()`][Char32#method.len_utf8]."]
            ///
            #[doc = "Will never panic if `CAP >= 4 && CAP <= `[`" $t "::MAX`]."]
            #[inline]
            #[cfg(feature = "text")]
            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "text")))]
            pub const fn from_char32(c: Char32) -> Result<Self> {
                Ok(unwrap![ok? Self::from_char(c.0)])
            }

            /* from bytes */

            /// Returns a string from a slice of `bytes`.
            ///
            /// # Errors
            /// Returns [`InvalidUtf8`] if the bytes are not valid UTF-8.
            #[inline]
            pub const fn from_bytes(bytes: [u8; CAP]) -> Result<Self> {
                match from_utf8(&bytes) {
                    Ok(_) => {
                        Ok(Self { arr: bytes, len: CAP as $t })
                    },
                    Err(e) => Err(InvalidUtf8(Some(e))),
                }
            }

            /// Returns a string from an array of `bytes` that must be valid UTF-8.
            ///
            /// # Safety
            /// The caller must ensure that the content of the slice is valid UTF-8.
            ///
            /// Use of a `str` whose contents are not valid UTF-8 is undefined behavior.
            #[inline]
            #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_str")))]
            pub const unsafe fn from_bytes_unchecked(bytes: [u8; CAP]) -> Self {
                Self { arr: bytes, len: CAP as $t }
            }

            /// Returns a string from an array of `bytes`,
            /// truncated to `n` bytes counting from the left.
            ///
            /// The new `length` is maxed out at `CAP`.
            ///
            /// # Errors
            /// Returns [`InvalidUtf8`] if the bytes are not valid UTF-8.
            #[inline]
            pub fn from_bytes_nleft(bytes: [u8; CAP], length: $t) -> Result<Self> {
                let length = Compare(length).min(CAP as $t);
                // IMPROVE make const. Limited by slice indexing
                match from_utf8(&bytes[0..length as usize]) {
                    Ok(_) => Ok(Self { arr: bytes, len: length }),
                    Err(e) => Err(InvalidUtf8(Some(e))),
                }
            }

            /// Returns a string from an array of `bytes`,
            /// truncated to `n` bytes counting from the left,
            /// which must be valid UTF-8.
            ///
            /// The new `length` is maxed out at `CAP`.
            ///
            /// # Safety
            /// The caller must ensure that the content of the truncated slice is valid UTF-8.
            ///
            /// Use of a `str` whose contents are not valid UTF-8 is undefined behavior.
            #[inline]
            #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_str")))]
            pub const unsafe fn from_bytes_nleft_unchecked(bytes: [u8; CAP], length: $t) -> Self {
                Self { arr: bytes, len: Compare(length).min(CAP as $t) }
            }

            /// Returns a string from an array of `bytes`,
            /// truncated to `n` bytes counting from the right.
            ///
            /// The new `length` is maxed out at `CAP`.
            /// Bytes are shift-copied without allocating a new array.
            ///
            /// # Errors
            /// Returns [`InvalidUtf8`] if the bytes are not valid UTF-8.
            #[inline]
            pub fn from_bytes_nright(mut bytes: [u8; CAP], length: $t) -> Result<Self> {
                let length = Compare(length).min(CAP as $t);
                let ulen = length as usize;
                let start = CAP - ulen;
                cfor![i in 0..ulen => {
                    bytes[i] = bytes[start + i];
                }];
                // IMPROVE make const. Limited by slice indexing
                match core::str::from_utf8(&bytes[0..ulen]) {
                    Ok(_) => Ok(Self { arr: bytes, len: length }),
                    Err(e) => Err(InvalidUtf8(Some(e))),
                }
            }

            /// Returns a string from an array of `bytes`,
            /// truncated to `n` bytes counting from the right,
            /// which must be valid UTF-8.
            ///
            /// The new `length` is maxed out at `CAP`.
            /// Bytes are shift-copied without allocating a new array.
            ///
            /// # Safety
            /// The caller must ensure that the content of the truncated slice is valid UTF-8.
            ///
            /// Use of a `str` whose contents are not valid UTF-8 is undefined behavior.
            #[inline]
            #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_str")))]
            pub const unsafe fn from_bytes_nright_unchecked(mut bytes: [u8; CAP], length: $t)
                -> Self {
                let length = Compare(length).min(CAP as $t);
                let ulen = length as usize;
                let start = CAP - ulen;
                cfor![i in 0..ulen => {
                    bytes[i] = bytes[start + i];
                }];
                Self { arr: bytes, len: length }
            }
        }

        /* traits implementations */

        impl<const CAP: usize> Default for [<String $t:camel>]<CAP> {
            /// Returns an empty string.
            ///
            /// # Panics
            #[doc = "Panics if `CAP > `[`" $t "::MAX`]."]
            #[inline]
            fn default() -> Self {
                Self::new().unwrap()
            }
        }

        impl<const CAP: usize> fmt::Display for [<String $t:camel>]<CAP> {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.as_str())
            }
        }

        impl<const CAP: usize> fmt::Debug for [<String $t:camel>]<CAP> {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{:?}", self.as_str())
            }
        }

        impl<const CAP: usize> Deref for [<String $t:camel>]<CAP> {
            type Target = str;
            #[inline]
            #[must_use]
            fn deref(&self) -> &Self::Target {
                self.as_str()
            }
        }

        impl<const CAP: usize> AsRef<str> for [<String $t:camel>]<CAP> {
            #[inline]
            #[must_use]
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl<const CAP: usize> AsRef<[u8]> for [<String $t:camel>]<CAP> {
            #[inline]
            #[must_use]
            fn as_ref(&self) -> &[u8] {
                self.as_bytes()
            }
        }

        impl<const CAP: usize> TryFrom<&str> for [<String $t:camel>]<CAP> {
            type Error = TextError;

            #[doc = "Tries to create a new `String" $t:camel "` from the given `string` slice."]
            ///
            /// # Errors
            #[doc = "Returns [`OutOfBounds`] if `CAP > `[`" $t "::MAX`]"]
            /// or [`NotEnoughCapacity`] if `CAP < string.len()`.
            fn try_from(string: &str) -> Result<Self> {
                if CAP < string.len() {
                    Err(NotEnoughCapacity(string.len()))
                } else {
                    let mut new_string = Self::new()?;
                    let bytes = string.as_bytes();
                    new_string.arr[..bytes.len()].copy_from_slice(bytes);
                    Ok(new_string)
                }
            }
        }

        impl<const CAP: usize> TryFrom<&[u8]> for [<String $t:camel>]<CAP> {
            type Error = TextError;

            #[doc = "Tries to create a new `String" $t:camel "` from the given slice of bytes."]
            ///
            /// # Errors
            #[doc = "Returns [`OutOfBounds`] if `CAP > `[`" $t "::MAX`],"]
            /// [`NotEnoughCapacity`] if `CAP < bytes.len()`
            /// or [`InvalidUtf8`] if the bytes are not valid UTF-8.
            fn try_from(bytes: &[u8]) -> Result<Self> {
                if CAP < bytes.len() {
                    Err(NotEnoughCapacity(bytes.len()))
                } else {
                    // Check if the byte slice is valid UTF-8
                    match from_utf8(bytes) {
                        Ok(_) => {
                            let mut arr = [0; CAP];
                            arr[..bytes.len()].copy_from_slice(bytes);
                            Ok(Self { arr, len: bytes.len() as $t })
                        },
                        Err(e) => Err(InvalidUtf8(Some(e))),
                    }
                }
            }
        }


        #[cfg(all(feature = "std", any(unix, target_os = "wasi")))]
        mod [< std_impls_ $t >] {
            use super::[<String $t:camel>];
            use std::ffi::OsStr;

            #[cfg(unix)]
            use std::os::unix::ffi::OsStrExt;
            #[cfg(target_os = "wasi")]
            use std::os::wasi::ffi::OsStrExt;

            #[cfg_attr(feature = "nightly_doc", doc(cfg(
                all(feature = "std", any(unix, target_os = "wasi"))
            )))]
            impl<const CAP: usize> AsRef<OsStr> for [<String $t:camel>]<CAP> {
            #[must_use]
                fn as_ref(&self) -> &OsStr {
                    OsStr::from_bytes(self.as_bytes())
                }
            }
        }
    }};
}
generate_array_string![u8, u16, u32];

impl_sized_alias![
    String, StringU8,
    "UTF-8–encoded string, backed by an array of ", ".":
    "A" 16, 1 "";
    "A" 32, 3 "s";
    "A" 64, 7 "s";
    "A" 128, 15 "s";
];

#[cfg(test)]
mod tests {
    use super::{String32, StringU8};

    #[test]
    fn push() {
        let mut s = String32::new().unwrap(); // max capacity == 3
        assert![s.try_push('ñ').is_ok()];
        assert_eq![2, s.len()];
        assert![s.try_push('ñ').is_err()];
        assert_eq![2, s.len()];
        assert![s.try_push('a').is_ok()];
        assert_eq![3, s.len()];
    }

    #[test]
    fn pop() {
        let mut s = String32::new().unwrap(); // max capacity == 3
        s.push('ñ');
        s.push('a');
        assert_eq![Some('a'), s.pop()];
        assert_eq![Some('ñ'), s.pop()];
        assert_eq![None, s.pop()];
    }
}
