// devela::data::bit::ops::wrapper
//
//!
//

mod primitives;

#[cfg(test)]
mod tests;

/// Provides constant bitwise operations on `T`.
///
/// See also [`BitOps`][super::BitOps] for the equivalent trait.
#[repr(transparent)]
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
