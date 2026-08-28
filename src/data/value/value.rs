// devela/src/data/value/value.rs
//
//! Defines Value<8|16|32|64|128>.
//

use crate::{ValueKind4, is, unwrap};

macro_rules! define_value {
    () => {
        define_value!(
            Value8, u8, i8, 1|8, 4;
            Value16, u16, i16, 2|16, 12;
            Value32, u32, i32, 4|32, 28;
            Value64, u64, i64, 8|64, 60;
            Value128, u128, i128, 16|128, 124;
        );
    };
    ($($Name:ident, $Unsigned:ty, $Signed:ty, $bytes:literal|$bits:literal, $payload:literal);+
     $(;)?) => {
        $( define_value!(% $Name, $Unsigned, $Signed, $bytes|$bits, $payload); )+
    };
    (% $Name:ident, $Unsigned:ty, $Signed:ty, $bytes:literal|$bits:literal, $payload:literal) => {
        #[doc = crate::_tags!(data value)]
        #[doc = concat!("A compact ", stringify!($bits), "-bit value with a 4-bit kind and ",
            stringify!($payload), "-bit payload.")]
        #[doc = crate::_doc_meta!{
            location("data/value", struct $Name),
            test_size_of($Name = $bytes|$bits; niche !Option),
        }]
        #[repr(transparent)]
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
        pub struct $Name($Unsigned);

        const _: () = {
            assert!(<$Unsigned>::BITS == $bits);
            assert!(<$Unsigned>::BITS - 4 == $payload);
        };

        impl $Name {
            /* private */

            const fn pack(kind: ValueKind4, payload: $Unsigned) -> Self {
                Self(((kind.code() as $Unsigned) << Self::KIND_SHIFT) | payload)
            }

            /* constants */

            /// The total word width.
            pub const BITS: u32 = <$Unsigned>::BITS;

            /// The width of the compact semantic kind.
            pub const KIND_BITS: u32 = 4;

            /// The available payload width.
            pub const PAYLOAD_BITS: u32 = Self::BITS - Self::KIND_BITS;

            /// The bit position where the compact kind begins.
            pub const KIND_SHIFT: u32 = Self::PAYLOAD_BITS;

            /// The mask selecting the payload bits.
            pub const PAYLOAD_MASK: $Unsigned = <$Unsigned>::MAX >> Self::KIND_BITS;

            /// The mask selecting the kind bits.
            pub const KIND_MASK: $Unsigned = !Self::PAYLOAD_MASK;

            /// The greatest raw payload value.
            pub const PAYLOAD_MAX: $Unsigned = Self::PAYLOAD_MASK;

            /// The greatest directly representable unsigned integer.
            pub const UINT_MAX: $Unsigned = Self::PAYLOAD_MAX;

            /// The least directly representable signed integer.
            pub const INT_MIN: $Signed = -(1 as $Signed << (Self::PAYLOAD_BITS - 1));

            /// The greatest directly representable signed integer.
            pub const INT_MAX: $Signed = (1 as $Signed << (Self::PAYLOAD_BITS - 1)) - 1;

            /// The canonical nil value.
            pub const NIL: Self = Self::pack(ValueKind4::Nil, 0);

            /// The canonical false value.
            pub const FALSE: Self = Self::pack(ValueKind4::Bool, 0);

            /// The canonical true value.
            pub const TRUE: Self = Self::pack(ValueKind4::Bool, 1);

            /* queries */

            /// Returns the compact semantic kind.
            #[must_use]
            pub const fn kind(self) -> ValueKind4 {
                unwrap![some ValueKind4::from_code((self.0 >> Self::KIND_SHIFT) as u8)]
            }
            /// Returns the raw payload.
            #[must_use]
            pub const fn payload(self) -> $Unsigned {
                self.0 & Self::PAYLOAD_MASK
            }
            /// Returns whether an unsigned value fits in the payload.
            #[must_use]
            pub const fn fits_payload(value: $Unsigned) -> bool {
                value <= Self::PAYLOAD_MAX
            }
            /// Returns whether an unsigned integer fits directly.
            #[must_use]
            pub const fn fits_uint(value: $Unsigned) -> bool {
                Self::fits_payload(value)
            }
            /// Returns whether a signed integer fits directly.
            #[must_use]
            pub const fn fits_int(value: $Signed) -> bool {
                value >= Self::INT_MIN && value <= Self::INT_MAX
            }

            /* constructors */

            /// Creates a boolean value.
            #[must_use]
            pub const fn from_bool(value: bool) -> Self {
                is! { value, Self::TRUE, Self::FALSE }
            }
            /// Creates an unsigned integer if it fits directly.
            #[must_use]
            pub const fn try_from_uint(value: $Unsigned) -> Option<Self> {
                is! { Self::fits_uint(value), Some(Self::pack(ValueKind4::UInt, value)), None }
            }
            /// Creates a signed integer if it fits directly.
            #[must_use]
            pub const fn try_from_int(value: $Signed) -> Option<Self> {
                if Self::fits_int(value) {
                    Some(Self::pack(ValueKind4::Int, (value as $Unsigned) & Self::PAYLOAD_MASK))
                } else {
                    None
                }
            }

            /// Returns the boolean value when this is a boolean.
            #[must_use]
            pub const fn to_bool(self) -> Option<bool> {
                is! { matches!(self.kind(), ValueKind4::Bool), Some(self.payload() != 0), None }
            }
            /// Returns the unsigned integer when this is an unsigned integer.
            #[must_use]
            pub const fn to_uint(self) -> Option<$Unsigned> {
                is! { matches!(self.kind(), ValueKind4::UInt), Some(self.payload()), None }
            }
            /// Returns the signed integer when this is a signed integer.
            #[must_use]
            pub const fn to_int(self) -> Option<$Signed> {
                if matches!(self.kind(), ValueKind4::Int) {
                    Some(((self.payload() << Self::KIND_BITS) as $Signed) >> Self::KIND_BITS)
                } else {
                    None
                }
            }
        }
    };
}
define_value!();

#[cfg(test)]
mod _test {
    use super::*;

    #[test]
    fn value8_immediates() {
        assert_eq!(Value8::BITS, 8);
        assert_eq!(Value8::KIND_BITS, 4);
        assert_eq!(Value8::PAYLOAD_BITS, 4);
        assert_eq!(Value8::PAYLOAD_MAX, 15);
        assert_eq!(Value8::NIL.kind(), ValueKind4::Nil);
        assert_eq!(Value8::NIL.payload(), 0);
        assert_eq!(Value8::FALSE.to_bool(), Some(false));
        assert_eq!(Value8::TRUE.to_bool(), Some(true));
        for value in 0..=Value8::UINT_MAX {
            let word = Value8::try_from_uint(value).unwrap();
            assert_eq!(word.kind(), ValueKind4::UInt);
            assert_eq!(word.to_uint(), Some(value));
        }
        for value in Value8::INT_MIN..=Value8::INT_MAX {
            let word = Value8::try_from_int(value).unwrap();
            assert_eq!(word.kind(), ValueKind4::Int);
            assert_eq!(word.to_int(), Some(value));
        }
        assert_eq!(Value8::try_from_int(-9), None);
        assert_eq!(Value8::try_from_int(8), None);
        assert_eq!(Value8::try_from_uint(16), None);
    }
}
