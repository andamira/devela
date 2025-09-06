// devela_base_core::text::string_u
//
//! `String` backed by an array.
//
// TOC
// - impl_str_u!
//   - definitions
//   - trait impls
// - tests

#[allow(unused, reason = "±unsafe")]
use crate::{Compare, cfor, unwrap};
use crate::{
    Debug, Deref, Display, FmtResult, Formatter, InvalidText, InvalidUtf8, IterChars, Mismatch,
    MismatchedCapacity, NotEnoughElements, Slice, Str, is, paste, text::char::*,
};

macro_rules! impl_str_u {
    () => { impl_str_u![u8, u16, u32, usize]; };

    (
    // $t: the length type. E.g.: u8.
    $($t:ty),+ $(,)?) => {
        $(
            paste! { impl_str_u![@[<String $t:camel>], $t]; }
        )+
    };
    (
    // $name: the name of the type. E.g.: StringU8.
    @$name:ty, $t:ty) => { paste! {
        /* definitions */

        #[allow(rustdoc::broken_intra_doc_links, reason = "±unsafe")]
        #[doc = crate::TAG_TEXT!()]
        #[doc = "A UTF-8–encoded string, backed by an array with [`" $t "::MAX`] bytes of capacity."]
        ///
        #[doc = "Internally, the current length is stored as a [`" $t "`]."]
        ///
        /// ## Methods
        /// - Construct:
        ///   [`new`][Self::new],
        ///   [`from_char`][Self::from_char]*(
        ///     [`7`](Self::from_char7),
        ///     [`8`](Self::from_char8),
        ///     [`16`](Self::from_char16).
        ///   )*.
        /// - Deconstruct:
        ///   [`into_array`][Self::into_array],
        ///   [`as_array`][Self::as_array],
        ///   [`as_bytes`][Self::as_bytes]
        ///     *([mut][Self::as_bytes_mut]<sup title="unsafe function">⚠</sup>)*,
        ///   [`as_str`][Self::as_str]
        ///     *([mut][Self::as_mut_str]<sup title="unsafe function">⚠</sup>)*,
        ///   [`chars`][Self::chars],
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
        pub struct $name<const CAP: usize> {
            // WAIT: for when we can use CAP: u8 for panic-less const boundary check.
            arr: [u8; CAP],
            len: $t,
        }

        impl<const CAP: usize> $name<CAP> {
            /* construct */

            #[doc = "Creates a new empty `String" $t:camel "` with a capacity of `CAP` bytes."]
            ///
            /// # Errors
            #[doc = "Returns [`MismatchedCapacity`] if `CAP > `[`" $t "::MAX`]."]
            pub const fn new() -> Result<Self, MismatchedCapacity> {
                if CAP <= $t::MAX as usize {
                    Ok(Self { arr: [0; CAP], len: 0 })
                } else {
                    Err(MismatchedCapacity::closed(0, <$t>::MAX as usize, CAP))
                }
            }

            /* query */

            /// Returns the current string length in bytes.
            #[must_use]
            #[inline(always)]
            pub const fn len(&self) -> usize { self.len as usize }

            /// Returns `true` if the current length is 0.
            #[must_use]
            #[inline(always)]
            pub const fn is_empty(&self) -> bool { self.len == 0 }

            /// Returns `true` if the current remaining capacity is 0.
            #[must_use]
            #[inline(always)]
            pub const fn is_full(&self) -> bool { self.len == CAP as $t }

            /// Returns the total capacity in bytes.
            #[must_use]
            #[inline(always)]
            pub const fn capacity() -> usize { CAP }

            /// Returns the remaining capacity in bytes.
            #[must_use]
            #[inline(always)]
            pub const fn remaining_capacity(&self) -> usize { CAP - self.len as usize }

            /* deconstruct */

            /// Returns the inner array with the full contents.
            ///
            /// The array contains all the bytes, including those outside the current length.
            #[must_use]
            #[inline(always)]
            pub const fn into_array(self) -> [u8; CAP] { self.arr }

            /// Returns a copy of the inner array with the full contents.
            ///
            /// The array contains all the bytes, including those outside the current length.
            #[must_use]
            #[inline(always)]
            pub const fn as_array(&self) -> [u8; CAP] { self.arr }

            /// Returns a byte slice of the inner string slice.
            #[must_use]
            #[inline(always)]
            pub const fn as_bytes(&self) -> &[u8] {
                #[cfg(any(base_safe_text, not(feature = "unsafe_str")))]
                return Slice::take_first(&self.arr, self.len as usize);

                #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
                unsafe { Slice::take_first_unchecked(&self.arr, self.len as usize) }
            }

            /// Returns an exclusive byte slice of the inner string slice.
            ///
            /// # Features
            /// Makes use of the `unsafe_str` feature if enabled.
            #[must_use]
            #[inline(always)]
            pub const fn as_bytes_mut(&mut self) -> &mut [u8] {
                #[cfg(any(base_safe_text, not(feature = "unsafe_str")))]
                return Slice::take_first_mut(&mut self.arr, self.len as usize);

                #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
                unsafe { Slice::take_first_mut_unchecked(&mut self.arr, self.len as usize) }
            }

            /// Returns the inner string slice.
            ///
            /// # Features
            /// Makes use of the `unsafe_str` feature if enabled.
            #[must_use]
            #[inline(always)]
            pub const fn as_str(&self) -> &str {
                #[cfg(any(base_safe_text, not(feature = "unsafe_str")))]
                return unwrap![ok_expect Str::from_utf8(self.as_bytes()), "Invalid UTF-8"];

                #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
                // SAFETY: we ensure to contain only valid UTF-8
                unsafe { Str::from_utf8_unchecked(self.as_bytes()) }
            }

            /// Returns the exclusive inner string slice.
            /// Makes use of the `unsafe_str` feature if enabled.
            ///
            /// # Safety
            /// The content must be valid UTF-8.
            #[must_use]
            #[inline(always)]
            #[cfg(all(not(base_safe_text), feature = "unsafe_slice"))]
            #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_slice")))]
            pub const unsafe fn as_mut_str(&mut self) -> &mut str {
                unsafe { &mut *(self.as_bytes_mut() as *mut [u8] as *mut str) }
            }

            /// Returns an iterator over the `chars` of this grapheme cluster.
            #[inline(always)]
            pub fn chars(&self) -> IterChars<'_> { self.as_str().chars() } // non-const

            /* operations */

            /// Sets the length to 0.
            #[inline(always)]
            pub const fn clear(&mut self) { self.len = 0; }

            /// Sets the length to 0, and resets all the bytes to 0.
            #[inline(always)]
            pub const fn reset(&mut self) { self.arr = [0; CAP]; self.len = 0; }

            /// Zeros all unused bytes while maintaining the current length.
            #[inline(always)]
            pub const fn sanitize(&mut self) {
                cfor![i in (self.len as usize)..CAP => { self.arr[i] = 0; }];
            }

            /// Removes the last character and returns it, or `None` if
            /// the string is empty.
            #[must_use]
            pub fn pop(&mut self) -> Option<char> {
                self.as_str().chars() // non-const
                    .last().map(|c| { self.len -= c.len_utf8() as $t; c })
            }

            /// Tries to remove the last character and returns it, or `None` if
            /// the string is empty.
            ///
            /// # Errors
            /// Returns a [`NotEnoughElements`] error
            /// if the capacity is not enough to hold the `character`.
            pub fn try_pop(&mut self) -> Result<char, NotEnoughElements> {
                self.as_str().chars() // non-const
                    .last().map(|c| { self.len -= c.len_utf8() as $t; c })
                    .ok_or(NotEnoughElements(Some(1)))
            }

            /// Appends to the end of the string the given `character`.
            ///
            /// Returns the number of bytes written.
            ///
            /// It will return 0 bytes if the given `character` doesn't fit in
            /// the remaining capacity.
            pub const fn push(&mut self, character: char) -> usize {
                let char_len = character.len_utf8();
                if self.remaining_capacity() >= char_len {
                    let beg = self.len as usize;
                    let end = beg + char_len;
                    // let _ = character.encode_utf8(&mut self.arr[beg..end]);
                    let _ = character.encode_utf8(Slice::range_mut(&mut self.arr, beg, end));
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
            /// Returns a [`MismatchedCapacity`] error
            /// if the capacity is not enough to hold the `character`.
            pub const fn try_push(&mut self, character: char) -> Result<usize, MismatchedCapacity> {
                let char_len = character.len_utf8();
                if self.remaining_capacity() >= char_len {
                    let beg = self.len as usize;
                    let end = beg + char_len;
                    // let _ = character.encode_utf8(&mut self.arr[beg..end]);
                    let _ = character.encode_utf8(Slice::range_mut(&mut self.arr, beg, end));
                    self.len += char_len as $t;
                    Ok(char_len)
                } else {
                    Err(MismatchedCapacity::closed(0, self.len() + character.len_utf8(), CAP))
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
                for character in string.chars() { // non-const
                    let char_len = character.len_utf8();
                    if self.len as usize + char_len <= CAP {
                        let start_pos = self.len as usize;
                        character.encode_utf8(&mut self.arr[start_pos..]);
                        // character.encode_utf8(Slice::range_from_mut(&mut self.arr, start_pos));
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
            /// Returns [`MismatchedCapacity`] if the capacity is not enough
            /// to hold even the first character.
            pub fn try_push_str(&mut self, string: &str) -> Result<usize, MismatchedCapacity> {
                is![string.is_empty(); return Ok(0)];
                let first_char_len = string.chars().next().unwrap().len_utf8(); // non-const
                if self.remaining_capacity() < first_char_len {
                    Err(MismatchedCapacity::closed(0, self.len() + first_char_len, CAP))
                } else {
                    Ok(self.push_str(string))
                }
            }

            /// Tries to append the complete `string` slice to the end.
            ///
            /// Returns the number of bytes written in success.
            ///
            /// # Errors
            /// Returns [`MismatchedCapacity`] if the slice wont completely fit.
            pub fn try_push_str_complete(&mut self, string: &str)
            -> Result<usize, MismatchedCapacity> {
                if self.remaining_capacity() >= string.len() {
                    Ok(self.push_str(string)) // non-const
                } else {
                    Err(MismatchedCapacity::closed(0, self.len() + string.len(), CAP))
                }
            }

            /* from char  */

            #[doc = "Creates a new `String" $t:camel "` from a `char`."]
            ///
            /// # Errors
            #[doc = "Returns [`MismatchedCapacity`] if `CAP > `[`" $t "::MAX`]."]
            /// or if `CAP < c.`[`len_utf8()`][crate::UnicodeScalar#method.len_utf8].
            ///
            #[doc = "It will always succeed if `CAP >= 4 && CAP <= `[`" $t "::MAX`]."]
            pub const fn from_char(c: char) -> Result<Self, MismatchedCapacity> {
                let mut new = unwrap![ok? Self::new()];
                let bytes = Char::to_utf8_bytes(c);
                new.len = Char::utf8_len(bytes[0]) as $t;
                new.arr[0] = bytes[0];
                if new.len > 1 { new.arr[1] = bytes[1]; }
                if new.len > 2 { new.arr[2] = bytes[2]; }
                if new.len > 3 { new.arr[3] = bytes[3]; }
                Ok(new)
            }

            #[doc = "Creates a new `String" $t:camel "` from a `char7`."]
            ///
            /// # Errors
            #[doc = "Returns [`MismatchedCapacity`] if `CAP > `[`" $t "::MAX`]."]
            /// or if `CAP < 1.
            ///
            #[doc = "It will always succeed if `CAP >= 1 && CAP <= `[`" $t "::MAX`]."]
            pub const fn from_char7(c: char7) -> Result<Self, MismatchedCapacity> {
                let mut new = unwrap![ok? Self::new()];
                new.arr[0] = c.to_utf8_bytes()[0];
                new.len = 1;
                Ok(new)
            }

            #[doc = "Creates a new `String" $t:camel "` from a `char8`."]
            ///
            /// # Errors
            #[doc = "Returns [`MismatchedCapacity`] if `CAP > `[`" $t "::MAX`]."]
            /// or if `CAP < 2.
            ///
            #[doc = "It will always succeed if `CAP >= 2 && CAP <= `[`" $t "::MAX`]."]
            pub const fn from_char8(c: char8) -> Result<Self, MismatchedCapacity> {
                let mut new = unwrap![ok? Self::new()];
                let bytes = c.to_utf8_bytes();
                new.len = Char::utf8_len(bytes[0]) as $t;
                new.arr[0] = bytes[0];
                if new.len > 1 { new.arr[1] = bytes[1]; }
                Ok(new)
            }

            #[doc = "Creates a new `String" $t:camel "` from a `char16`."]
            ///
            /// # Errors
            #[doc = "Returns [`MismatchedCapacity`] if `CAP > `[`" $t
                "::MAX`]` || CAP < c.`[`len_utf8()`][char16#method.len_utf8]."]
            ///
            #[doc = "It will always succeed if `CAP >= 3 && CAP <= `[`" $t "::MAX`]."]
            pub const fn from_char16(c: char16) -> Result<Self, MismatchedCapacity> {
                let mut new = unwrap![ok? Self::new()];
                let bytes = c.to_utf8_bytes();
                new.len = Char::utf8_len(bytes[0]) as $t;
                new.arr[0] = bytes[0];
                if new.len > 1 { new.arr[1] = bytes[1]; }
                if new.len > 2 { new.arr[2] = bytes[2]; }
                Ok(new)
            }

            /* from bytes */

            /// Returns a string from a slice of `bytes`.
            ///
            /// # Errors
            /// Returns [`InvalidUtf8`] if the bytes are not valid UTF-8.
            pub const fn from_bytes(bytes: [u8; CAP]) -> Result<Self, InvalidUtf8> {
                match Str::from_utf8(&bytes) {
                    Ok(_) => {
                        Ok(Self { arr: bytes, len: CAP as $t })
                    },
                    Err(e) => Err(e),
                }
            }

            /// Returns a string from an array of `bytes` that must be valid UTF-8.
            ///
            /// # Safety
            /// The caller must ensure that the content of the slice is valid UTF-8.
            ///
            /// Use of a `str` whose contents are not valid UTF-8 is undefined behavior.
            #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
            #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_str")))]
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
            pub const fn from_bytes_nleft(bytes: [u8; CAP], length: $t)
            -> Result<Self, InvalidUtf8> {
                let length = Compare(length).min(CAP as $t);
                match Str::from_utf8(Slice::take_first(&bytes, length as usize)) {
                    Ok(_) => Ok(Self { arr: bytes, len: length }),
                    Err(e) => Err(e),
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
            #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
            #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_str")))]
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
            pub const fn from_bytes_nright(mut bytes: [u8; CAP], length: $t)
            -> Result<Self, InvalidUtf8> {
                let length = Compare(length).min(CAP as $t);
                let ulen = length as usize;
                let start = CAP - ulen;
                cfor![i in 0..ulen => {
                    bytes[i] = bytes[start + i];
                }];
                match Str::from_utf8(Slice::take_first(&bytes, ulen)) {
                    Ok(_) => Ok(Self { arr: bytes, len: length }),
                    Err(e) => Err(e),
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
            #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
            #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_str")))]
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

        impl<const CAP: usize> Default for $name<CAP> {
            /// Returns an empty string.
            ///
            /// # Panics
            #[doc = "Panics if `CAP > `[`" $t "::MAX`]."]
            fn default() -> Self { Self::new().unwrap() }
        }

        impl<const CAP: usize> Display for $name<CAP> {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
                write!(f, "{}", self.as_str())
            }
        }

        impl<const CAP: usize> Debug for $name<CAP> {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
                write!(f, "{:?}", self.as_str())
            }
        }

        impl<const CAP: usize> PartialEq<&str> for $name<CAP> {
            fn eq(&self, slice: &&str) -> bool { self.as_str() == *slice }
        }
        // and for when &str is on the left-hand side of the comparison
        impl<const CAP: usize> PartialEq<$name<CAP>> for &str {
            fn eq(&self, string: & $name<CAP>) -> bool { *self == string.as_str() }
        }

        impl<const CAP: usize> Deref for $name<CAP> {
            type Target = str;
            fn deref(&self) -> &Self::Target { self.as_str() }
        }

        impl<const CAP: usize> AsRef<str> for $name<CAP> {
            fn as_ref(&self) -> &str { self.as_str() }
        }

        impl<const CAP: usize> AsRef<[u8]> for $name<CAP> {
            fn as_ref(&self) -> &[u8] { self.as_bytes() }
        }

        impl<const CAP: usize> TryFrom<&str> for $name<CAP> {
            type Error = MismatchedCapacity;

            #[doc = "Tries to create a new `String" $t:camel "` from the given `string` slice."]
            ///
            /// # Errors
            #[doc = "Returns [`MismatchedCapacity`] if `CAP > `[`" $t "::MAX`]"]
            /// or if `CAP < string.len()`.
            fn try_from(string: &str) -> Result<Self, MismatchedCapacity> {
                if CAP < string.len() {
                    Err(MismatchedCapacity::closed(0, CAP + string.len(), CAP))
                } else {
                    let mut new_string = Self::new()?;
                    let bytes = string.as_bytes();
                    new_string.arr[..bytes.len()].copy_from_slice(bytes);
                    Ok(new_string)
                }
            }
        }

        impl<const CAP: usize> TryFrom<&[u8]> for $name<CAP> {
            type Error = InvalidText;

            #[doc = "Tries to create a new `String" $t:camel "` from the given slice of` bytes`."]
            ///
            /// # Errors
            #[doc = "Returns [`InvalidText::Capacity`] if `CAP > `[`" $t "::MAX`], or if "]
            /// `CAP < bytes.len()`, and [`InvalidText::Utf8`] if the `bytes` are not valid UTF-8.
            fn try_from(bytes: &[u8]) -> Result<Self, InvalidText> {
                if CAP < bytes.len() {
                    return Err(InvalidText::Capacity(Mismatch::in_closed_interval(
                        0,
                        bytes.len(),
                        CAP,
                        "",
                    )));
                } else {
                    match Str::from_utf8(bytes) {
                        Ok(_) => {
                            let mut arr = [0; CAP];
                            arr[..bytes.len()].copy_from_slice(bytes);
                            Ok(Self { arr, len: bytes.len() as $t })
                        },
                        Err(e) => Err(e.into()),
                    }
                }
            }
        }
    }};
}
impl_str_u!();

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn push() {
        let mut s = StringU8::<3>::new().unwrap();
        assert![s.try_push('ñ').is_ok()];
        assert_eq![2, s.len()];
        assert![s.try_push('ñ').is_err()];
        assert_eq![2, s.len()];
        assert![s.try_push('a').is_ok()];
        assert_eq![3, s.len()];
    }

    #[test]
    fn pop() {
        let mut s = StringU8::<3>::new().unwrap();
        s.push('ñ');
        s.push('a');
        assert_eq![Some('a'), s.pop()];
        assert_eq![Some('ñ'), s.pop()];
        assert_eq![None, s.pop()];
    }
}
