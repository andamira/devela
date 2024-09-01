// devela::num::unit::traits
//
//! Unit prefixes traits.
//
// TOC
// - trait Unit
// - impl Unit for UnitBi, UnitSi.

use super::{UnitBi, UnitSi};
#[cfg(feature = "alloc")]
use crate::data::Vec;

/// Unit prefixes.
///
/// - <https://en.wikipedia.org/wiki/Unit_of_measurement>
pub trait Unit: Sized {
    /// Returns the symbol of the prefix.
    #[must_use]
    fn symbol(&self) -> &str;
    /// Returns the ASCII symbol of the prefix.
    #[must_use]
    fn symbol_ascii(&self) -> &str;
    /// Returns the name of the prefix.
    #[must_use]
    fn name(&self) -> &str;

    /// Returns the multiplication factor for the prefix as an f64.
    #[must_use]
    fn factor(&self) -> f64;
    /// Returns the multiplication factor for the prefix as an i64.
    #[must_use]
    fn factor_i64(&self) -> i64;
    /// Returns the multiplication factor for the prefix as an i128.
    #[must_use]
    fn factor_i128(&self) -> i128;

    /// Returns an iterator in ascending order of magnitude.
    fn asc_iter() -> impl Iterator<Item = Self>;
    /// Returns an iterator in descending order of magnitude.
    fn desc_iter() -> impl Iterator<Item = Self>;

    /// The base value for unit prefixes.
    const BASE: Option<i32> = None;
    /// Returns the exponent corresponding to the unit prefix.
    #[must_use]
    fn exp(&self) -> Option<i32> {
        None
    }

    /// Converts a value from one unit prefix variant to another,
    /// returning the converted value.
    #[must_use]
    fn convert(value: f64, from: Self, to: Self) -> f64;
    /// Converts a value from one prefix to another,
    /// returning the converted value and the remainder.
    #[must_use]
    fn convert_i64(value: i64, from: Self, to: Self) -> (i64, i64);
    /// Converts a value from one prefix to another,
    /// returning the converted value and the remainder.
    #[must_use]
    fn convert_i128(value: i128, from: Self, to: Self) -> (i128, i128);

    /// Reduces the given `value` to the most appropriate prefix as a f64,
    /// returning a tuple of the reduced size and the prefix.
    ///
    /// The input `value` is assumed to be non-negative, and in base units,
    /// meaning it has no prefix applied.
    ///
    /// This method simplifies large numerical values by scaling them down
    /// to the largest appropriate prefix (e.g., Kibi, Mebi, Gibi, etc.).
    #[must_use]
    #[cfg(any(feature = "std", feature = "_float_f64"))]
    fn reduce(value: f64) -> (f64, Self);
    /// Reduces the given value to the most appropriate prefix as a i64,
    /// returning a tuple of the reduced size, the prefix, and the remainder.
    ///
    /// The input `value` is assumed to be non-negative, and in base units,
    /// meaning it has no prefix applied.
    ///
    /// This method simplifies large numerical values by scaling them down
    /// to the largest appropriate prefix (e.g., Kibi, Mebi, Gibi, etc.).
    #[must_use]
    fn reduce_i64(value: i64) -> (i64, Self, i64);
    /// Reduces the given value to the most appropriate prefix as a i128,
    /// returning a tuple of the reduced size, the prefix, and the remainder.
    ///
    /// The input `value` is assumed to be non-negative, and in base units,
    /// meaning it has no prefix applied.
    ///
    /// This method simplifies large numerical values by scaling them down
    /// to the largest appropriate prefix (e.g., Kibi, Mebi, Gibi, etc.).
    #[must_use]
    fn reduce_i128(value: i128) -> (i128, Self, i128);

    /// Reduces the given value to a chain of appropriate prefixes as f64,
    /// stopping when the remainder is less than the given threshold.
    #[must_use]
    #[cfg(any(feature = "std", all(feature = "alloc", feature = "_float_f64")))]
    fn reduce_chain(value: f64, threshold: f64) -> Vec<(f64, Self)>;
    /// Reduces the given value to a chain of appropriate prefixes as i64,
    /// stopping when the remainder is less than the given threshold.
    #[must_use]
    #[cfg(feature = "alloc")]
    fn reduce_chain_i64(value: i64, threshold: i64) -> Vec<(i64, Self)>;
    /// Reduces the given value to a chain of appropriate prefixes as i128,
    /// stopping when the remainder is less than the given threshold.
    #[must_use]
    #[cfg(feature = "alloc")]
    fn reduce_chain_i128(value: i128, threshold: i128) -> Vec<(i128, Self)>;
}

// -----------------------------------------------------------------------------

macro_rules! impl_unit {
    ($($t:ty),+) => { $( impl_unit![@$t]; )+ };
    (@$t:ty) => {
        impl Unit for $t {
            fn symbol(&self) -> &str { self.symbol() }
            fn symbol_ascii(&self) -> &str { self.symbol_ascii() }
            fn name(&self) -> &str { self.name() }

            fn factor(&self) -> f64 { self.factor() }
            fn factor_i64(&self) -> i64 { self.factor_i64() }
            fn factor_i128(&self) -> i128 { self.factor_i128() }

            fn asc_iter() -> impl Iterator<Item = Self> { Self::asc_iter() }
            fn desc_iter() -> impl Iterator<Item = Self> { Self::desc_iter() }

            const BASE: Option<i32> = Some(Self::BASE);
            fn exp(&self) -> Option<i32> { Some(self.exp()) }

            fn convert(value: f64, from: Self, to: Self) -> f64 {
                Self::convert(value, from, to)
            }
            fn convert_i64(value: i64, from: Self, to: Self) -> (i64, i64) {
                Self::convert_i64(value, from, to)
            }
            fn convert_i128(value: i128, from: Self, to: Self) -> (i128, i128) {
                Self::convert_i128(value, from, to)
            }

            #[cfg(any(feature = "std", feature = "_float_f64"))]
            fn reduce(value: f64) -> (f64, Self) { Self::reduce(value) }
            fn reduce_i64(value: i64) -> (i64, Self, i64) { Self::reduce_i64(value) }
            fn reduce_i128(value: i128) -> (i128, Self, i128){ Self::reduce_i128(value) }

            #[cfg(any(feature = "std", all(feature = "alloc", feature = "_float_f64")))]
            fn reduce_chain(value: f64, threshold: f64) -> Vec<(f64, Self)> {
                Self::reduce_chain(value, threshold)
            }
            #[cfg(feature = "alloc")]
            fn reduce_chain_i64(value: i64, threshold: i64) -> Vec<(i64, Self)> {
                Self::reduce_chain_i64(value, threshold)
            }
            #[cfg(feature = "alloc")]
            fn reduce_chain_i128(value: i128, threshold: i128) -> Vec<(i128, Self)> {
                Self::reduce_chain_i128(value, threshold)
            }
        }
    };
}
impl_unit![UnitBi, UnitSi];
