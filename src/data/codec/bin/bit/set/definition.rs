// devela::data::codec::bit::bin::set::definition
//
//! Defines [`set!`]
//

#[doc = crate::_tags!(construction bit data_structure)]
/// Defines a compact set backed by an integer bit mask.
#[doc = crate::_doc_meta!{location("data/codec")}]
///
#[doc = include_str!["./_docs_set.md"]]
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
        #[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

            /* per-constant private metadata */
            /// The number of bits in $f.
            #[allow(non_upper_case_globals)]
            pub(crate) const [<_ $f _BITS>]: u32 = Self::$f.bits.count_ones();
            // /// Whether $f is composed of a single bit.
            // #[allow(non_upper_case_globals)]
            // pub(crate) const [<_ $f _IS_SINGLE>]: bool = Self::[<_ $f _BITS>] == 1;
        )* }}

        /// Convenience methods derived from each named constant.
        //
        // Note: The first method links to the named constant to serve as a group marker
        #[allow(dead_code)]
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
        #[allow(clippy::double_must_use, dead_code)]
        impl $Set {
            /* per-set private metadata */
            /// The binary width needed to show all declared bits in `Debug`.
            pub(crate) const _SET_DEBUG_WIDTH: usize = {
                let bits = Self::all().bits;
                if bits == 0 { 1 } else { (<$T>::BITS - bits.leading_zeros()) as usize }
            };
            /// The bit width of the declared set domain.
            pub(crate) const _SET_BIT_SIZE: usize = {
                let bits = Self::all().bits;
                if bits == 0 { 0 } else { (<$T>::BITS - bits.leading_zeros()) as usize }
            };
            /// The octal digit width needed to show all declared bits.
            pub(crate) const _SET_OCTAL_WIDTH: usize = Self::_SET_DEBUG_WIDTH.div_ceil(3);
            /// The hexadecimal digit width needed to show all declared bits.
            pub(crate) const _SET_HEX_WIDTH: usize = Self::_SET_DEBUG_WIDTH.div_ceil(4);
            // _FIELD_COUNT, _SINGLE_COUNT, _GROUP_COUNT, _FIELD_NAMES

            /* public methods */

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
        #[allow(clippy::double_must_use, dead_code)]
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
        #[allow(dead_code)]
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

        impl $crate::BitSized<{ $Set::_SET_BIT_SIZE }> for $Set {}
        impl $crate::ConstInit for $Set { const INIT: Self = Self::new(); }
        // formatting
        $crate::impl_trait! { fmt::Debug for
            /// Formats the set as a prefixed binary mask over its declared bit domain.
            $Set |self, f| write!(f, concat!(stringify!($Set),
                "(0b{:0w$b})"), self.bits, w = Self::_SET_DEBUG_WIDTH) }
        $crate::impl_trait! { fmt::Display for
            /// Formats the set as a prefixed binary mask over its declared bit domain.
            $Set |self, f| write!(f, "0b{:0w$b}", self.bits, w = Self::_SET_DEBUG_WIDTH)
        }
        $crate::impl_trait! { fmt::Binary for
            /// Formats the set as a binary mask over its declared bit domain.
            $Set |self, f|
                if f.alternate() { write!(f, "0b{:0w$b}", self.bits, w = Self::_SET_DEBUG_WIDTH) }
                else { write!(f, "{:0w$b}", self.bits, w = Self::_SET_DEBUG_WIDTH) }
        }
        $crate::impl_trait! { fmt::Octal for
            /// Formats the set as an octal mask over its declared bit domain.
            $Set |self, f|
                if f.alternate() { write!(f, "0o{:0w$o}", self.bits, w = Self::_SET_OCTAL_WIDTH) }
                else { write!(f, "{:0w$o}", self.bits, w = Self::_SET_OCTAL_WIDTH) }
        }
        $crate::impl_trait! { fmt::LowerHex for
            /// Formats the set as a hexadecimal mask over its declared bit domain.
            $Set |self, f|
                if f.alternate() { write!(f, "0x{:0w$x}", self.bits, w = Self::_SET_HEX_WIDTH) }
                else { write!(f, "{:0w$x}", self.bits, w = Self::_SET_HEX_WIDTH) }
        }
        $crate::impl_trait! { fmt::UpperHex for
            /// Formats the set as a hexadecimal mask over its declared bit domain.
            $Set |self, f|
                if f.alternate() { write!(f, "0x{:0w$X}", self.bits, w = Self::_SET_HEX_WIDTH) }
                else { write!(f, "{:0w$X}", self.bits, w = Self::_SET_HEX_WIDTH) }
        }

        impl $crate::DebugExt for $Set {
            type Ctx = $crate::ReprMode;
            fn fmt_with(&self, f: &mut $crate::Formatter<'_>, ctx: &Self::Ctx)
                -> $crate::FmtResult<()> {
                match ctx {
                    $crate::ReprMode::Raw => $crate::Debug::fmt(self, f),
                    $crate::ReprMode::Named => { self._fmt_named(f) }
                    $crate::ReprMode::RawNamed => {
                        write!(f, concat!(stringify!($Set), "("))?;
                        write!(f, "0b{:0width$b}", self.bits, width = Self::_SET_DEBUG_WIDTH)?;
                        write!(f, "; ")?;
                        self._fmt_named_inner(f)?;
                        write!(f, ")")
                    }
                }
            }
        }
        impl $Set {
            fn _fmt_named(&self, f: &mut $crate::Formatter<'_>) -> $crate::FmtResult<()> {
                write!(f, concat!(stringify!($Set), "("))?;
                self._fmt_named_inner(f)?;
                write!(f, ")")
            }
            fn _fmt_named_inner(&self, f: &mut $crate::Formatter<'_>) -> $crate::FmtResult<()> {
                let mut first = true;
                $crate::paste! { $(
                    // for now, only show single-bit fields
                    if $crate::cif![all(none($($($end)?)+), xone($(some($start)),+))]
                        && self.contains(Self::$f)
                    {
                        if !first { write!(f, " | ")?; }
                        write!(f, stringify!($f))?;
                        first = false;
                    }
                )* }
                if first { write!(f, "empty")?; }
                Ok(())
            }
        }

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
