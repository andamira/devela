// devela_base_core::num::quant::scale
//
//! Defines [`Scale`]
//
// > https://chat.deepseek.com/a/chat/s/352900c3-1360-4500-9223-a02a6f23a986
//
// IDEAS
// common used scales as constants…
// - IDENTITY, etc.
//

#![allow(missing_docs, unused)]

// CLARIFYING NOTES:
// - isize_up and usize_up are aliases defined for the current pointer width.
// - unwrap![some? value] is exactly the same as value? but works in const fns returning Option.

use crate::{is, isize_up, nz, unwrap, usize_up};

#[doc = crate::_TAG_QUANT!()]
/// Comprehensive scaling operations.
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
                    is![self.0 < imin; omin.wrapping_sub(scaled); omin.wrapping_add(scaled)]

                } else { // general case
                    let scaled = (v.abs_diff(imin as $UP).wrapping_mul(orange) / irange) as $T;
                    is![self.0 < imin; omin.wrapping_sub(scaled); omin.wrapping_add(scaled)]
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

/* clipboard */

/*
///
// ±PROS:
// -
// ±CONS:
// -
*/

/*

/// 1 Baseline: 64-Bit Intermediate (Safe for ≤32-Bit Values)
// ±PROS:
// - Works for all integer types ≤64-bit.
// - No floating-point, minimal precision loss.
// ±CONS:
// - Overflow risk: Fails if (omax - omin) * (v - imin) exceeds u64 (e.g., scaling u32 with large ranges).
// - Inefficient for u8/i8 (unnecessary upcasting).
fn scale<T>(v: T, imin: T, imax: T, omin: T, omax: T) -> T
where
    T: Copy + Into<u64> + TryFrom<u64>,
{
    let v = v.into();
    let imin = imin.into();
    let imax = imax.into();
    let omin = omin.into();
    let omax = omax.into();

    let scaled = (v - imin) * (omax - omin) / (imax - imin);
    (scaled + omin).try_into().unwrap_or_else(|_| omax)
}


/// 2. Fixed-Point (Precision-Configurable)
// ±PROS:
// - Tunable precision (precision_bits).
// - Efficient for small types (no 64-bit ops).
// ±CONS:
// - Precision loss if precision_bits too low.
// - Still overflows if (omax - omin) << precision_bits exceeds type.
fn scale_fixed<T>(v: T, imin: T, imax: T, omin: T, omax: T, precision_bits: u32) -> T
where
    T: IntOps + Shl<u32, Output = T> + Shr<u32, Output = T>,
{
    let range_ratio = (omax - omin) << precision_bits / (imax - imin);
    let scaled = (v - imin) * range_ratio >> precision_bits;
    scaled + omin
}


/// 3. Saturated Arithmetic (Overflow-Protected)
// ±PROS:
// - Never panics (strictly safe).
// - Good for adversarial inputs.
// ±CONS:
// - Precision loss: Saturates early on large ranges.
fn scale_saturating<T: Saturating>(v: T, imin: T, imax: T, omin: T, omax: T) -> T {
    let scaled = (v - imin).saturating_mul(omax - omin) / (imax - imin);
    scaled.saturating_add(omin)
}

/// 4. . 128-Bit Intermediate (For 64-Bit Values)
// ±PROS:
// - Handles full u64/i64 ranges.
// ±CONS:
// - Slower on non-128-bit platforms (emulated math).
fn scale_128bit<T: Into<u128>>(v: T, imin: T, imax: T, omin: T, omax: T) -> T {
    let scaled = (v - imin) as u128 * (omax - omin) as u128 / (imax - imin) as u128;
    (scaled + omin as u128) as T
}

*/
