// /devela/src/num/quant/scale/namespace.rs
//
//! Defines [`Scale`]
//

use crate::{
    IntError::{NonZeroRequired, Overflow},
    IntResult as Result,
    Sign::{Negative, Positive},
    cold_path, isize_up, unwrap, usize_up,
};

#[doc = crate::_tags!(quant primitive namespace)]
/// Provides const proportional scaling for primitive numbers.
#[doc = crate::_doc_meta! { location("num/quant") }]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Scale<T>(pub T);

// $T:   the primitive type
// $UP:  the upscaled type
macro_rules! _scale_impl_prims {
    () => {
        _scale_impl_prims![sint: i8:i16, i16:i32, i32:i64, i64:i128, isize:isize_up, i128:i128];
        _scale_impl_prims![uint: u8:u16, u16:u32, u32:u64, u64:u128, usize:usize_up, u128:u128];
        _scale_impl_prims![float: f32, f64];
    };

    /* floating point FUTURE API */

    (float: $($T:ty),+ $(,)?) => { $( _scale_impl_prims!(@float: $T); )+ };
    (@float: $T:ty) => {
        // impl Scale<$T> { }
    };

    /* shared integer API */

    (int: $( $T:ty : $UP:ty ),+ $(,)?) => { $( _scale_impl_prims!(@int: $T : $UP); )+ };
    (@int: $T:ty : $UP:ty) => {

        /// Computes `self × num / den`, rounding toward zero.
        ///
        /// # Errors
        /// Returns `NonZeroRequired` if `den == 0`.
        ///
        /// Returns `Overflow` if the intermediate product
        /// or final result is not representable.
        pub const fn mul_div_trunc(self, num: $T, den: $T) -> Result<$T> {
            if den == 0 { cold_path(); return Err(NonZeroRequired); }
            let product = unwrap![ok? Self::_mul_up(self.0, num)];
            let result = unwrap![ok? Self::_div_trunc_up(product, den as $UP)];
            Self::_up_to_prim(result)
        }
        /// Computes `self × num / den`, rounding toward negative infinity.
        ///
        /// # Errors
        /// Returns `NonZeroRequired` if `den == 0`.
        ///
        /// Returns `Overflow` if the intermediate product
        /// or final result is not representable.
        pub const fn mul_div_floor(self, num: $T, den: $T) -> Result<$T> {
            if den == 0 { cold_path(); return Err(NonZeroRequired); }
            let product = unwrap![ok? Self::_mul_up(self.0, num)];
            let result = unwrap![ok? Self::_div_floor_up(product, den as $UP)];
            Self::_up_to_prim(result)
        }

        /// Computes `self × num / den`, rounding toward positive infinity.
        ///
        /// # Errors
        /// Returns `NonZeroRequired` if `den == 0`.
        ///
        /// Returns `Overflow` if the intermediate product
        /// or final result is not representable.
        pub const fn mul_div_ceil(self, num: $T, den: $T) -> Result<$T> {
            if den == 0 { cold_path(); return Err(NonZeroRequired); }
            let product = unwrap![ok? Self::_mul_up(self.0, num)];
            let result = unwrap![ok? Self::_div_ceil_up(product, den as $UP)];
            Self::_up_to_prim(result)
        }

        /// Computes `self × num / den`, rounding to nearest.
        ///
        /// Halfway cases are rounded away from zero.
        ///
        /// # Errors
        /// Returns `NonZeroRequired` if `den == 0`.
        ///
        /// Returns `Overflow` if the intermediate product or final result
        /// is not representable.
        pub const fn mul_div_round(self, num: $T, den: $T) -> Result<$T> {
            if den == 0 { cold_path(); return Err(NonZeroRequired); }
            let product = unwrap![ok? Self::_mul_up(self.0, num)];
            let result = unwrap![ok? Self::_div_round_away_up(product, den as $UP)];
            Self::_up_to_prim(result)
        }
    };

    /* signed integers */

    (sint: $( $T:ty : $UP:ty ),+ $(,)?) => { $( _scale_impl_prims!(@sint: $T:$UP); )+ };
    (@sint: $T:ty : $UP:ty ) => {
        impl Scale<$T> {
            /// Multiplies two carrier values in the upscaled type.
            const fn _mul_up(a: $T, b: $T) -> Result<$UP> {
                let (a, b) = (a as $UP, b as $UP);
                let Some(product) = a.checked_mul(b) else {
                    cold_path();
                    let sign = if (a < 0) == (b < 0) { Positive } else { Negative };
                    return Err(Overflow(Some(sign)));
                };
                Ok(product)
            }
            /// Downcasts an upscaled result into the carrier.
            const fn _up_to_prim(value: $UP) -> Result<$T> {
                if value < <$T>::MIN as $UP { cold_path(); Err(Overflow(Some(Negative))) }
                else if value > <$T>::MAX as $UP { cold_path(); Err(Overflow(Some(Positive))) }
                else { Ok(value as $T) }
            }
            /// Divides, rounding toward zero.
            const fn _div_trunc_up(a: $UP, b: $UP) -> Result<$UP> {
                if a == <$UP>::MIN && b == -1 { cold_path(); Err(Overflow(Some(Positive))) }
                else { Ok(a / b) }
            }
            /// Divides, rounding toward negative infinity.
            const fn _div_floor_up(a: $UP, b: $UP) -> Result<$UP> {
                if a == <$UP>::MIN && b == -1 { cold_path(); return Err(Overflow(Some(Positive))); }
                let (q, r) = (a / b, a % b);
                if r != 0 && ((r > 0) != (b > 0)) {
                    let Some(q) = q.checked_sub(1) else {
                        cold_path();
                        return Err(Overflow(Some(Negative)));
                    };
                    Ok(q)
                } else {
                    Ok(q)
                }
            }
            /// Divides, rounding toward positive infinity.
            const fn _div_ceil_up(a: $UP, b: $UP) -> Result<$UP> {
                if a == <$UP>::MIN && b == -1 { cold_path(); return Err(Overflow(Some(Positive))); }
                let (q, r) = (a / b, a % b);
                if r != 0 && ((r > 0) == (b > 0)) {
                    let Some(q) = q.checked_add(1) else {
                        cold_path();
                        return Err(Overflow(Some(Positive)));
                    };
                    Ok(q)
                } else {
                    Ok(q)
                }
            }
            /// Divides to nearest, with ties away from zero.
            const fn _div_round_away_up(a: $UP, b: $UP) -> Result<$UP> {
                if a == <$UP>::MIN && b == -1 { cold_path(); return Err(Overflow(Some(Positive))); }
                let (q, r) = (a / b, a % b);
                if r == 0 { return Ok(q); }
                // `unsigned_abs` also handles MIN correctly:
                let (r_abs, b_abs) = (r.unsigned_abs(), b.unsigned_abs());
                let half = b_abs / 2;
                let above_half = r_abs > half;
                let at_half = b_abs % 2 == 0 && r_abs == half;
                if !above_half && !at_half { return Ok(q); }
                if (a < 0) == (b < 0) {
                    let Some(q) = q.checked_add(1) else {
                        cold_path();
                        return Err(Overflow(Some(Positive)));
                    };
                    Ok(q)
                } else {
                    let Some(q) = q.checked_sub(1) else {
                        cold_path();
                        return Err(Overflow(Some(Negative)));
                    };
                    Ok(q)
                }
            }

            _scale_impl_prims! {@int: $T:$UP }
        }
    };

    /* unsigned integers */

    (uint: $( $T:ty : $UP:ty ),+ $(,)?) => { $( _scale_impl_prims!(@uint: $T : $UP); )+ };
    (@uint: $T:ty : $UP:ty) => {
        impl Scale<$T> {
            /// Multiplies two carrier values in the upscaled type.
            const fn _mul_up(a: $T, b: $T) -> Result<$UP> {
                let Some(product) = (a as $UP).checked_mul(b as $UP) else {
                    cold_path();
                    return Err(Overflow(Some(Positive)));
                };
                Ok(product)
            }
            /// Downcasts an upscaled result into the carrier.
            const fn _up_to_prim(value: $UP) -> Result<$T> {
                if value > <$T>::MAX as $UP { cold_path(); Err(Overflow(Some(Positive))) }
                else { Ok(value as $T) }
            }
            /// Divides, rounding toward zero.
            const fn _div_trunc_up(a: $UP, b: $UP) -> Result<$UP> {
                Ok(a / b)
            }
            /// Divides, rounding toward negative infinity.
            ///
            /// For unsigned integers this is equivalent to truncation.
            const fn _div_floor_up(a: $UP, b: $UP) -> Result<$UP> {
                Ok(a / b)
            }
            /// Divides, rounding toward positive infinity.
            const fn _div_ceil_up(a: $UP, b: $UP) -> Result<$UP> {
                let (q, r) = (a / b, a % b);
                if r == 0 {
                    Ok(q)
                } else {
                    let Some(q) = q.checked_add(1) else {
                        cold_path();
                        return Err(Overflow(Some(Positive)));
                    };
                    Ok(q)
                }
            }
            /// Divides to nearest, with ties away from zero.
            const fn _div_round_away_up(a: $UP, b: $UP) -> Result<$UP> {
                let (q, r) = (a / b, a % b);
                let half = b / 2;
                let above_half = r > half;
                let at_half = b % 2 == 0 && r == half;
                if !above_half && !at_half { return Ok(q); }
                let Some(q) = q.checked_add(1) else {
                    cold_path();
                    return Err(Overflow(Some(Positive)));
                };
                Ok(q)
            }

            _scale_impl_prims! {@int: $T:$UP }
        }
    };
}
_scale_impl_prims!();
