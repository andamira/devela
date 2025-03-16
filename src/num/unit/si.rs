// devela::num::unit::si
//
//! SI unit prefixes.
//

use super::helpers::impl_try_from;
#[allow(unused_imports)]
#[cfg(feature = "_float_f64")]
use crate::ExtFloat;
#[allow(unused_imports)]
#[cfg(feature = "alloc")]
use crate::{vec_ as vec, Vec};

/// SI (metric) unit prefixes.
///
/// - <https://en.wikipedia.org/wiki/Metric_prefix>
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum UnitSi {
    /// 10^30
    Quetta,
    /// 10^27
    Ronna,
    /// 10^24
    Yotta,
    /// 10^21
    Zetta,
    /// 10^18
    Exa,
    /// 10^15
    Peta,
    /// 10^12
    Tera,
    /// 10^9
    Giga,
    /// 10^6
    Mega,
    /// 10^3
    Kilo,
    /// 10^2
    Hecto,
    /// 10^1
    Deca,

    /// 10^0 (no prefix)
    #[default]
    None,

    /// 10^-1
    Deci,
    /// 10^-2
    Centi,
    /// 10^-3
    Milli,
    /// 10^-6
    Micro,
    /// 10^-9
    Nano,
    /// 10^-12
    Pico,
    /// 10^-15
    Femto,
    /// 10^-18
    Atto,
    /// 10^-21
    Zepto,
    /// 10^-24
    Yocto,
    /// 10^-27
    Ronto,
    /// 10^-30
    Quecto,
}

/// # Aliases.
#[allow(non_upper_case_globals)]
impl UnitSi {
    /// Alias of `Quetta`.
    pub const Q: Self = Self::Quetta;
    /// Alias of `Ronna`.
    pub const R: Self = Self::Ronna;
    /// Alias of `Yotta`.
    pub const Y: Self = Self::Yotta;
    /// Alias of `Zetta`.
    pub const Z: Self = Self::Zetta;
    /// Alias of `Exa`.
    pub const E: Self = Self::Exa;
    /// Alias of `Peta`.
    pub const P: Self = Self::Peta;
    /// Alias of `Tera`.
    pub const T: Self = Self::Tera;
    /// Alias of `Giga`.
    pub const G: Self = Self::Giga;
    /// Alias of `Mega`.
    pub const M: Self = Self::Mega;
    /// Alias of `Kilo`.
    pub const k: Self = Self::Kilo;
    /// Alias of `Kilo` (alternative).
    pub const K: Self = Self::Kilo;
    /// Alias of `Hecto`.
    pub const h: Self = Self::Hecto;
    /// Alias of `Hecto` (alternative).
    pub const H: Self = Self::Hecto;
    /// Alias of `Deca`.
    pub const da: Self = Self::Deca;
    /// Alias of `Deca` (alternative).
    pub const D: Self = Self::Deca;
    //
    /// Alias of `Deci`.
    pub const d: Self = Self::Deci;
    /// Alias of `Centi`.
    pub const c: Self = Self::Centi;
    /// Alias of `Milli`.
    pub const m: Self = Self::Milli;
    /// Alias of `Micro` (alternative to µ).
    pub const u: Self = Self::Micro;
    /// Alias of `Nano`.
    pub const n: Self = Self::Nano;
    /// Alias of `Pico`.
    pub const p: Self = Self::Pico;
    /// Alias of `Femto`.
    pub const f: Self = Self::Femto;
    /// Alias of `Atto`.
    pub const a: Self = Self::Atto;
    /// Alias of `Zepto`.
    pub const z: Self = Self::Zepto;
    /// Alias of `Yocto`.
    pub const y: Self = Self::Yocto;
    /// Alias of `Ronto`.
    pub const r: Self = Self::Ronto;
    /// Alias of `Quecto`.
    pub const q: Self = Self::Quecto;
}

impl UnitSi {
    /// Returns the symbol of the prefix.
    ///
    /// # Example
    /// ```
    /// # use devela::UnitSi;
    /// assert_eq![UnitSi::Giga.symbol(), "G"];
    /// assert_eq![UnitSi::Micro.symbol(), "µ"];
    /// ```
    #[must_use]
    pub const fn symbol(&self) -> &str {
        match self {
            UnitSi::Quetta => "Q",
            UnitSi::Ronna => "R",
            UnitSi::Yotta => "Y",
            UnitSi::Zetta => "Z",
            UnitSi::Exa => "E",
            UnitSi::Peta => "P",
            UnitSi::Tera => "T",
            UnitSi::Giga => "G",
            UnitSi::Mega => "M",
            UnitSi::Kilo => "k",
            UnitSi::Hecto => "H",
            UnitSi::Deca => "D",
            UnitSi::None => "",
            UnitSi::Deci => "d",
            UnitSi::Centi => "c",
            UnitSi::Milli => "m",
            UnitSi::Micro => "µ",
            UnitSi::Nano => "n",
            UnitSi::Pico => "p",
            UnitSi::Femto => "f",
            UnitSi::Atto => "a",
            UnitSi::Zepto => "z",
            UnitSi::Yocto => "y",
            UnitSi::Ronto => "r",
            UnitSi::Quecto => "q",
        }
    }
    /// Returns the ASCII symbol of the prefix.
    #[must_use]
    pub const fn symbol_ascii(&self) -> &str {
        match self {
            UnitSi::Micro => "u",
            _ => self.symbol(),
        }
    }

    /// Returns the name of the prefix.
    #[must_use]
    pub const fn name(&self) -> &str {
        match self {
            UnitSi::Quetta => "quetta",
            UnitSi::Ronna => "ronna",
            UnitSi::Yotta => "yotta",
            UnitSi::Zetta => "zetta",
            UnitSi::Exa => "exa",
            UnitSi::Peta => "peta",
            UnitSi::Tera => "tera",
            UnitSi::Giga => "giga",
            UnitSi::Mega => "mega",
            UnitSi::Kilo => "kilo",
            UnitSi::Hecto => "hecto",
            UnitSi::Deca => "deca",
            UnitSi::None => "",
            UnitSi::Deci => "deci",
            UnitSi::Centi => "centi",
            UnitSi::Milli => "milli",
            UnitSi::Micro => "micro",
            UnitSi::Nano => "nano",
            UnitSi::Pico => "pico",
            UnitSi::Femto => "femto",
            UnitSi::Atto => "atoo",
            UnitSi::Zepto => "zepto",
            UnitSi::Yocto => "yocto",
            UnitSi::Ronto => "ronto",
            UnitSi::Quecto => "quecto",
        }
    }

    /// The base value for SI unit prefixes.
    pub const BASE: i32 = 10;

    /// Returns the exponent associated with the SI unit prefix.
    pub const fn exp(&self) -> i32 {
        match self {
            UnitSi::Quetta => 30,
            UnitSi::Ronna => 27,
            UnitSi::Yotta => 24,
            UnitSi::Zetta => 21,
            UnitSi::Exa => 18,
            UnitSi::Peta => 15,
            UnitSi::Tera => 12,
            UnitSi::Giga => 9,
            UnitSi::Mega => 6,
            UnitSi::Kilo => 3,
            UnitSi::Hecto => 2,
            UnitSi::Deca => 1,
            UnitSi::None => 0,
            UnitSi::Deci => -1,
            UnitSi::Centi => -2,
            UnitSi::Milli => -3,
            UnitSi::Micro => -6,
            UnitSi::Nano => -9,
            UnitSi::Pico => -12,
            UnitSi::Femto => -15,
            UnitSi::Atto => -18,
            UnitSi::Zepto => -21,
            UnitSi::Yocto => -24,
            UnitSi::Ronto => -27,
            UnitSi::Quecto => -30,
        }
    }

    /// Returns the multiplication factor for the SI prefix as an `f64`.
    pub const fn factor(&self) -> f64 {
        match self {
            UnitSi::Quetta => 1e30,
            UnitSi::Ronna => 1e27,
            UnitSi::Yotta => 1e24,
            UnitSi::Zetta => 1e21,
            UnitSi::Exa => 1e18,
            UnitSi::Peta => 1e15,
            UnitSi::Tera => 1e12,
            UnitSi::Giga => 1e9,
            UnitSi::Kilo => 1e3,
            UnitSi::Mega => 1e6,
            UnitSi::Hecto => 1e2,
            UnitSi::Deca => 1e1,
            UnitSi::None => 1e0,
            UnitSi::Deci => 1e-1,
            UnitSi::Centi => 1e-2,
            UnitSi::Milli => 1e-3,
            UnitSi::Micro => 1e-6,
            UnitSi::Nano => 1e-9,
            UnitSi::Pico => 1e-12,
            UnitSi::Femto => 1e-15,
            UnitSi::Atto => 1e-18,
            UnitSi::Zepto => 1e-21,
            UnitSi::Yocto => 1e-24,
            UnitSi::Ronto => 1e-27,
            UnitSi::Quecto => 1e-30,
        }
    }

    /// Returns the multiplication factor for the SI prefix as an `i64`.
    ///
    /// Negative values represent reciprocal factors, indicating
    /// that the unit corresponds to a fractional multiplier
    /// (e.g., -1000 represents a factor of 1/1000).
    ///
    /// This method only supports the range from `Exa` to `Atto`,
    /// returning 0 for `>= Zetta` and for `<= Zepto`.
    pub const fn factor_i64(&self) -> i64 {
        match self {
            UnitSi::Exa => 1_000_000_000_000_000_000,
            UnitSi::Peta => 1_000_000_000_000_000,
            UnitSi::Tera => 1_000_000_000_000,
            UnitSi::Giga => 1_000_000_000,
            UnitSi::Mega => 1_000_000,
            UnitSi::Kilo => 1_000,
            UnitSi::Hecto => 100,
            UnitSi::Deca => 10,
            UnitSi::None => 1,
            UnitSi::Deci => -10,
            UnitSi::Centi => -100,
            UnitSi::Milli => -1_000,
            UnitSi::Micro => -1_000_000,
            UnitSi::Nano => -1_000_000_000,
            UnitSi::Pico => -1_000_000_000_000,
            UnitSi::Femto => -1_000_000_000_000_000,
            UnitSi::Atto => -1_000_000_000_000_000_000,
            _ => todo![],
        }
    }

    /// Returns the multiplication factor for the SI prefix as an `i128`.
    pub const fn factor_i128(&self) -> i128 {
        match self {
            UnitSi::Quetta => 1_000_000_000_000_000_000_000_000_000_000,
            UnitSi::Ronna => 1_000_000_000_000_000_000_000_000_000,
            UnitSi::Yotta => 1_000_000_000_000_000_000_000_000,
            UnitSi::Zetta => 1_000_000_000_000_000_000_000,
            UnitSi::Zepto => -1_000_000_000_000_000_000_000,
            UnitSi::Yocto => -1_000_000_000_000_000_000_000_000,
            UnitSi::Ronto => -1_000_000_000_000_000_000_000_000_000,
            UnitSi::Quecto => -1_000_000_000_000_000_000_000_000_000_000,
            _ => self.factor_i64() as i128,
        }
    }

    /// Converts a value from one SI prefix to another,
    /// returning the converted value.
    #[must_use]
    pub fn convert(value: f64, from: Self, to: Self) -> f64 {
        if from == to {
            return value;
        }
        let (from_factor, to_factor) = (from.factor(), to.factor());
        value * (from_factor / to_factor)
    }

    /// Converts a value from one SI prefix to another,
    /// returning the converted value and the remainder.
    #[must_use]
    pub fn convert_i64(value: i64, from: Self, to: Self) -> (i64, i64) {
        if from == to {
            return (value, 0);
        }
        let (from_factor, to_factor) = (from.factor_i64(), to.factor_i64());

        // Determine the absolute conversion and handle positive/negative factors
        let converted = if from_factor > 0 && to_factor > 0 {
            value * from_factor / to_factor
        } else if from_factor > 0 && to_factor < 0 {
            value * from_factor * to_factor.abs()
        } else if from_factor < 0 && to_factor > 0 {
            value * from_factor.abs() / to_factor
        } else {
            value * from_factor.abs() * to_factor.abs()
        };

        let remainder = if from_factor > 0 && to_factor > 0 {
            value * from_factor % to_factor
        } else if from_factor > 0 && to_factor < 0 {
            value * from_factor
        } else if from_factor < 0 && to_factor > 0 {
            value % to_factor
        } else {
            0 // No remainder calculation needed for reciprocal to reciprocal conversion
        };

        (converted, remainder)
    }

    /// Converts a value from one SI prefix to another,
    /// returning the converted value and the remainder.
    #[must_use]
    pub fn convert_i128(value: i128, from: Self, to: Self) -> (i128, i128) {
        if from == to {
            return (value, 0);
        }
        let (from_factor, to_factor) = (from.factor_i128(), to.factor_i128());

        // Determine the absolute conversion and handle positive/negative factors
        let converted = if from_factor > 0 && to_factor > 0 {
            value * from_factor / to_factor
        } else if from_factor > 0 && to_factor < 0 {
            value * from_factor * to_factor.abs()
        } else if from_factor < 0 && to_factor > 0 {
            value * from_factor.abs() / to_factor
        } else {
            value * from_factor.abs() * to_factor.abs()
        };

        let remainder = if from_factor > 0 && to_factor > 0 {
            value * from_factor % to_factor
        } else if from_factor > 0 && to_factor < 0 {
            value * from_factor
        } else if from_factor < 0 && to_factor > 0 {
            value % to_factor
        } else {
            0 // No remainder calculation needed for reciprocal to reciprocal conversion
        };

        (converted, remainder)
    }

    /// Reduces the given value to the most appropriate SI prefix as an `f64`,
    /// returning a tuple of the reduced size and the prefix.
    #[cfg(any(feature = "std", feature = "_float_f64"))]
    #[cfg_attr(nightly_doc, doc(cfg(any(feature = "std", feature = "_float_f64"))))]
    pub fn reduce(value: f64) -> (f64, Self) {
        match value.abs() {
            value if value >= UnitSi::Quetta.factor() => {
                (value / UnitSi::Quetta.factor(), UnitSi::Quetta)
            }
            value if value >= UnitSi::Ronna.factor() => {
                (value / UnitSi::Ronna.factor(), UnitSi::Ronna)
            }
            value if value >= UnitSi::Yotta.factor() => {
                (value / UnitSi::Yotta.factor(), UnitSi::Yotta)
            }
            value if value >= UnitSi::Zetta.factor() => {
                (value / UnitSi::Zetta.factor(), UnitSi::Zetta)
            }
            value if value >= UnitSi::Exa.factor() => (value / UnitSi::Exa.factor(), UnitSi::Exa),
            value if value >= UnitSi::Peta.factor() => {
                (value / UnitSi::Peta.factor(), UnitSi::Peta)
            }
            value if value >= UnitSi::Tera.factor() => {
                (value / UnitSi::Tera.factor(), UnitSi::Tera)
            }
            value if value >= UnitSi::Giga.factor() => {
                (value / UnitSi::Giga.factor(), UnitSi::Giga)
            }
            value if value >= UnitSi::Mega.factor() => {
                (value / UnitSi::Mega.factor(), UnitSi::Mega)
            }
            value if value >= UnitSi::Kilo.factor() => {
                (value / UnitSi::Kilo.factor(), UnitSi::Kilo)
            }
            value if value >= UnitSi::Hecto.factor() => {
                (value / UnitSi::Hecto.factor(), UnitSi::Hecto)
            }
            value if value >= UnitSi::Deca.factor() => {
                (value / UnitSi::Deca.factor(), UnitSi::Deca)
            }
            //
            value if value >= UnitSi::Deci.factor() => {
                (value / UnitSi::Deci.factor(), UnitSi::Deci)
            }
            value if value >= UnitSi::Centi.factor() => {
                (value / UnitSi::Centi.factor(), UnitSi::Centi)
            }
            value if value >= UnitSi::Milli.factor() => {
                (value / UnitSi::Milli.factor(), UnitSi::Milli)
            }
            value if value >= UnitSi::Micro.factor() => {
                (value / UnitSi::Micro.factor(), UnitSi::Micro)
            }
            value if value >= UnitSi::Nano.factor() => {
                (value / UnitSi::Nano.factor(), UnitSi::Nano)
            }
            value if value >= UnitSi::Pico.factor() => {
                (value / UnitSi::Pico.factor(), UnitSi::Pico)
            }
            value if value >= UnitSi::Femto.factor() => {
                (value / UnitSi::Femto.factor(), UnitSi::Femto)
            }
            value if value >= UnitSi::Atto.factor() => {
                (value / UnitSi::Atto.factor(), UnitSi::Atto)
            }
            value if value >= UnitSi::Zepto.factor() => {
                (value / UnitSi::Zepto.factor(), UnitSi::Zepto)
            }
            value if value >= UnitSi::Yocto.factor() => {
                (value / UnitSi::Yocto.factor(), UnitSi::Yocto)
            }
            value if value >= UnitSi::Ronto.factor() => {
                (value / UnitSi::Ronto.factor(), UnitSi::Ronto)
            }
            value if value >= UnitSi::Quecto.factor() => {
                (value / UnitSi::Quecto.factor(), UnitSi::Quecto)
            }
            _ => (value, UnitSi::None),
        }
    }

    /// Reduces the given value to the most appropriate SI prefix as an `i64`,
    /// returning a tuple of the reduced size, the prefix, and the remainder.
    pub const fn reduce_i64(value: i64) -> (i64, Self, i64) {
        match value {
            value if value >= UnitSi::Exa.factor_i64() => {
                (value / UnitSi::Exa.factor_i64(), UnitSi::Exa, value % UnitSi::Exa.factor_i64())
            }
            value if value >= UnitSi::Peta.factor_i64() => {
                (value / UnitSi::Peta.factor_i64(), UnitSi::Peta, value % UnitSi::Peta.factor_i64())
            }
            value if value >= UnitSi::Tera.factor_i64() => {
                (value / UnitSi::Tera.factor_i64(), UnitSi::Tera, value % UnitSi::Tera.factor_i64())
            }
            value if value >= UnitSi::Giga.factor_i64() => {
                (value / UnitSi::Giga.factor_i64(), UnitSi::Giga, value % UnitSi::Giga.factor_i64())
            }
            value if value >= UnitSi::Mega.factor_i64() => {
                (value / UnitSi::Mega.factor_i64(), UnitSi::Mega, value % UnitSi::Mega.factor_i64())
            }
            value if value >= UnitSi::Kilo.factor_i64() => {
                (value / UnitSi::Kilo.factor_i64(), UnitSi::Kilo, value % UnitSi::Kilo.factor_i64())
            }
            value if value >= UnitSi::Hecto.factor_i64() => (
                value / UnitSi::Hecto.factor_i64(),
                UnitSi::Hecto,
                value % UnitSi::Hecto.factor_i64(),
            ),
            value if value >= UnitSi::Deca.factor_i64() => {
                (value / UnitSi::Deca.factor_i64(), UnitSi::Deca, value % UnitSi::Deca.factor_i64())
            }
            value if value <= UnitSi::Atto.factor_i64() => {
                (value * UnitSi::Atto.factor_i64().abs(), UnitSi::Atto, 0)
            }
            value if value <= UnitSi::Femto.factor_i64() => {
                (value * UnitSi::Femto.factor_i64().abs(), UnitSi::Femto, 0)
            }
            value if value <= UnitSi::Pico.factor_i64() => {
                (value * UnitSi::Pico.factor_i64().abs(), UnitSi::Pico, 0)
            }
            value if value <= UnitSi::Nano.factor_i64() => {
                (value * UnitSi::Nano.factor_i64().abs(), UnitSi::Nano, 0)
            }
            value if value <= UnitSi::Micro.factor_i64() => {
                (value * UnitSi::Micro.factor_i64().abs(), UnitSi::Micro, 0)
            }
            value if value <= UnitSi::Milli.factor_i64() => {
                (value * UnitSi::Milli.factor_i64().abs(), UnitSi::Milli, 0)
            }
            value if value <= UnitSi::Centi.factor_i64() => {
                (value * UnitSi::Centi.factor_i64().abs(), UnitSi::Centi, 0)
            }
            value if value <= UnitSi::Deci.factor_i64() => {
                (value * UnitSi::Deci.factor_i64().abs(), UnitSi::Deci, 0)
            }
            _ => (value, UnitSi::None, 0),
        }
    }

    /// Reduces the given value to the most appropriate SI prefix as an `i128`,
    /// returning a tuple of the reduced size, the prefix, and the remainder.
    pub const fn reduce_i128(value: i128) -> (i128, Self, i128) {
        match value {
            value if value >= UnitSi::Quetta.factor_i128() => (
                value / UnitSi::Quetta.factor_i128(),
                UnitSi::Quetta,
                value % UnitSi::Quetta.factor_i128(),
            ),
            value if value >= UnitSi::Ronna.factor_i128() => (
                value / UnitSi::Ronna.factor_i128(),
                UnitSi::Ronna,
                value % UnitSi::Ronna.factor_i128(),
            ),
            value if value >= UnitSi::Yotta.factor_i128() => (
                value / UnitSi::Yotta.factor_i128(),
                UnitSi::Yotta,
                value % UnitSi::Yotta.factor_i128(),
            ),
            value if value >= UnitSi::Zetta.factor_i128() => (
                value / UnitSi::Zetta.factor_i128(),
                UnitSi::Zetta,
                value % UnitSi::Zetta.factor_i128(),
            ),
            value if value <= UnitSi::Quecto.factor_i128() => {
                (value * UnitSi::Quecto.factor_i128().abs(), UnitSi::Quecto, 0)
            }
            value if value <= UnitSi::Ronto.factor_i128() => {
                (value * UnitSi::Ronto.factor_i128().abs(), UnitSi::Ronto, 0)
            }
            value if value <= UnitSi::Yocto.factor_i128() => {
                (value * UnitSi::Yocto.factor_i128().abs(), UnitSi::Yocto, 0)
            }
            value if value <= UnitSi::Zepto.factor_i128() => {
                (value * UnitSi::Zepto.factor_i128().abs(), UnitSi::Zepto, 0)
            }
            _ => {
                let (v, p, r) = Self::reduce_i64(value as i64);
                (v as i128, p, r as i128)
            }
        }
    }

    /// Reduces the given value to a chain of appropriate SI prefixes as an `f64`,
    /// stopping when the remainder is less than the given threshold.
    #[must_use]
    #[cfg(any(feature = "std", all(feature = "alloc", feature = "_float_f64")))]
    #[cfg_attr(
        nightly_doc,
        doc(cfg(any(feature = "std", all(feature = "alloc", feature = "_float_f64"))))
    )]
    pub fn reduce_chain(value: f64, threshold: f64) -> Vec<(f64, Self)> {
        if value == 0.0 {
            return vec![(0.0, UnitSi::None)];
        }

        let mut result = Vec::new();
        let mut remainder = value;

        // Ensure the threshold is positive and above a small epsilon value
        // in order to prevent infinite loops
        let effective_threshold =
            if threshold <= 0.0 { crate::FloatConst::MEDIUM_MARGIN } else { threshold };

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
            result.push((remainder, UnitSi::None));
        }
        result
    }

    /// Reduces the given value to a chain of appropriate SI prefixes as an `i64`,
    /// stopping when the remainder is less than the given threshold.
    #[must_use]
    #[cfg(feature = "alloc")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
    pub fn reduce_chain_i64(value: i64, threshold: i64) -> Vec<(i64, Self)> {
        let mut result = Vec::new();
        let mut remainder = value;

        while remainder.abs() > threshold.abs() {
            let (size, unit, new_remainder) = Self::reduce_i64(remainder);
            result.push((size, unit));
            remainder = new_remainder;

            if remainder.abs() < threshold.abs() {
                break;
            }
        }
        if remainder.abs() >= threshold.abs() {
            result.push((remainder, UnitSi::None));
        }
        result
    }

    /// Reduces the given value to a chain of appropriate SI prefixes as an `i128`,
    /// stopping when the remainder is less than the given threshold.
    #[must_use]
    #[cfg(feature = "alloc")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
    pub fn reduce_chain_i128(value: i128, threshold: i128) -> Vec<(i128, Self)> {
        let mut result = Vec::new();
        let mut remainder = value;

        while remainder.abs() > threshold.abs() {
            let (size, unit, new_remainder) = Self::reduce_i128(remainder);
            result.push((size, unit));
            remainder = new_remainder;

            if remainder.abs() < threshold.abs() {
                break;
            }
        }
        if remainder.abs() >= threshold.abs() {
            result.push((remainder, UnitSi::None));
        }
        result
    }
}

impl UnitSi {
    /// Returns an iterator in ascending order of magnitude.
    pub fn asc_iter() -> impl Iterator<Item = Self> {
        const UNITS: [UnitSi; 25] = [
            UnitSi::Quecto,
            UnitSi::Ronto,
            UnitSi::Yocto,
            UnitSi::Zepto,
            UnitSi::Atto,
            UnitSi::Femto,
            UnitSi::Pico,
            UnitSi::Nano,
            UnitSi::Micro,
            UnitSi::Milli,
            UnitSi::Centi,
            UnitSi::Deci,
            UnitSi::None,
            UnitSi::Deca,
            UnitSi::Hecto,
            UnitSi::Kilo,
            UnitSi::Mega,
            UnitSi::Giga,
            UnitSi::Tera,
            UnitSi::Peta,
            UnitSi::Exa,
            UnitSi::Zetta,
            UnitSi::Yotta,
            UnitSi::Ronna,
            UnitSi::Quetta,
        ];
        UNITS.iter().copied()
    }

    /// Returns an iterator in descending order of magnitude.
    pub fn desc_iter() -> impl Iterator<Item = Self> {
        const UNITS: [UnitSi; 25] = [
            UnitSi::Quetta,
            UnitSi::Ronna,
            UnitSi::Yotta,
            UnitSi::Zetta,
            UnitSi::Exa,
            UnitSi::Peta,
            UnitSi::Tera,
            UnitSi::Giga,
            UnitSi::Mega,
            UnitSi::Kilo,
            UnitSi::Hecto,
            UnitSi::Deca,
            UnitSi::None,
            UnitSi::Deci,
            UnitSi::Centi,
            UnitSi::Milli,
            UnitSi::Micro,
            UnitSi::Nano,
            UnitSi::Pico,
            UnitSi::Femto,
            UnitSi::Atto,
            UnitSi::Zepto,
            UnitSi::Yocto,
            UnitSi::Ronto,
            UnitSi::Quecto,
        ];
        UNITS.iter().copied()
    }
}

impl From<UnitSi> for f32 {
    fn from(from: UnitSi) -> f32 {
        from.factor() as f32
    }
}
impl From<UnitSi> for f64 {
    fn from(from: UnitSi) -> f64 {
        from.factor()
    }
}
impl From<UnitSi> for i64 {
    fn from(from: UnitSi) -> i64 {
        from.factor_i64()
    }
}
impl From<UnitSi> for i128 {
    fn from(from: UnitSi) -> i128 {
        from.factor_i128()
    }
}
impl_try_from![UnitSi, i64 => i32, i16, u64, u32, u16];
impl_try_from![UnitSi, i128 => u128];

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::{
        UnitSi,
        UnitSi::{
            Centi, Deca, Deci, Giga, Hecto, Kilo, Mega, Micro, Milli, Nano, Quetta, Ronna, Tera,
            Yotta, Zetta,
        },
    };
    #[cfg(any(feature = "std", all(feature = "alloc", feature = "_float_f64")))]
    use crate::vec_ as vec;
    use crate::{code::sf, num::FloatConst};

    /* reduce */

    #[test]
    fn unit_si_reduce_i64() {
        let value = 4 * i64::from(Tera);
        let (reduced_value, unit, remainder) = UnitSi::reduce_i64(value);
        assert_eq!(reduced_value, 4);
        assert_eq!(unit, Tera);
        assert_eq!(remainder, 0);

        let value = 15 * i64::from(Kilo) + 500; // 15.5 k
        let (reduced_value, unit, remainder) = UnitSi::reduce_i64(value);
        assert_eq!(reduced_value, 15);
        assert_eq!(unit, Kilo);
        assert_eq!(remainder, 500);

        let value = 1; // Base unit
        let (reduced_value, unit, remainder) = UnitSi::reduce_i64(value);
        assert_eq!(reduced_value, 1);
        assert_eq!(unit, UnitSi::None);
        assert_eq!(remainder, 0);

        let value = 999;
        let (reduced_value, unit, remainder) = UnitSi::reduce_i64(value);
        assert_eq!(reduced_value, 9);
        assert_eq!(unit, Hecto);
        assert_eq!(remainder, 99);
    }

    #[test]
    fn unit_si_reduce_i128() {
        let value = 3000 * i128::from(Ronna) + 5 * i128::from(Mega);
        let (reduced_value, unit, remainder) = UnitSi::reduce_i128(value);
        assert_eq!(reduced_value, 3);
        assert_eq!(unit, Quetta);
        assert_eq!(remainder, 5 * i128::from(Mega));

        let value = 900_033; // Between Kilo and Mega
        let (reduced_value, unit, remainder) = UnitSi::reduce_i128(value);
        assert_eq!(reduced_value, 900);
        assert_eq!(unit, Kilo);
        assert_eq!(remainder, 33);
    }

    /* reduce_chain */

    #[test]
    #[cfg(any(feature = "std", all(feature = "alloc", feature = "_float_f64")))]
    #[cfg_attr(
        nightly_doc,
        doc(cfg(any(feature = "std", all(feature = "alloc", feature = "_float_f64"))))
    )]
    fn unit_si_reduce_chain() {
        let margin = f64::MEDIUM_MARGIN;

        assert_eq![
            // single unit: 1G
            UnitSi::reduce_chain(Giga.factor(), margin),
            vec![(1.0, Giga)]
        ];
        assert_eq![
            // multiple units: 1.5G
            UnitSi::reduce_chain(1.5 * Giga.factor(), margin),
            vec![(1.0, Giga), (500.0, Mega)]
        ];
        assert_eq![
            // 1G + 1K
            UnitSi::reduce_chain(Giga.factor() + Kilo.factor(), margin),
            // vec![(1.0, Giga), (1.0, Kilo)] NOTE: rounding error:
            sf! { vec![
                (1.0, Giga), (9.0, Hecto), (9.0, Deca), (99.0, Deci),
                (9.0, Centi), (9.0, Milli), (999.0, Micro), (917.0, Nano)
            ]}
        ];
        assert_eq![
            // Small value (only 512K)
            UnitSi::reduce_chain(Mega.factor() / 2.0, margin),
            vec![(500.0, Kilo)]
        ];
        assert_eq![
            // Very large value (1Y + 1Z + 1G)
            UnitSi::reduce_chain(Yotta.factor() + Zetta.factor() + Giga.factor(), margin),
            // vec![(1.0, Yotta), (1.0, Zetta), (1.0, Giga)] // NOTE: rounding error:
            sf! { vec![
                (1.0, Yotta), (1.0, Zetta), (1.0, Giga), (88.0, Kilo), (9.0, Hecto),
                (5.0, Deci), (8.0, Centi), (2.0, Milli), (341.0, Micro), (11.0, Nano)
            ]}
        ];
        assert_eq![
            // Zero value
            UnitSi::reduce_chain(0.0, margin),
            vec![(0.0, UnitSi::None)]
        ];
        assert_eq![
            // Fractional value (0.5G)
            UnitSi::reduce_chain(Giga.factor() / 2., margin),
            vec![(500., Mega)]
        ];
    }
}
