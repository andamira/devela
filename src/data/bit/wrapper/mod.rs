// devela::data::bit::wrapper
//
//!
//

mod primitives;

#[cfg(test)]
mod tests;

#[doc = crate::TAG_NAMESPACE!()]
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
#[must_use]
#[repr(transparent)]
pub struct Bitwise<T>(pub T);

#[rustfmt::skip]
mod core_impls {
    use crate::{impl_trait, Bitwise, Hash, Hasher, Ordering};

    impl<T: Clone> Clone for Bitwise<T> { fn clone(&self) -> Self { Self(self.0.clone()) } }
    impl<T: Copy> Copy for Bitwise<T> {}

    impl_trait![fmt::Debug for Bitwise<T> where T |self, f|
        f.debug_tuple("Bitwise").field(&self.0).finish()
    ];
    impl_trait![fmt::Display for Bitwise<T> where T |self, f| self.0.fmt(f)];
    impl_trait![fmt::Binary for Bitwise<T> where T |self, f| self.0.fmt(f)];
    impl_trait![fmt::Octal for Bitwise<T> where T |self, f| self.0.fmt(f)];
    impl_trait![fmt::LowerHex for Bitwise<T> where T |self, f| self.0.fmt(f)];
    impl_trait![fmt::UpperHex for Bitwise<T> where T |self, f| self.0.fmt(f)];
    impl_trait![fmt::LowerExp for Bitwise<T> where T |self, f| self.0.fmt(f)];
    impl_trait![fmt::UpperExp for Bitwise<T> where T |self, f| self.0.fmt(f)];

    impl<T: PartialEq> PartialEq for Bitwise<T> {
        fn eq(&self, other: &Self) -> bool { self.0.eq(&other.0) }
    }
    impl<T: Eq> Eq for Bitwise<T> {}
    impl<T: PartialOrd> PartialOrd for Bitwise<T> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> { self.0.partial_cmp(&other.0) }
    }
    impl<T: Ord> Ord for Bitwise<T> {
        fn cmp(&self, other: &Self) -> Ordering { self.0.cmp(&other.0) }
    }
    impl<T: Hash> Hash for Bitwise<T> {
        fn hash<H: Hasher>(&self, state: &mut H) { self.0.hash(state); }
    }
    impl<T: Hasher> Hasher for Bitwise<T> {
        fn finish(&self) -> u64 { self.0.finish() }
        fn write(&mut self, bytes: &[u8]) { self.0.write(bytes); }
    }
}
