// devela_base_core::num::float::bits
//
//! Defines [`f32bits`]|`_niche`, [`f64bits`]|`_niche`.
//

#![allow(non_camel_case_types)]

use crate::{ConstInitCore, NonExtremeU32, NonExtremeU64, ne, paste};

// Macro helper to implement the types f32bits, f32bits_niche, ...
macro_rules! impl_fbits {
    () => { impl_fbits![f32+u32, f64+u64]; };
    ($($float:ident + $bits:ident),+ $(,)? ) => { paste! { $( impl_fbits![
        @[<$float bits>], [<$float bits_niche>], $float, $bits, [<NonExtreme $bits:upper>]
    ]; )+ }};
    (@$non_niche:ident, $niche:ident, $float:ident, $bits:ident, $NE:ident) => { paste! {
        /* non-niche */

        #[doc = crate::_TAG_NUM!()]
        #[doc = crate::_TAG_FFI!()]
        #[doc = "Bitwise wrapper for `" $float "` providing `Eq`, `Ord`, and `Hash`."]
        ///
        #[doc = "This stores the raw IEEE-754 bits of a `" $float "` in a `" $bits "`."]
        /// Ordering and hashing operate on the raw bit pattern, not the numeric value.
        /// All bit patterns are preserved, including NaNs and signed zeroes.
        ///
        #[doc = "For a masked and niche-compressed variant, see [`" $niche "`]."]
        #[must_use]
        #[repr(transparent)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $non_niche($bits);

        impl $non_niche {
            /// Converts a `$float` into its raw-bit representation.
            pub const fn new(float: $float) -> Self { Self(float.to_bits()) }
            /// Wraps an existing raw bit pattern.
            pub const fn from_bits(bits: $bits) -> Self { Self(bits) }

            /// Reinterprets the stored bits as a `$float`.
            pub const fn as_float(self) -> $float { <$float>::from_bits(self.0) }
            /// Returns the underlying raw bits unchanged.
            pub const fn as_bits(self) -> $bits { self.0 }

            /// Converts to the niche variant.
            pub const fn to_niche(self) -> $niche { $niche::from_bits(self.as_bits()) }
        }

        impl Default for $non_niche {
            /// The default value is the bit pattern of `0.0`.
            fn default() -> Self { Self::new(0.0) }
        }
        impl ConstInitCore for $non_niche {
            /// The initialization value is the bit pattern of `0.0`.
            const INIT: Self = Self::new(0.0);
        }

        impl From<$float> for $non_niche {
            fn from(from: $float) -> $non_niche { $non_niche::new(from) }
        }
        impl From<$non_niche> for $float {
            fn from(from: $non_niche) -> $float { from.as_float() }
        }

        /* niche */

        #[doc = crate::_TAG_NUM!()]
        #[doc = crate::_TAG_NICHE!()]
        #[doc = "Bitwise wrapper for `" $float "` stored through a masked [`" $NE "`]."]
        ///
        /// This preserves all IEEE-754 bit patterns except the prohibited value.
        /// Ordering and hashing operate on the masked bit pattern.
        /// Signed zeroes and all NaNs are represented, with a single NaN payload
        /// (the prohibited one) mapped to an adjacent payload.
        ///
        /// # Reserved NaN Payload
        #[doc = "The only prohibited bit pattern is `" $bits "::MAX`."]
        ///
        /// It is a quiet NaN that also matches the highest value of the
        /// underlying unsigned integer type. This makes it a good sentinel:
        /// it never appears in normal floating-point or millisecond values.
        ///
        /// If this bit pattern is used, it is automatically replaced with
        /// the previous representable payload, keeping every other value intact.
        ///
        #[doc = "For the unmasked, fully identity-preserving variant, see [`" $non_niche "`]."]
        #[must_use]
        #[repr(transparent)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $niche($NE);

        impl $niche {
            #[doc = "Converts a `" $float "` into its masked bit representation."]
            ///
            /// All bit patterns are preserved except the prohibited one.
            pub const fn new(float: $float) -> Self {
                Self(ne![lossy float.to_bits(), $bits])
            }
            #[doc = "Wraps an existing raw bit pattern, masking it through `" $NE "`."]
            pub const fn from_bits(bits: $bits) -> Self {
                Self(ne![lossy bits, $bits])
            }

            #[doc = "Reinterprets the stored bits as a `" $float "`."]
            pub const fn as_float(self) -> $float { <$float>::from_bits(self.as_bits()) }

            /// Returns the masked raw bits.
            ///
            /// The prohibited value is never returned; it is always replaced
            #[doc = "by the remapped payload defined by `" $NE "`."]
            pub const fn as_bits(self) -> $bits { self.0.get() }

            /// Converts to the non-niche variant.
            pub const fn to_non_niche(self) -> $non_niche { $non_niche::from_bits(self.as_bits()) }
        }

        impl Default for $niche {
            /// The default value is the bit pattern of `0.0`.
            fn default() -> Self { Self::new(0.0) }
        }
        impl ConstInitCore for $niche {
            /// The initialization value is the bit pattern of `0.0`.
            const INIT: Self = Self::new(0.0);
        }

        impl From<$float> for $niche {
            fn from(from: $float) -> $niche { $niche::new(from) }
        }
        impl From<$niche> for $float {
            fn from(from: $niche) -> $float { from.as_float() }
        }
    }};
}
impl_fbits!();
