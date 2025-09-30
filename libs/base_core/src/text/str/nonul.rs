// devela_base_core::text::nonul
//
//! Non-nul `String` backed by an array.
//
// TOC
// - definitions
// - trait impls

use crate::{
    Char, CharIter, Debug, Deref, Display, FmtResult, Formatter, InvalidText, Mismatch,
    MismatchedCapacity, NotEnoughElements, Str, cfor, char_utf8, char7, char8, char16, is, slice,
    unwrap,
};

/* definitions */

/// The nul character.
const NUL_CHAR: char = '\0';

#[doc = crate::_TAG_TEXT!()]
/// A UTF-8 string with up to [`u8::MAX`] bytes, excluding nul chars
///
#[doc = crate::_doc!(location: "text/str")]
///
/// Internally, the first 0 byte in the array indicates the end of the string.
///
/// ## Methods
///
/// - [Constructors](#constructors):
///   [`new`][Self::new],
///   [`from_str`][Self::from_str],
///     *([_truncate][Self::from_str_truncate],
///       [_unchecked][Self::from_str_unchecked])*,
///   [`from_char`][Self::from_char]
///     *([`7`](Self::from_char7),
///       [`8`](Self::from_char8),
///       [`16`](Self::from_char16)),
///       [`utf8`](Self::from_char_utf8))*.
///   [`from_byte_array`][Self::from_byte_array],
///    *([_unchecked][Self::from_str_unchecked])*.
///
/// - [Deconstructors](#deconstructors):
///   [`into_array`][Self::into_array],
///   [`as_array`][Self::as_array],
///   [`as_bytes`][Self::as_bytes]
#[cfg_attr(
    feature = "unsafe_str",
    doc = "*([mut][Self::as_bytes_mut]<sup title='unsafe function'>⚠</sup>)*."
)]
///   [`as_str`][Self::as_str]
#[cfg_attr(
    feature = "unsafe_str",
    doc = "*([mut][Self::as_mut_str]<sup title='unsafe function'>⚠</sup>)*."
)]
///   [`chars`][Self::chars],
///
/// - [Queries](#queries):
///   [`len`][Self::len],
///   [`is_empty`][Self::is_empty],
///   [`is_full`][Self::is_full],
///   [`capacity`][Self::capacity],
///   [`remaining_capacity`][Self::remaining_capacity].
///
/// - Operations:
///   [`clear`][Self::clear],
///   [`pop`][Self::pop]*([try][Self::try_pop])*,
///   [`push`][Self::push]*([try][Self::try_push])*.
///   [`push_str`][Self::push]*([try][Self::try_push_str])*,
///   [`try_push_str_complete`][Self::try_push_str_complete].
#[must_use]
#[derive(Clone, Copy, Hash, Eq, PartialOrd, Ord)]
pub struct StringNonul<const CAP: usize> {
    arr: [u8; CAP],
}

#[rustfmt::skip]
impl<const CAP: usize> StringNonul<CAP> {
    /// Creates a new empty `StringNonul`.
    ///
    /// # Panics
    /// Panics if `CAP` > [`u8::MAX`].
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::StringNonul;
    /// let mut s = StringNonul::<10>::new();
    /// assert_eq![size_of_val(&s), 10];
    /// ```
    pub const fn new() -> Self {
        assert![CAP <= u8::MAX as usize, "Mismatched capacity, greater than u8::MAX"];
        Self { arr: [0; CAP] }
    }

    /// Creates a new empty `StringNonul`.
    ///
    /// # Errors
    /// Returns [`MismatchedCapacity`] if `CAP` > [`u8::MAX`].
    pub const fn new_checked() -> Result<Self, MismatchedCapacity> {
        if CAP <= u8::MAX as usize {
            Ok(Self { arr: [0; CAP] })
        } else {
            Err(MismatchedCapacity::closed(0, u8::MAX as usize, CAP))
        }
    }

    /// Creates a new `StringNonul` from a complete `&str`.
    ///
    /// # Errors
    /// Returns [`MismatchedCapacity`] if `CAP` > [`u8::MAX`] or if `CAP < string.len()`.
    ///
    /// This is implemented via `Self::`[`try_push_str_complete()`][Self::try_push_str_complete].
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

    /// Creates a new `StringNonul` from a `&str`, truncating if it does not fit.
    ///
    /// Returns [`MismatchedCapacity`] if `CAP` > [`u8::MAX`].
    ///
    /// This is implemented via `Self::`[`push_str()`][Self::push_str].
    pub const fn from_str_truncate(string: &str) -> Result<Self, MismatchedCapacity> {
        let mut new_string = unwrap![ok? Self::new_checked()];
        let _ = new_string.push_str(string);
        Ok(new_string)
    }

    /// Creates a new `StringNonul` from a `&str`, truncating if it does not fit.
    ///
    /// # Panics
    /// Panics if `CAP` > [`u8::MAX`].
    ///
    /// This is implemented via `Self::`[`push_str()`][Self::push_str].
    #[inline(always)]
    pub const fn from_str_unchecked(string: &str) -> Self {
        let mut new_string = Self::new();
        let _ = new_string.push_str(string);
        new_string
    }

    /* queries */

    /// Returns the total capacity in bytes.
    #[must_use] #[inline(always)]
    pub const fn capacity() -> usize { CAP }

    /// Returns the remaining capacity.
    #[must_use] #[inline(always)]
    pub const fn remaining_capacity(&self) -> usize { CAP - self.len() }

    /// Returns the current length.
    ///
    /// # Examples
    /// ```
    /// # use devela_base_core::{StringNonul, MismatchedCapacity};
    /// # fn main() -> Result<(), MismatchedCapacity> {
    /// let mut s = StringNonul::<4>::new_checked()?;
    /// assert_eq![0, s.len()];
    ///
    /// assert_eq![1, s.push('a')];
    /// assert_eq![1, s.len()];
    ///
    /// assert_eq![3, s.push('€')];
    /// assert_eq![4, s.len()];
    /// # Ok(()) }
    /// ```
    #[must_use]
    pub const fn len(&self) -> usize {
        let mut position = 0;
        while position < CAP {
            is![self.arr[position] == 0; break];
            position += 1;
        }
        position
    }

    /// Returns `true` if the current length is 0.
    #[must_use] #[inline(always)]
    pub const fn is_empty(&self) -> bool { self.len() == 0 }

    /// Returns `true` if the current remaining capacity is 0.
    #[must_use] #[inline(always)]
    pub const fn is_full(&self) -> bool { self.len() == CAP }

    /// Checks the equality of two strings, with the same capacity and length.
    ///
    /// It only checks the first `self.len()` bytes.
    /// # Example
    /// ```
    /// # use devela_base_core::StringNonul;
    /// let mut a = StringNonul::<16>::from_str_unchecked("hello world!");
    /// let mut b = StringNonul::<16>::from_str_unchecked("hello world!!!");
    /// assert![!a.eq(&b)];
    /// b.pop();
    /// b.pop();
    /// assert![a.eq(&b)];
    /// ```
    #[must_use] #[inline(always)]
    pub const fn eq(&self, other: &Self) -> bool {
        let mut i = 0;
        while i < CAP {
            if self.arr[i] != other.arr[i] { return false; }
            if self.arr[i] == 0 { return true; }
            i += 1;
        }
        true
    }

    /* deconstruct */

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
    pub const fn as_bytes(&self) -> &[u8] { self.arr.split_at(self.len()).0 }

    /// Returns a mutable byte slice of the inner string slice.
    ///
    /// # Safety
    /// The caller must ensure that the content of the slice is valid UTF-8
    /// and that it doesn't contain any `NUL` characters before the borrow
    /// ends and the underlying `str` is used.
    ///
    /// # Features
    /// Makes use of the `unsafe_slice` feature if enabled.
    #[must_use] #[inline(always)]
    #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_str")))]
    pub const unsafe fn as_bytes_mut(&mut self) -> &mut [u8] {
        let len = self.len();

        #[cfg(any(base_safe_text, not(feature = "unsafe_slice")))]
        return slice![mut &mut self.arr, ..len];

        #[cfg(all(not(base_safe_text), feature = "unsafe_slice"))]
        // SAFETY: we ensure to uphold a valid length
        unsafe { slice![mut_unchecked &mut self.arr, ..len] }
    }

    /// Returns the inner string slice.
    /// # Features
    /// Makes use of the `unsafe_slice` feature if enabled.
    #[must_use]
    pub const fn as_str(&self) -> &str {
        #[cfg(any(base_safe_text, not(feature = "unsafe_slice")))]
        return unwrap![ok_expect Str::from_utf8(self.as_bytes()), "Invalid UTF-8"];

        #[cfg(all(not(base_safe_text), feature = "unsafe_slice"))]
        // SAFETY: we ensure to contain only valid UTF-8
        unsafe { Str::from_utf8_unchecked(self.as_bytes()) }
    }

    /// Returns the mutable inner string slice.
    ///
    /// # Safety
    /// The caller must ensure that the content of the slice is valid UTF-8
    /// and that it doesn't contain any `NUL` characters before the borrow
    /// ends and the underlying `str` is used.
    #[must_use]
    #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_str")))]
    pub const unsafe fn as_mut_str(&mut self) -> &mut str {
        // SAFETY: we ensure to contain only valid UTF-8
        unsafe { Str::from_utf8_unchecked_mut(self.as_bytes_mut()) }
    }

    /// Returns an iterator over the `chars` of the string.
    #[inline(always)]
    pub const fn chars(&self) -> CharIter<'_, &str> { CharIter::<&str>::new(self.as_str()) }

    /* operations */

    /// Sets the length to 0, by resetting all bytes to 0.
    pub const fn clear(&mut self) { self.arr = [0; CAP]; }

    /// Removes the last character and returns it, or `None` if the string is empty.
    #[must_use]
    pub const fn pop(&mut self) -> Option<char> {
        if self.is_empty() { None } else { Some(self.pop_unchecked()) }
    }

    /// Tries to remove the last character and return it.
    ///
    /// # Errors
    /// Returns [`NotEnoughElements`] if the string is empty.
    pub const fn try_pop(&mut self) -> Result<char, NotEnoughElements> {
        if self.is_empty() {
            Err(NotEnoughElements(Some(1)))
        } else {
            Ok(self.pop_unchecked())
        }
    }

    /// Removes the last character and returns it.
    ///
    /// # Panics
    /// Panics if the string is empty.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::StringNonul;
    /// let mut s = StringNonul::<16>::new();
    /// s.push_str("hello worlð!");
    /// assert_eq![s.len(), 13];
    ///
    /// assert_eq![s.pop_unchecked(), '!'];
    /// assert_eq![s.len(), 12];
    ///
    /// assert_eq![s.pop_unchecked(), 'ð'];
    /// assert_eq![s.len(), 10];
    /// ```
    #[must_use] #[rustfmt::skip]
    pub const fn pop_unchecked(&mut self) -> char {
        let (len, string) = (self.len(), self.as_str());
        let mut idx_last_char = len - 1;
        while idx_last_char > 0 && !string.is_char_boundary(idx_last_char) { idx_last_char -= 1; }

        let range = Str::range(string, idx_last_char, len);
        let last_char = unwrap![some CharIter::<&str>::new(range).next_char()];

        let mut i = idx_last_char; while i < len { self.arr[i] = 0; i += 1; } // clean char bytes
        last_char
    }

    /// Appends to the end of the string the given `character`.
    ///
    /// Returns the number of bytes written.
    ///
    /// It will return 0 bytes if the given `character` doesn't fit in
    /// the remaining capacity, or if it is the nul character.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::StringNonul;
    /// let mut s = StringNonul::<16>::new();
    /// s.push('h');
    /// assert_eq![s.len(), 1];
    ///
    /// s.push('€');
    /// assert_eq![s.len(), 4];
    ///
    /// assert_eq![s.as_str(), "h€"];
    /// ```
    pub const fn push(&mut self, character: char) -> usize {
        let char_len = character.len_utf8();
        if character != NUL_CHAR && self.remaining_capacity() >= char_len {
            let len = self.len();
            let _ = character.encode_utf8(slice![mut &mut self.arr, len, ..len + char_len]);
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
    /// Returns [`MismatchedCapacity`]
    /// if the capacity is not enough to hold the given character.
    pub const fn try_push(&mut self, character: char) -> Result<usize, MismatchedCapacity> {
        let char_len = character.len_utf8();
        if character == NUL_CHAR {
            Ok(0)
        } else if self.remaining_capacity() >= char_len {
            let len = self.len();
            let _ = character.encode_utf8(slice![mut &mut self.arr, len, ..len + char_len]);
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
    ///
    /// # Features
    /// Uses the `unsafe_str` feature to skip validation checks.
    pub const fn push_str(&mut self, string: &str) -> usize {
        let mut rem_cap = self.remaining_capacity();
        let mut bytes_written = 0;
        let mut chars = CharIter::<&str>::new(string);
        while let Some(character) = chars.next_char() {
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

    /// Tries to append to the end the fitting characters from the given `string` slice.
    ///
    /// Nul characters will be stripped out.
    ///
    /// Returns the number of bytes written.
    ///
    /// # Errors
    /// Returns [`MismatchedCapacity`] if the capacity is not enough
    /// to hold not even the first non-nul character.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::StringNonul;
    /// let mut s = StringNonul::<8>::new();
    /// s.push_str("hello");
    ///
    /// // successfully appends a string while removing NUL characters
    /// let result = s.try_push_str("\0\0\0\0\0 world");
    /// assert!(result.is_ok());
    /// assert_eq!(s.as_str(), "hello wo"); // truncates if it can't fit all
    /// assert!(s.try_push_str("!").is_err()); // fails if it can't fit 1 char
    ///
    /// // Insufficient capacity for the first non-NUL character
    /// let mut small = StringNonul::<3>::new();
    /// assert!(small.try_push_str("🚀").is_err()); // Needs 4 bytes for the rocket
    /// ```
    /// # Features
    /// Uses the `unsafe_str` feature to skip validation checks.
    pub const fn try_push_str(&mut self, string: &str) -> Result<usize, MismatchedCapacity> {
        let mut first_char_len = 0;
        let mut chars = CharIter::<&str>::new(string);
        while let Some(c) = chars.next_scalar() { // find the first non-zero length character:
            if c != NUL_CHAR as u32 { first_char_len = Char(c).len_utf8_unchecked(); break; }
        }
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
    /// Nul characters will not be taken into account.
    ///
    /// # Errors
    /// Returns [`MismatchedCapacity`] if the slice wont completely fit.
    ///
    /// # Features
    /// Uses the `unsafe_str` feature to skip validation checks.
    pub const fn try_push_str_complete(&mut self, string: &str) -> Result<usize, MismatchedCapacity> {
        let (mut non_nul_len, bytes, mut i) = (0, string.as_bytes(), 0);
        while i < bytes.len() { is![bytes[i] != 0; non_nul_len += 1]; i += 1; } // count !0 bytes
        if self.remaining_capacity() >= non_nul_len {
            Ok(self.push_str(string))
        } else {
            Err(MismatchedCapacity::closed(0, self.len() + string.len(), CAP))
        }
    }

    /* from char */

    /// Creates a new `StringNonul` from a `char`.
    ///
    /// If `c` is NUL an empty string will be returned.
    ///
    /// # Errors
    /// Returns [`MismatchedCapacity`] if `CAP` > [`u8::MAX`],
    /// or  if `!c.is_nul()` and `CAP` < `c.`[`len_utf8()`][Char::len_utf8].
    ///
    /// Will always succeed if `CAP` >= 4.
    pub const fn from_char(c: char) -> Result<Self, MismatchedCapacity> {
        let mut new = unwrap![ok? Self::new_checked()];
        if c != '\0' {
            let bytes = Char(c).to_utf8_bytes();
            let len = Char(bytes[0]).len_utf8_unchecked();
            is![CAP < len; return Err(MismatchedCapacity::closed(0, len, CAP))];
            new.arr[0] = bytes[0];
            if len > 1 { new.arr[1] = bytes[1]; }
            if len > 2 { new.arr[2] = bytes[2]; }
            if len > 3 { new.arr[3] = bytes[3]; }
        }
        Ok(new)
    }

    /// Creates a new `StringNonul` from a `char7`.
    ///
    /// If `c`.[`is_nul()`][char7#method.is_nul] an empty string will be returned.
    ///
    /// # Errors
    /// Returns [`MismatchedCapacity`] if `CAP` > [`u8::MAX`],
    /// or if `!c.is_nul()` and `CAP` < 1.
    ///
    /// Will always succeed if `CAP` >= 1.
    pub const fn from_char7(c: char7) -> Result<Self, MismatchedCapacity> {
        let mut new = unwrap![ok? Self::new_checked()];
        if !c.is_nul() {
            new.arr[0] = c.to_utf8_bytes()[0];
        }
        Ok(new)
    }

    /// Creates a new `StringNonul` from a `char8`.
    ///
    /// If `c`.[`is_nul()`][char8#method.is_nul] an empty string will be returned.
    ///
    /// # Errors
    /// Returns [`MismatchedCapacity`] if `CAP` > [`u8::MAX`],
    /// or if `!c.is_nul()` and `CAP` < `c.`[`len_utf8()`][char8#method.len_utf8].
    ///
    /// Will always succeed if `CAP` >= 2.
    pub const fn from_char8(c: char8) -> Result<Self, MismatchedCapacity> {
        let mut new = unwrap![ok? Self::new_checked()];
        if !c.is_nul() {
            let bytes = c.to_utf8_bytes();
            let len = Char(bytes[0]).len_utf8_unchecked();
            is![CAP < len; return Err(MismatchedCapacity::closed(0, len, CAP))];
            new.arr[0] = bytes[0];
            if len > 1 { new.arr[1] = bytes[1]; }
        }
        Ok(new)
    }

    /// Creates a new `StringNonul` from a `char16`.
    ///
    /// If `c`.[`is_nul()`][char16#method.is_nul] an empty string will be returned.
    ///
    /// # Errors
    /// Returns [`MismatchedCapacity`] if `CAP` > [`u8::MAX`],
    /// or if `!c.is_nul()` and `CAP` < `c.`[`len_utf8()`][char16#method.len_utf8].
    ///
    /// Will always succeed if `CAP` >= 3.
    pub const fn from_char16(c: char16) -> Result<Self, MismatchedCapacity> {
        let mut new = unwrap![ok? Self::new_checked()];
        if !c.is_nul() {
            let bytes = c.to_utf8_bytes();
            let len = Char(bytes[0]).len_utf8_unchecked();
            is![CAP < len; return Err(MismatchedCapacity::closed(0, len, CAP))];
            new.arr[0] = bytes[0];
            if len > 1 { new.arr[1] = bytes[1]; }
            if len > 2 { new.arr[2] = bytes[2]; }
        }
        Ok(new)
    }

    /// Creates a new `StringNonul` from a `char_utf8`.
    ///
    /// If `c`.[`is_nul()`][char_utf8#method.is_nul] an empty string will be returned.
    ///
    /// # Errors
    /// Returns [`MismatchedCapacity`] if `CAP` > [`u8::MAX`],
    /// or if `CAP` < `c.`[`len_utf8()`][char_utf8#method.len_utf8].
    ///
    /// Will always succeed if `CAP` >= 4.
    /// # Example
    /// ```
    /// # use devela_base_core::{StringNonul, char_utf8};
    /// let s = StringNonul::<3>::from_char_utf8(char_utf8::from_char('€')).unwrap();
    /// assert_eq![s.as_str(), "€"];
    ///
    /// assert![StringNonul::<2>::from_char_utf8(char_utf8::from_char('€')).is_err()];
    /// ```
    pub const fn from_char_utf8(c: char_utf8) -> Result<Self, MismatchedCapacity> {
        let mut new = unwrap![ok? Self::new_checked()];
        if !c.is_nul() {
            let len = c.len_utf8();
            if len <= CAP {
                let bytes = c.to_utf8_bytes();
                slice![mut &mut new.arr, 0,..len].copy_from_slice(slice![&bytes, 0,..len]);
            } else {
                return Err(MismatchedCapacity::closed(len, len, CAP));
            }
        }
        Ok(new)
    }

    /// Creates a new `StringNonul` from a `char_utf8`.
    ///
    /// If `c`.[`is_nul()`][char_utf8#method.is_nul] an empty string will be returned.
    ///
    /// # Panics
    /// Panics if `CAP` > [`u8::MAX`],
    /// or if `CAP` < `c.`[`len_utf8()`][char_utf8#method.len_utf8].
    ///
    /// Will always succeed if `CAP` >= 4.
    /// # Examples
    /// ```
    /// # use devela_base_core::{StringNonul, char_utf8};
    /// let s = StringNonul::<3>::from_char_utf8_unchecked(char_utf8::from_char('€'));
    /// assert_eq![s, "€"]
    /// ```
    /// ```should_panic
    /// # use devela_base_core::{StringNonul, char_utf8};
    /// StringNonul::<2>::from_char_utf8_unchecked(char_utf8::from_char('€'));
    /// ```
    pub const fn from_char_utf8_unchecked(c: char_utf8) -> Self {
        let mut new = Self::new();
        if !c.is_nul() {
            let len = c.len_utf8();
            let bytes = c.to_utf8_bytes();
            slice![mut &mut new.arr, 0,..len].copy_from_slice(slice![&bytes, 0,..len]);
        }
        new
    }

    /* from_byte_array* conversions */

    /// Returns a string from an array of `bytes`.
    ///
    /// # Errors
    /// Returns [`InvalidText::Utf8`] if the bytes are not valid UTF-8,
    /// and [`InvalidText::Char`] if the bytes contains a NUL character.
    pub const fn from_byte_array(bytes: [u8; CAP]) -> Result<Self, InvalidText> {
        // IMPROVE: use Str
        match ::core::str::from_utf8(&bytes) {
            Ok(_) => {
                cfor![index in 0..CAP => {
                    is![bytes[index] == 0; return Err(InvalidText::Char('\0'))];
                }];
                Ok(Self { arr: bytes })
            }
            Err(e) => Err(InvalidText::from_utf8_error(e)),
        }
    }

    /// Returns a string from an array of `bytes` that must be valid UTF-8.
    ///
    /// # Safety
    /// The caller must ensure that the content of the slice is valid UTF-8,
    /// and that it doesn't contain nul characters.
    ///
    /// Use of a `str` whose contents are not valid UTF-8 is undefined behavior.
    #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_str")))]
    pub const unsafe fn from_byte_array_unchecked(bytes: [u8; CAP]) -> Self {
        Self { arr: bytes }
    }
}

/* traits implementations */

impl<const CAP: usize> Default for StringNonul<CAP> {
    /// Returns an empty string.
    ///
    /// # Panics
    /// Panics if `CAP > [`u8::MAX`]`.
    fn default() -> Self {
        Self::new()
    }
}

impl<const CAP: usize> Display for StringNonul<CAP> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
        write!(f, "{}", self.as_str())
    }
}
impl<const CAP: usize> Debug for StringNonul<CAP> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
        write!(f, "{:?}", self.as_str())
    }
}

#[rustfmt::skip]
impl<const CAP: usize> PartialEq for StringNonul<CAP> {
    fn eq(&self, other: &Self) -> bool { self.eq(&other) }
}

#[rustfmt::skip]
impl<const CAP: usize> PartialEq<&str> for StringNonul<CAP> { // &str on the RHS
    fn eq(&self, slice: &&str) -> bool { self.as_str() == *slice }
}
#[rustfmt::skip]
impl<const CAP: usize> PartialEq<StringNonul<CAP>> for &str { // &str on the LHS
    fn eq(&self, string: &StringNonul<CAP>) -> bool { *self == string.as_str() }
}

impl<const CAP: usize> Deref for StringNonul<CAP> {
    type Target = str;
    #[rustfmt::skip]
    fn deref(&self) -> &Self::Target { self.as_str() }
}

impl<const CAP: usize> AsRef<str> for StringNonul<CAP> {
    #[rustfmt::skip]
    fn as_ref(&self) -> &str { self.as_str() }
}

impl<const CAP: usize> AsRef<[u8]> for StringNonul<CAP> {
    #[rustfmt::skip]
    fn as_ref(&self) -> &[u8] { self.as_bytes() }
}

impl<const CAP: usize> TryFrom<&str> for StringNonul<CAP> {
    type Error = MismatchedCapacity;

    /// Tries to create a new `StringNonul` from the given string slice.
    ///
    /// # Errors
    /// Returns [`MismatchedCapacity`] if `CAP > `[`u8::MAX`] or if `CAP < str.len()`.
    fn try_from(string: &str) -> Result<Self, MismatchedCapacity> {
        let non_nul_len = string.as_bytes().iter().filter(|x| **x != 0).count();
        if CAP < non_nul_len {
            Err(MismatchedCapacity::closed_open(0, non_nul_len, CAP))
        } else {
            let mut new_string = Self::new_checked()?;
            let copied_bytes = new_string.push_str(string);
            debug_assert_eq![non_nul_len, copied_bytes];
            Ok(new_string)
        }
    }
}

impl<const CAP: usize> TryFrom<&[u8]> for StringNonul<CAP> {
    type Error = InvalidText;

    /// Tries to create a new `StringNonul` from the given slice of `bytes`.
    ///
    /// The string will stop before the first nul character or the end of the slice.
    ///
    /// # Errors
    /// Returns [`InvalidText::Capacity`] if `CAP > `[u8::MAX`] or if `CAP < bytes.len()`
    /// or [`InvalidText::Utf8`] if the `bytes` are not valid UTF-8.
    fn try_from(bytes: &[u8]) -> Result<Self, InvalidText> {
        if bytes.len() >= CAP {
            #[rustfmt::skip]
            return Err(InvalidText::Capacity(
                Mismatch::in_closed_interval(0, bytes.len(), CAP, "")));
        }
        // IMPROVE: use Str
        match ::core::str::from_utf8(bytes) {
            Ok(_) => {
                let mut arr = [0; CAP];
                let mut idx = 0;

                for &byte in bytes.iter() {
                    if byte != 0 {
                        arr[idx] = byte;
                        idx += 1;
                    }
                }
                Ok(Self { arr })
            }
            Err(e) => Err(InvalidText::from_utf8_error(e)),
        }
    }
}

/* Extend & FromIterator */

impl<const CAP: usize> Extend<char> for StringNonul<CAP> {
    /// Creates an instance from an iterator of characters.
    ///
    /// Processes characters until it can fit no more, discarding the rest.
    ///
    /// # Panics
    /// Panics if `CAP > `[`u8::MAX`].
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::StringNonul;
    /// let chars = ['a', 'b', 'c', '€', 'さ'];
    /// let mut s = StringNonul::<6>::new();
    /// s.extend(chars);
    /// assert_eq![s, "abc€"];
    /// ```
    fn extend<I: IntoIterator<Item = char>>(&mut self, iter: I) {
        for i in iter {
            is![self.push(i) == 0; break];
        }
    }
}
impl<const CAP: usize> FromIterator<char> for StringNonul<CAP> {
    /// Creates an instance from an iterator of characters.
    ///
    /// Processes characters until it can fit no more, discarding the rest.
    ///
    /// # Panics
    /// Panics if `CAP > `[`u8::MAX`].
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::StringNonul;
    /// let chars = ['a', 'b', 'c', '€', 'さ'];
    /// assert_eq!(StringNonul::<9>::from_iter(chars), "abc€さ");
    /// assert_eq!(StringNonul::<6>::from_iter(chars), "abc€");
    /// assert_eq!(StringNonul::<5>::from_iter(chars), "abc");
    /// assert_eq!(StringNonul::<2>::from_iter(chars), "ab");
    /// assert_eq!(StringNonul::<0>::from_iter(chars), "");
    /// ```
    fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> Self {
        let mut string = StringNonul::new();
        string.extend(iter);
        string
    }
}
