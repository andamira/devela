// devela::data::codec::bit::bin::field::definition
//
//! Defines [`bitfield!`]
//

#[doc = crate::_tags!(construction bit data_structure)]
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
        $( // optional impl blocks
            $(#[$impl_attrs:meta])*
            impl { $($user_impl:item)* }
        )*
    } => {
        $crate::bitfield!(%guard_allowed_type $Bitfield, $T);

        $(#[$struct_attrs])*
        #[repr(transparent)]
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
        $vis struct $Bitfield { bits: $T, }
        $(
            $( #[$impl_attrs] )*
            impl $Bitfield { $($user_impl)* }
        )*
        impl $Bitfield {
            /* per-bitfield private metadata */

            /* public methods */

            /// Returns a new bitfield with all bits cleared.
            #[must_use] #[allow(clippy::double_must_use, dead_code)]
            $vis const fn new() -> Self { Self { bits: 0 } }
            /// Returns a new bitfield from raw bits.
            #[must_use] #[allow(clippy::double_must_use, dead_code)]
            $vis const fn from_bits(bits: $T) -> Self { Self { bits } }
            /// Returns the raw backing bits.
            #[must_use] #[allow(dead_code)]
            $vis const fn bits(self) -> $T { self.bits }
            /// Returns `true` if all bits are clear.
            #[must_use] #[allow(dead_code)]
            $vis const fn is_empty(self) -> bool { self.bits == 0 }
            /// Clears all bits.
            #[allow(dead_code)]
            $vis const fn clear(&mut self) { self.bits = 0; }
            // $vis const fn is_full(self) -> bool { todo![] } MAYBE
        }
        $(
            #[doc = concat!("# Methods for the `", stringify!($f), "` field.")]
            impl $Bitfield {
                $crate::bitfield!(%field_methods $vis $Bitfield $T;
                    $(#[$f_attrs])* $f; $start $(..= $end)?);
            }
        )*

        /* impl traits */

        impl $crate::ConstInit for $Bitfield { const INIT: Self = Self::new(); }
    };
    // single-bit field
    (%field_methods $vis:vis $Bitfield:ident $T:ty;
     $(#[$f_attrs:meta])* $f:ident; $bit:tt) => {
        //
        $crate::bitfield!(%field_methods_range $vis $Bitfield $T;
            $(#[$f_attrs])* $f; $bit, $bit);
    };
    // inclusive range field
    (%field_methods $vis:vis $Bitfield:ident $T:ty;
     $(#[$f_attrs:meta])* $f:ident; $start:tt ..= $end:tt) => {
        //
        $crate::bitfield!(%field_methods_range $vis $Bitfield $T;
            $(#[$f_attrs])* $f; $start, $end);
    };
    // field method expansion
    (%field_methods_range $vis:vis $Bitfield:ident $T:ty;
     $(#[$f_attrs:meta])* $f:ident; $start:tt, $end:tt) => { $crate::paste! {
        #[allow(non_upper_case_globals)]
            #[allow(non_upper_case_globals)]
            const [<__GUARD_FIELD_RANGE_ $Bitfield _ $f>]: () = {
                assert!($start <= $end, "bitfield!: field start must be <= field end");
                assert!(Self::[<_ $f _END>] < <$T>::BITS,
                    "bitfield!: field end exceeds backing integer width");
            };

            /* private metadata constants */

            /// The start bit of this field.
            #[allow(non_upper_case_globals)]
            pub(crate) const [<_ $f _START>]: u32 = ($start) as u32;
            /// The end bit of this field.
            #[allow(non_upper_case_globals)]
            pub(crate) const [<_ $f _END>]: u32 = ($end) as u32;
            /// The bit width of this field.
            #[allow(non_upper_case_globals)]
            pub(crate) const [<_ $f _WIDTH>]: u32 = (($end) - ($start)) as u32 + 1;
            /// The bit mask of this field.
            #[allow(non_upper_case_globals)]
            pub(crate) const [<_ $f _MASK>]: $T = $crate::Bitwise::<$T>::
                mask_range(($start) as u32, ($end) as u32).0;
            /// The maximum value of this field.
            #[allow(non_upper_case_globals)]
            pub(crate) const [<_ $f _MAX>]: $T =
                if Self::[<_ $f _WIDTH>] >= <$T>::BITS { !0 as $T }
                else { ((1 as $T) << Self::[<_ $f _WIDTH>]) - 1 };

            /* public metadata methods */

            /// Returns the start bit of this field.
            #[must_use] #[allow(dead_code)]
            $vis const fn [<$f:lower _start>]() -> u32 { Self::[<_ $f _START>] }
            /// Returns the end bit of this field.
            #[must_use] #[allow(dead_code)]
            $vis const fn [<$f:lower _end>]() -> u32 { Self::[<_ $f _END>] }
            /// Returns the bit width of this field.
            #[must_use] #[allow(dead_code)]
            $vis const fn [<$f:lower _width>]() -> u32 { Self::[<_ $f _WIDTH>] }
            /// Returns the shifted bit mask for this field.
            #[must_use] #[allow(dead_code)]
            $vis const fn [<$f:lower _mask>]() -> $T { Self::[<_ $f _MASK>] }
            /// Returns the maximum unshifted value that fits in this field.
            #[must_use] #[allow(dead_code)]
            $vis const fn [<$f:lower _max>]() -> $T { Self::[<_ $f _MAX>] }

            /* public methods */

            $(#[$f_attrs])*
            /// Returns the field value.
            #[must_use] #[allow(dead_code)]
            $vis const fn [<get_ $f:lower>](self) -> $T {
                $crate::Bitwise::<$T>(self.bits)
                    .get_value_range(($start) as u32, ($end) as u32).0
            }
            #[doc = "Returns `self` with the field value replaced."]
            ///
            /// The value is masked to fit the field width.
            #[must_use] #[allow(clippy::double_must_use)] #[allow(dead_code)]
            $vis const fn [<with_ $f:lower>](self, value: $T) -> Self {
                Self { bits: $crate::Bitwise::<$T>(self.bits)
                    .set_value_range(value, ($start) as u32, ($end) as u32).0 }
            }
            #[doc = "Returns `self` with the checked field value replaced."]
            ///
            /// # Errors
            /// Returns [`MismatchedBounds`] if the generated field range is invalid
            /// or if `value` does not fit within the field width.
            ///
            /// [`MismatchedBounds`]: crate::MismatchedBounds
            #[allow(dead_code)]
            $vis const fn [<try_with_ $f:lower>](self, value: $T)
                -> $crate::Result<Self, $crate::MismatchedBounds>
            {
                match $crate::Bitwise::<$T>(self.bits)
                    .set_value_range_checked_strict(value, ($start) as u32, ($end) as u32)
                {
                    Ok(bits) => Ok(Self { bits: bits.0 }),
                    Err(e) => Err(e),
                }
            }

            #[doc = "Replaces the field value."]
            ///
            /// The value is masked to fit the field width.
            #[allow(dead_code)]
            $vis const fn [<set_ $f:lower>](&mut self, value: $T) {
                self.bits = $crate::Bitwise::<$T>(self.bits)
                    .set_value_range(value, ($start) as u32, ($end) as u32).0;
            }
            #[doc = "Replaces the field value, checking the value capacity."]
            ///
            /// # Errors
            /// Returns [`MismatchedBounds`] if the generated field range is invalid
            /// or if `value` does not fit within the field width.
            ///
            /// [`MismatchedBounds`]: crate::MismatchedBounds
            #[allow(dead_code)]
            $vis const fn [<try_set_ $f:lower>](&mut self, value: $T)
                -> $crate::Result<(), $crate::MismatchedBounds>
            {
                match $crate::Bitwise::<$T>(self.bits)
                    .set_value_range_checked_strict(value, ($start) as u32, ($end) as u32)
                {
                    Ok(bits) => {
                        self.bits = bits.0;
                        Ok(())
                    },
                    Err(e) => Err(e),
                }
            }
            #[doc = "Clears the field."]
            #[allow(dead_code)]
            $vis const fn [<clear_ $f:lower>](&mut self) {
                self.bits &= !Self::[<$f:lower _mask>]();
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
