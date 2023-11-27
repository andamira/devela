// devela::math::ops::fns::scale_lerp
//
//! scaling and linearly interpolation functions
//
// TOC
// - sint|uint|float:
//   - scale
//   - lerp

use crate::meta::paste;

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
use crate::math::num::fsize;

// $t:   the input/output type
// $up:  the upcasted type to do the operations on (the ones that can overflow)
// $ft:  the floating-point type to do the operations on
macro_rules! impl_ops {
    (signed $( ($t:ty, $up:ty, $ft:ty) ),+) => { $( impl_ops![@signed($t, $up, $ft)]; )+ };
    (unsigned $( ($t:ty, $up:ty, $ft:ty) ),+) => { $( impl_ops![@unsigned($t, $up, $ft)]; )+ };
    (float $($t:ty ),+) => { $( impl_ops![@float($t)]; )+ };

    // implements signed ops
    (@signed($t:ty, $up:ty, $ft:ty) ) => { paste! {
        /* signed scale */

        #[doc = "Returns a scaled [`" $t
            "`] `v`alue in `[min..=max]` to a new range `[a..=b]`.\n\n"]
        #[doc = "It upcasts internally to [`" $up "`] for the intermediate operations."]
        ///
        /// # Formula
        /// $$ \large v' = (b - a) \frac{v - min}{max - min} + a $$
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::math::ops::scale_" $t ";\n\n"]
        #[doc = "assert_eq![scale_" $t "(60, 0, 120, 30, 50), 40];"]
        #[doc = "assert_eq![scale_" $t "(60, 0, 120, 30, 50), 40];"]
        /// ```
        pub const fn [<scale_ $t>](v: $t, min: $t, max: $t, a: $t, b: $t) -> $t {
            let (v, min, max, a, b) = (v as $up, min as $up, max as $up, a as $up, b as $up);
            ((b - a) * (v - min) / (max - min) + a) as $t
        }

        #[doc = "Returns an interpolated [`" $t "`] in `[a..=b]` with an [`" $ft
            "`] `pct` in `[0..=1]`.\n\n"]
        ///
        #[doc ="You can also use [`scale_" $t "`] for the same purpose."]
        /// Integer operations can have more precision for very large values.
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::math::ops::{lerp_" $t ", scale_" $t "};\n\n"]
        #[doc = "assert_eq![lerp_" $t "(0.5, 40, 80), 60];"]
        ///
        /// // equivalence using integer scaling:
        #[doc = "assert_eq![scale_" $t "(50, 0, 100, 40, 80), 60];"]
        /// ```
        pub fn [<lerp_ $t>](pct: $ft, a: $t, b: $t) -> $t {
            ((1.0 - pct) * (a as $ft) + pct * (b as $ft)) as $t
        }
    }};

    // implements unsigned ops
    (@unsigned($t:ty, $up:ty, $ft:ty) ) => { paste! {
        /* unsigned scale */

        #[doc = "Returns a scaled [`" $t
            "`] `v`alue in `[min..=max]` to a new range `[a..=b]`.\n\n"]
        #[doc = "It upcasts internally to [`" $up "`] for the intermediate operations."]
        ///
        /// # Formula
        /// $$ \large v' = (b - a) \frac{v - min}{max - min} + a $$
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::math::ops::scale_" $t ";\n\n"]
        #[doc = "assert_eq![scale_" $t "(60, 0, 120, 30, 50), 40];"]
        /// ```
        pub const fn [<scale_ $t>](v: $t, min: $t, max: $t, a: $t, b: $t) -> $t {
            let (v, min, max, a, b) = (v as $up, min as $up, max as $up, a as $up, b as $up);
            ((b - a) * (v - min) / (max - min) + a) as $t
        }

        #[doc = "Returns an interpolated [`" $t "`] in `[a..=b]` with an [`" $ft
            "`] `pct` in `[0..=1]`.\n\n"]
        ///
        #[doc ="You can also use the [`scale_" $t "`] function for the same purpose,"]
        /// which can have more precision for large values.
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::math::ops::{lerp_" $t ", scale_" $t "};\n\n"]
        #[doc = "assert_eq![lerp_" $t "(0.5, 40, 80), 60];"]
        ///
        /// // equivalence using integer scaling:
        #[doc = "assert_eq![scale_" $t "(50, 0, 100, 40, 80), 60];"]
        /// ```
        pub fn [<lerp_ $t>](pct: $ft, a: $t, b: $t) -> $t {
            ((1.0 - pct) * (a as $ft) + pct * (b as $ft)) as $t
        }
    }};

    (@float($t:ty) ) => { paste! {
        /* scale, lerp */

        #[doc = "Returns a scaled [` " $t
            " `] `v`alue in `[min..=max]` to a new range `[a..=b]`."]
        ///
        /// # Formula
        /// $$ \large v' = (b - a) \frac{v - min}{max - min} + a $$
        ///
        /// # Examples
        /// ```
        #[doc = "use devela::math::ops::scale_" $t ";\n\n"]
        #[doc = "assert_eq![scale_" $t "(45., 0., 360., 0., 1.), 0.125];"]
        #[doc = "assert_eq![scale_" $t "(45., 0., 360., -1., 1.), -0.75];"]
        ///
        #[doc = "assert_eq![scale_" $t "(0.125, 0., 1., 0., 360.), 45.];"]
        #[doc = "assert_eq![scale_" $t "(-0.75, -1., 1., 0., 360.), 45.];"]
        /// ```
        #[inline]
        #[must_use]
        pub fn [<scale_ $t>](v: $t, min: $t, max: $t, a: $t, b: $t) -> $t {
            (b - a) * (v - min) / (max - min) + a
        }

        #[doc = "Returns an interpolated [`" $t
            "`] in `[a..=b]` with a `pct` in `[0..=1]`.\n\n"]
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::math::ops::lerp_" $t ";\n\n"]
        #[doc = "assert_eq![lerp_" $t "(0.5, 40., 80.), 60.];"]
        /// ```
        pub fn [<lerp_ $t>](pct: $t, a: $t, b: $t) -> $t {
            (1.0 - pct) * a + pct * b
        }
    }};
}
impl_ops![
    signed(i8, i16, f32),
    (i16, i32, f32),
    (i32, i64, f32),
    (i64, i128, f64),
    (i128, i128, f64),
    (isize, isize, fsize)
];
impl_ops![
    unsigned(u8, u16, f32),
    (u16, u32, f32),
    (u32, u64, f32),
    (u64, u128, f64),
    (u128, u128, f64),
    (usize, usize, fsize)
];

impl_ops![float f32, f64];

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
impl_ops![float fsize];
