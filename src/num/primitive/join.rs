// devela::data::conversion::cast::join
//
//! construct an unsigned primitive by joining an array of smaller unsigned primitives.
//
// TOC
// - trait definition
// - trait implementation
// - wrapper implementations

use crate::{Cast, paste};

/// Offers methods to construct a primitive from an array or slice of smaller primitives.
///
/// Methods expecting an array are more efficient than the ones expecting an
/// slice. On the other hand slices of any lenght are supported as follows:
/// - If the slice contains fewer elements than required, the method will
///   fill in the missing values with zeros.
/// - If the slice contains more elements than required, the method will
///   ignore the extra elements.
///
/// See also the [`Cast`] type for the equivalent *const* methods, and the
/// [`PrimitiveSplit`][super::PrimitiveSplit] trait for the opposite operations.
pub trait PrimitiveJoin<T, U, const LEN: usize> {
    /// Constructs a primitive `T` from an array of `U` in big-endian order.
    #[must_use]
    fn from_array_be(values: [U; LEN]) -> T;
    /// Constructs a primitive `T` from an array of `U` in little-endian order.
    #[must_use]
    fn from_array_le(values: [U; LEN]) -> T;
    /// Constructs a primitive `T` from an array of `U` in native-endian order.
    #[must_use]
    fn from_array_ne(values: [U; LEN]) -> T;

    /// Constructs a primitive `T` from a slice of `U` in big-endian order.
    #[must_use]
    fn from_slice_be(values: &[U]) -> T;
    /// Constructs a primitive `T` from a slice of `U` in little-endian order.
    #[must_use]
    fn from_slice_le(values: &[U]) -> T;
    /// Constructs a primitive `T` from a slice of `U` in native-endian order.
    #[must_use]
    fn from_slice_ne(values: &[U]) -> T;
}

/// Implements the trait methods.
macro_rules! impl_from_trait {
    ( $( $T:ident, $U:ident, $LEN:literal );+ $(;)? ) => {
        $( impl_from_trait![@$T, $U, $LEN]; )+
    };
    (@$T:ident, $U:ident, $LEN:literal) => { paste! {
        impl PrimitiveJoin<$T, $U, $LEN> for $T {
            fn from_array_be(values: [$U; $LEN]) -> $T { Cast::<$T>::[<from_ $U _be>](values) }
            fn from_array_le(values: [$U; $LEN]) -> $T { Cast::<$T>::[<from_ $U _le>](values) }
            fn from_array_ne(values: [$U; $LEN]) -> $T { Cast::<$T>::[<from_ $U _ne>](values) }
            fn from_slice_be(values: &[$U]) -> $T {
                let mut array = [0; $LEN];
                for (i, &v) in values.iter().enumerate() {
                    array[i] = v;
                }
                Cast::<$T>::[<from_ $U _be>](array)
            }
            fn from_slice_le(values: &[$U]) -> $T {
                let mut array = [0; $LEN];
                for (i, &v) in values.iter().enumerate() {
                    array[i] = v;
                }
                Cast::<$T>::[<from_ $U _le>](array)
            }
            fn from_slice_ne(values: &[$U]) -> $T {
                let mut array = [0; $LEN];
                for (i, &v) in values.iter().enumerate() {
                    array[i] = v;
                }
                Cast::<$T>::[<from_ $U _ne>](array)
            }
        }
    }};
}
impl_from_trait![
    u128, u64, 2; u128, u32, 4; u128, u16, 8; u128, u8, 16;
    u64, u32, 2; u64, u16, 4; u64, u8, 8;
    u32, u16, 2; u32, u8, 4;
    u16, u8, 2;
];

/* implements the Cast wrapper methods */

#[rustfmt::skip]
#[cfg_attr(nightly_doc, doc(cfg(feature = "join")))]
impl Cast<u16> {
    /// Constructs a `u16` from an array of `[u8; 2]` in big-endian order.
    #[must_use]
    pub const fn from_u8_be(v: [u8; 2]) -> u16 { u16::from_be_bytes(v) }

    /// Constructs a `u16` from an array of `[u8; 2]` in little-endian order.
    #[must_use]
    pub const fn from_u8_le(v: [u8; 2]) -> u16 { u16::from_le_bytes(v) }

    /// Constructs a `u16` from an array of `[u8; 2]` in native-endian order.
    #[must_use]
    pub const fn from_u8_ne(v: [u8; 2]) -> u16 { u16::from_ne_bytes(v) }
}

#[rustfmt::skip]
#[cfg_attr(nightly_doc, doc(cfg(feature = "join")))]
impl Cast<u32> {
    /// Constructs a `u32` from an array of `[u16; 2]` in big-endian order.
    #[must_use]
    pub const fn from_u16_be(v: [u16; 2]) -> u32 {
        ((v[0] as u32) << 16) | (v[1] as u32)
    }

    /// Constructs a `u32` from an array of `[u16; 2]` in little-endian order.
    #[must_use]
    pub const fn from_u16_le(v: [u16; 2]) -> u32 {
        ((v[1] as u32) << 16) | (v[0] as u32)
    }

    /// Constructs a `u32` from an array of `[u16; 2]` in native-endian order.
    #[must_use]
    pub const fn from_u16_ne(v: [u16; 2]) -> u32 {
        if cfg!(target_endian = "big") {
            Cast::<u32>::from_u16_be(v)
        } else {
            Cast::<u32>::from_u16_le(v)
        }
    }

    /// Constructs a `u32` from an array of `[u8; 4]` in big-endian order.
    #[must_use]
    pub const fn from_u8_be(v: [u8; 4]) -> u32 { u32::from_be_bytes(v) }

    /// Constructs a `u32` from an array of `[u8; 4]` in little-endian order.
    #[must_use]
    pub const fn from_u8_le(v: [u8; 4]) -> u32 { u32::from_le_bytes(v) }

    /// Constructs a `u32` from an array of `[u8; 4]` in native-endian order.
    #[must_use]
    pub const fn from_u8_ne(v: [u8; 4]) -> u32 { u32::from_ne_bytes(v) }
}

#[rustfmt::skip]
#[cfg_attr(nightly_doc, doc(cfg(feature = "join")))]
impl Cast<u64> {
    /// Constructs a `u64` from an array of `[u32; 2]` in big-endian order.
    #[must_use]
    pub const fn from_u32_be(v: [u32; 2]) -> u64 {
        ((v[0] as u64) << 32) | (v[1] as u64)
    }

    /// Constructs a `u64` from an array of `[u32; 2]` in little-endian order.
    #[must_use]
    pub const fn from_u32_le(v: [u32; 2]) -> u64 {
        ((v[1] as u64) << 32) | (v[0] as u64)
    }

    /// Constructs a `u64` from an array of `[u32; 2]` in native-endian order.
    #[must_use]
    pub const fn from_u32_ne(v: [u32; 2]) -> u64 {
        if cfg!(target_endian = "big") {
            Cast::<u64>::from_u32_be(v)
        } else {
            Cast::<u64>::from_u32_le(v)
        }
    }

    /// Constructs a `u64` from an array of `[u16; 4]` in big-endian order.
    #[must_use]
    pub const fn from_u16_be(v: [u16; 4]) -> u64 {
        ((v[0] as u64) << (16 * 3))
            | ((v[1] as u64) << (16 * 2))
            | ((v[2] as u64) << 16)
            | (v[3] as u64)
    }

    /// Constructs a `u64` from an array of `[u16; 4]` in little-endian order.
    #[must_use]
    pub const fn from_u16_le(v: [u16; 4]) -> u64 {
        ((v[3] as u64) << (16 * 3))
            | ((v[2] as u64) << (16 * 2))
            | ((v[1] as u64) << 16)
            | (v[0] as u64)
    }

    /// Constructs a `u64` from an array of `[u16; 4]` in native-endian order.
    #[must_use]
    pub const fn from_u16_ne(v: [u16; 4]) -> u64 {
        if cfg!(target_endian = "big") {
            Cast::<u64>::from_u16_be(v)
        } else {
            Cast::<u64>::from_u16_le(v)
        }
    }

    /// Constructs a `u64` from an array of `[u8; 8]` in big-endian order.
    #[must_use]
    pub const fn from_u8_be(v: [u8; 8]) -> u64 { u64::from_be_bytes(v) }

    /// Constructs a `u64` from an array of `[u8; 8]` in little-endian order.
    #[must_use]
    pub const fn from_u8_le(v: [u8; 8]) -> u64 { u64::from_le_bytes(v) }

    /// Constructs a `u64` from an array of `[u8; 8]` in native-endian order.
    #[must_use]
    pub const fn from_u8_ne(v: [u8; 8]) -> u64 { u64::from_ne_bytes(v) }
}

#[rustfmt::skip]
#[cfg_attr(nightly_doc, doc(cfg(feature = "join")))]
impl Cast<u128> {
    /// Constructs a `u128` from an array of `[u64; 2]` in big-endian order.
    #[must_use]
    pub const fn from_u64_be(v: [u64; 2]) -> u128 {
        ((v[0] as u128) << 64) | (v[1] as u128)
    }

    /// Constructs a `u128` from an array of `[u64; 2]` in little-endian order.
    #[must_use]
    pub const fn from_u64_le(v: [u64; 2]) -> u128 {
        ((v[1] as u128) << 64) | (v[0] as u128)
    }

    /// Constructs a `u128` from an array of `[u64; 2]` in native-endian order.
    #[must_use]
    pub const fn from_u64_ne(v: [u64; 2]) -> u128 {
        if cfg!(target_endian = "big") {
            Cast::<u128>::from_u64_be(v)
        } else {
            Cast::<u128>::from_u64_le(v)
        }
    }

    /// Constructs a `u128` from an array of `[u32; 4]` in big-endian order.
    #[must_use]
    pub const fn from_u32_be(v: [u32; 4]) -> u128 {
        ((v[0] as u128) << (32 * 3))
            | ((v[1] as u128) << (32 * 2))
            | ((v[2] as u128) << 32)
            | (v[3] as u128)
    }

    /// Constructs a `u128` from an array of `[u32; 4]` in little-endian order.
    #[must_use]
    pub const fn from_u32_le(v: [u32; 4]) -> u128 {
        ((v[3] as u128) << (32 * 3))
            | ((v[2] as u128) << (32 * 2))
            | ((v[1] as u128) << 32)
            | (v[0] as u128)
    }

    /// Constructs a `u128` from an array of `[u32; 4]` in native-endian order.
    #[must_use]
    pub const fn from_u32_ne(v: [u32; 4]) -> u128 {
        if cfg!(target_endian = "big") {
            Cast::<u128>::from_u32_be(v)
        } else {
            Cast::<u128>::from_u32_le(v)
        }
    }

    /// Constructs a `u128` from an array of `[u16; 8]` in big-endian order.
    #[must_use]
    pub const fn from_u16_be(v: [u16; 8]) -> u128 {
        ((v[0] as u128) << (16 * 7))
            | ((v[1] as u128) << (16 * 6))
            | ((v[2] as u128) << (16 * 5))
            | ((v[3] as u128) << (16 * 4))
            | ((v[4] as u128) << (16 * 3))
            | ((v[5] as u128) << (16 * 2))
            | ((v[6] as u128) << 16)
            | (v[7] as u128)
    }

    /// Constructs a `u128` from an array of `[u16; 8]` in little-endian order.
    #[must_use]
    pub const fn from_u16_le(v: [u16; 8]) -> u128 {
        ((v[7] as u128) << (16 * 7))
            | ((v[6] as u128) << (16 * 6))
            | ((v[5] as u128) << (16 * 5))
            | ((v[4] as u128) << (16 * 4))
            | ((v[3] as u128) << (16 * 3))
            | ((v[2] as u128) << (16 * 2))
            | ((v[1] as u128) << 16)
            | (v[0] as u128)
    }

    /// Constructs a `u128` from an array of `[u16; 8]` in native-endian order.
    #[must_use]
    pub const fn from_u16_ne(v: [u16; 8]) -> u128 {
        if cfg!(target_endian = "big") {
            Cast::<u128>::from_u16_be(v)
        } else {
            Cast::<u128>::from_u16_le(v)
        }
    }

    /// Constructs a `u128` from an array of `[u8; 16]` in big-endian order.
    #[must_use]
    pub const fn from_u8_be(v: [u8; 16]) -> u128 { u128::from_be_bytes(v) }

    /// Constructs a `u128` from an array of `[u8; 16]` in little-endian order.
    #[must_use]
    pub const fn from_u8_le(v: [u8; 16]) -> u128 { u128::from_le_bytes(v) }

    /// Constructs a `u128` from an array of `[u8; 16]` in native-endian order.
    #[must_use]
    pub const fn from_u8_ne(v: [u8; 16]) -> u128 { u128::from_ne_bytes(v) }
}
