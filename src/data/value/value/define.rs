// devela/src/data/value/value/define.rs
//
//! Defines Value<8|16|32|64|128>.
//

use crate::{InvalidValue, ValueKind4, cast, is, unwrap, word};

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
        #[must_use]
        #[repr(transparent)]
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
        pub struct $Name($Unsigned);

        const _: () = {
            assert!(<$Unsigned>::BITS == $bits);
            assert!(<$Unsigned>::BITS - 4 == $payload);
        };

        /// # Constants
        impl $Name {
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
        }
        /// # Methods
        impl $Name {
            /* queries */

            /// Returns the compact semantic kind.
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

            /// Returns whether `payload` is canonical for `kind` in this value representation.
            #[must_use]
            pub const fn is_canonical_payload(kind: ValueKind4, payload: $Unsigned) -> bool {
                is! { payload > Self::PAYLOAD_MAX, return false }
                match kind {
                    ValueKind4::Nil => payload == 0,
                    ValueKind4::Bool => payload <= 1,
                    ValueKind4::Int | ValueKind4::UInt => true,
                    ValueKind4::Float => false, // FUTURE IMPROVE
                    ValueKind4::Char => Self::payload_to_char(payload).is_some(),
                    ValueKind4::Symbol
                    | ValueKind4::Enum
                    | ValueKind4::Ref
                    | ValueKind4::Bytes
                    | ValueKind4::Text
                    | ValueKind4::List
                    | ValueKind4::Set
                    | ValueKind4::Table
                    | ValueKind4::Callable
                    | ValueKind4::Escape => true,
                }
            }

            /* representation */

            /// Creates a value from a kind and payload when the combination is canonical.
            #[must_use]
            pub const fn try_from_parts(kind: ValueKind4, payload: $Unsigned) -> Option<Self> {
                is![Self::is_canonical_payload(kind, payload),
                    Some(Self::pack(kind, payload)), None]
            }
            /// Decomposes this value into its compact kind and payload.
            ///
            /// The returned parts can be passed to [`try_from_parts`](#method.try_from_parts)
            /// to reconstruct the same value.
            pub const fn into_parts(self) -> (ValueKind4, $Unsigned) {
                (self.kind(), self.payload())
            }

            /* immediate values */

            /// Creates a boolean value.
            pub const fn from_bool(value: bool) -> Self {
                is! { value, Self::TRUE, Self::FALSE }
            }
            /// Returns the boolean value when this is a boolean.
            #[must_use]
            pub const fn as_bool(self) -> Option<bool> {
                is! { !matches!(self.kind(), ValueKind4::Bool), return None }
                match self.payload() {
                    0 => Some(false),
                    1 => Some(true),
                    _ => None,
                }
            }

            /// Creates an unsigned integer if it fits directly.
            #[must_use]
            pub const fn try_from_uint(value: $Unsigned) -> Option<Self> {
                is! { Self::fits_uint(value), Some(Self::pack(ValueKind4::UInt, value)), None }
            }
            /// Returns the unsigned integer when this is an unsigned integer.
            #[must_use]
            pub const fn as_uint(self) -> Option<$Unsigned> {
                is! { matches!(self.kind(), ValueKind4::UInt), Some(self.payload()), None }
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
            /// Returns the signed integer when this is a signed integer.
            #[must_use]
            pub const fn as_int(self) -> Option<$Signed> {
                if matches!(self.kind(), ValueKind4::Int) {
                    Some(((self.payload() << Self::KIND_BITS) as $Signed) >> Self::KIND_BITS)
                } else {
                    None
                }
            }

            /// Creates a character value if its Unicode scalar fits in the payload.
            #[must_use]
            pub const fn try_from_char(value: char) -> Option<Self> {
                let payload = unwrap![some? Self::char_to_payload(value)];
                Some(Self::pack(ValueKind4::Char, payload))
            }
            /// Returns the character when this is a canonical character value.
            #[must_use]
            pub const fn as_char(self) -> Option<char> {
                is! { !matches!(self.kind(), ValueKind4::Char), return None }
                Self::payload_to_char(self.payload())
            }
        }
        // Private
        impl $Name {
            const fn pack(kind: ValueKind4, payload: $Unsigned) -> Self {
                Self(((kind.code() as $Unsigned) << Self::KIND_SHIFT) | payload)
            }
            /// Converts a Unicode scalar into this value's payload carrier when it fits.
            const fn char_to_payload(value: char) -> Option<$Unsigned> {
                let code = value as u32;
                let payload = unwrap![ok_or cast![checked code => $Unsigned], return None];
                is![Self::fits_payload(payload), Some(payload), None]
            }
            /// Converts a payload into a Unicode scalar when canonical.
            const fn payload_to_char(payload: $Unsigned) -> Option<char> {
                is! { payload > Self::PAYLOAD_MAX, return None }
                let code = unwrap![ok_or cast![checked payload => u32], return None];
                char::from_u32(code)
            }
        }
        // impl WordTry
        word! {
            impl $Name => $Unsigned {
                type Error = InvalidValue;
                raw(value) { value.0 }
                try_from_raw(raw) {
                    let code = (raw >> Self::KIND_SHIFT) as u8;
                    match ValueKind4::from_code(code) {
                        Some(kind) => {
                            let payload = raw & Self::PAYLOAD_MASK;
                            unwrap![some_ok_or Self::try_from_parts(kind, payload), InvalidValue]
                        }
                        None => Err(InvalidValue),
                    }
                }
            }
        }
    };
}
define_value!();
