// devela::num::quant::ratio
//
//! Defines [`Ratio`].
//
// IMPROVE
// - to Frac

use crate::{NumConst, Rem};

// use crate::Frac;

#[doc = crate::_TAG_QUANT!()]
/// A generic ratio representing a proportional relationship between two values.
///
/// This type models a mathematical ratio `n/d`, where:
/// - `N` is the numerator type.
/// - `D` is the denominator type.
///
/// Unlike `Frac<T>`, which represents a fraction as a single number,
/// `Ratio<N, D>` represents **a relationship between different quantities**.
///
/// # Examples
/// ```
/// # use devela::Ratio;
/// let aspect_ratio = Ratio::new(16u32, 9u32);
/// let velocity = Ratio::new(100.0, 2.0); // Distance / Time
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ratio<N, D> {
    /// The numerator.
    pub n: N,
    /// The denominator.
    pub d: D,
}

#[rustfmt::skip]
impl<N, D> Ratio<N, D> {
    /// Creates a new `Ratio<N, D>`.
    pub const fn new(n: N, d: D) -> Self {
        Self { n, d }
    }
    /// Creates a new `Ratio<N, D>`, ensuring `d != 0`.
    pub fn new_checked(n: N, d: D) -> Option<Self> where D: NumConst<Num = D> + PartialEq {
        let new = Self::new(n, d);
        if let Some(zero) = D::NUM_ZERO {
            if new.d == zero { None } else { Some(new) }
        } else {
            Some(new)
        }
    }
}

#[rustfmt::skip]
impl<N: Clone, D: Clone> Ratio<N, D> {
    /// Returns the ratio as a floating-point approximation.
    pub fn as_f32(&self) -> f32 where N: Into<f32>, D: Into<f32> {
        self.n.clone().into() / self.d.clone().into()
    }
    /// Returns the ratio as a floating-point approximation.
    pub fn as_f64(&self) -> f64 where N: Into<f64>, D: Into<f64> {
        self.n.clone().into() / self.d.clone().into()
    }

    /// Returns whether the ratio is effectively a whole number (`n % d == 0`).
    ///
    /// Returns `None` if `NUM_ZERO` is not defined for `N`, meaning divisibility
    /// cannot be determined.
    pub fn is_whole(&self) -> Option<bool>
    where
        N: NumConst<Num = N> + Rem<D, Output = N> + PartialEq,
    {
        N::NUM_ZERO.map(|zero| (self.n.clone() % self.d.clone()) == zero)
    }

    // /// Attempts to reduce the ratio into a simplified `Frac<N>`, if `N == D`.
    // pub fn reduce(self) -> Option<Frac<N>>
    // where
    //     N: NumConst<Num = N> + Rem<D, Output = N> + PartialEq,
    // {
    //     if self.n == self.d { Some(Frac(self.n)) } else { None }
    // }
}
