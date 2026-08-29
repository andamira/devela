// devela/src/data/value/value/regrade.rs
//
//! Implements regrading conversions between compact value grades.
//

use crate::{InvalidValue, Value8, Value16, Value32, Value64, Value128, ValueKind4};
use crate::{cast, const_assert, paste, unwrap};

_impl_value_regrade!(Value8,  u8,  i8,   8 => Value16,  u16,  i16,   16);
_impl_value_regrade!(Value8,  u8,  i8,   8 => Value32,  u32,  i32,   32);
_impl_value_regrade!(Value8,  u8,  i8,   8 => Value64,  u64,  i64,   64);
_impl_value_regrade!(Value8,  u8,  i8,   8 => Value128, u128, i128, 128);
_impl_value_regrade!(Value16, u16, i16, 16 => Value32,  u32,  i32,   32);
_impl_value_regrade!(Value16, u16, i16, 16 => Value64,  u64,  i64,   64);
_impl_value_regrade!(Value16, u16, i16, 16 => Value128, u128, i128, 128);
_impl_value_regrade!(Value32, u32, i32, 32 => Value64,  u64,  i64,   64);
_impl_value_regrade!(Value32, u32, i32, 32 => Value128, u128, i128, 128);
_impl_value_regrade!(Value64, u64, i64, 64 => Value128, u128, i128, 128);

_impl_value_regrade_adjacent!(Value8, 8 => Value16, 16);
_impl_value_regrade_adjacent!(Value16, 16 => Value32, 32);
_impl_value_regrade_adjacent!(Value32, 32 => Value64, 64);
_impl_value_regrade_adjacent!(Value64, 64 => Value128, 128);

/// Implements direct regrading between two compact value grades.
macro_rules! _impl_value_regrade {
    (
        $Small:ident, $SmallU:ty, $SmallI:ty, $small_bits:literal =>
        $Large:ident, $LargeU:ty, $LargeI:ty, $large_bits:literal
    ) => {
        impl $Small {
            paste! {
                #[doc = "Regrades this value to the wider `" $Large "` representation."]
                pub const fn [<to_ $large_bits>](self) -> $Large {
                    match self.kind() {
                        ValueKind4::Int => {
                            let value = unwrap![some self.as_int()];
                            let value = cast![checked_unwrap value => $LargeI];
                            unwrap![some $Large::try_from_int(value)]
                        }
                        kind => {
                            let payload =
                                cast![checked_unwrap self.payload() => $LargeU];
                            unwrap![some $Large::try_from_parts(kind, payload)]
                        }
                    }
                }
            }
        }
        impl $Large {
            paste! {
                #[doc = "Tries to regrade this value to the narrower `" $Small "` representation."]
                #[must_use]
                pub const fn [<try_to_ $small_bits>](self) -> Option<$Small> {
                    match self.kind() {
                        ValueKind4::Int => {
                            let value = unwrap![some self.as_int()];
                            let value = unwrap![ok_or
                                cast![checked value => $SmallI],
                                return None
                            ];
                            $Small::try_from_int(value)
                        }
                        kind => {
                            let payload = unwrap![ok_or
                                cast![checked self.payload() => $SmallU],
                                return None
                            ];
                            $Small::try_from_parts(kind, payload)
                        }
                    }
                }
            }
        }
        impl From<$Small> for $Large {
            fn from(value: $Small) -> Self {
                paste! { value.[<to_ $large_bits>]() }
            }
        }
        impl TryFrom<$Large> for $Small {
            type Error = InvalidValue;
            fn try_from(value: $Large) -> Result<Self, Self::Error> {
                paste! {
                    unwrap![some_ok_or value.[<try_to_ $small_bits>](), InvalidValue]
                }
            }
        }
    };
}
use _impl_value_regrade;

/// Implements adjacent-grade aliases for compact value regrading.
macro_rules! _impl_value_regrade_adjacent {
    ($Small:ident, $small_bits:literal => $Large:ident, $large_bits:literal) => {
        impl $Small {
            /// Widens this value to the next canonical grade.
            pub const fn widen(self) -> $Large {
                paste! { self.[<to_ $large_bits>]() }
            }
        }
        impl $Large {
            /// Narrows this value to the previous canonical grade when representable.
            #[must_use]
            pub const fn try_narrow(self) -> Option<$Small> {
                paste! { self.[<try_to_ $small_bits>]() }
            }
        }
    };
}
use _impl_value_regrade_adjacent;

#[cfg(test)]
mod _test {
    use super::*;

    #[test]
    fn signed_regrading_preserves_value() {
        let v8 = Value8::try_from_int(-1).unwrap();
        let v16 = v8.widen();
        let v32 = v16.widen();
        let v64 = v32.widen();
        let v128 = v64.widen();
        assert_eq!(v8.as_int(), Some(-1));
        assert_eq!(v16.as_int(), Some(-1));
        assert_eq!(v32.as_int(), Some(-1));
        assert_eq!(v64.as_int(), Some(-1));
        assert_eq!(v128.as_int(), Some(-1));
        assert_ne!(v8.payload() as u16, v16.payload());
    }
    #[test]
    fn narrowing_checks_target_capacity() {
        assert_eq!(Value16::try_from_uint(15).unwrap().try_narrow(), Value8::try_from_uint(15));
        assert!(Value16::try_from_uint(16).unwrap().try_narrow().is_none());
        assert!(Value16::try_from_int(-8).unwrap().try_narrow().is_some());
        assert!(Value16::try_from_int(7).unwrap().try_narrow().is_some());
        assert!(Value16::try_from_int(-9).unwrap().try_narrow().is_none());
        assert!(Value16::try_from_int(8).unwrap().try_narrow().is_none());
        assert!(Value16::try_from_char('\u{000F}').unwrap().try_narrow().is_some());
        assert!(Value16::try_from_char('\u{0010}').unwrap().try_narrow().is_none());
        let symbol = Value16::try_from_parts(ValueKind4::Symbol, 16).unwrap();
        assert!(symbol.try_narrow().is_none());
        let escape = Value16::try_from_parts(ValueKind4::Escape, 15).unwrap();
        assert_eq!(escape.try_narrow().unwrap().into_parts(), (ValueKind4::Escape, 15));
    }
    #[test]
    fn value8_full_regrade_net_roundtrip() {
        for raw in u8::MIN..=u8::MAX {
            let v8 = unwrap![ok_or Value8::try_from_raw(raw), continue];
            let v16 = v8.to_16();
            let v32 = v8.to_32();
            let v64 = v8.to_64();
            let v128 = v8.to_128();
            // Direct and stepped regrading agree.
            assert_eq!(v16, v8.widen());
            assert_eq!(v32, v16.to_32());
            assert_eq!(v64, v16.to_64());
            assert_eq!(v64, v32.to_64());
            assert_eq!(v128, v16.to_128());
            assert_eq!(v128, v32.to_128());
            assert_eq!(v128, v64.to_128());
            // Every wider grade returns to the original.
            assert_eq!(v16.try_to_8(), Some(v8));
            assert_eq!(v32.try_to_8(), Some(v8));
            assert_eq!(v64.try_to_8(), Some(v8));
            assert_eq!(v128.try_to_8(), Some(v8));
            // Adjacent aliases agree.
            assert_eq!(v16.try_narrow(), Some(v8));
            assert_eq!(v32.try_narrow(), Some(v16));
            assert_eq!(v64.try_narrow(), Some(v32));
            assert_eq!(v128.try_narrow(), Some(v64));
            // Standard widening traits agree with the const API.
            assert_eq!(Value16::from(v8), v16);
            assert_eq!(Value32::from(v8), v32);
            assert_eq!(Value64::from(v8), v64);
            assert_eq!(Value128::from(v8), v128);
            assert_eq!(Value32::from(v16), v32);
            assert_eq!(Value64::from(v16), v64);
            assert_eq!(Value128::from(v16), v128);
            assert_eq!(Value64::from(v32), v64);
            assert_eq!(Value128::from(v32), v128);
            assert_eq!(Value128::from(v64), v128);
            // Standard narrowing traits agree too.
            assert_eq!(Value8::try_from(v16), Ok(v8));
            assert_eq!(Value8::try_from(v32), Ok(v8));
            assert_eq!(Value8::try_from(v64), Ok(v8));
            assert_eq!(Value8::try_from(v128), Ok(v8));
            assert_eq!(Value16::try_from(v32), Ok(v16));
            assert_eq!(Value16::try_from(v64), Ok(v16));
            assert_eq!(Value16::try_from(v128), Ok(v16));
            assert_eq!(Value32::try_from(v64), Ok(v32));
            assert_eq!(Value32::try_from(v128), Ok(v32));
            assert_eq!(Value64::try_from(v128), Ok(v64));
        }
    }
    #[test]
    const fn regrading_methods_are_const() {
        const V8: Value8 = unwrap![some Value8::try_from_int(-3)];
        const V16: Value16 = V8.widen();
        const V128: Value128 = V8.to_128();
        const V8_ADJ: Option<Value8> = V16.try_narrow();
        const V8_DIRECT: Option<Value8> = V128.try_to_8();
        const_assert!(eq V16.as_int().unwrap(), -3);
        const_assert!(eq V128.as_int().unwrap(), -3);
        const_assert!(eq V8_ADJ.unwrap().as_int().unwrap(), V8.as_int().unwrap());
        const_assert!(eq V8_DIRECT.unwrap().as_int().unwrap(), V8.as_int().unwrap());
    }
    #[test]
    fn direct_narrowing_checks_target_capacity() {
        let uint = Value128::try_from_uint(16).unwrap();
        assert_eq!(uint.try_to_8(), None);
        assert!(Value8::try_from(uint).is_err());
        let int = Value128::try_from_int(-9).unwrap();
        assert_eq!(int.try_to_8(), None);
        let char = Value128::try_from_char('\u{0010}').unwrap();
        assert_eq!(char.try_to_8(), None);
        let symbol = Value128::try_from_parts(ValueKind4::Symbol, 16).unwrap();
        assert_eq!(symbol.try_to_8(), None);
    }
}
