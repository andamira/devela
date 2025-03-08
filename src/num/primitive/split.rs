// devela::data::conversion::cast::split
//
//! fns to split a primitive into an array of smaller primitives.
//
// TOC
// - trait definition
// - trait implementation
// - wrapper implementations

use crate::{paste, Cast};

/// Offers methods to split a primitive into an array of smaller primitives.
///
/// See also the [`Cast`] type for the equivalent *const* methods, and the
/// [`PrimitiveJoin`][super::PrimitiveJoin] trait for the opposite operations.
pub trait PrimitiveSplit<T, const LEN: usize> {
    /// Splits `self` into an array of `T` in big-endian order.
    #[must_use]
    fn into_array_be(self) -> [T; LEN];
    /// Splits `self` into an array of `T` in little-endian order.
    #[must_use]
    fn into_array_le(self) -> [T; LEN];
    /// Splits `self` into an array of `T` in native-endian order.
    #[must_use]
    fn into_array_ne(self) -> [T; LEN];
}

// Implements the trait methods
macro_rules! impl_into_trait {
    ( $( $P:ident, $T:ident, $LEN:literal );+ $(;)? ) => {
        $( impl_into_trait![@$P, $T, $LEN]; )+
    };
    (@$P:ident, $T:ident, $LEN:literal) => { paste! {
        impl PrimitiveSplit<$T, $LEN> for $P {
            fn into_array_be(self) -> [$T; $LEN] { Cast(self).[<into_ $T _be>]() }
            fn into_array_le(self) -> [$T; $LEN] { Cast(self).[<into_ $T _le>]() }
            fn into_array_ne(self) -> [$T; $LEN] { Cast(self).[<into_ $T _ne>]() }
        }
    }};
}
impl_into_trait![
    u128, u64, 2; u128, u32, 4; u128, u16, 8; u128, u8, 16;
    u64, u32, 2; u64, u16, 4; u64, u8, 8;
    u32, u16, 2; u32, u8, 4;
    u16, u8, 2;
];

/* implements the Cast wrapper methods */

#[rustfmt::skip]
#[cfg_attr(nightly_doc, doc(cfg(feature = "split")))]
impl Cast<u16> {
    /// Splits a `u16` into an array of `[u8; 2]` in big-endian order.
    #[must_use]
    pub const fn into_u8_be(self) -> [u8; 2] { self.0.to_be_bytes() }

    /// Splits a `u16` into an array of `[u8; 2]` in little-endian order.
    #[must_use]
    pub const fn into_u8_le(self) -> [u8; 2] { self.0.to_le_bytes() }

    /// Splits a `u16` into an array of `[u8; 2]` in native-endian order.
    #[must_use]
    pub const fn into_u8_ne(self) -> [u8; 2] { self.0.to_ne_bytes() }
}

#[rustfmt::skip]
#[cfg_attr(nightly_doc, doc(cfg(feature = "split")))]
impl Cast<u32> {
    /// Splits a `u32` into an array of `[u16; 2]` in big-endian order.
    #[must_use]
    pub const fn into_u16_be(self) -> [u16; 2] {
        let v0: u16 = ((self.0 >> 16) & u16::MAX as u32) as u16;
        let v1: u16 = (self.0 & u16::MAX as u32) as u16;
        [v0, v1]
    }

    /// Splits a `u32` into an array of `[u16; 2]` in little-endian order.
    #[must_use]
    pub const fn into_u16_le(self) -> [u16; 2] {
        let v1: u16 = ((self.0 >> 16) & u16::MAX as u32) as u16;
        let v0: u16 = (self.0 & u16::MAX as u32) as u16;
        [v0, v1]
    }

    /// Splits a `u32` into an array of `[u16; 2]` in native-endian order.
    #[must_use]
    pub const fn into_u16_ne(self) -> [u16; 2] {
        if cfg!(target_endian = "big") {
            Cast::<u32>::into_u16_be(self)
        } else {
            Cast::<u32>::into_u16_le(self)
        }
    }

    /// Splits a `u32` into an array of `[u8; 4]` in big-endian order.
    #[must_use]
    pub const fn into_u8_be(self) -> [u8; 4] { self.0.to_be_bytes() }

    /// Splits a `u32` into an array of `[u8; 4]` in little-endian order.
    #[must_use]
    pub const fn into_u8_le(self) -> [u8; 4] { self.0.to_le_bytes() }

    /// Splits a `u32` into an array of `[u8; 4]` in native-endian order.
    #[must_use]
    pub const fn into_u8_ne(self) -> [u8; 4] { self.0.to_ne_bytes() }
}

#[rustfmt::skip]
#[cfg_attr(nightly_doc, doc(cfg(feature = "split")))]
impl Cast<u64> {
    /// Splits a `u64` into an array of `[u32; 2]` in big-endian order.
    #[must_use]
    pub const fn into_u32_be(self) -> [u32; 2] {
        let v0: u32 = ((self.0 >> 32) & u32::MAX as u64) as u32;
        let v1: u32 = (self.0 & u32::MAX as u64) as u32;
        [v0, v1]
    }

    /// Splits a `u64` into an array of `[u32; 2]` in little-endian order.
    #[must_use]
    pub const fn into_u32_le(self) -> [u32; 2] {
        let v1: u32 = ((self.0 >> 32) & u32::MAX as u64) as u32;
        let v0: u32 = (self.0 & u32::MAX as u64) as u32;
        [v0, v1]
    }

    /// Splits a `u64` into an array of `[u32; 2]` in native-endian order.
    #[must_use]
    pub const fn into_u32_ne(self) -> [u32; 2] {
        if cfg!(target_endian = "big") {
            Cast::<u64>::into_u32_be(self)
        } else {
            Cast::<u64>::into_u32_le(self)
        }
    }

    /// Splits a `u64` into an array of `[u16; 4]` in big-endian order.
    #[must_use]
    pub const fn into_u16_be(self) -> [u16; 4] {
        let v0: u16 = ((self.0 >> (16 * 3)) & u16::MAX as u64) as u16;
        let v1: u16 = ((self.0 >> (16 * 2)) & u16::MAX as u64) as u16;
        let v2: u16 = ((self.0 >> 16) & u16::MAX as u64) as u16;
        let v3: u16 = (self.0 & u16::MAX as u64) as u16;
        [v0, v1, v2, v3]
    }

    /// Splits a `u64` into an array of `[u16; 4]` in little-endian order.
    #[must_use]
    pub const fn into_u16_le(self) -> [u16; 4] {
        let v3: u16 = ((self.0 >> (16 * 3)) & u16::MAX as u64) as u16;
        let v2: u16 = ((self.0 >> (16 * 2)) & u16::MAX as u64) as u16;
        let v1: u16 = ((self.0 >> 16) & u16::MAX as u64) as u16;
        let v0: u16 = (self.0 & u16::MAX as u64) as u16;
        [v0, v1, v2, v3]
    }

    /// Splits a `u64` into an array of `[u16; 4]` in native-endian order.
    #[must_use]
    pub const fn into_u16_ne(self) -> [u16; 4] {
        if cfg!(target_endian = "big") {
            Cast::<u64>::into_u16_be(self)
        } else {
            Cast::<u64>::into_u16_le(self)
        }
    }

    /// Splits a `u64` into an array of `[u8; 8]` in big-endian order.
    #[must_use]
    pub const fn into_u8_be(self) -> [u8; 8] { self.0.to_be_bytes() }

    /// Splits a `u64` into an array of `[u8; 8]` in little-endian order.
    #[must_use]
    pub const fn into_u8_le(self) -> [u8; 8] { self.0.to_le_bytes() }

    /// Splits a `u64` into an array of `[u8; 8]` in native-endian order.
    #[must_use]
    pub const fn into_u8_ne(self) -> [u8; 8] {
        if cfg!(target_endian = "big") {
            Cast::<u64>::into_u8_be(self)
        } else {
            Cast::<u64>::into_u8_le(self)
        }
    }
}

#[rustfmt::skip]
#[cfg_attr(nightly_doc, doc(cfg(feature = "split")))]
impl Cast<u128> {
    /// Splits a `u128` into an array of `[u64; 2]` in big-endian order.
    #[must_use]
    pub const fn into_u64_be(self) -> [u64; 2] {
        let v0: u64 = (self.0 >> 64) as u64;
        let v1: u64 = (self.0 & u64::MAX as u128) as u64;
        [v0, v1]
    }

    /// Splits a `u128` into an array of `[u64; 2]` in little-endian order.
    #[must_use]
    pub const fn into_u64_le(self) -> [u64; 2] {
        let v1: u64 = (self.0 >> 64) as u64;
        let v0: u64 = (self.0 & u64::MAX as u128) as u64;
        [v0, v1]
    }

    /// Splits a `u128` into an array of `[u64; 2]` in native-endian order.
    #[must_use]
    pub const fn into_u64_ne(self) -> [u64; 2] {
        if cfg!(target_endian = "big") {
            Cast::<u128>::into_u64_be(self)
        } else {
            Cast::<u128>::into_u64_le(self)
        }
    }

    /// Splits a `u128` into an array of `[u32; 4]` in big-endian order.
    #[must_use]
    pub const fn into_u32_be(self) -> [u32; 4] {
        let v0: u32 = (self.0 >> (32 * 3)) as u32;
        let v1: u32 = ((self.0 >> (32 * 2)) & u32::MAX as u128) as u32;
        let v2: u32 = ((self.0 >> 32) & u32::MAX as u128) as u32;
        let v3: u32 = (self.0 & u32::MAX as u128) as u32;
        [v0, v1, v2, v3]
    }

    /// Splits a `u128` into an array of `[u32; 4]` in little-endian order.
    #[must_use]
    pub const fn into_u32_le(self) -> [u32; 4] {
        let v3: u32 = (self.0 >> (32 * 3)) as u32;
        let v2: u32 = ((self.0 >> (32 * 2)) & u32::MAX as u128) as u32;
        let v1: u32 = ((self.0 >> 32) & u32::MAX as u128) as u32;
        let v0: u32 = (self.0 & u32::MAX as u128) as u32;
        [v0, v1, v2, v3]
    }

    /// Splits a `u128` into an array of `[u32; 4]` in native-endian order.
    #[must_use]
    pub const fn into_u32_ne(self) -> [u32; 4] {
        if cfg!(target_endian = "big") {
            Cast::<u128>::into_u32_be(self)
        } else {
            Cast::<u128>::into_u32_le(self)
        }
    }

    /// Splits a `u128` into an array of `[u16; 8]` in big-endian order.
    #[must_use]
    pub const fn into_u16_be(self) -> [u16; 8] {
        let v0: u16 = (self.0 >> (16 * 7)) as u16;
        let v1: u16 = ((self.0 >> (16 * 6)) & u16::MAX as u128) as u16;
        let v2: u16 = ((self.0 >> (16 * 5)) & u16::MAX as u128) as u16;
        let v3: u16 = ((self.0 >> (16 * 4)) & u16::MAX as u128) as u16;
        let v4: u16 = ((self.0 >> (16 * 3)) & u16::MAX as u128) as u16;
        let v5: u16 = ((self.0 >> (16 * 2)) & u16::MAX as u128) as u16;
        let v6: u16 = ((self.0 >> 16) & u16::MAX as u128) as u16;
        let v7: u16 = (self.0 & u16::MAX as u128) as u16;
        [v0, v1, v2, v3, v4, v5, v6, v7]
    }

    /// Splits a `u128` into an array of `[u16; 8]` in little-endian order.
    #[must_use]
    pub const fn into_u16_le(self) -> [u16; 8] {
        let v7: u16 = (self.0 >> (16 * 7)) as u16;
        let v6: u16 = ((self.0 >> (16 * 6)) & u16::MAX as u128) as u16;
        let v5: u16 = ((self.0 >> (16 * 5)) & u16::MAX as u128) as u16;
        let v4: u16 = ((self.0 >> (16 * 4)) & u16::MAX as u128) as u16;
        let v3: u16 = ((self.0 >> (16 * 3)) & u16::MAX as u128) as u16;
        let v2: u16 = ((self.0 >> (16 * 2)) & u16::MAX as u128) as u16;
        let v1: u16 = ((self.0 >> 16) & u16::MAX as u128) as u16;
        let v0: u16 = (self.0 & u16::MAX as u128) as u16;
        [v0, v1, v2, v3, v4, v5, v6, v7]
    }

    /// Splits a `u128` into an array of `[u16; 8]` in native-endian order.
    #[must_use]
    pub const fn into_u16_ne(self) -> [u16; 8] {
        if cfg!(target_endian = "big") {
            Cast::<u128>::into_u16_be(self)
        } else {
            Cast::<u128>::into_u16_le(self)
        }
    }

    /// Splits a `u128` into an array of `[u8; 16]` in big-endian order.
    #[must_use]
    pub const fn into_u8_be(self) -> [u8; 16] { self.0.to_be_bytes() }

    /// Splits a `u128` into an array of `[u8; 16]` in little-endian order.
    #[must_use]
    pub const fn into_u8_le(self) -> [u8; 16] { self.0.to_le_bytes() }

    /// Splits a `u128` into an array of `[u8; 16]` in native-endian order.
    #[must_use]
    pub const fn into_u8_ne(self) -> [u8; 16] { self.0.to_ne_bytes() }
}
