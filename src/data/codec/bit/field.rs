// devela::data::codec::bit::field
//
//! Defines [`bitfield!`]
//

#[cfg(feature = "_docs_examples")]
bitfield! {
    #[doc = crate::_tags!(example bit data_structure)]
    /// A compact packet header.
    #[doc = crate::_doc_location!("data/codec")]
    ///
    /// It has been generated with the [`bitfield!`] macro like this:
    /// ```
    /// # use devela::bitfield;
    /// bitfield! {
    ///     /// A compact packet header.
    ///     pub struct BitfieldExample(u16) {
    ///         KIND    = 0..=3;
    ///         VERSION = 4..=7;
    ///         LENGTH  = 8..=15;
    ///     }
    /// }
    /// ```
    pub struct BitfieldExample(u16) {
        KIND    = 0..=3;
        VERSION = 4..=7;
        LENGTH  = 8..=15;
    }
}

#[doc = crate::_tags!(construction bit data_structure)]
/// Defines a compact packed-field wrapper backed by an unsigned integer.
#[doc = crate::_doc_location!("data/codec")]
///
/// Each named field represents one bit or an inclusive range of bits.
/// Field values are read and written as the same unsigned integer type
/// used by the backing storage.
///
/// Unchecked setters mask values to the field width.
/// Checked setters return an error when the value does not fit.
///
/// One or more custom `impl { ... }` blocks may be provided after the declarations.
/// They are emitted before all generated associated constants and methods,
/// and may still refer to them through `Self`.
///
/// # Examples
/// ```
/// # use devela::bitfield;
/// bitfield! {
///     struct Header(u16) {
///         KIND = 0..=3;
///         LEN  = 4..=11;
///         FLAG = 12;
///     }
///     /// Custom semantic helpers emitted before generated methods.
///     impl {
///         /// Returns whether this header has a payload.
///         pub const fn has_payload(self) -> bool { self.get_len() != 0 }
///     }
/// }
/// let mut h = Header::new().with_kind(3).with_len(120).with_flag(1);
///
/// assert_eq!(h.get_kind(), 3);
/// assert_eq!(h.get_len(), 120);
/// assert_eq!(h.get_flag(), 1);
/// assert!(h.has_payload());
///
/// assert!(h.try_set_kind(15).is_ok());
/// assert!(h.try_set_kind(16).is_err()); // 16 does not fit in 4 bits.
/// ```
/// See also the [`BitfieldExample`] and the [`set!`][crate::set] macro.
#[doc(hidden)]
#[macro_export]
macro_rules! bitfield· {
    {
        $(#[$struct_attrs:meta])*
        $vis:vis struct $Bitfield:ident($T:ty) {
            $(
                $(#[$field_attrs:meta])*
                $field:ident = $start:tt $(..= $end:tt)?;
            )*
        }
        $( // optional impl blocks
            $(#[$impl_attrs:meta])*
            impl { $($user_impl:item)* }
        )*
    } => {
        $crate::bitfield!(%guard_allowed_type $T);

        $(#[$struct_attrs])*
        #[repr(transparent)]
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
        $vis struct $Bitfield { bits: $T, }
        $(
            $( #[$impl_attrs] )*
            impl $Bitfield { $($user_impl)* }
        )*
        impl $Bitfield {
            /// Returns a new bitfield with all bits cleared.
            #[must_use]
            $vis const fn new() -> Self { Self { bits: 0 } }
            /// Returns a new bitfield from raw bits.
            #[must_use]
            $vis const fn from_bits(bits: $T) -> Self { Self { bits } }
            /// Returns the raw backing bits.
            #[must_use]
            $vis const fn bits(self) -> $T { self.bits }
            /// Returns `true` if all bits are clear.
            #[must_use]
            $vis const fn is_empty(self) -> bool { self.bits == 0 }
            /// Clears all bits.
            $vis const fn clear(&mut self) { self.bits = 0; }
        }
        $(
            #[doc = concat!("# Methods for the `", stringify!($field), "` field.")]
            impl $Bitfield {
                $crate::bitfield!(%field_methods $vis $Bitfield $T;
                    $(#[$field_attrs])* $field; $start $(..= $end)?);
            }
        )*

        impl $crate::ConstInit for $Bitfield { const INIT: Self = Self::new(); }
    };
    // single-bit field.
    (%field_methods $vis:vis $Bitfield:ident $T:ty;
     $(#[$field_attrs:meta])* $field:ident; $bit:tt) => {
        $crate::bitfield!(%field_methods_range $vis $Bitfield $T;
            $(#[$field_attrs])* $field; $bit, $bit);
    };
    // inclusive range field.
    (%field_methods $vis:vis $Bitfield:ident $T:ty;
     $(#[$field_attrs:meta])* $field:ident; $start:tt ..= $end:tt) => {
        $crate::bitfield!(%field_methods_range $vis $Bitfield $T;
            $(#[$field_attrs])* $field; $start, $end);
    };
    // shared field method expansion.
    (%field_methods_range $vis:vis $Bitfield:ident $T:ty;
     $(#[$field_attrs:meta])* $field:ident; $start:tt, $end:tt) => { $crate::paste! {
            $(#[$field_attrs])*
            /// Returns the field value.
            #[must_use]
            $vis const fn [<get_ $field:lower>](self) -> $T {
                $crate::Bitwise::<$T>(self.bits)
                    .get_value_range(($start) as u32, ($end) as u32).0
            }
            #[doc = "Returns the raw mask for this field."]
            #[must_use]
            $vis const fn [<$field:lower _mask>]() -> $T {
                $crate::Bitwise::<$T>::mask_range(($start) as u32, ($end) as u32).0
            }

            #[doc = "Returns `self` with the field value replaced."]
            ///
            /// The value is masked to fit the field width.
            #[must_use]
            $vis const fn [<with_ $field:lower>](self, value: $T) -> Self {
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
            $vis const fn [<try_with_ $field:lower>](self, value: $T)
                -> $crate::Result<Self, $crate::MismatchedBounds>
            {
                match $crate::Bitwise::<$T>(self.bits)
                    .set_checked_value_checked_range(value, ($start) as u32, ($end) as u32)
                {
                    Ok(bits) => Ok(Self { bits: bits.0 }),
                    Err(e) => Err(e),
                }
            }

            #[doc = "Replaces the field value."]
            ///
            /// The value is masked to fit the field width.
            $vis const fn [<set_ $field:lower>](&mut self, value: $T) {
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
            $vis const fn [<try_set_ $field:lower>](&mut self, value: $T)
                -> $crate::Result<(), $crate::MismatchedBounds>
            {
                match $crate::Bitwise::<$T>(self.bits)
                    .set_checked_value_checked_range(value, ($start) as u32, ($end) as u32)
                {
                    Ok(bits) => {
                        self.bits = bits.0;
                        Ok(())
                    },
                    Err(e) => Err(e),
                }
            }

            #[doc = "Clears the field."]
            $vis const fn [<clear_ $field:lower>](&mut self) {
                self.bits &= !Self::[<$field:lower _mask>]();
            }
    }};
    // only allow unsigned primitive backing integers
    (%guard_allowed_type $P:ty) => {
        const _: () = {
            const fn __allowed_types<P: $crate::PrimUint>() {}
            __allowed_types::<$P>();
        };
    };
}
#[doc(inline)]
pub use bitfield· as bitfield;

#[cfg(test)]
mod tests {
    #![allow(unused)]

    crate::bitfield! {
        struct TestBitfield(u16) {
            a = 0;
            b = 1..=3;
            c = 4..=7;
            D = 8..=15; // check uppercase field
        }
    }
    #[test]
    fn bitfield_gets_and_sets_fields() {
        let f = TestBitfield::new().with_a(1).with_b(0b101).with_c(0b1100).with_d(0xAB);
        assert_eq!(f.get_a(), 1);
        assert_eq!(f.get_b(), 0b101);
        assert_eq!(f.get_c(), 0b1100);
        assert_eq!(f.get_d(), 0xAB);
        assert_eq!(TestBitfield::a_mask(), 0b0000_0000_0000_0001);
        assert_eq!(TestBitfield::b_mask(), 0b0000_0000_0000_1110);
        assert_eq!(TestBitfield::c_mask(), 0b0000_0000_1111_0000);
        assert_eq!(TestBitfield::d_mask(), 0b1111_1111_0000_0000);
    }
    #[test]
    fn bitfield_mutates_fields() {
        let mut f = TestBitfield::new();
        f.set_b(0b111);
        assert_eq!(f.get_b(), 0b111);
        f.set_b(0b010);
        assert_eq!(f.get_b(), 0b010);
        f.clear_b();
        assert_eq!(f.get_b(), 0);
        assert_eq!(f.bits(), 0);
    }
    #[test]
    fn bitfield_overwrites_only_target_range() {
        let f = TestBitfield::new().with_b(0b111).with_c(0b1111).with_b(0b001);
        assert_eq!(f.get_b(), 0b001);
        assert_eq!(f.get_c(), 0b1111);
    }
    #[test]
    fn bitfield_checked_operations_validate_capacity() {
        crate::bitfield! {
            struct CheckedHeader(u16) {
                KIND = 0..=3;
                TWO  = 4..=5;
                FLAG = 6;
            }
        }
        let mut h = CheckedHeader::new();

        assert!(h.try_set_kind(15).is_ok());
        assert_eq!(h.get_kind(), 15);
        assert!(h.try_set_kind(16).is_err());
        assert_eq!(h.get_kind(), 15);

        assert!(h.try_set_two(3).is_ok());
        assert_eq!(h.get_two(), 3);
        assert!(h.try_set_two(4).is_err());
        assert_eq!(h.get_two(), 3);

        assert!(h.try_set_flag(1).is_ok());
        assert_eq!(h.get_flag(), 1);
        assert!(h.try_set_flag(2).is_err());
        assert_eq!(h.get_flag(), 1);

        assert!(CheckedHeader::new().try_with_kind(15).is_ok());
        assert!(CheckedHeader::new().try_with_kind(16).is_err());
        assert!(CheckedHeader::new().try_with_two(3).is_ok());
        assert!(CheckedHeader::new().try_with_two(4).is_err());
    }
    #[test]
    fn bitfield_uppercase_fields_generate_lowercase_methods() {
        crate::bitfield! {
            struct CaseHeader(u8) {
                KIND = 0..=3;
                IS_SET = 4;
            }
        }
        let h = CaseHeader::new().with_kind(7).with_is_set(1);
        assert_eq!(h.get_kind(), 7);
        assert_eq!(h.get_is_set(), 1);
    }
}
