// devela_base_core::num::quant::scale::namespace
//
//! Defines [`Scale`]
//

#![allow(missing_docs, unused)]

// CLARIFYING NOTES:
// - isize_up and usize_up are aliases defined for the current pointer width.
// - unwrap![some? value] is exactly the same as value? but works in const fns returning Option.

use crate::{is, isize_up, nz, unwrap, usize_up};

#[doc = crate::_tags!(quant)]
/// Comprehensive scaling operations.
#[doc = crate::_doc_location!("num/quant")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Scale<T>(T);

macro_rules! impl_scale {
    () => {
        impl_scale![sint: i8:i16, i16:i32, i32:i64, i64:i128, isize:isize_up, i128:i128];
        impl_scale![uint: u8:u16, u16:u32, u32:u64, u64:u128, usize:usize_up, u128:u128];
    };
    ( // signed integers specific methods and optimizations.
      //
      // $T:   the signed type
      // $UP:  the signed upscaled type
      sint: $( $T:ty : $UP:ty ),+ $(,)?) => { $( impl_scale!(@sint: $T:$UP); )+ };
    (@sint:    $T:ty : $UP:ty ) => {
        impl Scale<$T> {
            /// Standard scaling.
            ///
            /// Upcasts internally (except for 128-bit).
            ///
            /// # Panics
            /// Can panic in 128-bit for very large values.
            ///
            /// # Performance
            /// Includes the following optimizations:
            /// - avoids sign checks during arithmetic (no branching).
            /// - uses shift instead of division when `(imax - imin)` is a power of 2.
            pub const fn scale(self, imin: $T, imax: $T, omin: $T, omax: $T) -> $T {
                if imax == imin { return omin; } // early return, avoids division by 0

                // Compute absolute ranges to avoid sign checks during arithmetic (no branching)
                let irange = (imax as $UP).abs_diff(imin as $UP);
                let orange = (omax as $UP).abs_diff(omin as $UP);

                // general case:
                // let scaled = (self.0 as $UP).abs_diff(imin as $UP) * orange / irange;
                // // Restore sign (compiles to conditional move)
                // if self.0 < imin { omin - scaled as $T } else { omin + scaled as $T }

                let v = self.0 as $UP;
                let shift = irange.trailing_zeros();
                if irange == (1 << shift) { // power-of-two fast path
                    let scaled = (v.abs_diff(imin as $UP).wrapping_mul(orange) >> shift) as $T;
                    is![self.0 < imin, omin.wrapping_sub(scaled), omin.wrapping_add(scaled)]

                } else { // general case
                    let scaled = (v.abs_diff(imin as $UP).wrapping_mul(orange) / irange) as $T;
                    is![self.0 < imin, omin.wrapping_sub(scaled), omin.wrapping_add(scaled)]
                }
            }

            impl_scale! {@shared: $T:$UP }
        }
    };
    ( // unsigned integers specific methods and optimizations.
      uint: $( $T:ty : $UP:ty ),+ $(,)?) => { $( impl_scale!(@uint: $T : $UP); )+ };
    (@uint:    $T:ty : $UP:ty) => {
        impl Scale<$T> {
            /// Standard scaling.
            ///
            /// Upcasting for the operations (except for 128-bit).
            ///
            /// # Panics
            /// Can panic in 128-bit for very large values.
            pub const fn scale(self, imin: $T, imax: $T, omin: $T, omax: $T) -> $T {
                if imax == imin { return omin; } // early return, avoids division by 0

                let v = self.0 as $UP;
                let (imin, imax) = (imin as $UP, imax as $UP);
                let (omin, omax) = (omin as $UP, omax as $UP);

                // general case:
                // ((v - imin) * (omax - omin) / (imax - imin) + omin) as $T

                let divisor = imax.wrapping_sub(imin);
                let shift = divisor.trailing_zeros() as $UP;

                if divisor == (1 << shift) { // Power-of-two fast path
                    (v.wrapping_sub(imin)
                        .wrapping_mul(omax.wrapping_sub(omin)) >> shift + omin) as $T
                } else { // general case:
                    ((v - imin) * (omax - omin) / divisor + omin) as $T
                }

            }

            impl_scale! {@shared: $T:$UP }
        }
    };
    ( // shared methods
      shared: $( $T:ty : $UP:ty ),+ $(,)?) => { $( impl_scale!(@shared: $T : $UP); )+ };
    (@shared:    $T:ty : $UP:ty) => {
        /// Returns a scaled value
        /// between `[imin..=imax]` to a new range `[omin..=omax]`.
        ///
        /// The value saturates around the type boundaries.
        // FIXME: doesn't work. Saturating should maybe done at the end
        pub const fn scale_sat(self, imin: $T, imax: $T, omin: $T, omax: $T) -> $T {
            let v = self.0;
            let scaled = (v - imin).saturating_mul(omax - omin) / (imax - imin);
            scaled.saturating_add(omin)
        }
        /// Returns a scaled value
        /// between `[imin..=imax]` to a new range `[omin..=omax]`.
        ///
        /// - If the value lies outside of `[imin..=imax]` it will result in extrapolation.
        /// - If the final value doesn't fit in the type it will wrap around its boundaries.
        pub const fn scale_wrap(self, imin: $T, imax: $T, omin: $T, omax: $T) -> $T {
            let v = self.0 as $UP;
            let (imin, imax) = (imin as $UP, imax as $UP);
            let (omin, omax) = (omin as $UP, omax as $UP);

            ((v - imin) * (omax - omin) / (imax - imin) + omin) as $T
        }
    };
}
impl_scale!();

#[cfg(test)]
mod tests {
    use super::Scale;

    #[test]
    fn uint_scale() {
        /* power of two divisor (uses shift) */
        // upscale
        assert_eq![Scale(32u8).scale(0, 32, 0, 64), 64];
        assert_eq![Scale(32u8).scale(0, 64, 0, 128), 64];
        // downscale
        assert_eq![Scale(32u8).scale(0, 32, 0, 16), 16];

        /* non power of 2 divisor (uses division) */
        // upscale
        assert_eq![Scale(31u8).scale(0, 31, 0, 62), 62];
        assert_eq![Scale(31u8).scale(0, 63, 0, 126), 62];
    }

    #[test]
    fn sint_scale() {
        // TODO
    }

    // FIXME
    // #[test]
    // fn scale_sat() {
    //     assert_eq![Scale(32u8).scale_sat(0, 32, 0, 64), 64]; // FAILS: 7
    //     assert_eq![Scale(32u8).scale_sat(0, 64, 0, 128), 64]; // FAILS: 3
    //
    //     // saturates at max
    //     assert_eq!(Scale(200u8).scale_sat(0, 255, 0, 100), 100); // FAILS: 1
    //     // saturates at min
    //     assert_eq!(Scale(-50i8).scale_sat(0, 100, 0, 127), 0); // FAILS: -1
    // }

    #[test]
    fn scale_wrap() {}
}
