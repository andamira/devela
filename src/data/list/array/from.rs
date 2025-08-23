// devela::data::list::array::from
//
//! Defines [`ArrayFrom`].
//

/// Compile-time conversion from slice-like types to arrays.
///
/// # Features
/// Uses `Ptr::copy_nonoverlapping` internally when unsafe operations are allowed.
///
/// # Panics
/// When `N` does not match the actual length of the input.
///
/// # Example
/// ```
/// # use devela::ArrayFrom;
/// let arr = ArrayFrom("hello").to_array::<5>();
/// assert_eq!(arr, *b"hello");
/// ```
#[derive(Debug)]
pub struct ArrayFrom<T>(pub T);

#[rustfmt::skip]
#[allow(clippy::len_without_is_empty)]
mod impls {
    use crate::{Slice, ArrayFrom};

    // byte
    impl ArrayFrom<u8> {
        /// Returns 1 (length of single byte).
        pub const fn len(self) -> usize { 1 }
        /// Converts to `[u8; N]` (must have N == 1).
        pub const fn to_array<const N: usize>(self) -> [u8; N] { Slice::to_array(&[self.0]) }
    }
    // byte array
    impl<const L: usize> ArrayFrom<[u8; L]> {
        /// Returns array length.
        pub const fn len(&self) -> usize { L }
        /// Converts to `[u8; N]` (must have N == len()).
        pub const fn to_array<const N: usize>(&self) -> [u8; N] { Slice::to_array(&self.0) }
    }
    // byte array ref
    impl<const L: usize> ArrayFrom<&[u8; L]> {
        /// Returns L (array length).
        pub const fn len(&self) -> usize { L }
        /// Converts to `[u8; N]` (must have N == L).
        pub const fn to_array<const N: usize>(&self) -> [u8; N] { Slice::to_array(self.0) }
    }
    // byte slice
    impl ArrayFrom<&[u8]> {
        /// Returns slice length.
        pub const fn len(&self) -> usize { self.0.len() }
        /// Converts to `[u8; N]` (must have N == len()).
        pub const fn to_array<const N: usize>(&self) -> [u8; N] { Slice::to_array(self.0) }
    }
    // slice of byte slices
    impl ArrayFrom<&[&[u8]]> {
        /// Returns total length of all slices.
        pub const fn len(&self) -> usize {
            let parts = self.0; let mut sum = 0; let mut i = 0;
            while i < parts.len() { sum += parts[i].len(); i += 1; }
            sum
        }
        /// Concatenates into `[u8; N]` (must have N == len()).
        pub const fn to_array<const N: usize>(&self) -> [u8; N] {
            let (mut buf, mut pos, parts, mut i) = ([0; N], 0, self.0, 0);
            while i < parts.len() {
                let part = parts[i];
                Slice::copy_array_at(&mut buf, part, pos);
                pos += part.len();
                i += 1;
            }
            assert!(pos == N);
            buf
        }
    }
    // string slice
    impl ArrayFrom<&str> {
        /// Returns byte length.
        pub const fn len(&self) -> usize { self.0.len() }
        /// Converts to `[u8; N]` (must have N == len()).
        pub const fn to_array<const N: usize>(&self) -> [u8; N] {
            Slice::to_array(self.0.as_bytes())
        }
    }
    // slice of string slices
    impl ArrayFrom<&[&str]> {
        /// Returns total length of all string slices.
        pub const fn len(&self) -> usize {
            let (mut len, mut rem) = (0, self.0);
            while let [first, rest @ ..] = rem {
                len += first.len();
                rem = rest;
            }
            len
        }
        /// Concatenates into `[u8; N]` (must have N == len()).
        pub const fn to_array<const N: usize>(&self) -> [u8; N] {
            let (mut buf, mut pos, mut i) = ([0; N], 0, 0);
            while i < self.0.len() {
                let bytes = self.0[i].as_bytes();
                crate::Slice::<u8>::copy_array_at(&mut buf, bytes, pos);
                pos += bytes.len();
                i += 1;
            }
            assert!(pos == N);
            buf
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_u8() {
        assert_eq!(ArrayFrom(42u8).to_array::<1>(), [42]);
    }
    #[test]
    fn from_array() {
        assert_eq!(ArrayFrom([1, 2, 3]).to_array::<3>(), [1, 2, 3]);
    }
    #[test]
    fn from_array_ref() {
        assert_eq!(ArrayFrom(&[1, 2, 3]).to_array::<3>(), [1, 2, 3]);
    }
    #[test]
    fn from_str() {
        assert_eq!(ArrayFrom("foo").to_array::<3>(), *b"foo");
    }
    // #[test]
    // fn from_slice_slice() {
    //     assert_eq!(ArrayFrom(&[&[1,2][..],&[3,4][..]]).to_array::<4>(), [1,2,3,4]);
    // }
    #[test]
    #[should_panic]
    fn from_length_mismatch() {
        let _ = ArrayFrom(&[1, 2, 3]).to_array::<2>();
    }
}
