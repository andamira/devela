// devela/src/num/prob/probability.rs
//
//! Defines [`Probability`].
//

use crate::{_impl_init, RatioU64, is, unwrap};

#[doc = crate::_tags!(num)]
/// An exact probability represented by a canonical rational value.
#[doc = crate::_doc_meta!{
    location("num/prob", struct Probability),
    test_size_of(Probability = 16|128; niche Option),
}]
/// A probability is a value in the closed unit interval:
/// $$
/// 0 \le P(A) \le 1
/// $$
///
/// This representation stores an exact reduced ratio
/// $$
/// P(A) = \frac{n}{d}, \qquad 0 \le n \le d, \qquad d > 0.
/// $$
///
/// Equivalent ratios have the same canonical representation.
///
/// If $g = \gcd(n,d)$, construction reduces the terms as
/// $$
/// \frac{n}{d} = \frac{n/g}{d/g}
/// $$
///
/// Therefore `1/2`, `2/4`, and `50/100` construct equal values.
///
/// [`ZERO`](Self::ZERO) represents impossibility and
/// [`ONE`](Self::ONE) represents certainty.
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Probability {
    ratio: RatioU64,
}

_impl_init! { Self::ZERO => Probability }
impl Default for Probability {
    fn default() -> Self {
        Self::ZERO
    }
}

#[rustfmt::skip]
impl Probability {
    /// The impossible probability, $P = 0$.
    pub const ZERO: Self = Self { ratio: RatioU64::ZERO };
    /// The certain probability, $P = 1$.
    pub const ONE: Self = Self { ratio: RatioU64::ONE };

    /// Constructs an exact probability from the ratio `num / den`.
    ///
    /// Equivalent ratios are reduced to one canonical representation.
    ///
    /// Returns `None` if `den == 0` or `num > den`.
    #[must_use]
    pub const fn new(num: u64, den: u64) -> Option<Self> {
        let ratio = unwrap![some? RatioU64::new(num, den)];
        Self::from_ratio(ratio)
    }
    /// Constructs an exact probability from a [`RatioU64`].
    ///
    /// The ratio is reduced to its canonical representation.
    ///
    /// Returns `None` if the ratio is greater than one.
    #[must_use]
    pub const fn from_ratio(ratio: RatioU64) -> Option<Self> {
        is! { ratio.num() > ratio.den(), return None }
        Some(Self { ratio: ratio.reduced() })
    }
    /// Returns the canonical ratio representation.
    #[must_use]
    pub const fn as_ratio(&self) -> &RatioU64 { &self.ratio }

    /// Returns the canonical ratio representation by value.
    #[must_use]
    pub const fn into_ratio(self) -> RatioU64 { self.ratio }

    /// Returns the canonical numerator.
    #[must_use]
    pub const fn num(self) -> u64 { self.ratio.num() }

    /// Returns the canonical denominator.
    #[must_use]
    pub const fn den(self) -> u64 { self.ratio.den() }

    /// Returns the canonical `(numerator, denominator)` pair.
    #[must_use]
    pub const fn num_den(self) -> (u64, u64) { self.ratio.num_den() }

    /// Returns whether this probability is impossible, $P = 0$.
    #[must_use]
    pub const fn is_zero(self) -> bool { self.ratio.is_zero() }

    /// Returns whether this probability is certain, $P = 1$.
    #[must_use]
    pub const fn is_one(self) -> bool { self.ratio.is_one() }

    /// Returns the complementary probability, $1 - P$.
    ///
    /// If
    /// $$
    /// P(A) = \frac{n}{d},
    /// $$
    ///
    /// then
    /// $$
    /// P(A^\complement) = 1 - P(A) = \frac{d-n}{d}.
    /// $$
    #[must_use]
    pub const fn complement(self) -> Self {
        let (num, den) = self.ratio.num_den();
        // `num <= den` is a Probability invariant, and the existing denominator is nonzero.
        let ratio = RatioU64::new_nonzero(den - num, self.ratio.den_nonzero());
        Self { ratio: ratio.reduced() }
    }
}

#[cfg(test)]
mod _test {
    use super::*;

    const HALF: Probability = match Probability::new(2, 4) {
        Some(p) => p,
        None => panic!("valid probability"),
    };

    #[test]
    fn probability_const_construction() {
        assert_eq!(HALF.num_den(), (1, 2));
    }
    #[test]
    fn probability_accepts_unit_interval() {
        assert_eq!(Probability::new(0, 1), Some(Probability::ZERO));
        assert_eq!(Probability::new(1, 1), Some(Probability::ONE));
        assert!(Probability::new(1, 2).is_some());
        assert!(Probability::new(5, 7).is_some());
    }
    #[test]
    fn probability_rejects_invalid_terms() {
        assert_eq!(Probability::new(0, 0), None);
        assert_eq!(Probability::new(1, 0), None);
        assert_eq!(Probability::new(2, 1), None);
        assert_eq!(Probability::new(u64::MAX, 1), None);
    }
    #[test]
    fn probability_canonicalizes_equivalent_ratios() {
        let a = Probability::new(1, 2).unwrap();
        let b = Probability::new(2, 4).unwrap();
        let c = Probability::new(50, 100).unwrap();
        assert_eq!(a, b);
        assert_eq!(b, c);
        assert_eq!(a.num_den(), (1, 2));
        assert_eq!(b.num_den(), (1, 2));
        assert_eq!(c.num_den(), (1, 2));
    }
    #[test]
    fn probability_canonicalizes_zero_and_one() {
        assert_eq!(Probability::new(0, 999).unwrap(), Probability::ZERO);
        assert_eq!(Probability::new(999, 999).unwrap(), Probability::ONE);
        assert_eq!(Probability::ZERO.num_den(), (0, 1));
        assert_eq!(Probability::ONE.num_den(), (1, 1));
    }
    #[test]
    fn probability_constructs_from_ratio() {
        let ratio = RatioU64::new(6, 8).unwrap();
        let probability = Probability::from_ratio(ratio).unwrap();
        assert_eq!(probability.num_den(), (3, 4));
        let invalid = RatioU64::new(5, 4).unwrap();
        assert_eq!(Probability::from_ratio(invalid), None);
    }
    #[test]
    fn probability_complement_is_exact() {
        assert_eq!(Probability::new(1, 4).unwrap().complement(), Probability::new(3, 4).unwrap(),);
        assert_eq!(Probability::ZERO.complement(), Probability::ONE);
        assert_eq!(Probability::ONE.complement(), Probability::ZERO);
        assert_eq!(HALF.complement(), HALF);
    }
}
