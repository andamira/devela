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
    ArrayStringError, Result,
};
use core::{fmt, ops::Deref};

#[cfg(feature = "alloc")]
use crate::_deps::alloc::{ffi::CString, str::Chars, string::ToString};

#[cfg(feature = "text")]
use super::char::{
    char_utf8_2bytes_len, char_utf8_3bytes_len, Char16, Char24, Char32, Char7, Char8,
};

macro_rules! generate_array_string {
    ($($t:ty),+ $(,)?) => {
        $( generate_array_string![@$t]; )+
    };
    (@$t:ty) => { $crate::code::paste! {

        /* definitions */

        #[doc = "A UTF-8–encoded string, backed by an array with [`" $t "::MAX`] bytes of capacity."]
        ///
        #[doc = "Internally, the current length is stored as a [`" u8 "`]."]
        #[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
        pub struct [<String $t:upper>]<const CAP: usize> {
            // WAITING for when we can use CAP: u8 for panic-less const boundary check.
            arr: [u8; CAP],
            len: $t,
        }

        impl<const CAP: usize> [<String $t:upper>]<CAP> {
            #[doc = "Creates a new empty `Array" $t:upper "String>]` with a capacity of `CAP` bytes."]
            ///
            /// # Panics
            #[doc = "Panics if `CAP > `[`" $t "::MAX`]."]
            #[inline]
            #[must_use]
            pub const fn new() -> Self {
                assert![CAP <= $t::MAX as usize];
                Self {
                    arr: [0; CAP],
                    len: 0,
                }
            }

            /// Creates a new `Array $t:upper String>]` from a `Char7`.
            ///
            /// # Panics
            #[doc = "Panics if `CAP > `[`" $t "::MAX`]` || CAP < 1`."]
            ///
            #[doc = "Will never panic if `CAP >= 1 && CAP <= `[`" $t "::MAX`]."]
            #[inline]
            #[must_use]
            #[cfg(feature = "text")]
            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "text")))]
            pub const fn from_char7(c: Char7) -> Self {
                let mut new = Self::new();
                new.arr[0] = c.to_utf8_bytes()[0];
                new.len = 1;
                new
            }

            /// Creates a new `Array $t:upper String>]` from a `Char8`.
            ///
            /// # Panics
            #[doc = "Panics if `CAP > `[`" $t
                "::MAX`]` || CAP < c.`[`len_utf8()`][Char8#method.len_utf8]."]
            ///
            #[doc = "Will never panic if `CAP >= 2 && CAP <= `[`" $t "::MAX`]."]
            #[inline]
            #[must_use]
            #[cfg(feature = "text")]
            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "text")))]
            pub const fn from_char8(c: Char8) -> Self {
                let mut new = Self::new();

                let bytes = c.to_utf8_bytes();
                new.len = char_utf8_2bytes_len(bytes) as $t;

                new.arr[0] = bytes[0];
                if new.len > 1 {
                    new.arr[1] = bytes[1];
                }
                new
            }

            /// Creates a new `Array $t:upper String>]` from a `Char16`.
            ///
            /// # Panics
            #[doc = "Panics if `CAP > `[`" $t
                "::MAX`]` || CAP < c.`[`len_utf8()`][Char16#method.len_utf8]."]
            ///
            #[doc = "Will never panic if `CAP >= 3 && CAP <= `[`" $t "::MAX`]."]
            #[inline]
            #[must_use]
            #[cfg(feature = "text")]
            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "text")))]
            pub const fn from_char16(c: Char16) -> Self {
                let mut new = Self::new();

                let bytes = c.to_utf8_bytes();
                new.len = char_utf8_3bytes_len(bytes) as $t;

                new.arr[0] = bytes[0];
                if new.len > 1 {
                    new.arr[1] = bytes[1];
                }
                if new.len > 2 {
                    new.arr[2] = bytes[2];
                }
                new
            }

            /// Creates a new `Array $t:upper String>]` from a `Char24`.
            ///
            /// # Panics
            #[doc = "Panics if `CAP > `[`" $t
                "::MAX`]` || CAP < c.`[`len_utf8()`][Char24#method.len_utf8]."]
            ///
            #[doc = "Will never panic if `CAP >= 4 && CAP <= `[`" $t "::MAX`]."]
            #[inline]
            #[must_use]
            #[cfg(feature = "text")]
            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "text")))]
            pub const fn from_char24(c: Char24) -> Self {
                let mut new = Self::new();

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
                new
            }

            /// Creates a new `Array $t:upper String>]` from a `Char32`.
            ///
            /// # Panics
            #[doc = "Panics if `CAP > `[`" $t
                "::MAX`]` || CAP < c.`[`len_utf8()`][Char32#method.len_utf8]."]
            ///
            #[doc = "Will never panic if `CAP >= 4 && CAP <= `[`" $t "::MAX`]."]
            #[inline]
            #[must_use]
            #[cfg(feature = "text")]
            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "text")))]
            pub const fn from_char32(c: Char32) -> Self {
                Self::from_char(c.0)
            }

            /// Creates a new `Array $t:upper String>]` from a `char`.
            ///
            /// # Panics
            #[doc = "Panics if `CAP > `[`" $t
                "::MAX`]` || CAP < c.`[`len_utf8()`][UnicodeScalar#method.len_utf8]."]
            ///
            #[doc = "Will never panic if `CAP >= 4 && CAP <= `[`" $t "::MAX`]."]
            #[inline]
            #[must_use]
            pub const fn from_char(c: char) -> Self {
                let mut new = Self::new();

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
                new
            }

            //

            /// Returns the total capacity in bytes.
            #[inline]
            #[must_use]
            pub const fn capacity() -> usize {
                CAP
            }

            /// Returns the remaining capacity.
            #[inline]
            #[must_use]
            pub const fn remaining_capacity(&self) -> usize {
                CAP - self.len as usize
            }

            /// Returns the current length.
            #[inline]
            #[must_use]
            pub const fn len(&self) -> usize {
                self.len as usize
            }

            /// Returns `true` if the current length is 0.
            #[inline]
            #[must_use]
            pub const fn is_empty(&self) -> bool {
                self.len == 0
            }

            /// Returns `true` if the current remaining capacity is 0.
            #[inline]
            #[must_use]
            pub const fn is_full(&self) -> bool {
                self.len == CAP as $t
            }

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

            //

            /// Returns a byte slice of the inner string slice.
            /// # Features
            /// Makes use of the `unsafe_slice` feature if enabled.
            #[inline]
            #[must_use]
            pub fn as_bytes(&self) -> &[u8] {
                #[cfg(any(feature = "safe_text", not(feature = "unsafe_slice")))]
                return self.arr
                    .get(0..self.len as usize)
                    .expect("len must be <= arr.len()");

                #[cfg(all(not(feature = "safe_text"), feature = "unsafe_slice"))]
                unsafe {
                    self.arr.get_unchecked(0..self.len as usize)
                }
            }

            /// Returns a mutable byte slice of the inner string slice.
            /// # Safety
            /// TODO
            #[inline]
            #[must_use]
            #[cfg(all(not(feature = "safe_text"), feature = "unsafe_slice"))]
            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_slice")))]
            pub unsafe fn as_bytes_mut(&mut self) -> &mut [u8] {
                self.arr.get_unchecked_mut(0..self.len as usize)
            }

            /// Returns a copy of the inner array with the full contents.
            ///
            /// The array contains all the bytes, including those outside the current length.
            #[inline]
            #[must_use]
            pub const fn as_array(&self) -> [u8; CAP] {
                self.arr
            }

            /// Returns the inner array with the full contents.
            ///
            /// The array contains all the bytes, including those outside the current length.
            #[inline]
            #[must_use]
            pub const fn into_array(self) -> [u8; CAP] {
                self.arr
            }

            /// Returns the inner string slice.
            /// # Features
            /// Makes use of the `unsafe_slice` feature if enabled.
            #[inline]
            #[must_use]
            pub fn as_str(&self) -> &str {
                #[cfg(any(feature = "safe_text", not(feature = "unsafe_slice")))]
                return core::str::from_utf8(
                    self.arr
                        .get(0..self.len as usize)
                        .expect("len must be <= arr.len()"),
                )
                .expect("must be valid utf-8");

                #[cfg(all(not(feature = "safe_text"), feature = "unsafe_slice"))]
                // SAFETY: TODO
                unsafe {
                    core::str::from_utf8_unchecked(
                        self.arr
                            .get(0..self.len as usize)
                            .expect("len must be <= arr.len()"),
                    )
                }
            }

            /// Returns the mutable inner string slice.
            /// # Safety
            /// TODO
            #[must_use]
            #[cfg(all(not(feature = "safe_text"), feature = "unsafe_slice"))]
            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_slice")))]
            pub fn as_str_mut(&mut self) -> &mut str {
                unsafe { &mut *(self.as_bytes_mut() as *mut [u8] as *mut str) }
            }

            /// Returns an iterator over the `chars` of this grapheme cluster.
            #[inline]
            #[cfg(feature = "alloc")]
            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
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

            //

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
            /// Returns a [`NotEnoughElements`][ArrayStringError::NotEnoughElements] error
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
                    .ok_or(ArrayStringError::NotEnoughElements(1))
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
            /// Returns a [`NotEnoughCapacity`][ArrayStringError::NotEnoughCapacity] error
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
                    Err(ArrayStringError::NotEnoughCapacity(char_len))
                }
            }
        }

        /* traits */

        impl<const CAP: usize> Default for [<String $t:upper>]<CAP> {
            /// Returns an empty string.
            ///
            /// # Panics
            /// Panics if `CAP` > 255.
            #[inline]
            #[must_use]
            fn default() -> Self {
                Self::new()
            }
        }

        impl<const CAP: usize> fmt::Display for [<String $t:upper>]<CAP> {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.as_str())
            }
        }

        impl<const CAP: usize> fmt::Debug for [<String $t:upper>]<CAP> {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{:?}", self.as_str())
            }
        }

        impl<const CAP: usize> Deref for [<String $t:upper>]<CAP> {
            type Target = str;
            #[inline]
            #[must_use]
            fn deref(&self) -> &Self::Target {
                self.as_str()
            }
        }

        impl<const CAP: usize> AsRef<str> for [<String $t:upper>]<CAP> {
            #[inline]
            #[must_use]
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl<const CAP: usize> AsRef<[u8]> for [<String $t:upper>]<CAP> {
            #[inline]
            #[must_use]
            fn as_ref(&self) -> &[u8] {
                self.as_bytes()
            }
        }

        #[cfg(all(feature = "std", any(unix, target_os = "wasi")))]
        mod [< std_impls_ $t >] {
            use super::[<String $t:upper>];
            use std::ffi::OsStr;

            #[cfg(unix)]
            use std::os::unix::ffi::OsStrExt;
            #[cfg(target_os = "wasi")]
            use std::os::wasi::ffi::OsStrExt;

            #[cfg_attr(feature = "nightly_doc", doc(cfg(
                all(feature = "std", any(unix, target_os = "wasi"))
            )))]
            impl<const CAP: usize> AsRef<OsStr> for [<String $t:upper>]<CAP> {
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
    use super::*;

    #[test]
    fn push() {
        let mut s = String32::new(); // max capacity == 3

        assert![s.try_push('ñ').is_ok()];
        assert_eq![2, s.len()];
        assert![s.try_push('ñ').is_err()];
        assert_eq![2, s.len()];
        assert![s.try_push('a').is_ok()];
        assert_eq![3, s.len()];
    }

    // TODO
    #[test]
    fn pop() {
        let mut s = String32::new(); // max capacity == 3

        s.push('ñ');
        s.push('a');
        assert_eq![Some('a'), s.pop()];
        assert_eq![Some('ñ'), s.pop()];
        assert_eq![None, s.pop()];
    }
}
