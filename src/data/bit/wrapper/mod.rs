// devela::data::bit::ops::wrapper
//
//!
//

mod primitives;

#[cfg(test)]
mod tests;

/// Provides constant bitwise operations on `T`.
///
/// It's implemented for:
/// [`i8`], [`i16`], [`i32`], [`i64`], [`i128`], [`isize`],
/// [`u8`], [`u16`], [`u32`], [`u64`], [`u128`] and [`usize`].
///
/// See also [`BitOps`][super::BitOps] for the corresponding trait.
///
/// [`i8`]: Self#implementation-for-i8
/// [`i16`]: Self#implementation-for-i16
/// [`i32`]: Self#implementation-for-i32
/// [`i64`]: Self#implementation-for-i64
/// [`i128`]: Self#implementation-for-i128
/// [`isize`]: Self#implementation-for-isize
/// [`u8`]: Self#implementation-for-u8
/// [`u16`]: Self#implementation-for-u16
/// [`u32`]: Self#implementation-for-u32
/// [`u64`]: Self#implementation-for-u64
/// [`u128`]: Self#implementation-for-u128
/// [`usize`]: Self#implementation-for-usize
#[repr(transparent)]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "data")))]
pub struct Biting<T>(pub T);

#[rustfmt::skip]
mod core_impls {
    use {super::Biting, core::{fmt, cmp, hash}};

    impl<T: Clone> Clone for Biting<T> {
        #[inline] #[must_use]
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }
    impl<T: Copy> Copy for Biting<T> {}
    impl<T: fmt::Debug> fmt::Debug for Biting<T> {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_tuple("Biting").field(&self.0).finish()
        }
    }
    impl<T: fmt::Display> fmt::Display for Biting<T> {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { fmt::Display::fmt(&self.0, f) }
    }
    impl<T: fmt::Binary> fmt::Binary for Biting<T> {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { fmt::Binary::fmt(&self.0, f) }
    }
    impl<T: fmt::Octal> fmt::Octal for Biting<T> {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { fmt::Octal::fmt(&self.0, f) }
    }
    impl<T: fmt::LowerHex> fmt::LowerHex for Biting<T> {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { fmt::LowerHex::fmt(&self.0, f) }
    }
    impl<T: fmt::UpperHex> fmt::UpperHex for Biting<T> {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { fmt::UpperHex::fmt(&self.0, f) }
    }
    impl<T: fmt::UpperExp> fmt::UpperExp for Biting<T> {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { fmt::UpperExp::fmt(&self.0, f) }
    }
    impl<T: fmt::LowerExp> fmt::LowerExp for Biting<T> {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { fmt::LowerExp::fmt(&self.0, f) }
    }

    impl<T: PartialEq> PartialEq for Biting<T> {
        #[inline] #[must_use]
        fn eq(&self, other: &Self) -> bool { self.0.eq(&other.0) }
    }
    impl<T: Eq> Eq for Biting<T> {}
    impl<T: PartialOrd> PartialOrd for Biting<T> {
        #[inline] #[must_use]
        fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
            self.0.partial_cmp(&other.0)
        }
    }
    impl<T: Ord> Ord for Biting<T> {
        #[inline] #[must_use]
        fn cmp(&self, other: &Self) -> cmp::Ordering {
            self.0.cmp(&other.0)
        }
    }
    impl<T: hash::Hash> hash::Hash for Biting<T> {
        #[inline]
        fn hash<H: hash::Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }
    impl<T: hash::Hasher> hash::Hasher for Biting<T> {
        #[inline] #[must_use]
        fn finish(&self) -> u64 {
            self.0.finish()
        }
        #[inline]
        fn write(&mut self, bytes: &[u8]) {
            self.0.write(bytes)
        }
    }
}
