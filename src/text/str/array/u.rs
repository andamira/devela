// devela::text::str::u
//
//! `String` backed by an array.
//
// TOC
// - impl_str_u!
//   - definitions
//   - trait impls
// - tests

use crate::{AddAssign, ConstInit, Deref, Hash, Hasher, is, lets, paste, slice, whilst};
use crate::{Char, CharIter, InvalidText, Str, StringNonul, char7, char8, char16, charu};
#[allow(unused, reason = "±unsafe")]
use crate::{Cmp, unwrap};
use crate::{Debug, Display, FmtError, FmtResult, FmtWrite, Formatter, Ordering};
use crate::{MismatchedCapacity, NotEnoughElements, NotEnoughSpace};

macro_rules! impl_str_u {
    // in sync with devela::code::const_init % _stringu
    // () => { impl_str_u![u8, u16]; };
    () => { impl_str_u![u8]; }; // TEMP
    (
    $($P:ty),+ $(,)?) => { paste! { $(
        impl_str_u![%[<String $P:camel>], $P, [<NonMax $P:camel>], $crate::[<NonMax $P:camel>]];
    )+ }};
    (%
    // $name: the name of the type. E.g.: StringU8.
    // $P: the inner primitive length type. E.g.: u8.
    // $ni: the niche length type. E.g.: NonMax8.
    // $NI: the full path niche type. E.g.: $crate::NonMaxU8.
    $name:ident, $P:ty, $ni:ty, $NI:ty) => {
        /* definitions */

        #[allow(rustdoc::broken_intra_doc_links, reason = "±unsafe")]
        #[doc = crate::_tags!(string)]
        /// A UTF-8 string with fixed capacity that stores length explicitly.
        #[doc = crate::_doc_meta!{location("text/str")}]
        ///
        /// Suited for frequently inspected or manipulated text where constant-time
        /// length access is important. Uses extra space to provide O(1) length operations.
        /// For the opposite trade-off see [`StringNonul`].
        ///
        #[doc = concat!(
            "Internally, the current length is stored as a [`",
            stringify!($ni), "`][$crate::", stringify!($ni), "].")]
        ///
        /// ## Methods
        ///
        /// - [Constructors](#constructors):
        ///   - [new](#method.new)
        ///     *([_checked](#method.new_checked))*.
        ///   - [from_str](#method.from_str),
        ///     *([_truncate](#method.from_str_truncate),
        ///       [_unchecked](#method.from_str_unchecked))*.
        ///   - [from_char](#method.from_char)
        ///     *([7](#method.from_char7),
        ///       [8](#method.from_char8),
        ///       [16](#method.from_char16),
        ///       [utf8](#method.from_charu))*.
        ///   - [from_array](#method.from_array) *(
        ///     [_unchecked](#method.from_array_unchecked)<sup title='unsafe function'>⚠</sup>,
        ///     [_nleft](#method.from_array_nleft),
        ///     [_nleft_unchecked](#method.from_array_nleft_unchecked)<sup title='unsafe function'>⚠</sup>,
        ///     [_nright](#method.from_array_nleft),
        ///     [_nright_unchecked](#method.from_array_nright_unchecked)<sup title='unsafe function'>⚠</sup>)*.
        ///
        /// - [Deconstructors](#deconstructors):
        ///   - [into_array](#method.into_array).
        ///   - [as_array](#method.as_array).
        ///   - [as_bytes](#method.as_bytes)
        ///     *([mut](#method.as_bytes_mut)<sup title='unsafe function'>⚠</sup>)*.
        ///   - [as_str](#method.as_str)
        ///     *([mut](#method.as_mut_str)<sup title='unsafe function'>⚠</sup>)*.
        ///   - [chars](#method.chars).
        ///
        /// - [Queries](#queries):
        ///   - [len](#method.len).
        ///   - [is_empty](#method.is_empty).
        ///   - [is_full](#method.is_full).
        ///   - [capacity](#method.capacity).
        ///   - [remaining_capacity](#method.remaining_capacity).
        ///
        /// - [Modifiers](#modifiers):
        ///   - [clear](#method.clear),
        ///   - [reset](#method.reset),
        ///   - [sanitize](#method.sanitize),
        ///   - [pop](#method.pop)
        ///     *([try_](#method.try_pop),
        ///       [_unchecked](#method.pop_unchecked))*,
        ///   - [push](#method.push)
        ///     *([try_](#method.try_push))*.
        ///   - [push_str](#method.push_str)
        ///     *([try_](#method.try_push_str),
        ///       [try_ _complete](#method.try_push_str_complete))*.
        #[must_use]
        #[derive(Clone, Copy, Eq)]
        pub struct $name<const CAP: usize> {
            arr: [u8; CAP], // WAIT: for when possible CAP:u8|u16 for panic-less boundary check
            len: $crate::MaybeNiche<$NI>,
            // len: $P, // BENCH non-niche len
        }

        /* helpers */
        impl<const CAP: usize> $name<CAP> {
            /* niche construction */
            #[inline(always)]
            const fn _ni_zero() -> $crate::MaybeNiche<$NI> {
                // SAFETY-INVARIANT: `$NI` is a `NonMaxU*`; zero is always representable.
                $crate::unwrap![some_guaranteed_or_ub $crate::MaybeNiche::<$NI>::ZERO]
            }
            #[inline(always)]
            const fn _ni_prim(p: $P) -> $crate::MaybeNiche<$NI> {
                $crate::unwrap![ok $crate::MaybeNiche::<$NI>::try_from_prim(p)]
            }
            #[inline(always)]
            const fn _ni_usize(p: usize) -> $crate::MaybeNiche<$NI> {
                $crate::unwrap![ok $crate::MaybeNiche::<$NI>::try_from_usize(p)]
            }
            /* cap validation */
            #[inline(always)]
            const fn _valid_cap() -> bool { CAP < <$P>::MAX as usize }
            #[inline(always)]
            const fn _assert_cap() {
                assert![Self::_valid_cap(),
                    concat!["Mismatched capacity, greater or equal than ", stringify![$P], "::MAX"]
                ];
            }
            #[inline(always)]
            const fn _check_cap() -> Result<(), MismatchedCapacity> {
                if Self::_valid_cap() { Ok(()) }
                else { Err(MismatchedCapacity::too_large(CAP, <$P>::MAX as usize - 1)) }
            }
            #[inline(always)]
            const fn _check_cap_invalid_text() -> Result<(), InvalidText> {
                if Self::_valid_cap() { Ok(()) } else {
                    let err = MismatchedCapacity::too_large(CAP, <$P>::MAX as usize - 1);
                    Err(InvalidText::from_mismatched_capacity(err)) }
            }
            /* len */
            #[inline(always)]
            const fn _add_len(&mut self, extra: usize) {
                self.len = Self::_ni_usize(self.len() + extra); }
            #[inline(always)]
            const fn _len_prim(&self) -> $P { self.len.prim() }
            #[inline(always)]
            const fn _set_len_prim(&mut self, len: $P) { self.len = Self::_ni_prim(len); }
            #[inline(always)]
            const fn _set_len(&mut self, len: usize) { self.len = Self::_ni_usize(len); }
            /* constructors */
            #[inline(always)]
            const fn _empty() -> Self { Self { arr: [0; CAP], len: Self::_ni_zero() } }
            #[inline(always)]
            const fn _from_parts_unchecked(arr: [u8; CAP], len: usize) -> Self {
                Self { arr, len: Self::_ni_usize(len) }
            }
        }
        // helper macro for constructors from chars
        macro_rules! _str_u_copy_utf8 {
            ($dst:expr, $src:expr, $len:expr; $N:tt) => {{
                let dst = &mut $dst; let src = &$src; let len = $len;
                $crate::punroll! { $N |i| if i < len { dst[i] = src[i]; } }
            }};
        }

        /// # Constants
        impl<const CAP: usize> $name<CAP> {
            /// The maximum allowed capacity.
            pub const MAX_CAPACITY: usize = <$P>::MAX as usize - 1;
        }
        /// # Constructors
        impl<const CAP: usize> $name<CAP> { $crate::paste! {
            /// Creates a new empty string from with a capacity of `CAP` bytes.
            ///
            /// # Panics
            /// Panics if `CAP > Self::MAX_CAPACITY`.
            ///
            /// # Examples
            /// ```
            #[doc = "# use devela::" $name ";"]
            #[doc = "let mut s = " $name "::<10>::new();"]
            #[doc = "assert![size_of_val(&s) >= 10 + size_of::<" $P ">()]; // + padding"]
            /// ```
            pub const fn new() -> Self {
                Self::_assert_cap();
                Self::_empty()
            }
        }//paste!
            /// Creates a new empty string from with a capacity of `CAP` bytes.
            ///
            /// # Errors
            /// Returns [`MismatchedCapacity`] if `CAP > Self::MAX_CAPACITY`
            pub const fn new_checked() -> Result<Self, MismatchedCapacity> {
                match Self::_check_cap() {
                    Ok(()) => Ok(Self::_empty()),
                    Err(e) => Err(e),
                }
            }

            /* from_str* conversions */

            /// Creates a new string from a complete `&str`.
            ///
            /// # Errors
            /// Returns [`MismatchedCapacity`] if `CAP > Self::MAX_CAPACITY`
            /// or if `CAP < string.len()`.
            ///
            /// This is implemented via `Self::`[`try_push_str_complete()`][Self::try_push_str_complete].
            ///
            /// # Examples
            /// ```
            /// # use devela::StringU8;
            /// let s = StringU8::<13>::from_str("Hello Wørld!").unwrap();
            /// assert_eq![s.as_str(), "Hello Wørld!"];
            /// ```
            pub const fn from_str(string: &str) -> Result<Self, MismatchedCapacity> {
                let mut new_string = unwrap![ok? Self::new_checked()];
                if let Ok(_) = new_string.try_push_str_complete(string) { Ok(new_string) }
                else { Err(MismatchedCapacity::too_small(CAP, string.len())) }
            }

            /// Creates a new string from a `&str`, truncating if it does not fit.
            ///
            /// Returns [`MismatchedCapacity`] if `CAP > Self::MAX_CAPACITY`
            ///
            /// This is implemented via `Self::`[`push_str()`][Self::push_str].
            pub const fn from_str_truncate(string: &str) -> Result<Self, MismatchedCapacity> {
                let mut new_string = unwrap![ok? Self::new_checked()];
                let _ = new_string.push_str(string);
                Ok(new_string)
            }

            /// Creates a new string from a `&str`, truncating if it does not fit.
            ///
            /// # Panics
            /// Panics if `CAP > Self::MAX_CAPACITY`.
            ///
            /// This is implemented via `Self::`[`push_str()`][Self::push_str].
            pub const fn from_str_unchecked(string: &str) -> Self {
                let mut new_string = Self::new();
                let _ = new_string.push_str(string);
                new_string
            }

            /* from_char* conversions */

            /// Creates a new string from a `char`.
            ///
            /// # Errors
            /// Returns [`MismatchedCapacity`] if `CAP > Self::MAX_CAPACITY`
            /// or if `CAP < c.`[`len_utf8()`][crate::UnicodeScalar::len_utf8].
            ///
            /// Will always succeed if `CAP >= 4 && CAP <= Self::MAX_CAPACITY`.
            /// # Examples
            /// ```
            /// # use devela::{StringU8, char};
            /// assert_eq![StringU8::<4>::from_char('🐛').unwrap().as_str(), "🐛"];
            /// assert![StringU8::<3>::from_char('🐛').is_err()];
            /// ```
            pub const fn from_char(c: char) -> Result<Self, MismatchedCapacity> {
                let bytes = Char(c).to_utf8_bytes();
                let len = Char(bytes[0]).len_utf8_unchecked();
                is![CAP < len, return Err(MismatchedCapacity::too_small(CAP, len))];
                let mut new = unwrap![ok? Self::new_checked()];
                _str_u_copy_utf8!(new.arr, bytes, len; 4);
                new.len = Self::_ni_usize(len);
                Ok(new)
            }

            /// Creates a new string from a `char7`.
            ///
            /// # Errors
            /// Returns [`MismatchedCapacity`] if `CAP > Self::MAX_CAPACITY`
            /// or if `CAP < 1`.
            ///
            /// Will always succeed if `CAP >= 1 && CAP <= Self::MAX_CAPACITY`.
            /// # Examples
            /// ```
            /// # use devela::{StringU8, char7};
            /// let s = StringU8::<1>::from_char7(char7::try_from_char('@').unwrap()).unwrap();
            /// assert_eq![s.as_str(), "@"];
            ///
            /// assert![StringU8::<0>::from_char7(char7::try_from_char('@').unwrap()).is_err()];
            /// ```
            pub const fn from_char7(c: char7) -> Result<Self, MismatchedCapacity> {
                is![CAP == 0, return Err(MismatchedCapacity::too_small(CAP, 1))];
                let mut new = unwrap![ok? Self::new_checked()];
                new.arr[0] = c.to_utf8_bytes()[0];
                new.len = Self::_ni_prim(1);
                Ok(new)
            }

            /// Creates a new string from a `char8`.
            ///
            /// # Errors
            /// Returns [`MismatchedCapacity`] if `CAP > Self::MAX_CAPACITY`
            /// or if `CAP < 2.
            ///
            /// Will always succeed if `CAP >= 2 && CAP <= Self::MAX_CAPACITY`.
            /// # Examples
            /// ```
            /// # use devela::{StringU8, char8};
            /// let s = StringU8::<2>::from_char8(char8::try_from_char('ß').unwrap()).unwrap();
            /// assert_eq![s.as_str(), "ß"];
            ///
            /// assert![StringU8::<1>::from_char8(char8::try_from_char('ß').unwrap()).is_err()];
            /// ```
            pub const fn from_char8(c: char8) -> Result<Self, MismatchedCapacity> {
                let bytes = c.to_utf8_bytes();
                let len = Char(bytes[0]).len_utf8_unchecked();
                is![CAP < len, return Err(MismatchedCapacity::too_small(CAP, len))];
                let mut new = unwrap![ok? Self::new_checked()];
                _str_u_copy_utf8!(new.arr, bytes, len; 2);
                new._set_len(len);
                Ok(new)
            }

            /// Creates a new string from a `char16`.
            ///
            /// # Errors
            /// Returns [`MismatchedCapacity`] if `CAP > Self::MAX_CAPACITY`
            /// || `CAP < c.`[`len_utf8()`][char16#method.len_utf8]."]
            ///
            /// Will always succeed if `CAP >= 3 && CAP <= Self::MAX_CAPACITY`.
            /// # Examples
            /// ```
            /// # use devela::{StringU8, char16};
            /// let s = StringU8::<3>::from_char16(char16::try_from_char('€').unwrap()).unwrap();
            /// assert_eq![s.as_str(), "€"];
            ///
            /// assert![StringU8::<2>::from_char16(char16::try_from_char('€').unwrap()).is_err()];
            /// ```
            pub const fn from_char16(c: char16) -> Result<Self, MismatchedCapacity> {
                let bytes = c.to_utf8_bytes();
                let len = Char(bytes[0]).len_utf8_unchecked();
                is![CAP < len, return Err(MismatchedCapacity::too_small(CAP, len))];
                let mut new = unwrap![ok? Self::new_checked()];
                _str_u_copy_utf8!(new.arr, bytes, len; 3);
                new._set_len(len);
                Ok(new)
            }

            /// Creates a new string from a `charu`.
            ///
            /// # Errors
            /// Returns [`MismatchedCapacity`] if `CAP > Self::MAX_CAPACITY`
            /// || `CAP < c.`[`len_utf8()`][charu#method.len_utf8]."]
            ///
            /// Will always succeed if `CAP >= 4 && CAP <= Self::MAX_CAPACITY`.
            /// # Examples
            /// ```
            /// # use devela::{StringU8, charu};
            /// let s = StringU8::<4>::from_charu(charu::from_char('🐛')).unwrap();
            /// assert_eq![s.as_str(), "🐛"];
            ///
            /// assert![StringU8::<3>::from_charu(charu::from_char('🐛')).is_err()];
            /// ```
            pub const fn from_charu(c: charu) -> Result<Self, MismatchedCapacity> {
                let (bytes, len) = (c.to_utf8_bytes(), c.len_utf8());
                if len <= CAP {
                    let mut new = unwrap![ok? Self::new_checked()];
                    _str_u_copy_utf8!(new.arr, bytes, len; 4);
                    new._set_len(len);
                    Ok(new)
                } else {
                    Err(MismatchedCapacity::too_small(CAP, len))
                }
            }

            /// Creates a new string from a `charu`.
            ///
            /// # Panics
            /// Panics if `CAP > Self::MAX_CAPACITY`
            /// || `CAP < c.`[`len_utf8()`][charu#method.len_utf8].
            ///
            /// Will always succeed if `CAP >= 3 && CAP <= Self::MAX_CAPACITY`.
            /// # Examples
            /// ```
            /// # use devela::{StringU8, charu};
            /// let s = StringU8::<3>::from_charu_unchecked(charu::from_char('€'));
            /// assert_eq![s, "€"]
            /// ```
            /// ```should_panic
            /// # use devela::{StringU8, charu};
            /// StringU8::<2>::from_charu_unchecked(charu::from_char('€'));
            /// ```
            pub const fn from_charu_unchecked(c: charu) -> Self {
                let (bytes, len) = (c.to_utf8_bytes(), c.len_utf8());
                let mut new = Self::new();
                _str_u_copy_utf8!(new.arr, bytes, len; 4);
                new._set_len(len);
                new
            }

            /* from_array* conversions */

            /// Returns a string from a slice of `bytes`.
            ///
            /// # Errors
            /// Returns [`InvalidText::MismatchedCapacity`] if `CAP > Self::MAX_CAPACITY`
            /// and [`InvalidText::Utf8`] if `bytes` are not valid UTF-8.
            pub const fn from_array(bytes: [u8; CAP]) -> Result<Self, InvalidText> {
                $crate::unwrap![ok? Self::_check_cap_invalid_text()];
                match Str::from_utf8(&bytes) {
                    Ok(_) => { Ok(Self::_from_parts_unchecked(bytes, CAP)) },
                    Err(e) => Err(InvalidText::from_invalid_utf8(e)),
                }
            }

            /// Returns a string from an array of `bytes` that must be valid UTF-8.
            ///
            /// # Panics
            /// Panics if `CAP > Self::MAX_CAPACITY`.
            /// # Safety
            /// The caller must ensure that the content of the slice is valid UTF-8.
            ///
            /// Use of a `str` whose contents are not valid UTF-8 is undefined behavior.
            #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
            #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_str")))]
            pub const unsafe fn from_array_unchecked(bytes: [u8; CAP]) -> Self {
                Self::_assert_cap();
                Self::_from_parts_unchecked(bytes, CAP)
            }
            /// Internal accessor for trusted formatting operations, avoiding UTF-8 re-validation.
            /// # Panics
            /// Panics if `CAP > Self::MAX_CAPACITY` or if `len > CAP`.
            #[inline(always)]
            pub(crate) const fn _from_array_len_trusted(array: [u8; CAP], len: $P) -> Self {
                Self::_assert_cap();
                assert!(len as usize <= CAP, "length greater than capacity");
                Self::_from_parts_unchecked(array, len as usize)
            }

            /// Returns a string from an array of `bytes`,
            /// truncated to `n` bytes counting from the left.
            ///
            /// The new `length` is maxed out at `CAP`.
            ///
            /// # Errors
            /// Returns [`InvalidText::MismatchedCapacity`] if `CAP > Self::MAX_CAPACITY`
            /// and [`InvalidText::Utf8`] if `bytes` are not valid UTF-8.
            pub const fn from_array_nleft(bytes: [u8; CAP], length: $P)
            -> Result<Self, InvalidText> {
                $crate::unwrap![ok? Self::_check_cap_invalid_text()];
                let length = Cmp(length).min(CAP as $P);
                match Str::from_utf8(slice![&bytes, ..length as usize]) {
                    Ok(_) => Ok(Self { arr: bytes, len: Self::_ni_prim(length) }),
                    Err(e) => Err(InvalidText::from_invalid_utf8(e)),
                }
            }

            /// Returns a string from an array of `bytes`, which must be valid UTF-8,
            /// truncated to `n` bytes counting from the left.
            ///
            /// The new `length` is maxed out at `CAP`.
            ///
            /// # Panics
            /// Panics if `CAP > Self::MAX_CAPACITY`.
            ///
            /// # Safety
            /// The caller must ensure that the content of the truncated slice is valid UTF-8.
            ///
            /// Use of a `str` whose contents are not valid UTF-8 is undefined behavior.
            #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
            #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_str")))]
            pub const unsafe fn from_array_nleft_unchecked(bytes: [u8; CAP], length: $P) -> Self {
                Self::_assert_cap();
                let len = Cmp(length).min(CAP as $P);
                Self { arr: bytes, len: Self::_ni_prim(len) }
            }

            /// Returns a string from an array of `bytes`,
            /// truncated to `n` bytes counting from the right.
            ///
            /// The new `length` is maxed out at `CAP`.
            /// Bytes are shift-copied without allocating a new array.
            ///
            /// # Errors
            /// Returns [`InvalidText::MismatchedCapacity`] if `CAP > Self::MAX_CAPACITY`
            /// and [`InvalidText::Utf8`] if `bytes` are not valid UTF-8.
            pub const fn from_array_nright(mut bytes: [u8; CAP], length: $P)
            -> Result<Self, InvalidText> {
                $crate::unwrap![ok? Self::_check_cap_invalid_text()];
                let length = Cmp(length).min(CAP as $P);
                let ulen = length as usize;
                let start = CAP - ulen;
                whilst![i in 0..ulen; bytes[i] = bytes[start + i]];
                match Str::from_utf8(slice![&bytes, ..ulen]) {
                    Ok(_) => Ok(Self { arr: bytes, len: Self::_ni_prim(length) }),
                    Err(e) => Err(InvalidText::from_invalid_utf8(e)),
                }
            }

            /// Returns a string from an array of `bytes`, which must be valid UTF-8,
            /// truncated to `n` bytes counting from the right.
            ///
            /// The new `length` is maxed out at `CAP`.
            /// Bytes are shift-copied without allocating a new array.
            ///
            /// # Panics
            /// Panics if `CAP > Self::MAX_CAPACITY`.
            ///
            /// # Safety
            /// The caller must ensure that the content of the truncated slice is valid UTF-8.
            ///
            /// Use of a `str` whose contents are not valid UTF-8 is undefined behavior.
            #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
            #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_str")))]
            pub const unsafe fn from_array_nright_unchecked(mut bytes: [u8; CAP], length: $P)
                -> Self {
                Self::_assert_cap();
                let length = Cmp(length).min(CAP as $P);
                let ulen = length as usize;
                let start = CAP - ulen;
                whilst![i in 0..ulen; bytes[i] = bytes[start + i]];
                Self { arr: bytes, len: Self::_ni_prim(length) }
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
            pub const fn as_array(&self) -> &[u8; CAP] { &self.arr }

            /// Returns a byte slice of the inner string slice.
            ///
            /// # Features
            /// Uses the `unsafe_slice` feature to skip validation checks.
            #[must_use] #[inline(always)]
            pub const fn as_bytes(&self) -> &[u8] {
                cfg_select! { all(feature = "unsafe_slice", not(feature = "safe_text")) => {
                    // SAFETY: we ensure to contain a correct length
                    unsafe { slice![unchecked &self.arr, ..self.len()] }
                } _ => { slice![&self.arr, ..self.len()] }}
            }

            /// Returns an exclusive byte slice of the inner string slice.
            ///
            /// # Safety
            /// The caller must ensure that the content of the slice is valid UTF-8
            /// before the borrow ends and the underlying `str` is used.
            ///
            /// # Features
            /// Uses the `unsafe_slice` feature to skip validation checks.
            #[must_use] #[inline(always)]
            #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
            #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_str")))]
            pub const unsafe fn as_bytes_mut(&mut self) -> &mut [u8] {
                let len = self.len();
                cfg_select! { all(feature = "unsafe_slice", not(feature = "safe_text")) => {
                    // SAFETY: we ensure to contain a correct length
                    unsafe { slice![mut_unchecked &mut self.arr, ..len] }
                } _ => { slice![mut &mut self.arr, ..len] }}
            }

            /// Returns a reference to the inner string slice.
            ///
            /// # Features
            /// Uses the `unsafe_str` feature to skip validation checks.
            #[must_use]
            #[inline(always)]
            pub const fn as_str(&self) -> &str {
                cfg_select! { all(feature = "unsafe_str", not(feature = "safe_text")) => {
                    // SAFETY: we ensure to contain only valid UTF-8
                    unsafe { Str::from_utf8_unchecked(self.as_bytes()) }
                } _ => { unwrap![ok_expect Str::from_utf8(self.as_bytes()), "Invalid UTF-8"] }}
            }

            /// Returns an exclusive reference to the inner string slice.
            ///
            /// # Safety
            /// The caller must ensure that the content of the slice is valid UTF-8
            /// before the borrow ends and the underlying `str` is used.
            #[must_use] #[inline(always)]
            #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
            #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_str")))]
            pub const unsafe fn as_mut_str(&mut self) -> &mut str {
                // SAFETY: we ensure to contain only valid UTF-8
                unsafe { Str::from_utf8_unchecked_mut(self.as_bytes_mut()) }
            }

            /// Returns an iterator over the `chars` of the string.
            ///
            /// # Features
            /// Uses the `unsafe_str` feature to skip validation checks.
            #[inline(always)]
            pub const fn chars(&self) -> CharIter<'_, &str> {
                CharIter::<&str>::new(self.as_str())
            }
        }

        /// # Queries
        impl<const CAP: usize> $name<CAP> {
            /// Returns the current string length in bytes.
            #[must_use]
            #[inline(always)]
            pub const fn len(&self) -> usize { self.len.prim() as usize }

            /// Returns `true` if the current length is 0.
            #[must_use]
            #[inline(always)]
            pub const fn is_empty(&self) -> bool { self.len.prim() == 0 }

            /// Returns `true` if the current remaining capacity is 0.
            #[must_use]
            #[inline(always)]
            pub const fn is_full(&self) -> bool { self.len() == CAP }

            /// Returns the total capacity in bytes.
            #[must_use]
            #[inline(always)]
            pub const fn capacity() -> usize { CAP }

            /// Returns the remaining capacity in bytes.
            #[must_use]
            #[inline(always)]
            pub const fn remaining_capacity(&self) -> usize { CAP - self.len() }

            /// Checks the equality of two strings, with the same capacity and length.
            ///
            /// It only checks the first `self.len()` bytes.
            /// # Examples
            /// ```
            /// # use devela::StringU8;
            /// let mut a = StringU8::<16>::from_str_unchecked("hello world!");
            /// let mut b = StringU8::<16>::from_str_unchecked("hello world!!!");
            /// assert![!a.eq(&b)];
            /// b.pop();
            /// b.pop();
            /// assert![a.eq(&b)];
            /// ```
            #[must_use]
            #[inline(always)]
            pub const fn eq(&self, other: &Self) -> bool {
                self.len.prim() == other.len.prim() && {
                    whilst![i in 0..self.len(); is![self.arr[i] != other.arr[i], return false]];
                    true
                }
            }
        }

        /// # Modifiers
        impl<const CAP: usize> $name<CAP> {
            /// Sets the length to 0.
            #[inline(always)]
            pub const fn clear(&mut self) { self.len = Self::_ni_zero() }

            /// Sets the length to 0, and resets all the bytes to 0.
            #[inline(always)]
            pub const fn reset(&mut self) { self.arr = [0; CAP]; self.len = Self::_ni_zero(); }

            /// Zeros all unused bytes while maintaining the current length.
            #[inline(always)]
            pub const fn sanitize(&mut self) {
                whilst![i in (self.len()),..CAP; self.arr[i] = 0];
            }

            /// Removes the last character and returns it, or `None` if the string is empty.
            #[must_use]
            pub const fn pop(&mut self) -> Option<char> {
                if self.is_empty() { None } else { Some(self.pop_unchecked()) }
            }

            /// Tries to remove the last character and returns it.
            ///
            /// # Errors
            /// Returns a [`NotEnoughElements`] error
            /// if the capacity is not enough to hold the `character`.
            pub const fn try_pop(&mut self) -> Result<char, NotEnoughElements> {
                is![self.is_empty(), Err(NotEnoughElements(Some(1))), Ok(self.pop_unchecked())]
            }

            /// Removes the last character and returns it.
            ///
            /// # Panics
            /// Panics if the string is empty.
            ///
            /// # Examples
            /// ```
            /// # use devela::StringU8;
            /// let mut s = StringU8::<16>::new();
            /// s.push_str("hello worlð!");
            /// assert_eq![s.len(), 13];
            ///
            /// assert_eq![s.pop_unchecked(), '!'];
            /// assert_eq![s.len(), 12];
            ///
            /// assert_eq![s.pop_unchecked(), 'ð'];
            /// assert_eq![s.len(), 10];
            /// ```
            pub const fn pop_unchecked(&mut self) -> char {
                let string = self.as_str();
                let mut index = string.len();
                whilst![index > 0 && !string.is_char_boundary(index - 1); index -= 1];
                let idx_last_char = index - 1;
                let range = Str::range_from(string, idx_last_char);
                let last_char = unwrap![some CharIter::<&str>::new(range).next_char()];
                let new_len = self.len.prim() - last_char.len_utf8() as $P;
                self.len = Self::_ni_prim(new_len);
                last_char
            }

            /// Appends to the end of the string the given `character`.
            ///
            /// Returns the number of bytes written.
            ///
            /// Returns 0 bytes if the given `character` doesn't fit in the remaining capacity.
            pub const fn push(&mut self, character: char) -> usize {
                match self.try_push(character) { Ok(n) => n, Err(_) => 0 }
            }

            /// Tries to append to the end of the string the given `character`.
            ///
            /// Returns the number of bytes written.
            ///
            /// # Errors
            /// Returns [`NotEnoughSpace`]
            /// if the available capacity is not enough to hold the given `character`.
            pub const fn try_push(&mut self, character: char) -> Result<usize, NotEnoughSpace> {
                let (start, char_len) = (self.len(), character.len_utf8());
                let end = start + char_len;
                if end <= CAP {
                    let _ = character.encode_utf8(slice![mut &mut self.arr, start, ..end]);
                    self._set_len(end);
                    Ok(char_len)
                } else {
                    Err(NotEnoughSpace(Some(char_len)))
                }
            }

            /// Appends to the end of the string the given character.
            ///
            /// Returns the number of bytes written.
            ///
            /// Returns 0 bytes if the given `character` doesn't fit in the remaining capacity.
            pub const fn push_charu(&mut self, c: charu) -> usize {
                let (bytes, len) = (c.to_utf8_bytes(), c.len_utf8());
                if self.remaining_capacity() >= len {
                    let start = self.len();
                    let end = start + len;
                    slice![mut &mut self.arr, start,..end].copy_from_slice(slice![&bytes, 0,..len]);
                    self.len = Self::_ni_usize(end);
                    len
                } else {
                    0
                }
            }

            /// Appends as many complete characters from `string` as will fit.
            ///
            /// Returns the number of bytes written. UTF-8 characters are never split.
            ///
            /// # Examples
            /// ```
            /// # use devela::StringU8;
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
                lets! { start = self.len(); remaining = CAP - start }
                is! { remaining == 0, return 0 }
                let string_len = string.len();
                let bytes_to_write = if string_len <= remaining {
                    string_len
                } else {
                    let mut amount = remaining;
                    while amount > 0 && !string.is_char_boundary(amount) { amount -= 1; }
                    amount
                };
                if bytes_to_write > 0 {
                    let end = start + bytes_to_write;
                    slice![mut &mut self.arr, start, ..end]
                        .copy_from_slice(slice![string.as_bytes(), ..bytes_to_write]);
                    self._set_len(end);
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
                is![bytes_written == string.len(), Ok(bytes_written), Err(bytes_written)]
            }

            /// Appends the entire `string` or nothing at all.
            ///
            /// Returns `Ok(bytes)` if the string fits completely, or `Err(0)` if it doesn't.
            /// No partial writes will occur to the buffer.
            pub const fn try_push_str_complete(&mut self, string: &str) -> Result<usize, usize> {
                is![self.remaining_capacity() >= string.len(), Ok(self.push_str(string)), Err(0)]
            }
        }

        /* utility traits */

        impl<const CAP: usize> Default for $name<CAP> {
            /// Returns an empty string.
            /// # Panics
            /// Panics if `CAP > Self::MAX_CAPACITY`.
            fn default() -> Self { Self::new() }
        }
        impl<const CAP: usize> ConstInit for $name<CAP> {
            /// Returns an empty string.
            /// # Panics
            /// Panics if `CAP > Self::MAX_CAPACITY`.
            const INIT: Self = Self::new();
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

        /// Writes as much UTF-8-complete text as fits.
        ///
        /// If the input does not fit completely,
        /// a prefix may have been written before returning [`FmtError`].
        impl<const CAP: usize> FmtWrite for $name<CAP> {
            fn write_str(&mut self, s: &str) -> FmtResult<()> {
                self.try_push_str(s).map_err(|_| FmtError)?; // RETHINK using try_push_complete_str
                Ok(())
            }
            fn write_char(&mut self, c: char) -> FmtResult<()> {
                self.try_push(c).map_err(|_| FmtError)?;
                Ok(())
            }
        }

        impl<const CAP: usize> PartialEq for $name<CAP> {
            fn eq(&self, other: &Self) -> bool { self.eq(&other) }
        }

        impl<const CAP: usize> PartialEq<str> for $name<CAP> { // str on the RHS
            fn eq(&self, string: &str) -> bool { self.as_str() == string }
        }
        impl<const CAP: usize> PartialEq<&str> for $name<CAP> { // &str on the RHS
            fn eq(&self, string: &&str) -> bool { self.as_str() == *string }
        }
        impl<const CAP: usize> PartialEq<&[u8]> for $name<CAP> { // &[u8] on the RHS
            fn eq(&self, bytes: &&[u8]) -> bool { self.as_bytes() == *bytes }
        }

        impl<const CAP: usize> PartialEq<$name<CAP>> for str { // str on the LHS
            fn eq(&self, string: &$name<CAP>) -> bool { self == string.as_str() }
        }
        impl<const CAP: usize> PartialEq<$name<CAP>> for &str { // &str on the LHS
            fn eq(&self, string: &$name<CAP>) -> bool { *self == string.as_str() }
        }
        impl<const CAP: usize> PartialEq<$name<CAP>> for &[u8] { // &[u8] on the LHS
            fn eq(&self, string: &$name<CAP>) -> bool { *self == string.as_bytes() }
        }

        impl<const CAP: usize> PartialOrd for $name<CAP> {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
        }
        impl<const CAP: usize> Ord for $name<CAP> {
            fn cmp(&self, other: &Self) -> Ordering { self.as_str().cmp(other.as_str()) }
        }

        impl<const CAP: usize> Hash for $name<CAP> {
            fn hash<H: Hasher>(&self, state: &mut H) { self.as_str().hash(state); }
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

        /* AddAssign */

        impl<const CAP: usize> AddAssign<char> for $name<CAP> {
            /// Appends the character if it fits.
            fn add_assign(&mut self, rhs: char) {
                let _ = self.push(rhs);
            }
        }
        impl<const CAP: usize> AddAssign<&str> for $name<CAP> {
            /// Appends text in place, keeping the fitted UTF-8 prefix.
            fn add_assign(&mut self, rhs: &str) {
                let _ = self.push_str(rhs);
            }
        }
        impl<const CAP: usize, const RHS: usize> AddAssign<&$name<RHS>> for $name<CAP> {
            /// Concatenates by appending what fits into `self`'s capacity.
            fn add_assign(&mut self, rhs: &$name<RHS>) {
                let _ = self.push_str(rhs.as_str());
            }
        }
        impl<const CAP: usize, const RHS: usize> AddAssign<$name<RHS>> for $name<CAP> {
            /// Concatenates by appending what fits into `self`'s capacity.
            fn add_assign(&mut self, rhs: $name<RHS>) {
                let _ = self.push_str(rhs.as_str());
            }
        }
        /// Appends non-NUL text in place, keeping the fitted UTF-8 prefix.
        impl<const CAP: usize, const RHS: usize> AddAssign<&StringNonul<RHS>> for StringU8<CAP> {
            fn add_assign(&mut self, rhs: &StringNonul<RHS>) {
                let _ = self.push_str(rhs.as_str());
            }
        }
        /// Appends non-NUL text in place, keeping the fitted UTF-8 prefix.
        impl<const CAP: usize, const RHS: usize> AddAssign<StringNonul<RHS>> for StringU8<CAP> {
            fn add_assign(&mut self, rhs: StringNonul<RHS>) {
                let _ = self.push_str(rhs.as_str());
            }
        }

        /* conversions */

        impl<const CAP: usize> TryFrom<&str> for $name<CAP> {
            type Error = MismatchedCapacity;

            /// Tries to create a new string from the given `string` slice.
            ///
            /// This is implemented via `Self::`[`from_str()`][Self::from_str].
            /// # Errors
            /// Returns [`MismatchedCapacity`] if `CAP > Self::MAX_CAPACITY`
            /// or if `CAP < string.len()`.
            fn try_from(string: &str) -> Result<Self, MismatchedCapacity> { Self::from_str(string) }
        }

        impl<const CAP: usize> TryFrom<&[u8]> for $name<CAP> {
            type Error = InvalidText;

            /// Tries to create a new string from the given slice of` bytes`.
            ///
            /// # Errors
            /// Returns [`InvalidText::MismatchedCapacity`] if `CAP > Self::MAX_CAPACITY`
            /// or if `CAP < bytes.len()`,
            /// and [`InvalidText::Utf8`] if `bytes` are not valid UTF-8.
            fn try_from(bytes: &[u8]) -> Result<Self, InvalidText> {
                $crate::unwrap![ok? Self::_check_cap_invalid_text()];
                let bytes_len = bytes.len();
                // CHECK MAYBE use this in more places? and made a helper fn?
                if CAP < bytes_len { Err(MismatchedCapacity::too_small(CAP, bytes_len).into()) }
                else {
                    match Str::from_utf8(bytes) {
                        Ok(_) => {
                            let mut arr = [0; CAP];
                            arr[..bytes_len].copy_from_slice(bytes);
                            Ok(Self { arr, len: Self::_ni_usize(bytes_len) })
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
            /// Panics if `CAP > Self::MAX_CAPACITY`.
            ///
            /// # Examples
            /// ```
            /// # use devela::StringU8;
            /// let chars = ['a', 'b', 'c', '€', 'さ'];
            /// let mut s = StringU8::<6>::new();
            /// s.extend(chars);
            /// assert_eq![s, "abc€"];
            /// ```
            fn extend<I: IntoIterator<Item=char>>(&mut self, iter: I) {
                for i in iter { is![self.push(i) == 0, break]; }
            }
        }
        impl<const CAP: usize> FromIterator<char> for $name<CAP> {
            /// Creates an instance from an iterator of characters.
            ///
            /// Processes characters until it can fit no more, discarding the rest.
            ///
            /// # Panics
            /// Panics if `CAP > Self::MAX_CAPACITY`.
            ///
            /// # Examples
            /// ```
            /// # use devela::StringU8;
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
    };
}
impl_str_u!();

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn niche_size() {
        assert_eq!(size_of::<StringU8::<0>>(), 1);
        assert_eq!(size_of::<Option<StringU8::<0>>>(), 1);
        assert_eq!(size_of::<StringU8::<3>>(), 4);
        assert_eq!(size_of::<Option<StringU8::<3>>>(), 4);
        assert_eq!(size_of::<StringU8::<254>>(), 255);
        assert_eq!(size_of::<Option<StringU8::<254>>>(), 255);
    }
    #[test]
    fn capacity_boundary() {
        assert!(StringU8::<254>::new_checked().is_ok());
        assert!(StringU8::<255>::new_checked().is_err());
        assert!(StringU8::<256>::new_checked().is_err());
    }
    #[test]
    #[should_panic]
    fn new_panics_at_forbidden_capacity() {
        let _ = StringU8::<255>::new();
    }
    #[test]
    fn max_valid_len_is_254_for_u8() {
        let s = StringU8::<254>::from_str(&"a".repeat(254)).unwrap();
        assert_eq!(s.len(), 254);
        assert!(s.is_full());
        assert_eq!(s.remaining_capacity(), 0);
    }
    #[test]
    fn rejects_len_255_for_u8() {
        let text = "a".repeat(255);
        assert!(StringU8::<254>::from_str(&text).is_err());
    }
    #[test]
    fn from_array_max_valid_capacity() {
        let bytes = [b'a'; 254];
        let s = StringU8::<254>::from_array(bytes).unwrap();
        assert_eq!(s.len(), 254);
        assert!(s.is_full());
        assert![StringU8::<255>::from_array([b'a'; 255]).is_err()];
    }
    #[test]
    fn nleft_max_valid_capacity() {
        let s = StringU8::<254>::from_array_nleft([b'a'; 254], u8::MAX).unwrap();
        assert_eq!(s.len(), 254);
        assert![StringU8::<255>::from_array_nleft([b'a'; 255], 1).is_err()];
    }
    #[test]
    fn nright_rejects_split_utf8_suffix() {
        let bytes = [0x82, 0xAC]; // continuation bytes, invalid as standalone UTF-8
        assert!(StringU8::<2>::from_array_nright(bytes, 2).is_err());
    }
    #[test]
    #[should_panic]
    fn trusted_len_must_not_exceed_cap() {
        let _ = StringU8::<4>::_from_array_len_trusted([0; 4], 5);
    }
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
    fn push_str_never_splits_utf8() {
        let mut s = StringU8::<4>::new();
        assert_eq!(s.push_str("€€"), 3);
        assert_eq!(s.as_str(), "€");
        assert_eq!(s.len(), 3);
    }
    #[test]
    fn try_push_str_complete_is_atomic() {
        let mut s = StringU8::<4>::from_str("ab").unwrap();
        assert_eq!(s.try_push_str_complete("€"), Err(0));
        assert_eq!(s.as_str(), "ab");
        assert_eq!(s.len(), 2);
    }
    #[test]
    fn push_charu_appends() {
        let mut s = StringU8::<5>::from_str("a").unwrap();
        assert_eq!(s.push_charu(charu::from_char('€')), 3);
        assert_eq!(s.as_str(), "a€");
        assert_eq!(s.len(), 4);
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
    #[test]
    fn ord_ignores_unused_bytes() {
        let a = StringU8::<2>::from_array_nleft([b'a', 1], 1).unwrap();
        let b = StringU8::<2>::from_array_nleft([b'a', 2], 1).unwrap();
        assert_eq!(a, b);
        assert_eq!(a.cmp(&b), core::cmp::Ordering::Equal);
    }
}
