// devela::text::str::small::alloc
//
//! Defines [`StringSmallAlloc`].
//

use crate::{MismatchedCapacity, NotEnoughElements, String, StringU8};

// (15 in 64-bit, 7 in 32-bit, 3 in 16 bit)
const STRING_SMALL_ALLOC_DEFAULT_CAP: usize = 2 * size_of::<usize>() - 1;

#[doc = crate::_tags!(text)]
/// A UTF-8 string with inline storage and heap spillover.
#[doc = crate::_doc_location!("text/str")]
///
/// Stores strings of up to `CAP` bytes inline,
/// and spills to the heap when more capacity is needed.
///
/// The default capacity, `23`, matches the usual inline payload target for
/// a `String`-sized compact string on 64-bit platforms.
#[must_use]
#[derive(Clone)]
pub struct StringSmallAlloc<const CAP: usize = STRING_SMALL_ALLOC_DEFAULT_CAP> {
    repr: StringSmallAllocRepr<CAP>,
}
#[derive(Clone)]
enum StringSmallAllocRepr<const CAP: usize> {
    Inline(StringU8<CAP>),
    Heap(String),
}

impl<const CAP: usize> StringSmallAlloc<CAP> {
    /* constants*/

    /// The inline byte capacity of this type.
    pub const INLINE_CAPACITY: usize = CAP;

    /// The default inline byte capacity.
    ///
    /// Chosen so the safe enum-backed `StringSmallAlloc`
    /// occupies the same stack size as `String`.
    pub const DEFAULT_CAPACITY: usize = STRING_SMALL_ALLOC_DEFAULT_CAP;

    /// Inline capacity that makes `StringU8<CAP>` the same size as `String`.
    // (23 in 64-bit, 11 in 32-bit, 5 in 16 bit)
    pub const STRING_INLINE_CAPACITY: usize = size_of::<String>() - 1;

    /// Inline capacity that usually makes this safe enum-backed type
    /// the same size as `String`.
    // (15 in 64-bit, 7 in 32-bit, 3 in 16 bit)
    pub const STRING_SIZED_CAPACITY: usize = 2 * size_of::<usize>() - 1;

    /* constructors */

    /// Creates a new empty string.
    ///
    /// # Panics
    /// Panics if `CAP > StringU8::MAX_CAPACITY`.
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            repr: StringSmallAllocRepr::Inline(StringU8::<CAP>::new()),
        }
    }
    /// Creates a new empty string.
    ///
    /// # Errors
    /// Returns [`MismatchedCapacity`] if `CAP > StringU8::MAX_CAPACITY`.
    pub fn new_checked() -> Result<Self, MismatchedCapacity> {
        Ok(Self {
            repr: StringSmallAllocRepr::Inline(StringU8::<CAP>::new_checked()?),
        })
    }

    /// Creates a new string from `string`.
    ///
    /// Uses inline storage when `string.len() <= CAP`, otherwise spills to the heap.
    ///
    /// # Errors
    /// Returns [`MismatchedCapacity`] if `CAP > StringU8::MAX_CAPACITY`.
    #[allow(clippy::should_implement_trait, reason = "it does as well")]
    pub fn from_str(string: &str) -> Result<Self, MismatchedCapacity> {
        let _ = StringU8::<CAP>::new_checked()?;
        if string.len() <= CAP {
            Ok(Self {
                repr: StringSmallAllocRepr::Inline(StringU8::<CAP>::from_str(string)?),
            })
        } else {
            Ok(Self {
                repr: StringSmallAllocRepr::Heap(String::from(string)),
            })
        }
    }

    /// Creates a new string from `string`.
    ///
    /// Reuses `string` directly when it does not fit inline.
    ///
    /// # Errors
    /// Returns [`MismatchedCapacity`] if `CAP > StringU8::MAX_CAPACITY`.
    pub fn from_string(string: String) -> Result<Self, MismatchedCapacity> {
        let _ = StringU8::<CAP>::new_checked()?;
        if string.len() <= CAP {
            Ok(Self {
                repr: StringSmallAllocRepr::Inline(StringU8::<CAP>::from_str(string.as_str())?),
            })
        } else {
            Ok(Self { repr: StringSmallAllocRepr::Heap(string) })
        }
    }

    /* views */

    /// Returns the string slice.
    #[must_use]
    #[inline(always)]
    pub fn as_str(&self) -> &str {
        match &self.repr {
            StringSmallAllocRepr::Inline(s) => s.as_str(),
            StringSmallAllocRepr::Heap(s) => s.as_str(),
        }
    }

    /// Returns the initialized bytes.
    #[must_use]
    #[inline(always)]
    pub fn as_bytes(&self) -> &[u8] {
        self.as_str().as_bytes()
    }

    /// Converts into a heap-allocated [`String`].
    #[must_use]
    pub fn into_string(self) -> String {
        match self.repr {
            StringSmallAllocRepr::Inline(s) => String::from(s.as_str()),
            StringSmallAllocRepr::Heap(s) => s,
        }
    }

    /* queries */

    /// Returns the inline capacity in bytes.
    #[must_use]
    #[inline(always)]
    pub const fn inline_capacity() -> usize {
        CAP
    }

    /// Returns the current active capacity in bytes.
    ///
    /// This is `CAP` while inline, and the heap capacity after spilling.
    #[must_use]
    #[inline(always)]
    pub fn capacity(&self) -> usize {
        match &self.repr {
            StringSmallAllocRepr::Inline(_) => CAP,
            StringSmallAllocRepr::Heap(s) => s.capacity(),
        }
    }

    /// Returns the current length in bytes.
    #[must_use]
    #[inline(always)]
    pub fn len(&self) -> usize {
        match &self.repr {
            StringSmallAllocRepr::Inline(s) => s.len(),
            StringSmallAllocRepr::Heap(s) => s.len(),
        }
    }

    /// Returns the current remaining active capacity in bytes.
    #[must_use]
    #[inline(always)]
    pub fn remaining_capacity(&self) -> usize {
        self.capacity().saturating_sub(self.len())
    }

    /// Returns `true` if the string is empty.
    #[must_use]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns `true` if the string is currently stored inline.
    #[must_use]
    #[inline(always)]
    pub const fn is_inline(&self) -> bool {
        matches!(self.repr, StringSmallAllocRepr::Inline(_))
    }

    /// Returns `true` if the string has spilled to the heap.
    #[must_use]
    #[inline(always)]
    pub const fn is_heap(&self) -> bool {
        matches!(self.repr, StringSmallAllocRepr::Heap(_))
    }

    /* modifiers */

    /// Clears the string.
    ///
    /// If the string has spilled to the heap, the heap allocation is retained.
    #[inline(always)]
    pub fn clear(&mut self) {
        match &mut self.repr {
            StringSmallAllocRepr::Inline(s) => s.clear(),
            StringSmallAllocRepr::Heap(s) => s.clear(),
        }
    }

    /// Truncates the string to `new_len` bytes.
    ///
    /// Does nothing if `new_len >= self.len()`.
    ///
    /// # Panics
    /// Panics if `new_len` is not a UTF-8 boundary.
    pub fn truncate(&mut self, new_len: usize) {
        if new_len >= self.len() {
            return;
        }
        assert![self.as_str().is_char_boundary(new_len), "not a UTF-8 boundary"];

        match &mut self.repr {
            StringSmallAllocRepr::Inline(s) => {
                while s.len() > new_len {
                    let _ = s.pop();
                }
            }
            StringSmallAllocRepr::Heap(s) => s.truncate(new_len),
        }
    }

    /// Reserves capacity for at least `additional` more bytes.
    ///
    /// Spills to the heap if the current inline storage is not enough.
    pub fn reserve(&mut self, additional: usize) {
        if additional <= self.remaining_capacity() {
            return;
        }

        match &mut self.repr {
            StringSmallAllocRepr::Inline(s) => {
                let mut heap = String::with_capacity(s.len() + additional);
                heap.push_str(s.as_str());
                self.repr = StringSmallAllocRepr::Heap(heap);
            }
            StringSmallAllocRepr::Heap(s) => s.reserve(additional),
        }
    }

    /// Moves the string back inline if it fits.
    ///
    /// Returns `true` if the string is inline after the call.
    pub fn shrink_to_inline(&mut self) -> bool {
        match &self.repr {
            StringSmallAllocRepr::Inline(_) => true,
            StringSmallAllocRepr::Heap(s) if s.len() <= CAP => {
                let inline = StringU8::<CAP>::from_str(s.as_str())
                    .expect("validated StringSmallAlloc inline capacity");
                self.repr = StringSmallAllocRepr::Inline(inline);
                true
            }
            StringSmallAllocRepr::Heap(_) => false,
        }
    }

    /// Shrinks heap allocation if spilled, or moves back inline if possible.
    pub fn shrink_to_fit(&mut self) {
        if self.shrink_to_inline() {
            return;
        }
        if let StringSmallAllocRepr::Heap(s) = &mut self.repr {
            s.shrink_to_fit();
        }
    }

    /// Removes the last character and returns it.
    #[must_use]
    pub fn pop(&mut self) -> Option<char> {
        match &mut self.repr {
            StringSmallAllocRepr::Inline(s) => s.pop(),
            StringSmallAllocRepr::Heap(s) => s.pop(),
        }
    }

    /// Tries to remove the last character and return it.
    ///
    /// # Errors
    /// Returns [`NotEnoughElements`] if the string is empty.
    pub fn try_pop(&mut self) -> Result<char, NotEnoughElements> {
        match self.pop() {
            Some(c) => Ok(c),
            None => Err(NotEnoughElements(Some(1))),
        }
    }

    /// Appends `character`, spilling to the heap if needed.
    ///
    /// Returns the number of bytes written.
    pub fn push(&mut self, character: char) -> usize {
        let len = character.len_utf8();

        match &mut self.repr {
            StringSmallAllocRepr::Inline(s) => {
                if s.try_push(character).is_ok() {
                    len
                } else {
                    let mut heap = String::with_capacity(s.len() + len);
                    heap.push_str(s.as_str());
                    heap.push(character);
                    self.repr = StringSmallAllocRepr::Heap(heap);
                    len
                }
            }
            StringSmallAllocRepr::Heap(s) => {
                s.push(character);
                len
            }
        }
    }

    /// Appends `string`, spilling to the heap if needed.
    ///
    /// Returns the number of bytes written.
    pub fn push_str(&mut self, string: &str) -> usize {
        if string.is_empty() {
            return 0;
        }

        match &mut self.repr {
            StringSmallAllocRepr::Inline(s) => {
                if let Ok(n) = s.try_push_str_complete(string) {
                    n
                } else {
                    let mut heap = String::with_capacity(s.len() + string.len());
                    heap.push_str(s.as_str());
                    heap.push_str(string);
                    self.repr = StringSmallAllocRepr::Heap(heap);
                    string.len()
                }
            }
            StringSmallAllocRepr::Heap(s) => {
                s.push_str(string);
                string.len()
            }
        }
    }
}

#[rustfmt::skip]
mod impl_traits {
    use crate::{
        Borrow, ConstInit, Debug, Deref, Display, FmtResult, FmtWrite, Formatter, FromStr, Hash,
        Hasher, MismatchedCapacity, Ordering, StringSmallAlloc,
    };

    impl<const CAP: usize> ConstInit for StringSmallAlloc<CAP> { const INIT: Self =  Self::new(); }
    impl<const CAP: usize> Default for StringSmallAlloc<CAP> { fn default() -> Self { Self::new() }}

    impl<const CAP: usize> FromStr for StringSmallAlloc<CAP> {
        type Err = MismatchedCapacity;
        fn from_str(string: &str) -> Result<Self, Self::Err> { Self::from_str(string) }
    }

    impl<const CAP: usize> FmtWrite for StringSmallAlloc<CAP> {
        fn write_str(&mut self, s: &str) -> FmtResult<()> { self.push_str(s); Ok(()) }
    }
    impl<const CAP: usize> Deref for StringSmallAlloc<CAP> {
        type Target = str;
        fn deref(&self) -> &Self::Target { self.as_str() }
    }
    impl<const CAP: usize> AsRef<str> for StringSmallAlloc<CAP> {
        fn as_ref(&self) -> &str { self.as_str() }
    }
    impl<const CAP: usize> Borrow<str> for StringSmallAlloc<CAP> {
        fn borrow(&self) -> &str { self.as_str() }
    }
    impl<const CAP: usize> Debug for StringSmallAlloc<CAP> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> { Debug::fmt(self.as_str(), f) }
    }
    impl<const CAP: usize> Display for StringSmallAlloc<CAP> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> { Display::fmt(self.as_str(), f) }
    }
    impl<const CAP: usize> Hash for StringSmallAlloc<CAP> {
        fn hash<H: Hasher>(&self, state: &mut H) { Hash::hash(self.as_str(), state); } }
    impl<const CAP: usize> PartialEq for StringSmallAlloc<CAP> {
        fn eq(&self, other: &Self) -> bool { self.as_str() == other.as_str() }
    }

    impl<const CAP: usize> Eq for StringSmallAlloc<CAP> {}
    impl<const CAP: usize> PartialEq<str> for StringSmallAlloc<CAP> {
        fn eq(&self, other: &str) -> bool {
            self.as_str() == other
        }
    }
    impl<const CAP: usize> PartialEq<&str> for StringSmallAlloc<CAP> {
        fn eq(&self, other: &&str) -> bool { self.as_str() == *other }
    }

    impl<const CAP: usize> Ord for StringSmallAlloc<CAP> {
        fn cmp(&self, other: &Self) -> Ordering { self.as_str().cmp(other.as_str()) }
    }
    impl<const CAP: usize> PartialOrd for StringSmallAlloc<CAP> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
    }

    impl<const CAP: usize> Extend<char> for StringSmallAlloc<CAP> {
        fn extend<I: IntoIterator<Item = char>>(&mut self, iter: I) {
            for c in iter { self.push(c); }
        }
    }
    impl<'a, const CAP: usize> Extend<&'a str> for StringSmallAlloc<CAP> {
        fn extend<I: IntoIterator<Item = &'a str>>(&mut self, iter: I) {
            for s in iter { self.push_str(s); }
        }
    }
    impl<const CAP: usize> FromIterator<char> for StringSmallAlloc<CAP> {
        fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> Self {
            let mut string = Self::new();
            string.extend(iter);
            string
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{STRING_SMALL_ALLOC_DEFAULT_CAP, StringSmallAlloc};
    use crate::{FmtWrite, String, StringU8};

    #[test]
    fn inline_default_preserves_option_niche() {
        type Inline = StringU8<STRING_SMALL_ALLOC_DEFAULT_CAP>;
        assert_eq![size_of::<Option<Inline>>(), size_of::<Inline>()];
    }
    #[test]
    fn small_alloc_explicit_sizes() {
        #[cfg(target_pointer_width = "64")]
        {
            assert_eq![size_of::<String>(), 24];
            assert_eq![size_of::<Option<Option<Option<Option<Option<String>>>>>>(), 24];

            assert_eq![size_of::<StringU8<23>>(), 24];
            assert_eq![size_of::<Option<StringU8<23>>>(), 24];
            assert_eq![size_of::<StringSmallAlloc<23>>(), 32];
            assert_eq![size_of::<Option<StringSmallAlloc<23>>>(), 32];
            assert_eq![size_of::<StringSmallAlloc<15>>(), 24];
            assert_eq![size_of::<Option<StringSmallAlloc<15>>>(), 24];
        }
        #[cfg(target_pointer_width = "32")]
        {
            assert_eq![size_of::<StringU8<11>>(), 12];
            assert_eq![size_of::<Option<StringU8<11>>>(), 12];
            assert_eq![size_of::<StringSmallAlloc<11>>(), 16];
            assert_eq![size_of::<Option<StringSmallAlloc<11>>>(), 16];
            assert_eq![size_of::<StringSmallAlloc<7>>(), 16];
            assert_eq![size_of::<Option<StringSmallAlloc<7>>>(), 16];
        }
    }

    #[test]
    fn new() {
        let s = StringSmallAlloc::<8>::new();
        assert_eq![s.as_str(), ""];
        assert_eq![s.len(), 0];
        assert_eq![s.capacity(), 8];
        assert![s.is_inline()];
        assert![!s.is_heap()];
    }
    #[test]
    fn from_str_inline() {
        let s = StringSmallAlloc::<8>::from_str("hello").unwrap();
        assert_eq![s.as_str(), "hello"];
        assert_eq![s.len(), 5];
        assert![s.is_inline()];
    }
    #[test]
    fn from_str_heap() {
        let s = StringSmallAlloc::<4>::from_str("hello").unwrap();
        assert_eq![s.as_str(), "hello"];
        assert_eq![s.len(), 5];
        assert![s.is_heap()];
    }
    #[test]
    fn from_string_reuses_heap_when_large() {
        let input = String::from("hello world");
        let s = StringSmallAlloc::<4>::from_string(input).unwrap();
        assert_eq![s.as_str(), "hello world"];
        assert![s.is_heap()];
    }
    #[test]
    fn push_spills() {
        let mut s = StringSmallAlloc::<4>::from_str("café").unwrap();
        assert![s.is_heap() || s.is_inline()];
        // "café" is exactly 5 bytes, so with CAP=4 it starts heap.
        assert![s.is_heap()];
        assert_eq![s.push('!'), 1];
        assert_eq![s.as_str(), "café!"];
    }
    #[test]
    fn push_str_spills_without_truncating() {
        let mut s = StringSmallAlloc::<4>::from_str("abc").unwrap();
        assert![s.is_inline()];
        assert_eq![s.push_str("€"), 3];
        assert_eq![s.as_str(), "abc€"];
        assert![s.is_heap()];
    }
    #[test]
    fn clear_keeps_heap() {
        let mut s = StringSmallAlloc::<4>::from_str("hello").unwrap();
        assert![s.is_heap()];
        s.clear();
        assert_eq![s.as_str(), ""];
        assert![s.is_heap()];
    }
    #[test]
    fn shrink_to_inline() {
        let mut s = StringSmallAlloc::<4>::from_str("hello").unwrap();
        assert![s.is_heap()];
        s.truncate(4);
        assert_eq![s.as_str(), "hell"];
        assert![s.shrink_to_inline()];
        assert![s.is_inline()];
    }
    #[test]
    fn pop_utf8() {
        let mut s = StringSmallAlloc::<4>::from_str("a€").unwrap();
        assert_eq![s.pop(), Some('€')];
        assert_eq![s.pop(), Some('a')];
        assert_eq![s.pop(), None];
    }
    #[test]
    fn equality_ignores_repr() {
        let a = StringSmallAlloc::<4>::from_str("abc").unwrap();
        let mut b = StringSmallAlloc::<4>::from_str("abc€").unwrap();
        assert![b.is_heap()];
        b.truncate(3);
        assert_eq![a, b];
    }
    #[test]
    fn fmt_write_spills() {
        let mut s = StringSmallAlloc::<4>::new();
        assert![write!(&mut s, "{} {}", "hello", 42).is_ok()];
        assert_eq![s.as_str(), "hello 42"];
        assert![s.is_heap()];
    }
    #[test]
    fn into_string() {
        let s = StringSmallAlloc::<8>::from_str("hello").unwrap();
        let heap = s.into_string();
        assert_eq![heap, "hello"];
    }
}
