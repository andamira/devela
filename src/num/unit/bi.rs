// devela::num::unit::bi
//
//! Binary unit prefixes.
//

use super::helpers::impl_try_from;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use crate::data::{vec_, Vec};
#[cfg(feature = "_float_f64")]
#[allow(unused_imports)]
use crate::num::ExtFloat;

/// Binary prefixes.
///
/// - <https://en.wikipedia.org/wiki/Binary_prefix>
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum UnitBi {
    /// 2^80.
    Yobi = 80,
    /// 2^70.
    Zebi = 70,
    /// 2^60.
    Exbi = 60,
    /// 2^50.
    Pebi = 50,
    /// 2^40.
    Tebi = 40,
    /// 2^30.
    Gibi = 30,
    /// 2^20.
    Mebi = 20,
    /// 2^10.
    Kibi = 10,
    /// 2^0 (no prefix).
    #[default]
    None = 0,
}

/// # Aliases.
#[allow(non_upper_case_globals)]
impl UnitBi {
    /// Alias of `Yobi`.
    pub const Yi: Self = Self::Yobi;
    /// Alias of `Yobi`.
    pub const Y: Self = Self::Yobi;
    /// Alias of `Zebi`.
    pub const Zi: Self = Self::Zebi;
    /// Alias of `Zebi`.
    pub const Z: Self = Self::Zebi;
    /// Alias of `Exbi`.
    pub const Ei: Self = Self::Exbi;
    /// Alias of `Exbi`.
    pub const E: Self = Self::Exbi;
    /// Alias of `Pebi`.
    pub const Pi: Self = Self::Pebi;
    /// Alias of `Pebi`.
    pub const P: Self = Self::Pebi;
    /// Alias of `Tebi`.
    pub const Ti: Self = Self::Tebi;
    /// Alias of `Tebi`.
    pub const T: Self = Self::Tebi;
    /// Alias of `Gibi`.
    pub const Gi: Self = Self::Gibi;
    /// Alias of `Gibi`.
    pub const G: Self = Self::Gibi;
    /// Alias of `Mebi`.
    pub const Mi: Self = Self::Mebi;
    /// Alias of `Mebi`.
    pub const M: Self = Self::Mebi;
    /// Alias of `Kibi`.
    pub const Ki: Self = Self::Kibi;
    /// Alias of `Kibi`.
    pub const k: Self = Self::Kibi;
    /// Alias of `Kibi`.
    pub const K: Self = Self::Kibi;
}

impl UnitBi {
    /// Returns the symbol of the prefix.
    ///
    /// # Example
    /// ```
    /// # use devela::UnitBi;
    /// assert_eq![UnitBi::Gibi.symbol(), "Gi"];
    /// ```
    #[must_use]
    pub const fn symbol(&self) -> &str {
        match self {
            UnitBi::Yobi => "Yi",
            UnitBi::Zebi => "Zi",
            UnitBi::Exbi => "Ei",
            UnitBi::Pebi => "Pi",
            UnitBi::Tebi => "Ti",
            UnitBi::Gibi => "Gi",
            UnitBi::Mebi => "Mi",
            UnitBi::Kibi => "Ki",
            UnitBi::None => "",
        }
    }
    /// Returns the ASCII symbol of the prefix.
    #[inline]
    #[must_use]
    pub const fn symbol_ascii(&self) -> &str {
        self.symbol()
    }

    /// Returns the name of the prefix.
    #[must_use]
    pub const fn name(&self) -> &str {
        match self {
            UnitBi::Yobi => "yobi",
            UnitBi::Zebi => "zebi",
            UnitBi::Exbi => "exbi",
            UnitBi::Pebi => "pebi",
            UnitBi::Tebi => "tibi",
            UnitBi::Gibi => "gibi",
            UnitBi::Mebi => "mibi",
            UnitBi::Kibi => "kibi",
            UnitBi::None => "",
        }
    }

    /// The base value for binary unit prefixes.
    pub const BASE: i32 = 2;

    /// Returns the exponent corresponding to the binary unit prefix.
    ///
    /// For example, `Mebi` corresponds to an exponent of 20, meaning
    /// `Self::BASE^self.exp() = 1_048_576`.
    #[must_use]
    pub const fn exp(&self) -> i32 {
        match self {
            UnitBi::Yobi => 80,
            UnitBi::Zebi => 70,
            UnitBi::Exbi => 60,
            UnitBi::Pebi => 50,
            UnitBi::Tebi => 40,
            UnitBi::Gibi => 30,
            UnitBi::Mebi => 20,
            UnitBi::Kibi => 10,
            UnitBi::None => 0,
        }
    }

    /// Returns the multiplication factor for the binary prefix as an `f64`.
    #[must_use]
    pub const fn factor(&self) -> f64 {
        match self {
            UnitBi::Yobi => 1_208_925_819_614_629_174_706_176.,
            UnitBi::Zebi => 1_180_591_620_717_411_303_424.,
            UnitBi::Exbi => 1_152_921_504_606_846_976.,
            UnitBi::Pebi => 1_125_899_906_842_624.,
            UnitBi::Tebi => 1_099_511_627_776.,
            UnitBi::Gibi => 1_073_741_824.,
            UnitBi::Mebi => 1_048_576.,
            UnitBi::Kibi => 1_024.,
            UnitBi::None => 1.,
        }
    }

    /// Returns the multiplication factor for the binary prefix as an `i64`.
    ///
    /// Only supports the range up to `Exbi`, returning `None` for `Zebi` and `Yobi`.
    #[must_use]
    pub const fn factor_i64_checked(&self) -> Option<i64> {
        match self {
            UnitBi::Exbi => Some(1_152_921_504_606_846_976),
            UnitBi::Pebi => Some(1_125_899_906_842_624),
            UnitBi::Tebi => Some(1_099_511_627_776),
            UnitBi::Gibi => Some(1_073_741_824),
            UnitBi::Mebi => Some(1_048_576),
            UnitBi::Kibi => Some(1_024),
            UnitBi::None => Some(1),
            _ => None,
        }
    }

    /// Returns the multiplication factor for the binary prefix as an `i64`.
    ///
    /// Only supports the range up to `Exbi`, returning 0 for `Zebi` and `Yobi`.
    #[must_use]
    pub const fn factor_i64(&self) -> i64 {
        match self {
            UnitBi::Exbi => 1_152_921_504_606_846_976,
            UnitBi::Pebi => 1_125_899_906_842_624,
            UnitBi::Tebi => 1_099_511_627_776,
            UnitBi::Gibi => 1_073_741_824,
            UnitBi::Mebi => 1_048_576,
            UnitBi::Kibi => 1_024,
            UnitBi::None => 1,
            _ => 0,
        }
    }

    /// Returns the multiplication factor for the binary prefix as an `i128`.
    pub const fn factor_i128(&self) -> i128 {
        match self {
            UnitBi::Yobi => 1_208_925_819_614_629_174_706_176,
            UnitBi::Zebi => 1_180_591_620_717_411_303_424,
            _ => self.factor_i64() as i128,
        }
    }

    /// Converts a value from one binary prefix to another,
    /// returning the converted value.
    #[inline]
    #[must_use]
    pub fn convert(value: f64, from: Self, to: Self) -> f64 {
        if from == to {
            return value;
        }
        let (from_factor, to_factor) = (from.factor(), to.factor());
        value * (from_factor / to_factor)
    }

    /// Converts a value from one binary prefix to another,
    /// returning the converted value and the remainder.
    #[must_use]
    pub fn convert_i64(value: i64, from: Self, to: Self) -> (i64, i64) {
        if from == to {
            return (value, 0);
        }
        let (from_factor, to_factor) = (from.factor_i64(), to.factor_i64());
        let converted = value * from_factor / to_factor;
        let remainder = value * from_factor % to_factor;
        (converted, remainder)
    }

    /// Converts a value from one binary prefix to another,
    /// returning the converted value and the remainder.
    #[must_use]
    pub fn convert_i128(value: i128, from: Self, to: Self) -> (i128, i128) {
        if from == to {
            return (value, 0);
        }
        let (from_factor, to_factor) = (from.factor_i128(), to.factor_i128());
        let converted = value * from_factor / to_factor;
        let remainder = value * from_factor % to_factor;
        (converted, remainder)
    }

    /// Reduces the given `value` to the most appropriate binary prefix as an `f64`,
    /// returning a tuple of the reduced size and the prefix.
    ///
    /// The input `value` is assumed to be non-negative, and in base units,
    /// meaning it has no prefix applied.
    ///
    /// This method simplifies large numerical values by scaling them down
    /// to the largest appropriate binary prefix (e.g., Kibi, Mebi, Gibi, etc.).
    #[must_use]
    #[cfg(any(feature = "std", feature = "_float_f64"))]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(any(feature = "std", feature = "_float_f64"))))]
    pub fn reduce(value: f64) -> (f64, Self) {
        match value.abs() {
            value if value >= UnitBi::Yobi.factor() => {
                (value / UnitBi::Yobi.factor(), UnitBi::Yobi)
            }
            value if value >= UnitBi::Zebi.factor() => {
                (value / UnitBi::Zebi.factor(), UnitBi::Zebi)
            }
            value if value >= UnitBi::Exbi.factor() => {
                (value / UnitBi::Exbi.factor(), UnitBi::Exbi)
            }
            value if value >= UnitBi::Pebi.factor() => {
                (value / UnitBi::Pebi.factor(), UnitBi::Pebi)
            }
            value if value >= UnitBi::Tebi.factor() => {
                (value / UnitBi::Tebi.factor(), UnitBi::Tebi)
            }
            value if value >= UnitBi::Gibi.factor() => {
                (value / UnitBi::Gibi.factor(), UnitBi::Gibi)
            }
            value if value >= UnitBi::Mebi.factor() => {
                (value / UnitBi::Mebi.factor(), UnitBi::Mebi)
            }
            value if value >= UnitBi::Kibi.factor() => {
                (value / UnitBi::Kibi.factor(), UnitBi::Kibi)
            }
            _ => (value, UnitBi::None),
        }
    }

    /// Reduces the given value to the most appropriate binary prefix as an `i64`,
    /// returning a tuple of the reduced size, the prefix, and the remainder.
    ///
    /// The input `value` is assumed to be non-negative, and in base units,
    /// meaning it has no prefix applied.
    ///
    /// This method simplifies large numerical values by scaling them down
    /// to the largest appropriate binary prefix (e.g., Kibi, Mebi, Gibi, etc.).
    #[must_use]
    pub const fn reduce_i64(value: i64) -> (i64, Self, i64) {
        match value {
            value if value >= UnitBi::Exbi.factor_i64() => {
                (value / UnitBi::Exbi.factor_i64(), UnitBi::Exbi, value % UnitBi::Exbi.factor_i64())
            }
            value if value >= UnitBi::Pebi.factor_i64() => {
                (value / UnitBi::Pebi.factor_i64(), UnitBi::Pebi, value % UnitBi::Pebi.factor_i64())
            }
            value if value >= UnitBi::Tebi.factor_i64() => {
                (value / UnitBi::Tebi.factor_i64(), UnitBi::Tebi, value % UnitBi::Tebi.factor_i64())
            }
            value if value >= UnitBi::Gibi.factor_i64() => {
                (value / UnitBi::Gibi.factor_i64(), UnitBi::Gibi, value % UnitBi::Gibi.factor_i64())
            }
            value if value >= UnitBi::Mebi.factor_i64() => {
                (value / UnitBi::Mebi.factor_i64(), UnitBi::Mebi, value % UnitBi::Mebi.factor_i64())
            }
            value if value >= UnitBi::Kibi.factor_i64() => {
                (value / UnitBi::Kibi.factor_i64(), UnitBi::Kibi, value % UnitBi::Kibi.factor_i64())
            }
            _ => (value, UnitBi::None, 0),
        }
    }

    /// Reduces the given value to the most appropriate binary prefix as an `i128`,
    /// returning a tuple of the reduced size, the prefix, and the remainder.
    ///
    /// The input `value` is assumed to be non-negative, and in base units,
    /// meaning it has no prefix applied.
    ///
    /// This method simplifies large numerical values by scaling them down
    /// to the largest appropriate binary prefix (e.g., Kibi, Mebi, Gibi, etc.).
    #[must_use]
    pub const fn reduce_i128(value: i128) -> (i128, Self, i128) {
        match value {
            value if value >= UnitBi::Yobi.factor_i128() => (
                value / UnitBi::Yobi.factor_i128(),
                UnitBi::Yobi,
                value % UnitBi::Yobi.factor_i128(),
            ),
            value if value >= UnitBi::Zebi.factor_i128() => (
                value / UnitBi::Zebi.factor_i128(),
                UnitBi::Zebi,
                value % UnitBi::Zebi.factor_i128(),
            ),
            value if value >= UnitBi::Exbi.factor_i128() => (
                value / UnitBi::Exbi.factor_i128(),
                UnitBi::Exbi,
                value % UnitBi::Exbi.factor_i128(),
            ),
            _ => {
                let (v, p, r) = Self::reduce_i64(value as i64);
                (v as i128, p, r as i128)
            }
        }
    }

    /// Reduces the given value to a chain of appropriate binary prefixes as an `f64`,
    /// stopping when the remainder is less than the given threshold.
    #[must_use]
    #[cfg(any(feature = "std", all(feature = "alloc", feature = "_float_f64")))]
    #[cfg_attr(
        feature = "nightly_doc",
        doc(cfg(any(feature = "std", all(feature = "alloc", feature = "_float_f64"))))
    )]
    pub fn reduce_chain(value: f64, threshold: f64) -> Vec<(f64, Self)> {
        if value == 0.0 {
            return vec_![(0.0, UnitBi::None)];
        }

        let mut result = Vec::new();
        let mut remainder = value;

        // Ensure the threshold is positive and above a small epsilon value
        // in order to prevent infinite loops
        let effective_threshold =
            if threshold <= 0.0 { crate::ExtFloatConst::MEDIUM_MARGIN } else { threshold };

        while remainder.abs() > effective_threshold {
            let (size, unit) = Self::reduce(remainder);
            let integer_part = size.trunc();
            let fractional_part = size - integer_part;
            result.push((integer_part, unit));
            remainder = fractional_part * unit.factor();

            if remainder.abs() < effective_threshold {
                break;
            }
        }
        if remainder.abs() >= effective_threshold {
            result.push((remainder, UnitBi::None));
        }
        result
    }

    /// Reduces the given value to a chain of appropriate binary prefixes as an `i64`,
    /// stopping when the remainder is less than the given threshold.
    #[must_use]
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
    pub fn reduce_chain_i64(value: i64, threshold: i64) -> Vec<(i64, Self)> {
        let mut result = Vec::new();
        let mut remainder = value;

        while remainder > threshold {
            let (size, unit, new_remainder) = Self::reduce_i64(remainder);
            result.push((size, unit));
            remainder = new_remainder;

            if remainder < threshold {
                break;
            }
        }
        if remainder >= threshold {
            result.push((remainder, UnitBi::None));
        }
        result
    }

    /// Reduces the given value to a chain of appropriate binary prefixes as an `i128`,
    /// stopping when the remainder is less than the given threshold.
    #[must_use]
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
    pub fn reduce_chain_i128(value: i128, threshold: i128) -> Vec<(i128, Self)> {
        let mut result = Vec::new();
        let mut remainder = value;

        while remainder > threshold {
            let (size, unit, new_remainder) = Self::reduce_i128(remainder);
            result.push((size, unit));
            remainder = new_remainder;

            if remainder < threshold {
                break;
            }
        }
        if remainder > threshold {
            result.push((remainder, UnitBi::None));
        }
        result
    }
}

impl UnitBi {
    /// Returns an iterator in ascending order of magnitude.
    pub fn asc_iter() -> impl Iterator<Item = Self> {
        const UNITS: [UnitBi; 9] = [
            UnitBi::None,
            UnitBi::Kibi,
            UnitBi::Mebi,
            UnitBi::Gibi,
            UnitBi::Tebi,
            UnitBi::Pebi,
            UnitBi::Exbi,
            UnitBi::Zebi,
            UnitBi::Yobi,
        ];
        UNITS.iter().copied()
    }

    /// Returns an iterator in descending order of magnitude.
    pub fn desc_iter() -> impl Iterator<Item = Self> {
        const UNITS: [UnitBi; 9] = [
            UnitBi::Yobi,
            UnitBi::Zebi,
            UnitBi::Exbi,
            UnitBi::Pebi,
            UnitBi::Tebi,
            UnitBi::Gibi,
            UnitBi::Mebi,
            UnitBi::Kibi,
            UnitBi::None,
        ];
        UNITS.iter().copied()
    }
}

impl From<UnitBi> for f32 {
    #[inline]
    fn from(from: UnitBi) -> f32 {
        from.factor() as f32
    }
}
impl From<UnitBi> for f64 {
    #[inline]
    fn from(from: UnitBi) -> f64 {
        from.factor()
    }
}
impl From<UnitBi> for i64 {
    #[inline]
    fn from(from: UnitBi) -> i64 {
        from.factor_i64()
    }
}
impl From<UnitBi> for i128 {
    #[inline]
    fn from(from: UnitBi) -> i128 {
        from.factor_i128()
    }
}
impl_try_from![UnitBi, i64 => i32, i16, u64, u32, u16];
impl_try_from![UnitBi, i128 => u128];

#[cfg(test)]
mod tests {
    use super::{
        UnitBi,
        UnitBi::{Exbi, Gibi, Kibi, Mebi, Yobi, Zebi},
    };
    #[cfg(any(feature = "std", all(feature = "alloc", feature = "_float_f64")))]
    use crate::data::vec_;
    #[allow(unused_imports)]
    use crate::num::ExtFloatConst;

    /* reduce */

    #[test]
    fn unit_bi_reduce_i64() {
        let value = i64::from(Exbi);
        let (reduced_value, unit, remainder) = UnitBi::reduce_i64(value);
        assert_eq!(reduced_value, 1);
        assert_eq!(unit, Exbi);
        assert_eq!(remainder, 0);

        let value = 2_000_000_000; // Between Gibi and Tebi
        let (reduced_value, unit, remainder) = UnitBi::reduce_i64(value);
        assert_eq!(reduced_value, 1);
        assert_eq!(unit, Gibi);
        assert_eq!(remainder, 926_258_176); // 2_000_000_000 - 1_073_741_824

        let value = 2 * i64::from(Kibi) + 512;
        let (reduced_value, unit, remainder) = UnitBi::reduce_i64(value);
        assert_eq!(reduced_value, 2);
        assert_eq!(unit, Kibi);
        assert_eq!(remainder, 512);
    }

    #[test]
    fn unit_bi_reduce_i128() {
        let value = Yobi.factor_i128();
        let (reduced_value, unit, remainder) = UnitBi::reduce_i128(value);
        assert_eq!(reduced_value, 1);
        assert_eq!(unit, Yobi);
        assert_eq!(remainder, 0);

        let value = i128::from(Zebi) + i128::from(Mebi);
        let (reduced_value, unit, remainder) = UnitBi::reduce_i128(value);
        assert_eq!(reduced_value, 1);
        assert_eq!(unit, Zebi);
        assert_eq!(remainder, Mebi.factor_i128());
    }

    /* reduce_chain */

    #[test]
    #[cfg(any(feature = "std", all(feature = "alloc", feature = "_float_f64")))]
    #[cfg_attr(
        feature = "nightly_doc",
        doc(cfg(any(feature = "std", all(feature = "alloc", feature = "_float_f64"))))
    )]
    fn unit_bi_reduce_chain() {
        let margin = f64::MEDIUM_MARGIN;

        assert_eq![
            // single unit: 1Gi
            UnitBi::reduce_chain(Gibi.factor(), margin),
            vec_![(1.0, Gibi)]
        ];
        assert_eq![
            // multiple unit: 1.5Gi
            UnitBi::reduce_chain(1.5 * Gibi.factor(), margin),
            vec_![(1.0, Gibi), (512.0, Mebi)]
        ];
        assert_eq![
            // 1Gi + 1Ki
            UnitBi::reduce_chain(Gibi.factor() + Kibi.factor(), margin),
            vec_![(1., Gibi), (1., Kibi)]
        ];
        assert_eq![
            // Small value (only 512Ki)
            UnitBi::reduce_chain(Mebi.factor() / 2., margin),
            vec_![(512., Kibi)]
        ];
        assert_eq![
            // Very large value (3Yi + 2Zi + 1Gi)
            UnitBi::reduce_chain(3. * Yobi.factor() + 2. * Zebi.factor() + Gibi.factor(), margin),
            vec_![(3.0, Yobi), (2.0, Zebi), (1.0, Gibi)]
        ];
        assert_eq![
            // Zero value
            UnitBi::reduce_chain(0.0, margin),
            vec_![(0.0, UnitBi::None)]
        ];
        assert_eq![
            // Fractional value (0.5 Gi)
            UnitBi::reduce_chain(Gibi.factor() / 2., margin),
            vec_![(512., Mebi)]
        ];
    }
}
