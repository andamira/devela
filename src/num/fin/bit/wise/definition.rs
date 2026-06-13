// devela/src/num/fin/bit/wise/definition.rs
//
//! Defines the [`Bitwise`] namespace.
//

#[doc = crate::_tags!(num namespace)]
/// Provides constant bitwise operations on `T`.
#[doc = crate::_doc_meta!{location("num/fin")}]
///
/// It's implemented for: [`u8`], [`u16`], [`u32`], [`u64`], [`u128`] and [`usize`].
///
/// ## Panic behavior
/// Unchecked bit operations panic in debug builds when a bit index or range
/// is out of bounds. In release builds they do not panic; the invalid
/// index produces a wrapped shift and a non-meaningful result.
///
/// Checked variants never panic and return an error instead.
///
/// # Related items
/// See also [`BitOps`][crate::BitOps] for the related trait.
///
/// # Methods
///
/// The methods are the same for all unsigned primitives.
/// The following list of methods links to the `u8` implementation:
///
/// - [Constants and mask constructors](#constants-and-mask-constructors-for-u8)
///   - [mask_range](#method.mask_range) ([*checked*](#method.mask_range_checked)).
///   - [is_set_mask](#method.is_set_mask).
///   - [set_mask](#method.set_mask).
///   - [is_unset_mask](#method.is_unset_mask).
///   - [unset_mask](#method.unset_mask).
///
/// - [Get methods](#get-methods-for-u8)
///   - [get_range](#method.get_range) ([*checked*](#method.get_range_checked)).
///   - [get_value_range](#method.get_value_range) ([*checked*](#method.get_value_range_checked)).
///
/// - [Set ops](#set-ops-for-u8)
///   - [is_set](#method.is_set) ([*checked*](#method.is_set_checked)).
///   - [set](#method.set) ([*checked*](#method.set_checked)).
///   - [is_set_range](#method.is_set_range) ([*checked*](#method.is_set_range_checked)).
///   - [set_range](#method.set_range) ([*checked*](#method.set_range_checked)).
///   - [set_all](#method.set_all).
///   - [set_value_range](#method.set_value_range)
///     ([*checked*](#method.set_value_range_checked),
///     [*checked_strict*](#method.set_value_range_checked_strict)).
///
/// - [Unset ops](#unset-ops-for-u8)
///   - [is_unset](#method.is_unset) ([*checked*](#method.is_unset_checked)).
///   - [unset](#method.unset) ([*checked*](#method.unset_checked)).
///   - [is_unset_range](#method.is_unset_range) ([*checked*](#method.is_unset_range_checked)).
///   - [unset_range](#method.unset_range) ([*checked*](#method.unset_range_checked)).
///   - [unset_all](#method.unset_all).
///
/// - [Flip ops](#flip-ops-for-u8)
///   - [flip](#method.flip) ([*checked*](#method.flip_checked)).
///   - [flip_range](#method.flip_range) ([*checked*](#method.flip_range_checked)).
///   - [flip_range_if](#method.flip_range_if) ([*checked*](#method.flip_range_if_checked)).
///
/// - [Reverse ops](#reverse-ops-for-u8)
///   - [reverse_range](#method.reverse_range) ([*checked*](#method.reverse_range_checked)).
///
/// - [Count ops](#count-ops-for-u8)
///   - [count_ones_range](#method.count_ones_range)
///     ([*checked*](#method.count_ones_range_checked)).
///   - [count_zeros_range](#method.count_zeros_range)
///     ([*checked*](#method.count_zeros_range_checked)).
///
/// - [Find ops](#find-ops-for-u8)
///   - [find_first_one_range](#method.find_first_one_range)
///     ([*checked*](#method.find_first_one_range_checked)).
///   - [find_first_zero_range](#method.find_first_zero_range)
///     ([*checked*](#method.find_first_zero_range_checked)).
///   - [find_last_one_range](#method.find_last_one_range)
///     ([*checked*](#method.find_last_one_range_checked)).
///   - [find_last_zero_range](#method.find_last_zero_range)
///     ([*checked*](#method.find_last_zero_range_checked)).
#[must_use]
#[repr(transparent)]
pub struct Bitwise<T>(pub T);

#[rustfmt::skip]
mod core_impls {
    use crate::{impl_trait, Bitwise, Hash, Hasher, Ordering};

    impl<T: Clone> Clone for Bitwise<T> { fn clone(&self) -> Self { Self(self.0.clone()) } }
    impl<T: Copy> Copy for Bitwise<T> {}

    impl_trait![fmt::Debug for Bitwise[T][T] where T |self, f|
        f.debug_tuple("Bitwise").field(&self.0).finish()
    ];
    impl_trait![fmt::Display for Bitwise[T][T] where T |self, f| self.0.fmt(f)];
    impl_trait![fmt::Binary for Bitwise[T][T] where T |self, f| self.0.fmt(f)];
    impl_trait![fmt::Octal for Bitwise[T][T] where T |self, f| self.0.fmt(f)];
    impl_trait![fmt::LowerHex for Bitwise[T][T] where T |self, f| self.0.fmt(f)];
    impl_trait![fmt::UpperHex for Bitwise[T][T] where T |self, f| self.0.fmt(f)];
    impl_trait![fmt::LowerExp for Bitwise[T][T] where T |self, f| self.0.fmt(f)];
    impl_trait![fmt::UpperExp for Bitwise[T][T] where T |self, f| self.0.fmt(f)];

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
