// devela_base_core::num::float::bits
//
//! Defines [`f32bits`], [`f64bits`].
//

#![allow(non_camel_case_types)]

macro_rules! impl_fbits {
    () => { impl_fbits![f32+u32, f64+u64]; };
    ($($float:ident + $bits:ident),+) => { $crate::paste! {
        $( impl_fbits![@[<$float bits>], $float, $bits]; )+
    }};
    (@$name:ident, $float:ident, $bits:ident) => { $crate::paste! {
        #[doc = "Bitwise wrapper for `" $float "` providing `Eq`, `Ord`, and `Hash`."]
        ///
        #[doc = "This stores the raw IEEE-754 bits of a `" $float "` in a `" $bits "`."]
        /// Ordering and hashing operate on the raw bit pattern, not the numeric value.
        /// All bit patterns are preserved, including NaNs and signed zeroes.
        #[repr(transparent)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name($bits);

        impl $name {
            /// Converts a `$float` into its raw-bit representation.
            pub const fn new(float: $float) -> Self { Self(float.to_bits()) }
            /// Wraps an existing raw bit pattern.
            pub const fn from_bits(bits: $bits) -> Self { Self(bits) }

            /// Reinterprets the stored bits as a `$float`.
            pub const fn as_float(self) -> $float { <$float>::from_bits(self.0) }
            /// Returns the underlying raw bits unchanged.
            pub const fn as_bits(self) -> $bits { self.0 }
        }

        impl Default for $name {
            /// The default value is the bit pattern of `0.0`.
            fn default() -> Self { Self::new(0.0) }
        }
        impl $crate::ConstInitCore for $name {
            /// The initialization value is the bit pattern of `0.0`.
            const INIT: Self = Self::new(0.0);
        }

        impl From<$float> for $name {
            fn from(from: $float) -> $name { $name::new(from) }
        }
        impl From<$name> for $float {
            fn from(from: $name) -> $float { from.as_float() }
        }
    }};
}
impl_fbits!();
