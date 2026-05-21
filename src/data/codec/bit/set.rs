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
/// One or more custom `impl { ... }` blocks may be provided after the declarations.
/// They are emitted before all generated associated constants and methods,
/// and may still refer to them through `Self`.
///
/// # Generated methods
///
/// For a generated set type `Set`, the macro defines the following methods.
///
/// ## Core construction and access
/// - `new()`
/// - `from_bits(bits)`
/// - `bits()`
/// - `all()`
///
/// ## Core predicates
/// - `is_empty()`
/// - `is_full()`
/// - `contains(other)`
/// - `has(other)`
/// - `intersects(other)`
/// - `is_subset(other)`
/// - `is_superset(other)`
///
/// `contains` and `has` return whether all bits in `other` are present.
/// `has` is a shorter alias intended for flag-like use.
///
/// ## By-value modification
/// - `with(other)`
/// - `without(other)`
/// - `toggled(other)`
/// - `with_if(condition, other)`
///
/// ## Set algebra
/// - `union(other)`
/// - `intersection(other)`
/// - `difference(other)`
/// - `symmetric_difference(other)`
///
/// ## In-place modification
/// - `clear()`
/// - `insert(other)`
/// - `remove(other)`
/// - `toggle(other)`
///
/// ## Per-constant methods
///
/// For each named constant `NAME`, the macro generates:
/// - `contains_name()`
/// - `intersects_name()`
/// - `with_name()`
/// - `with_name_if(condition)`
/// - `without_name()`
/// - `set_name()`
/// - `set_name_if(condition)`
/// - `unset_name()`
///
/// For constants declared as a single bit, it also generates:
/// - `has_name()`
///
/// For grouped constants, use `contains_name()` for “all bits are present”
/// and `intersects_name()` for “at least one bit is present”.
///
/// # Example
/// ```
/// # use devela::set;
/// set! {
///     /// A small set example.
///     pub struct SmallSet(u16) {
///         /// A single bit.
///         A = 0;
///
///         /// Another single bit.
///         B = 1;
///
///         /// A grouped constant from explicit bits.
///         AB = 0, 1;
///
///         /// A grouped constant from an inclusive range.
///         BC = 1..=2;
///
///         /// A grouped constant from mixed bits and ranges.
///         MIXED = 0, 3..=5, 7;
///     }
///     /// Custom semantic helpers emitted before generated methods.
///     impl {
///         /// A custom named combination.
///         pub const ABC: Self = Self::AB.with_bc();
///
///         pub const fn contains_abc(self) -> bool { self.contains(Self::ABC) }
///     }
/// }
///
/// let mut s = SmallSet::new()
///     .with_a()
///     .with_mixed_if(true);
///
/// assert!(s.has_a());
/// assert!(s.contains_mixed());
/// assert!(s.intersects_ab());
///
/// s.unset_a();
/// assert!(!s.has_a());
/// ```
///
/// - Some examples of structs defined with the `set!` macro are:
///   [`AsciiSet`] and [`EventButtons`].
/// - Some other macros that leverages `set!` are:
///   [`bitfield!`][crate::bitfield], [`enumset!`][crate::enumset].
///
/// [`AsciiSet`]: crate::AsciiSet
/// [`EventButtons`]: crate::EventButtons
/// [`enumset!`]: crate::enumset
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! set· {
    {
        $(#[$struct_attrs:meta])*
        $vis:vis struct $Set:ident($T:ty) {
            $(
                $(#[$f_attrs:meta])*
                $f:ident = $( $start:tt $(..= $end:tt)? ),+ $(,)?;
            )*
        }
        $( // optional impl blocks
            $(#[$impl_attrs:meta])*
            impl { $($user_impl:item)* }
        )*
    } => {
        $crate::set!(%guard_allowed_type $Set, $T);

        $( #[$struct_attrs] )*
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
        $vis struct $Set { bits: $T }

        $(
            $( #[$impl_attrs] )*
            impl $Set { $($user_impl)* }
        )*

        /// Named constants
        impl $Set { $crate::paste! { $(
            // single-bit constants
            #[$crate::compile[all(none($($($end)?)+), xone($(some($start)),+))]]
            $crate::items! {
                /// <span class='stab portability' title='single-bit constant'>`1`</span>
                $(#[$f_attrs])*
                #[allow(non_upper_case_globals)]
                $vis const $f: Self = Self {
                    bits: $crate::set!(%mask $T; $( $start $(..= $end)? ),+)
                };
            }
            // grouped constants
            #[$crate::compile[any(some($($($end)?)+), not(xone($(some($start)),+)))]]
            $crate::items! {
                /// <span class='stab portability' title='grouped constant'>`+`</span>
                $(#[$f_attrs])*
                #[allow(non_upper_case_globals)]
                $vis const $f: Self = Self {
                    bits: $crate::set!(%mask $T; $( $start $(..= $end)? ),+)
                };
            }

            /* per-constant metadata */
            // #[doc(hidden)]
            // #[doc = "The number of bits in `" $f "`."]
            #[allow(non_upper_case_globals)]
            pub(crate) const [<_ $f _BITS>]: u32 = Self::$f.bits.count_ones();
            // #[doc = "Whether `" $f "` is composed of a single bit."]
            // #[allow(non_upper_case_globals)]
            // pub(crate) const [<_ $f _IS_SINGLE>]: bool = Self::[<_ $f _BITS>] == 1;
        )* }}

        /// Convenience methods derived from each named constant.
        //
        // Note: The first method links to the named constant to serve as a group marker
        impl $Set { $crate::paste! { $(
            // for single-bit constants
            #[$crate::compile[all(none($($($end)?)+), xone($(some($start)),+))]]
            $crate::items! {
                #[doc = "Returns `true` if [`" $f "`][Self::" $f "] is present."]
                $vis const fn [<has_ $f:lower>](self) -> bool { self.contains(Self::$f) }
                #[doc = "Returns `true` if all bits in `" $f "` are present."]
                $vis const fn [<contains_ $f:lower>](self) -> bool { self.contains(Self::$f) }
            }
            // for grouped constants
            #[$crate::compile[any(some($($($end)?)+), not(xone($(some($start)),+)))]]
            $crate::items! {
                #[doc = "Returns `true` if the set contains all bits in [`" $f "`][Self::" $f "]."]
                $vis const fn [<contains_ $f:lower>](self) -> bool { self.contains(Self::$f) }
            }
            #[doc = "Returns `true` if the set shares any bit with `" $f "`."]
            $vis const fn [<intersects_ $f:lower>](self) -> bool { self.intersects(Self::$f) }

            #[doc = "Returns `self` with `" $f "` inserted."]
            $vis const fn [<with_ $f:lower>](self) -> Self { self.with(Self::$f) }
            #[doc = "Returns `self` with `" $f "` inserted if `condition` is true."]
            $vis const fn [<with_ $f:lower _if>](self, condition: bool) -> Self {
                self.with_if(condition, Self::$f) }
            #[doc = "Returns `self` with `" $f "` removed."]
            $vis const fn [<without_ $f:lower>](self) -> Self { self.without(Self::$f) }

            /* mutating */
            #[doc = "Sets `" $f "`."]
            $vis const fn [<set_ $f:lower>](&mut self) { self.insert(Self::$f); }
            #[doc = "Sets `" $f "` if `condition` is true."]
            $vis const fn [<set_ $f:lower _if>](&mut self, condition: bool) {
                if condition { self.insert(Self::$f); } }
            #[doc = "Unsets `" $f "`."]
            $vis const fn [<unset_ $f:lower>](&mut self) { self.remove(Self::$f); }
        )* }}

        /// Common set methods
        #[allow(clippy::double_must_use)]
        impl $Set {
            /// Returns an empty set.
            #[must_use]
            $vis const fn new() -> Self { Self { bits: 0 } }
            /// Returns a set from raw bits.
            #[must_use]
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
            $vis const fn contains(self, other: Self) -> bool { self.is_superset(other) }
            /// An alias of `contains`.
            #[must_use]
            $vis const fn has(self, other: Self) -> bool { self.contains(other) }

            /// Returns `self` with `other` inserted.
            #[must_use]
            $vis const fn with(self, other: Self) -> Self {
                Self { bits: self.bits | other.bits }
            }
            /// Returns `self` with `other` inserted if `condition` is true.
            #[must_use]
            $vis const fn with_if(self, condition: bool, other: Self) -> Self {
                if condition { Self { bits: self.bits | other.bits } } else { self }
            }
            /// Returns `self` with `other` removed.
            #[must_use]
            $vis const fn without(self, other: Self) -> Self {
                Self { bits: self.bits & !other.bits }
            }
            /// Returns `self` with `other` toggled.
            #[must_use]
            $vis const fn toggled(self, other: Self) -> Self {
                Self { bits: self.bits ^ other.bits }
            }
        }

        /// Set operations
        #[allow(clippy::double_must_use)]
        impl $Set {
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

            /// Returns the union of `self` and `other`.
            #[must_use]
            $vis const fn union(self, other: Self) -> Self {
                Self { bits: self.bits | other.bits }
            }
            /// Returns the intersection of `self` and `other`.
            #[must_use]
            $vis const fn intersection(self, other: Self) -> Self {
                Self { bits: self.bits & other.bits }
            }
            /// Returns the difference of `self` and `other`.
            #[must_use]
            $vis const fn difference(self, other: Self) -> Self {
                Self { bits: self.bits & !other.bits }
            }
            /// Returns the symmetric difference of `self` and `other`.
            #[must_use]
            $vis const fn symmetric_difference(self, other: Self) -> Self {
                Self { bits: self.bits ^ other.bits }
            }
        }

        /// Mutating set operations
        impl $Set {
            /// Clears the set.
            $vis const fn clear(&mut self) { self.bits = 0; }
            /// Inserts all bits from `other`.
            $vis const fn insert(&mut self, other: Self) { self.bits |= other.bits; }
            /// Removes all bits from `other`.
            $vis const fn remove(&mut self, other: Self) { self.bits &= !other.bits; }
            /// Toggles all bits from `other`.
            $vis const fn toggle(&mut self, other: Self) { self.bits ^= other.bits; }
        }

        /* impl traits */

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
    // reduces a comma-list of bits/ranges into one backing integer mask.
    (%mask $T:ty; $($start:tt $(..= $end:tt)?),+ $(,)?) => {{
        0 as $T $(| $crate::set!(%item $T; $start $(..= $end)?))+
    }};
    // single bit.
    (%item $T:ty; $bit:tt) => {
        $crate::Bitwise::<$T>::mask_range(($bit) as u32, ($bit) as u32).0
    };
    // inclusive bit range.
    (%item $T:ty; $start:tt ..= $end:tt) => {
        $crate::Bitwise::<$T>::mask_range(($start) as u32, ($end) as u32).0
    };
    // only allow implementions over unsigned integers
    (%guard_allowed_type $Set:ident, $T:ty) => { $crate::paste! {
        const [<__GUARD_ALLOWED_TYPE_ $Set:upper>]: () = {
            const fn __allowed_types<P: $crate::PrimUint>() {}
            __allowed_types::<$T>();
        };
    }};
    ($($tt:tt)*) => { compile_error! { concat![
        "set!: wrong syntax.\n\n",
        "Expected:\n",
        "    set! { $vis struct Name($ty) { CONST = bits; ... } }\n\n",
        "Examples of constant syntax:\n",
        "    SPACE = b' ';\n",
        "    HWS   = b' ', b'\\t';\n",
        "    DIGIT = b'0'..=b'9';\n",
        "    ALNUM = b'0'..=b'9', b'A'..=b'Z', b'a'..=b'z';",
        // "Input:\n",
        // "    ",
        // stringify!($($tt)*)
    ]}};
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
    #[test]
    fn generic_convenience_methods_work() {
        let s =
            TestSet::new().with(TestSet::A).with_if(true, TestSet::C).with_if(false, TestSet::D);
        assert!(s.has(TestSet::A));
        assert!(s.contains(TestSet::C));
        assert!(!s.has(TestSet::D));
        assert_eq!(s.without(TestSet::A).bits(), TestSet::C.bits());
        assert_eq!(s.toggled(TestSet::C).bits(), TestSet::A.bits());
    }
    #[test]
    fn per_constant_methods_work_for_single_bit_constants() {
        let mut s = TestSet::new();
        assert!(!s.has_a());
        assert!(!s.contains_a());
        assert!(!s.intersects_a());
        s.set_a();
        assert!(s.has_a());
        assert!(s.contains_a());
        assert!(s.intersects_a());
        s.set_b_if(true);
        s.set_c_if(false);
        assert!(s.has_b());
        assert!(!s.has_c());
        assert_eq!(s.without_a().bits(), TestSet::B.bits());
        s.unset_b();
        assert!(!s.is_empty());
        s.unset_a();
        assert!(s.is_empty());
    }
    #[test]
    fn per_constant_methods_work_for_grouped_constants() {
        let s = TestSet::AB;
        assert!(s.contains_ab());
        assert!(s.intersects_ab());
        assert!(!TestSet::A.contains_ab());
        assert!(TestSet::A.intersects_ab());
        assert_eq!(TestSet::new().with_ab().bits(), TestSet::AB.bits());
        assert_eq!(TestSet::AB.without_b().bits(), TestSet::A.bits());
        let mut s = TestSet::new();
        s.set_mixed();
        assert!(s.contains_mixed());
        s.unset_ad();
        assert_eq!(s.bits(), TestSet::MIXED.without(TestSet::AD).bits());
    }
    #[test]
    fn generated_metadata_counts_bits() {
        assert_eq!(TestSet::_A_BITS, 1);
        assert_eq!(TestSet::_B_BITS, 1);
        assert_eq!(TestSet::_AB_BITS, 2);
        assert_eq!(TestSet::_BC_BITS, 2);
        assert_eq!(TestSet::_AD_BITS, 2);
        assert_eq!(TestSet::_MIXED_BITS, 5);
    }
    #[test]
    fn only_single_bit_constants_get_has_aliases() {
        let _ = TestSet::A.has_a();
        let _ = TestSet::B.has_b();
    }
}

// The following tests intentionally should not compile
#[cfg(doctest)]
mod doctests {
    /**
    ```compile_fail
    devela::set! { struct TestSet(u16) { A = 0; B = 1; C = 2; AB = 0, 1; BC = 1..=2; } }
    let _ = TestSet::AB.has_ab();
    ```
    ```compile_fail
    devela::set! { struct TestSet(u16) { A = 0; B = 1; C = 2; AB = 0, 1; BC = 1..=2; } }
    let _ = TestSet::BC.has_bc();
    ```
    **/
    fn grouped_constants_do_not_have_has_aliases() {}
}
