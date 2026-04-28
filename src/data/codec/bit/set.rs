// devela::data::codec::bit::set
//
//! Defines [`set!`]
//

#[doc = crate::_tags!(construction bit data_structure)]
/// Defines a compact set backed by an integer bit mask.
#[doc = crate::_doc_location!("data/codec")]
///
/// Each named constant represents one or more bits.
/// Constants may be defined from single bit indices,
/// inclusive bit ranges, or comma-separated mixtures of both.
///
/// The generated struct derives
/// `Clone`, `Copy`, `Debug`, `Default`, `PartialEq`, `Eq` and `Hash`.
///
/// # Example
/// ```
/// # use devela::set;
/// set! {
///     /// A small test set.
///     pub struct SmallSet(u8) {
///         A = 0;
///         B = 1;
///         C = 2;
///         AB = 0, 1;
///         HIGH = 4..=7;
///         MIXED = 0, 3..=5, 7;
///     }
/// }
/// let mut set = SmallSet::A | SmallSet::B;
/// assert_eq!(SmallSet::A.bits(), 0b0000_0001);
/// assert_eq!(SmallSet::AB.bits(), 0b0000_0011);
/// assert_eq!(SmallSet::HIGH.bits(), 0b1111_0000);
/// assert_eq!(SmallSet::MIXED.bits(), 0b1011_1001);
///
/// assert!(set.contains(SmallSet::A));
/// assert!(set.intersects(SmallSet::AB));
///
/// set.insert(SmallSet::C);
/// assert_eq!(set.bits(), 0b0000_0111);
///
/// set.remove(SmallSet::A);
/// assert_eq!(set, SmallSet::B | SmallSet::C);
/// ```
/// See also the [`enumset!`][crate::enumset] macro.
#[doc(hidden)]
#[macro_export]
macro_rules! set· {
    {
        $(#[$struct_attrs:meta])*
        $vis:vis struct $Set:ident($T:ty) {
            $(
                $(#[$f_attrs:meta])*
                $f:ident = $( $start:tt $(..= $end:tt)? ),+ $(,)?;
            )*
        }
    } => {
        $( #[$struct_attrs] )*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[must_use]
        $vis struct $Set { bits: $T }

        $crate::set!(%guard_allowed_type $T);

        /// Common methods
        impl $Set {
            /// Returns an empty set.
            $vis const fn new() -> Self { Self { bits: 0 } }
            /// Returns a set from raw bits.
            $vis const fn from_bits(bits: $T) -> Self { Self { bits } }

            /// Returns the raw backing bits.
            #[must_use]
            $vis const fn bits(self) -> $T { self.bits }
            /// Returns the union of all declared sets.
            #[must_use]
            $vis const fn all() -> Self { Self { bits: 0 $(| Self::$f.bits)* } }

            /// Returns `true` if no bits are set.
            #[must_use]
            $vis const fn is_empty(self) -> bool { self.bits == 0 }
            /// Returns `true` if all the declared bits are set.
            #[must_use]
            $vis const fn is_full(self) -> bool { self.bits == Self::all().bits }
            /// Returns `true` if all bits in `other` are also set in `self`.
            #[must_use]
            $vis const fn contains(self, other: Self) -> bool {
                self.is_superset(other)
            }

            /// Returns `self` with `other` inserted.
            $vis const fn with(self, other: Self) -> Self {
                Self { bits: self.bits | other.bits }
            }
            /// Returns `self` with `other` removed.
            $vis const fn without(self, other: Self) -> Self {
                Self { bits: self.bits & !other.bits }
            }
            /// Returns `self` with `other` toggled.
            $vis const fn toggled(self, other: Self) -> Self {
                Self { bits: self.bits ^ other.bits }
            }
        }

        /// Set operations.
        impl $Set {
            /// Returns the union of `self` and `other`.
            $vis const fn union(self, other: Self) -> Self {
                Self { bits: self.bits | other.bits }
            }
            /// Returns the intersection of `self` and `other`.
            $vis const fn intersection(self, other: Self) -> Self {
                Self { bits: self.bits & other.bits }
            }
            /// Returns the difference of `self` and `other`.
            $vis const fn difference(self, other: Self) -> Self {
                Self { bits: self.bits & !other.bits }
            }
            /// Returns the symmetric difference of `self` and `other`.
            $vis const fn symmetric_difference(self, other: Self) -> Self {
                Self { bits: self.bits ^ other.bits }
            }

            /// Returns `true` if `self` and `other` share any bit.
            $vis const fn intersects(self, other: Self) -> bool {
                (self.bits & other.bits) != 0
            }
            /// Returns `true` if all bits in `self` are also set in `other`.
            $vis const fn is_subset(self, other: Self) -> bool {
                (self.bits & other.bits) == self.bits
            }
            /// Returns `true` if all bits in `other` are also set in `self`.
            $vis const fn is_superset(self, other: Self) -> bool {
                (self.bits & other.bits) == other.bits
            }
        }

        /// Mutating set operations.
        impl $Set {
            /// Inserts all bits from `other`.
            $vis const fn insert(&mut self, other: Self) { self.bits |= other.bits; }
            /// Removes all bits from `other`.
            $vis const fn remove(&mut self, other: Self) { self.bits &= !other.bits; }
            /// Toggles all bits from `other`.
            $vis const fn toggle(&mut self, other: Self) { self.bits ^= other.bits; }
            /// Clears the set.
            $vis const fn clear(&mut self) { self.bits = 0; }
        }

        /// Named constants
        impl $Set {
            $(
                $(#[$f_attrs])*
                $vis const $f: Self = Self {
                    bits: $crate::set!(%mask $T; $( $start $(..= $end)? ),+)
                };
            )*
        }

        /* impl traits */

        impl Default for $Set { fn default() -> Self { Self::new() } }
        impl $crate::ConstInit for $Set { const INIT: Self = Self::new(); }
        $crate::impl_trait![fmt::Display for $Set |self, f| $crate::Display::fmt(&self.bits, f)];
        $crate::impl_trait![fmt::Binary for $Set |self, f| $crate::Binary::fmt(&self.bits, f)];
        $crate::impl_trait![fmt::Octal for $Set |self, f| $crate::Octal::fmt(&self.bits, f)];
        $crate::impl_trait![fmt::LowerHex for $Set |self, f| $crate::LowerHex::fmt(&self.bits, f)];
        $crate::impl_trait![fmt::UpperHex for $Set |self, f| $crate::UpperHex::fmt(&self.bits, f)];
        $crate::impl_trait![fmt::LowerExp for $Set |self, f| $crate::LowerExp::fmt(&self.bits, f)];
        $crate::impl_trait![fmt::UpperExp for $Set |self, f| $crate::UpperExp::fmt(&self.bits, f)];

        /* operator implementations */

        impl ::core::ops::BitOr for $Set {
            type Output = Self;
            fn bitor(self, rhs: Self) -> Self::Output { self.union(rhs) }
        }
        impl ::core::ops::BitOrAssign for $Set {
            fn bitor_assign(&mut self, rhs: Self) { self.insert(rhs); }
        }
        impl ::core::ops::BitAnd for $Set {
            type Output = Self;
            fn bitand(self, rhs: Self) -> Self::Output { self.intersection(rhs) }
        }
        impl ::core::ops::BitAndAssign for $Set {
            fn bitand_assign(&mut self, rhs: Self) { *self = self.intersection(rhs); }
        }
        impl ::core::ops::Sub for $Set {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output { self.difference(rhs) }
        }
        impl ::core::ops::SubAssign for $Set {
            fn sub_assign(&mut self, rhs: Self) { self.remove(rhs); }
        }
        impl ::core::ops::BitXor for $Set {
            type Output = Self;
            fn bitxor(self, rhs: Self) -> Self::Output { self.symmetric_difference(rhs) }
        }
        impl ::core::ops::BitXorAssign for $Set {
            fn bitxor_assign(&mut self, rhs: Self) { self.toggle(rhs); }
        }
        impl ::core::ops::Not for $Set {
            type Output = Self;
            fn not(self) -> Self::Output { Self::all().difference(self) }
        }
    };
    // Reduces a comma-list of bits/ranges into one backing integer mask.
    (%mask $T:ty; $($start:tt $(..= $end:tt)?),+ $(,)?) => {{
        0 as $T $(| $crate::set!(%item $T; $start $(..= $end)?))+
    }};
    // Single bit.
    (%item $T:ty; $bit:tt) => {
        $crate::Bitwise::<$T>::mask_range(($bit) as u32, ($bit) as u32).0
    };
    // Inclusive bit range.
    (%item $T:ty; $start:tt ..= $end:tt) => {
        $crate::Bitwise::<$T>::mask_range(($start) as u32, ($end) as u32).0
    };
    // only allow implementions over unsigned integers
    (%guard_allowed_type $T:ty) => {
        const __GUARD_ALLOWED_TYPE: () = {
            const fn __allowed_types<P: $crate::PrimUint>() {}
            __allowed_types::<$T>();
        };
    };
}
#[doc(inline)]
pub use set· as set;

#[cfg(test)]
mod tests {
    #![allow(unused)]

    crate::set! {
        struct TestSet(u16) {
            A = 0;
            B = 1;
            C = 2;
            D = 3;
            AB = 0, 1;
            BC = 1..=2;
            AD = 0, 3;
            MIXED = 0, 3..=5, 7;
        }
    }

    #[test]
    fn constants_accept_bits_and_ranges() {
        assert_eq!(TestSet::A.bits(), 0b0000_0001);
        assert_eq!(TestSet::B.bits(), 0b0000_0010);
        assert_eq!(TestSet::AB.bits(), 0b0000_0011);
        assert_eq!(TestSet::BC.bits(), 0b0000_0110);
        assert_eq!(TestSet::AD.bits(), 0b0000_1001);
        assert_eq!(TestSet::MIXED.bits(), 0b1011_1001);
    }

    #[test]
    fn mathematical_set_operations() {
        let a = TestSet::AB; // 0, 1
        let b = TestSet::BC; // 1, 2

        assert_eq!(a.union(b).bits(), 0b0000_0111);
        assert_eq!(a.intersection(b).bits(), 0b0000_0010);
        assert_eq!(a.difference(b).bits(), 0b0000_0001);
        assert_eq!(b.difference(a).bits(), 0b0000_0100);
        assert_eq!(a.symmetric_difference(b).bits(), 0b0000_0101);

        assert!(a.intersects(b));
        assert!(!TestSet::A.intersects(TestSet::C));

        assert!(TestSet::A.is_subset(a));
        assert!(a.is_superset(TestSet::A));
        assert!(a.contains(TestSet::A));

        assert!(!a.is_subset(TestSet::A));
        assert!(!TestSet::A.is_superset(a));
        assert!(!TestSet::A.contains(a));
    }

    #[test]
    fn mutating_core_operations_match_set_operations() {
        let mut s = TestSet::A;

        s.insert(TestSet::C);
        assert_eq!(s.bits(), TestSet::A.union(TestSet::C).bits());

        s.toggle(TestSet::A);
        assert_eq!(s.bits(), TestSet::C.bits());

        s.remove(TestSet::C);
        assert!(s.is_empty());
    }
}
