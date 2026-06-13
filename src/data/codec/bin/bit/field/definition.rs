// devela/src/data/codec/bin/bit/field/definition.rs
//
//! Defines [`bitfield!`]
//

#[doc = crate::_tags!(construction bit mem)]
/// Defines a compact packed-field wrapper backed by an unsigned integer.
#[doc = crate::_doc_meta!{location("data/codec")}]
///
#[doc = include_str!["./_docs_field.md"]]
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! bitfield· {
    {
        $(#[$struct_attrs:meta])*
        $vis:vis struct $Bitfield:ident($T:ty) {
            $(
                $(#[$f_attrs:meta])*
                $f:ident = $start:tt $(..= $end:tt)?;
            )*
        }
        $( traits( $( ! $no_trait:ident ),* $(,)? ); )?
        $( // optional impl blocks
            $(#[$impl_attrs:meta])*
            impl { $($user_impl:item)* }
        )*
    } => {
        $crate::bitfield!(%guard_allowed_type $Bitfield, $T);

        $(#[$struct_attrs])*
        #[repr(transparent)]
        #[derive(Clone, Copy)]
        $vis struct $Bitfield { bits: $T, }
        $(
            $( #[$impl_attrs] )*
            impl $Bitfield { $($user_impl)* }
        )*

        // private metadata
        impl $Bitfield { $crate::paste! {
            pub(crate) const _BITFIELD_MASK: $T = 0 as $T $(| Self::[<_ $f _MASK>])*;
            pub(crate) const _BITFIELD_FIELD_WIDTH_SUM: u32 = 0 $(+ Self::[<_ $f _WIDTH>])*;
            pub(crate) const _BITFIELD_BIT_WIDTH: usize = {
                let bits = Self::_BITFIELD_MASK;
                if bits == 0 { 1 } else { (<$T>::BITS - bits.leading_zeros()) as usize }
            };
            pub(crate) const _BITFIELD_BIT_SIZE: usize = {
                let bits = Self::_BITFIELD_MASK;
                if bits == 0 { 0 } else { (<$T>::BITS - bits.leading_zeros()) as usize }
            };
            pub(crate) const _BITFIELD_OCTAL_WIDTH: usize = Self::_BITFIELD_BIT_WIDTH.div_ceil(3);
            pub(crate) const _BITFIELD_HEX_WIDTH: usize = Self::_BITFIELD_BIT_WIDTH.div_ceil(4);
        }}
        const _: () = {
            assert!(
                $Bitfield::_BITFIELD_FIELD_WIDTH_SUM == $Bitfield::_BITFIELD_MASK.count_ones(),
                "bitfield!: fields must not overlap");
        };

        /// # Named bit field masks.
        ///
        /// These constants represent each field's occupied bit range, not a field value.
        ///
        /// Use them with mask-level methods such as
        /// [`contains_mask`], [`with_mask`] and [`without_mask`].
        ///
        /// [`contains_mask`]: Self::contains_mask
        /// [`with_mask`]: Self::with_mask
        /// [`without_mask`]: Self::without_mask
        impl $Bitfield { $crate::paste! { $(
            // single-bit fields
            #[$crate::compile[none($($end)?)]]
            $crate::items! {
                #[doc = "<span class='stab portability' title='single-bit field methods'>\
                    <a href='#methods-for-" $f:lower "'><code>1</code></a></span>"]
                $(#[$f_attrs])*
                #[allow(non_upper_case_globals)]
                $vis const $f: Self = Self { bits: Self::[<_ $f _MASK>], };
            }
            // multi-bit fields
            #[$crate::compile[some($($end)?)]]
            $crate::items! {
                #[doc = "<span class='stab portability' title='multi-bit field methods'>\
                    <a href='#methods-for-" $f:lower "'><code>+</code></a></span>"]
                $(#[$f_attrs])*
                #[allow(non_upper_case_globals)]
                $vis const $f: Self = Self { bits: Self::[<_ $f _MASK>], };
            }
        )* }}

        // public methods
        impl $Bitfield {
            /// Returns a new bitfield with all bits cleared.
            #[must_use] #[allow(clippy::double_must_use, dead_code)]
            $vis const fn new() -> Self { Self { bits: 0 } }
            /// Returns a new bitfield from raw bits.
            #[must_use] #[allow(clippy::double_must_use, dead_code)]
            $vis const fn from_bits(bits: $T) -> Self { Self { bits } }

            /// Returns the raw backing bits.
            #[must_use] #[allow(dead_code)]
            $vis const fn bits(self) -> $T { self.bits }
            /// Returns the raw mask covering all declared fields.
            #[must_use] #[allow(dead_code)]
            $vis const fn mask() -> $T { Self::_BITFIELD_MASK }

            /// Returns a bitfield value with every declared field bit set.
            #[must_use] #[allow(clippy::double_must_use, dead_code)]
            $vis const fn all() -> Self { Self { bits: Self::_BITFIELD_MASK } }

            /// Returns `true` if all raw bits are clear.
            #[must_use] #[allow(dead_code)]
            $vis const fn is_empty(self) -> bool { self.bits == 0 }
            /// Returns `true` if the raw bits exactly equal the declared field mask.
            #[must_use] #[allow(dead_code)]
            $vis const fn is_full(self) -> bool { self.bits == Self::all().bits }

            /// Returns `true` if all declared field bits are clear.
            #[must_use] #[allow(dead_code)]
            pub const fn fields_are_zero(self) -> bool { self.bits & Self::_BITFIELD_MASK == 0 }
            /// Returns `true` if all declared field bits are set.
            #[must_use] #[allow(dead_code)]
            pub const fn fields_are_full(self) -> bool {
                self.bits & Self::_BITFIELD_MASK == Self::_BITFIELD_MASK
            }
            /// Returns `true` if all bits in `other` are set in `self`.
            ///
            /// This is a mask-level predicate. When used with a named field constant,
            /// it checks whether every bit occupied by that field is set.
            #[must_use] #[allow(dead_code)]
            pub const fn contains_mask(self, other: Self) -> bool {
                (self.bits & other.bits) == other.bits
            }
            /// Returns `true` if any bit outside the declared field mask is set.
            #[must_use] #[allow(dead_code)]
            pub const fn has_extra_bits(self) -> bool {
                (self.bits & Self::_BITFIELD_MASK) != self.bits
            }

            /// Clears all bits.
            #[allow(dead_code)]
            $vis const fn clear(&mut self) { self.bits = 0; }
            /// Clears all declared field bits, preserving bits outside the declared field mask.
            #[allow(dead_code)]
            pub const fn clear_fields(&mut self) { self.bits &= !Self::_BITFIELD_MASK; }
            /// Returns `self` with all declared field bits cleared, preserving extra bits.
            #[must_use] #[allow(clippy::double_must_use, dead_code)]
            pub const fn without_fields(self) -> Self {
                Self { bits: self.bits & !Self::_BITFIELD_MASK }
            }

            /// Returns `self` with all bits in `other` set.
            ///
            /// This is a mask-level operation. Use `with_FIELD(value)` to replace a field value.
            #[must_use] #[allow(clippy::double_must_use, dead_code)]
            pub const fn with_mask(self, other: Self) -> Self {
                Self { bits: self.bits | other.bits }
            }
            /// Returns `self` with all bits in `other` set if `condition` is true.
            #[must_use] #[allow(clippy::double_must_use, dead_code)]
            pub const fn with_mask_if(self, condition: bool, other: Self) -> Self {
                if condition { self.with_mask(other) } else { self }
            }
            /// Returns `self` with all bits in `other` cleared.
            ///
            /// This is a mask-level operation.
            /// It clears the occupied bits, regardless of the current field value.
            #[must_use] #[allow(clippy::double_must_use, dead_code)]
            pub const fn without_mask(self, other: Self) -> Self {
                Self { bits: self.bits & !other.bits }
            }
            /// Returns `self` with all bits in `other` cleared if `condition` is true.
            #[must_use] #[allow(clippy::double_must_use, dead_code)]
            pub const fn without_mask_if(self, condition: bool, other: Self) -> Self {
                if condition { self.without_mask(other) } else { self }
            }
        }
        // impl per-field methods
        $( $crate::bitfield!(%field_methods $vis $Bitfield $T; $f; $start $(..= $end)?); )*

        /* impl utility traits */

        $( #[$crate::compile[nota(PartialEq $(, $no_trait)*)]] )?
        impl ::core::cmp::PartialEq for $Bitfield {
            fn eq(&self, other: &Self) -> bool { self.bits == other.bits }
        }
        $( #[$crate::compile[all(nota(Eq $(, $no_trait)*), nota(PartialEq $(, $no_trait)*))]] )?
        impl ::core::cmp::Eq for $Bitfield {}

        $( #[$crate::compile[nota(PartialOrd $(, $no_trait)*)]] )?
        #[allow(clippy::non_canonical_partial_ord_impl, reason = "would always need Ord")]
        impl ::core::cmp::PartialOrd for $Bitfield {
            fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering> {
                self.bits.partial_cmp(&other.bits) // Some(self.cmp(other))
            }
        }
        $( #[$crate::compile[all(
            nota(Ord $(, $no_trait)*),
            nota(PartialOrd $(, $no_trait)*),
            nota(Eq $(, $no_trait)*),
            nota(PartialEq $(, $no_trait)*)
        )]] )?
        impl ::core::cmp::Ord for $Bitfield {
            fn cmp(&self, other: &Self) -> ::core::cmp::Ordering { self.bits.cmp(&other.bits) }
        }
        $( #[$crate::compile[nota(Hash $(, $no_trait)*)]] )?
        impl ::core::hash::Hash for $Bitfield {
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) { self.bits.hash(state) }
        }

        impl $crate::BitSized<{ $Bitfield::_BITFIELD_BIT_SIZE }> for $Bitfield {}
        impl $crate::ConstInit for $Bitfield { const INIT: Self = Self::new(); }
        $( #[$crate::compile[nota(Default $(, $no_trait)*)]] )?
        impl Default for $Bitfield { fn default() -> Self { Self::new() } }

        /* impl formatting traits */

        impl $Bitfield {
            fn _fmt_display(&self, f: &mut $crate::Formatter<'_>) -> $crate::FmtResult<()> {
                let mut first = true;
                $crate::paste! { $(
                    let value = self.[<get_ $f:lower>]();
                    if value != 0 {
                        if !first { write!(f, " ")?; }
                        write!(f, concat!(stringify!($f), "={}"), value)?;
                        first = false;
                    }
                )* }
                if first { write!(f, "empty")?; }
                Ok(())
            }
            fn _fmt_debug_named_all(&self, f: &mut $crate::Formatter<'_>) -> $crate::FmtResult<()> {
                write!(f, concat!(stringify!($Bitfield), " {{ "))?;
                self._fmt_debug_fields_all(f)?;
                write!(f, " }}")
            }
            fn _fmt_debug_fields_all(&self, f: &mut $crate::Formatter<'_>) -> $crate::FmtResult<()> {
                let mut first = true;
                $crate::paste! { $(
                    if !first { write!(f, ", ")?; }
                    write!(f, concat!(stringify!($f), ": {}/{}"),
                        self.[<get_ $f:lower>](), Self::[<_ $f _MAX>])?;
                    first = false;
                )* }
                if first { write!(f, "empty")?; }
                Ok(())
            }
            fn _fmt_debug_named_active(&self, f: &mut $crate::Formatter<'_>) -> $crate::FmtResult<()> {
                write!(f, concat!(stringify!($Bitfield), " {{ "))?;
                self._fmt_debug_fields_active(f)?;
                write!(f, " }}")
            }
            fn _fmt_debug_fields_active(&self, f: &mut $crate::Formatter<'_>) -> $crate::FmtResult<()> {
                let mut first = true;
                $crate::paste! { $(
                    let value = self.[<get_ $f:lower>]();
                    if value != 0 {
                        if !first { write!(f, ", ")?; }
                        write!(f, concat!(stringify!($f), ": {}/{}"), value, Self::[<_ $f _MAX>])?;
                        first = false;
                    }
                )* }
                if first { write!(f, "empty")?; }
                Ok(())
            }
            fn _fmt_raw_binary(&self, f: &mut $crate::Formatter<'_>) -> $crate::FmtResult<()> {
                write!(f, concat!(stringify!($Bitfield), "(0b{:0w$b})"),
                    self.bits, w = Self::_BITFIELD_BIT_WIDTH)
            }
        }
        $crate::impl_trait! { fmt::Debug for
            $( #[$crate::compile[nota(Debug $(, $no_trait)*)]] )?
            /// Formats the bitfield as named field values.
            $Bitfield |self, f| self._fmt_debug_named_all(f)
        }
        $crate::impl_trait! { fmt::Display for
            $( #[$crate::compile[nota(Display $(, $no_trait)*)]] )?
            /// Formats the bitfield as compact named field values.
            $Bitfield |self, f| self._fmt_display(f)
        }
        $crate::impl_trait! { fmt::Binary for
            $( #[$crate::compile[nota(Binary $(, $no_trait)*)]] )?
            /// Formats the set as a binary mask over its declared bit domain.
            $Bitfield |self, f|
                if f.alternate() { write!(f, "0b{:0w$b}", self.bits, w = Self::_BITFIELD_BIT_WIDTH) }
                else { write!(f, "{:0w$b}", self.bits, w = Self::_BITFIELD_BIT_WIDTH) }
        }
        $crate::impl_trait! { fmt::Octal for
            $( #[$crate::compile[nota(Octal $(, $no_trait)*)]] )?
            /// Formats the set as an octal mask over its declared bit domain.
            $Bitfield |self, f|
                if f.alternate() { write!(f, "0o{:0w$o}", self.bits, w = Self::_BITFIELD_OCTAL_WIDTH) }
                else { write!(f, "{:0w$o}", self.bits, w = Self::_BITFIELD_OCTAL_WIDTH) }
        }
        $crate::impl_trait! { fmt::LowerHex for
            $( #[$crate::compile[nota(LowerHex $(, $no_trait)*)]] )?
            /// Formats the set as a hexadecimal mask over its declared bit domain.
            $Bitfield |self, f|
                if f.alternate() { write!(f, "0x{:0w$x}", self.bits, w = Self::_BITFIELD_HEX_WIDTH) }
                else { write!(f, "{:0w$x}", self.bits, w = Self::_BITFIELD_HEX_WIDTH) }
        }
        $crate::impl_trait! { fmt::UpperHex for
            $( #[$crate::compile[nota(UpperHex $(, $no_trait)*)]] )?
            /// Formats the set as a hexadecimal mask over its declared bit domain.
            $Bitfield |self, f|
                if f.alternate() { write!(f, "0x{:0w$X}", self.bits, w = Self::_BITFIELD_HEX_WIDTH) }
                else { write!(f, "{:0w$X}", self.bits, w = Self::_BITFIELD_HEX_WIDTH) }
        }

        $( #[$crate::compile[nota(DebugExt $(, $no_trait)*)]] )?
        impl $crate::DebugExt for $Bitfield where $Bitfield: $crate::Debug {
            type Ctx = $crate::ReprMode;
            fn fmt_with(&self, f: &mut $crate::Formatter<'_>, ctx: &Self::Ctx)
                -> $crate::FmtResult<()> {
                match ctx {
                    $crate::ReprMode::Raw => self._fmt_raw_binary(f),
                    $crate::ReprMode::Named => self._fmt_debug_named_active(f),
                    $crate::ReprMode::RawNamed => {
                        write!(f, concat!(stringify!($Bitfield), "("))?;
                        write!(f, "0b{:0w$b}", self.bits, w = Self::_BITFIELD_BIT_WIDTH)?;
                        write!(f, "; ")?;
                        self._fmt_debug_fields_all(f)?;
                        write!(f, ")")
                    }
                }
            }
        }
    };
    // impl single-bit field
    (%field_methods $vis:vis $Bitfield:ident $T:ty;
     $f:ident; $bit:tt) => {
        $crate::bitfield!(%field_methods_range $vis $Bitfield $T; $f; $bit, $bit);
    };
    // impl inclusive range field
    (%field_methods $vis:vis $Bitfield:ident $T:ty;
     $f:ident; $start:tt ..= $end:tt) => {
        $crate::bitfield!(%field_methods_range $vis $Bitfield $T; $f; $start, $end);
    };
    // impl field methods (unified start..end syntax)
    (%field_methods_range $vis:vis $Bitfield:ident $T:ty;
     $f:ident; $start:tt, $end:tt) => { $crate::paste! {
        #[doc = "# Methods for [`" $f "`](#associatedconstant." $f ")"]
        impl $Bitfield {
            #[allow(non_upper_case_globals)]
            const [<_ $f _GUARD_FIELD_RANGE>]: () = {
                assert!($start <= $end, "bitfield!: field start must be <= field end");
                assert!(Self::[<_ $f _END>] < <$T>::BITS,
                    "bitfield!: field end exceeds backing integer width");
            };

            /* public methods */

            /// Returns the field value.
            #[must_use] #[allow(dead_code)]
            $vis const fn [<get_ $f:lower>](self) -> $T {
                $crate::Bitwise::<$T>(self.bits).get_value_range(($start) as u32, ($end) as u32).0
            }
            /// Returns `self` with the field value replaced, masking it to fit the field width.
            #[must_use] #[allow(clippy::double_must_use)] #[allow(dead_code)]
            $vis const fn [<with_ $f:lower>](self, value: $T) -> Self {
                Self { bits: $crate::Bitwise::<$T>(self.bits)
                    .set_value_range(value, ($start) as u32, ($end) as u32).0 }
            }
            /// Returns `self` with the checked field value replaced, if it fits.
            #[allow(dead_code)]
            $vis const fn [<try_with_ $f:lower>](self, value: $T)
                -> $crate::Result<Self, $crate::MismatchedCapacity> {
                match $crate::Bitwise::<$T>(self.bits)
                    .set_value_range_checked_strict(value, ($start) as u32, ($end) as u32)
                {
                    Ok(bits) => Ok(Self { bits: bits.0 }),
                    // This should be the only reachable error after macro guards.
                    Err($crate::MismatchedBounds::MismatchedCapacity {
                        bound: _, value: _, limit: _ }) => {
                        Err($crate::MismatchedCapacity::too_large(value as usize,
                            Self::[<_ $f _MAX>] as usize))
                    },
                    // Guarded by generated compile-time assertions.
                    Err(_) => unreachable!(),
                }
            }

            /// Replaces the field value, masking it to fit the field width.
            #[allow(dead_code)]
            $vis const fn [<set_ $f:lower>](&mut self, value: $T) {
                self.bits = $crate::Bitwise::<$T>(self.bits)
                    .set_value_range(value, ($start) as u32, ($end) as u32).0;
            }
            /// Replaces the field value, if it fits.
            #[allow(dead_code)]
            $vis const fn [<try_set_ $f:lower>](&mut self, value: $T)
                -> $crate::Result<(), $crate::MismatchedCapacity> {
                match self.[<try_with_ $f:lower>](value) {
                    Ok(new) => {
                        *self = new;
                        Ok(())
                    }
                    Err(e) => Err(e),
                }
            }

            /// Clears the field value.
            #[allow(dead_code)]
            $vis const fn [<clear_ $f:lower>](&mut self) { self.bits &= !Self::$f.bits; }
            /// Returns `true` if the field value is zero.
            #[must_use] #[allow(dead_code)]
            $vis const fn [<is_ $f:lower _zero>](self) -> bool { self.[<get_ $f:lower>]() == 0 }
            /// Returns `true` if the field value equals its maximum.
            #[must_use] #[allow(dead_code)]
            $vis const fn [<is_ $f:lower _max>](self) -> bool {
                self.[<get_ $f:lower>]() == Self::[<_ $f _MAX>]
            }
            // /// Returns `self` with the field value replaced if `condition` is true.
            // ///
            // /// The value is masked to fit the field width.
            // #[must_use] #[allow(clippy::double_must_use, dead_code)]
            // $vis const fn [<with_ $f:lower _if>](self, condition: bool, value: $T) -> Self {
            //     if condition { self.[<with_ $f:lower>](value) } else { self }
            // }

            /// Returns metadata for this field.
            #[must_use]
            #[allow(dead_code)]
            $vis const fn [<$f:lower _span>]() -> $crate::BitSpan<$T> {
                $crate::BitSpan::<$T>::from_parts(
                    Self::[<_ $f _START>],
                    Self::[<_ $f _END>],
                    Self::[<_ $f _MASK>],
                    Self::[<_ $f _MAX>],
                )
            }

            /* private metadata constants */

            /// The start bit of this field.
            #[allow(non_upper_case_globals)]
            pub(crate) const [<_ $f _START>]: u32 = ($start) as u32;
            /// The end bit of this field.
            #[allow(non_upper_case_globals)]
            pub(crate) const [<_ $f _END>]: u32 = ($end) as u32;
            /// The bit width of this field.
            #[allow(non_upper_case_globals)]
            pub(crate) const [<_ $f _WIDTH>]: u32 = {
                let _ = Self::[<_ $f _GUARD_FIELD_RANGE>];
                (($end) - ($start)) as u32 + 1
            };
            /// The bit mask of this field.
            #[allow(non_upper_case_globals)]
            pub(crate) const [<_ $f _MASK>]: $T = {
                let _ = Self::[<_ $f _GUARD_FIELD_RANGE>];
                $crate::Bitwise::<$T>::mask_range(($start) as u32, ($end) as u32).0
            };
            /// The maximum value of this field.
            #[allow(non_upper_case_globals)]
            pub(crate) const [<_ $f _MAX>]: $T = {
                if Self::[<_ $f _WIDTH>] >= <$T>::BITS { !0 as $T }
                else { ((1 as $T) << Self::[<_ $f _WIDTH>]) - 1 }
            };
            /// Whether this field occupies exactly one bit.
            #[allow(non_upper_case_globals)]
            pub(crate) const [<_ $f _IS_SINGLE>]: bool = Self::[<_ $f _WIDTH>] == 1;
        }
    }};
    // only allow unsigned primitive backing integers
    (%guard_allowed_type $Bitfield:ident, $T:ty) => { $crate::paste! {
        const [<__GUARD_ALLOWED_TYPE_ $Bitfield:upper>]: () = {
            const fn __allowed_types<P: $crate::PrimUint>() {}
            __allowed_types::<$T>();
        };
    }};
    ($($tt:tt)*) => { compile_error! { concat![
        "bitfield!: wrong syntax.\n\n",
        "Expected:\n",
        "    bitfield! { $vis struct Name($ty) { CONST = bits; ... } }\n\n",
        "Examples of constant syntax:\n",
        "    FLAG = 0;\n"
        "    KIND = 1..=4;\n"
        "    LEN  = 5..=15;\n"
        // "Input:\n",
        // "    ",
        // stringify!($($tt)*)
    ]}};
}
#[doc(inline)]
pub use bitfield· as bitfield;
