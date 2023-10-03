// devela::cmp::always_fns
//
//! Comparison standalone functions.
//!
//! Always available for internal use.
//

#![allow(unused)]

use crate::codegen::paste;

// generate the const fns for primitive comparison
macro_rules! primitive_const_cmp {
    // $p: the types of the primitives
    ($($p:expr),+ $(,)?) => { paste! { $( primitive_const_cmp![@$p]; )+ }};

    // $p: the type of the primitive
    (@$p:expr) => { paste! {
        #[doc = "Compares and returns a clamped `" $p "` between `min` and `max`."]
        #[inline]
        #[must_use]
        pub const fn [<clamp_$p>](value: $p, min: $p, max: $p) -> $p {
            [<min_$p>]([<max_$p>](value, min), max)
        }

        #[doc = "Compares and returns the maximum of two `" $p "` values."]
        #[inline]
        #[must_use]
        pub const fn [<max_$p>](a: $p, b: $p) -> $p { if a > b { a } else { b } }

        #[doc = "Compares and returns the minimum of two `" $p "` values."]
        #[inline]
        #[must_use]
        pub const fn [<min_$p>](a: $p, b: $p) -> $p { if a < b { a } else { b } }
    }};
}
primitive_const_cmp![u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize];
