// devela_base_core::text::u
//
//! `String` backed by an array.
//
// TOC
// - impl_str_u!
//   - definitions
//   - trait impls
// - tests

#[allow(unused, reason = "±unsafe")]
use crate::{Cmp, cfor, unwrap};
use crate::{
    Debug, Deref, DerefMut, Display, FmtResult, Formatter, InvalidText, InvalidUtf8, IterChars,
    Mismatch, MismatchedCapacity, NotEnoughElements, Slice, Str, is, paste, text::char::*,
};

macro_rules! impl_str_u {
    () => { impl_str_u![u8, u16, u32, usize]; };
    (
    // $t: the length type. E.g.: u8.
    $($t:ty),+ $(,)?) => { $( paste! { impl_str_u![@[<String $t:camel>], $t]; } )+ };
    (
    // $name: the name of the type. E.g.: StringU8.
    @$name:ty, $t:ty) => { paste! {
        /* definitions */

        #[allow(rustdoc::broken_intra_doc_links, reason = "±unsafe")]
        #[doc = crate::_TAG_TEXT!()]
        #[doc = "A UTF-8–encoded string, backed by an array with [`" $t "::MAX`] bytes of capacity."]
        ///
        #[doc = crate::_doc!(location: "text/str")]
        ///
        #[doc = "Internally, the current length is stored as a [`" $t "`]."]
        ///
        /// ## Methods
        /// - [Constructors](#constructors):
        ///   [`new`][Self::new],
        ///     *([_checked][Self::new_checked])*,
        ///   [`from_str`][Self::from_str],
        ///     *([_truncate][Self::from_str_truncate])*,
        ///   [`from_char`][Self::from_char]
        ///   *([7][Self::from_char7],
        ///     [8][Self::from_char8],
        ///     [16][Self::from_char16])*,
        ///   [`from_bytes`][Self::from_bytes]
        ///   *([_nleft][Self::from_bytes_nleft],
        ///   [_nright][Self::from_bytes_nleft])*.
        ///
        /// - [Deconstructors](#deconstructors):
        ///   [`into_array`][Self::into_array],
        ///   [`as_array`][Self::as_array],
        ///   [`as_bytes`][Self::as_bytes]
        ///     *([_mut][Self::as_bytes_mut])*,
        ///   [`as_str`][Self::as_str]
        ///     *([_mut][Self::as_mut_str])*,
        ///   [`chars`][Self::chars].
        ///
        /// - [Queries](#queries):
        ///   [`len`][Self::len],
        ///   [`is_empty`][Self::is_empty],
        ///   [`is_full`][Self::is_full],
        ///   [`capacity`][Self::capacity],
        ///   [`remaining_capacity`][Self::remaining_capacity].
        ///
        /// - [Modifiers](#modifiers):
        ///   [`clear`][Self::clear],
        ///   [`reset`][Self::reset],
        ///   [`sanitize`][Self::sanitize],
        ///   [`pop`][Self::pop]
        ///     *([try_][Self::try_pop])*,
        ///   [`push`][Self::push]
        ///     *([try_][Self::try_push])*.
        ///   [`push_str`][Self::push]
        ///     *([try_][Self::try_push_str],
        ///     [try__complete][Self::try_push_str_complete])*.
        #[must_use]
        #[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $name<const CAP: usize> {
            arr: [u8; CAP], // WAIT: for when possible CAP:u8 for panic-less const boundary check.
            len: $t,
        }

        /// # Constructors
        impl<const CAP: usize> $name<CAP> {
            #[doc = "Creates a new empty `String" $t:camel "` with a capacity of `CAP` bytes."]
            ///
            /// # Panics
            #[doc = "Panics if `CAP > `[`" $t "::MAX`]."]
            pub const fn new() -> Self {
                assert![CAP <= $t::MAX as usize,
                    concat!["Mismatched capacity, greater than ", stringify![$t], "::MAX"]
                ];
                Self { arr: [0; CAP], len: 0 }
            }
            #[doc = "Creates a new empty `String" $t:camel "` with a capacity of `CAP` bytes."]
            ///
            /// # Errors
            #[doc = "Returns [`MismatchedCapacity`] if `CAP > `[`" $t "::MAX`]."]
            pub const fn new_checked() -> Result<Self, MismatchedCapacity> {
                if CAP <= $t::MAX as usize {
                    Ok(Self { arr: [0; CAP], len: 0 })
                } else {
                    Err(MismatchedCapacity::closed(0, <$t>::MAX as usize, CAP))
                }
            }

            /* from str */

            #[doc = "Creates a new `String" $t:camel "` from a complete `&str`."]
            ///
            /// # Errors
            #[doc = "Returns [`MismatchedCapacity`] if `CAP > `[`" $t "::MAX`]."]
            /// or if `CAP < string.len()`.
            ///
            /// It calls [`try_push_str_complete()`][Self::try_push_str_complete].
            ///
            /// # Example
            /// ```
            /// # use devela_base_core::StringU8;
            /// let s = StringU8::<13>::from_str("Hello Wørld!").unwrap();
            /// assert_eq![s.as_str(), "Hello Wørld!"];
            /// ```
            pub const fn from_str(string: &str) -> Result<Self, MismatchedCapacity> {
                let mut new_string = unwrap![ok? Self::new_checked()];
                if let Ok(_) = new_string.try_push_str_complete(string) { Ok(new_string) }
                else { Err(MismatchedCapacity::closed(0, string.len(), CAP)) }
            }

            #[doc = "Creates a new `String" $t:camel "` from a `&str`,"]
            /// truncating if it does not fit.
            ///
            #[doc = "Returns [`MismatchedCapacity`] if `CAP > `[`" $t "::MAX`]"]
            ///
            /// It calls [`try_push_str_complete()`][Self::try_push_str_complete].
            pub const fn from_str_truncate(string: &str) -> Result<Self, MismatchedCapacity> {
                let mut new_string = unwrap![ok? Self::new_checked()];
                let _ = new_string.push_str(string);
                Ok(new_string)
            }

            /* from char */

            #[doc = "Creates a new `String" $t:camel "` from a `char`."]
            ///
            /// # Errors
            #[doc = "Returns [`MismatchedCapacity`] if `CAP > `[`" $t "::MAX`]."]
            /// or if `CAP < c.`[`len_utf8()`][crate::UnicodeScalar#method.len_utf8].
            ///
            #[doc = "It will always succeed if `CAP >= 4 && CAP <= `[`" $t "::MAX`]."]
            pub const fn from_char(c: char) -> Result<Self, MismatchedCapacity> {
                let mut new = unwrap![ok? Self::new_checked()];
                let bytes = Char(c).to_utf8_bytes();
                new.len = Char(bytes[0]).utf8_len_unchecked() as $t;
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
                let mut new = unwrap![ok? Self::new_checked()];
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
                let mut new = unwrap![ok? Self::new_checked()];
                let bytes = c.to_utf8_bytes();
                new.len = Char(bytes[0]).utf8_len_unchecked() as $t;
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
                let mut new = unwrap![ok? Self::new_checked()];
                let bytes = c.to_utf8_bytes();
                new.len = Char(bytes[0]).utf8_len_unchecked() as $t;
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
                    Ok(_) => { Ok(Self { arr: bytes, len: CAP as $t }) },
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
                let length = Cmp(length).min(CAP as $t);
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
                Self { arr: bytes, len: Cmp(length).min(CAP as $t) }
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
                let length = Cmp(length).min(CAP as $t);
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
                let length = Cmp(length).min(CAP as $t);
                let ulen = length as usize;
                let start = CAP - ulen;
                cfor![i in 0..ulen => {
                    bytes[i] = bytes[start + i];
                }];
                Self { arr: bytes, len: length }
            }
        }

        /// # Deconstructors
        impl<const CAP: usize> $name<CAP> {
            /// Returns the inner array with the full contents.
            ///
            /// The array contains all the bytes, including those outside the current length.
            #[must_use] #[inline(always)]
            pub const fn into_array(self) -> [u8; CAP] { self.arr }

            /// Returns a copy of the inner array with the full contents.
            ///
            /// The array contains all the bytes, including those outside the current length.
            #[must_use] #[inline(always)]
            pub const fn as_array(&self) -> [u8; CAP] { self.arr }

            /// Returns a byte slice of the inner string slice.
            ///
            /// # Features
            /// Makes use of the `unsafe_slice` feature if enabled.
            #[must_use] #[inline(always)]
            pub const fn as_bytes(&self) -> &[u8] {
                #[cfg(any(base_safe_text, not(feature = "unsafe_slice")))]
                return Slice::take_first(&self.arr, self.len as usize);

                #[cfg(all(not(base_safe_text), feature = "unsafe_slice"))]
                // SAFETY: we ensure to contain a correct length
                unsafe { Slice::take_first_unchecked(&self.arr, self.len as usize) }
            }

            /// Returns an exclusive byte slice of the inner string slice.
            ///
            /// # Features
            /// Makes use of the `unsafe_slice` feature if enabled.
            #[must_use] #[inline(always)]
            pub const fn as_bytes_mut(&mut self) -> &mut [u8] {
                #[cfg(any(base_safe_text, not(feature = "unsafe_slice")))]
                return Slice::take_first_mut(&mut self.arr, self.len as usize);

                #[cfg(all(not(base_safe_text), feature = "unsafe_slice"))]
                // SAFETY: we ensure to contain a correct length
                unsafe { Slice::take_first_mut_unchecked(&mut self.arr, self.len as usize) }
            }

            /// Returns a reference to the inner string slice.
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

            /// Returns an exclusive reference to the inner string slice.
            ///
            /// # Features
            /// Makes use of the `unsafe_str` feature if enabled.
            #[must_use] #[inline(always)]
            pub const fn as_mut_str(&mut self) -> &mut str {
                #[cfg(any(base_safe_text, not(feature = "unsafe_str")))]
                return unwrap![ok_expect Str::from_utf8_mut(self.as_bytes_mut()), "Invalid UTF-8"];

                #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
                // SAFETY: we ensure to contain only valid UTF-8
                unsafe { Str::from_utf8_unchecked_mut(self.as_bytes_mut()) }
            }

            /// Returns an iterator over the `chars` of this grapheme cluster.
            #[inline(always)]
            pub fn chars(&self) -> IterChars<'_> { self.as_str().chars() } // non-const
        }

        /// # Queries
        impl<const CAP: usize> $name<CAP> {
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
        }

        /// # Modifiers
        impl<const CAP: usize> $name<CAP> {
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
                self.as_str().chars().last().map(|c| { self.len -= c.len_utf8() as $t; c })
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
            /// Returns 0 bytes if the given `character` doesn't fit in the remaining capacity.
            pub const fn push(&mut self, character: char) -> usize {
                let char_len = character.len_utf8();
                if self.remaining_capacity() >= char_len {
                    let beg = self.len as usize;
                    let end = beg + char_len;
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
                    let _ = character.encode_utf8(Slice::range_mut(&mut self.arr, beg, end));
                    self.len += char_len as $t;
                    Ok(char_len)
                } else {
                    Err(MismatchedCapacity::closed(0, self.len() + character.len_utf8(), CAP))
                }
            }

            /// Appends as many complete characters from `string` as will fit.
            ///
            /// Returns the number of bytes written. UTF-8 characters are never split.
            ///
            /// # Example
            /// ```
            /// # use devela_base_core::StringU8;
            /// let mut s = StringU8::<5>::new();
            /// assert_eq!(s.push_str("café"), 5);
            /// assert_eq!(s, "café");
            ///
            /// let mut s = StringU8::<4>::new();
            /// assert_eq!(s.push_str("café"), 3);
            /// assert_eq!(s, "caf");
            ///
            /// let mut s = StringU8::<2>::new();
            /// assert_eq!(s.push_str("サ"), 0);
            /// assert_eq!(s, "");
            /// ```
            pub const fn push_str(&mut self, string: &str) -> usize {
                let remaining_capacity = CAP - self.len as usize;
                is! { remaining_capacity == 0; return 0 }
                let string_len = string.len();
                let bytes_to_write = if string_len <= remaining_capacity {
                    string_len
                } else {
                    let mut amount = remaining_capacity;
                    while amount > 0 && !string.is_char_boundary(amount) { amount -= 1; }
                    amount
                };
                if bytes_to_write > 0 {
                    let start_pos = self.len as usize;
                    Slice::range_mut(&mut self.arr, start_pos, start_pos + bytes_to_write)
                        .copy_from_slice(Slice::range_to(string.as_bytes(), bytes_to_write));
                    self.len += bytes_to_write as $t;
                    bytes_to_write
                } else {
                    0
                }
            }

            /// Appends characters from `string`, returning `Ok` if all fit, `Err` if partial.
            ///
            /// - `Ok(bytes)`: Entire string was written successfully
            /// - `Err(partial)`: Only `partial` bytes could be written (UTF-8 safe)
            ///
            /// In both cases, the bytes are appended to the buffer.
            pub const fn try_push_str(&mut self, string: &str) -> Result<usize, usize> {
                let bytes_written = self.push_str(string);
                is![bytes_written == string.len(); Ok(bytes_written); Err(bytes_written)]
            }

            /// Appends the entire `string` or nothing at all.
            ///
            /// Returns `Ok(bytes)` if the string fits completely, or `Err(0)` if it doesn't.
            /// No partial writes will occur to the buffer.
            pub const fn try_push_str_complete(&mut self, string: &str) -> Result<usize, usize> {
                is![self.remaining_capacity() >= string.len(); Ok(self.push_str(string)); Err(0)]
            }
        }

        /* utility traits */

        impl<const CAP: usize> Default for $name<CAP> {
            /// Returns an empty string.
            ///
            /// # Panics
            #[doc = "Panics if `CAP > `[`" $t "::MAX`]."]
            fn default() -> Self { Self::new() }
        }

        impl<const CAP: usize> Display for $name<CAP> {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
                f.write_str(self.as_str())
            }
        }

        impl<const CAP: usize> Debug for $name<CAP> {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
                write!(f, "{:?}", self.as_str())
            }
        }

        impl<const CAP: usize> PartialEq<&str> for $name<CAP> { // &str on the RHS
            fn eq(&self, slice: &&str) -> bool { self.as_str() == *slice }
        }
        impl<const CAP: usize> PartialEq<$name<CAP>> for &str { // &str on the LHS
            fn eq(&self, string: & $name<CAP>) -> bool { *self == string.as_str() }
        }

        impl<const CAP: usize> Deref for $name<CAP> {
            type Target = str;
            fn deref(&self) -> &Self::Target { self.as_str() }
        }
        impl<const CAP: usize> DerefMut for $name<CAP> {
            fn deref_mut(&mut self) -> &mut str { self.as_mut_str() }
        }

        impl<const CAP: usize> AsRef<str> for $name<CAP> {
            fn as_ref(&self) -> &str { self.as_str() }
        }
        impl<const CAP: usize> AsMut<str> for $name<CAP> {
            fn as_mut(&mut self) -> &mut str { self.as_mut_str() }
        }

        impl<const CAP: usize> AsRef<[u8]> for $name<CAP> {
            fn as_ref(&self) -> &[u8] { self.as_bytes() }
        }

        /* conversions */

        impl<const CAP: usize> TryFrom<&str> for $name<CAP> {
            type Error = MismatchedCapacity;

            #[doc = "Tries to create a new `String" $t:camel "` from the given `string` slice."]
            ///
            /// It calls [`from_str()`][Self::from_str].
            /// # Errors
            #[doc = "Returns [`MismatchedCapacity`] if `CAP > `[`" $t "::MAX`]"]
            /// or if `CAP < string.len()`.
            fn try_from(string: &str) -> Result<Self, MismatchedCapacity> { Self::from_str(string) }
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
                    Err(InvalidText::Capacity(Mismatch::in_closed_interval(0, bytes.len(), CAP, "")))
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

        /* Extend & FromIterator */

        impl<const CAP: usize> Extend<char> for $name<CAP> {
            /// Creates an instance from an iterator of characters.
            ///
            /// Processes characters until it can fit no more, discarding the rest.
            ///
            /// # Panics
            #[doc = "Panics if `CAP > `[`" $t "::MAX`]."]
            ///
            /// # Example
            /// ```
            /// # use devela_base_core::StringU8;
            /// let chars = ['a', 'b', 'c', '€', 'さ'];
            /// let mut s = StringU8::<6>::new();
            /// s.extend(chars);
            /// assert_eq![s, "abc€"];
            /// ```
            fn extend<I: IntoIterator<Item=char>>(&mut self, iter: I) {
                for i in iter { is![self.push(i) == 0; break]; }
            }
        }
        impl<const CAP: usize> FromIterator<char> for $name<CAP> {
            /// Creates an instance from an iterator of characters.
            ///
            /// Processes characters until it can fit no more, discarding the rest.
            ///
            /// # Panics
            #[doc = "Panics if `CAP > `[`" $t "::MAX`]."]
            ///
            /// # Example
            /// ```
            /// # use devela_base_core::StringU8;
            /// let chars = ['a', 'b', 'c', '€', 'さ'];
            /// assert_eq!(StringU8::<9>::from_iter(chars), "abc€さ");
            /// assert_eq!(StringU8::<6>::from_iter(chars), "abc€");
            /// assert_eq!(StringU8::<5>::from_iter(chars), "abc");
            /// assert_eq!(StringU8::<2>::from_iter(chars), "ab");
            /// assert_eq!(StringU8::<0>::from_iter(chars), "");
            /// ```
            fn from_iter<I: IntoIterator<Item=char>>(iter: I) -> Self {
                let mut string = $name::new();
                string.extend(iter);
                string
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
        let mut s = StringU8::<3>::new();
        assert![s.try_push('ñ').is_ok()];
        assert_eq![2, s.len()];
        assert![s.try_push('ñ').is_err()];
        assert_eq![2, s.len()];
        assert![s.try_push('a').is_ok()];
        assert_eq![3, s.len()];
    }
    #[test]
    fn push_str() {
        let mut s = StringU8::<5>::new();
        assert_eq!(s.push_str("café"), 5);

        let mut s = StringU8::<4>::new();
        assert_eq!(s.push_str("café"), 3);
        assert_eq!(s, "caf")
    }
    #[test]
    fn try_push_str() {
        let mut s = StringU8::<5>::new();
        assert_eq!(s.try_push_str("café"), Ok(5));

        let mut s = StringU8::<4>::new();
        assert_eq!(s.try_push_str("café"), Err(3));
        assert_eq!(s, "caf")
    }
    #[test]
    fn try_push_str_complete() {
        let mut s = StringU8::<5>::new();
        assert_eq!(s.try_push_str_complete("café"), Ok(5));

        let mut s = StringU8::<4>::new();
        assert_eq!(s.try_push_str_complete("café"), Err(0));
        assert_eq!(s, "")
    }

    #[test]
    fn pop() {
        let mut s = StringU8::<3>::new();
        s.push('ñ');
        s.push('a');
        assert_eq![Some('a'), s.pop()];
        assert_eq![Some('ñ'), s.pop()];
        assert_eq![None, s.pop()];
    }
}
