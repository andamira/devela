// devela/src/num/prob/dist/categorical.rs
//
//! Defines [`DistCategorical`].
//

use crate::{DistError, NonZeroU64, Pcg32, Probability, Rand, Sign, is, unwrap, whilst};

#[doc = crate::_tags!(num)]
/// A finite categorical distribution represented by integer weights.
#[doc = crate::_doc_meta!{
    location("num/prob/dist", struct DistCategorical),
    #[cfg(target_pointer_width = "32")]
    test_size_of(DistCategorical = 16|128; niche Option),
    #[cfg(target_pointer_width = "64")]
    test_size_of(DistCategorical = 24|192; niche Option),
}]
/// Each category is identified by its index in the weight slice and has probability
/// `weights[index] / total_weight`.
///
/// For category $i$ with weight $w_i$,
/// $$
/// P(X=i) = \frac{w_i} {\sum_{j=0}^{k-1} w_j}
/// $$
///
/// Zero individual weights are allowed.
/// The total weight must be positive and must fit in `u64`.
///
/// Weights are retained exactly and are not normalized or reduced.
///
/// Sampling uses a direct linear scan over the weights and requires no
/// allocation or prepared auxiliary storage.
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DistCategorical<'a> {
    weights: &'a [u64],
    total: NonZeroU64,
}

impl<'a> DistCategorical<'a> {
    /// Constructs a categorical distribution from relative integer weights.
    ///
    /// # Errors
    /// Returns [`PositiveRequired`][DistError::PositiveRequired]
    /// if all weights are zero or the slice is empty.
    ///
    /// Returns [`Overflow`][DistError::Overflow]
    /// if their sum exceeds `u64::MAX`.
    pub const fn new(weights: &'a [u64]) -> Result<Self, DistError> {
        let mut total = 0u64;
        whilst! { i in 0..weights.len(); {
            total = unwrap![some_ok_or? total.checked_add(weights[i]),
                DistError::Overflow(Some(Sign::Positive))]
        }}
        unwrap![some_ok_map_or NonZeroU64::new(total),
            |total| Self { weights, total }, DistError::PositiveRequired]
    }
    /// Returns the relative weights of all categories.
    #[must_use]
    pub const fn weights(&self) -> &'a [u64] {
        self.weights
    }
    /// Returns the number of categories.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.weights.len()
    }
    /// Returns the total weight.
    #[must_use]
    pub const fn total_weight(&self) -> u64 {
        self.total.get()
    }
    /// Returns the weight of the category at `index`.
    #[must_use]
    pub const fn weight(&self, index: usize) -> Option<u64> {
        is! { index < self.weights.len(), Some(self.weights[index]), None }
    }
    /// Returns the category containing `ticket` in cumulative weight space.
    ///
    /// Returns `None` when `ticket >= self.total_weight()`.
    ///
    /// This performs a linear scan over the weights.
    #[must_use]
    pub const fn index_at(&self, mut ticket: u64) -> Option<usize> {
        is! { ticket >= self.total.get(), return None }
        whilst! { i in 0..self.weights.len(); {
            let weight = self.weights[i];
            is! { ticket < weight, return Some(i) }
            ticket -= weight;
        }}
        None
    }
    /// Returns the exact probability of the category at `index`.
    #[must_use]
    pub const fn probability_of(&self, index: usize) -> Option<Probability> {
        let weight = unwrap![some? self.weight(index)];
        Probability::new(weight, self.total.get())
    }
    /// Samples a category index using an infallible random source.
    ///
    /// This performs a linear scan over the weights.
    #[must_use]
    pub fn sample<R: Rand + ?Sized>(&self, rng: &mut R) -> usize {
        let ticket = rng.rand_below(self.total.get());
        // SAFETY: `ticket < total`, and `total` is the exact sum of all weights,
        // so `index_at(ticket)` must resolve to a category.
        unwrap![some_guaranteed_or_ub self.index_at(ticket)]
    }
    /// Samples a category index using the canonical const-capable [`Pcg32`].
    ///
    /// This performs a linear scan over the weights.
    #[must_use]
    pub const fn sample_pcg32(&self, rng: &mut Pcg32) -> usize {
        let ticket = rng.next_bounded_u64(self.total.get());
        // SAFETY: `ticket < total`, and `total` is the exact sum of all weights,
        // so `index_at(ticket)` must resolve to a category.
        unwrap![some_guaranteed_or_ub self.index_at(ticket)]
    }
}

#[cfg(test)]
mod _test {
    use super::*;

    const CONST_WEIGHTS: [u64; 3] = [2, 3, 1];

    const CONST_DIST: DistCategorical<'static> =
        unwrap![ok_expect DistCategorical::new(&CONST_WEIGHTS), "valid categorical distribution"];

    const CONST_SAMPLE: usize = {
        let mut rng = Pcg32::new(1, 2);
        CONST_DIST.sample_pcg32(&mut rng)
    };

    #[test]
    fn categorical_const_and_generic_pcg32_sampling_agree() {
        let mut rng = Pcg32::new(1, 2);
        assert_eq!(CONST_SAMPLE, CONST_DIST.sample(&mut rng));
        assert!(CONST_SAMPLE < CONST_DIST.len());
    }
    #[test]
    fn categorical_constructs_and_exposes_weights() {
        let weights = [2, 3, 1];
        let dist = DistCategorical::new(&weights).unwrap();
        assert_eq!(dist.weights(), &weights);
        assert_eq!(dist.len(), 3);
        assert_eq!(dist.total_weight(), 6);
        assert_eq!(dist.weight(0), Some(2));
        assert_eq!(dist.weight(2), Some(1));
        assert_eq!(dist.weight(3), None);
    }
    #[test]
    fn categorical_rejects_zero_total() {
        assert_eq!(DistCategorical::new(&[]), Err(DistError::PositiveRequired));
        assert_eq!(DistCategorical::new(&[0, 0, 0]), Err(DistError::PositiveRequired));
    }
    #[test]
    fn categorical_rejects_total_overflow() {
        assert_eq!(
            DistCategorical::new(&[u64::MAX, 1]),
            Err(DistError::Overflow(Some(Sign::Positive)))
        );
    }
    #[test]
    fn categorical_maps_weight_space() {
        let dist = DistCategorical::new(&[2, 3, 1]).unwrap();
        assert_eq!(dist.index_at(0), Some(0));
        assert_eq!(dist.index_at(1), Some(0));
        assert_eq!(dist.index_at(2), Some(1));
        assert_eq!(dist.index_at(3), Some(1));
        assert_eq!(dist.index_at(4), Some(1));
        assert_eq!(dist.index_at(5), Some(2));
        assert_eq!(dist.index_at(6), None);
    }
    #[test]
    fn categorical_skips_zero_weight_categories() {
        let dist = DistCategorical::new(&[0, 2, 0, 3]).unwrap();
        assert_eq!(dist.index_at(0), Some(1));
        assert_eq!(dist.index_at(1), Some(1));
        assert_eq!(dist.index_at(2), Some(3));
        assert_eq!(dist.index_at(3), Some(3));
        assert_eq!(dist.index_at(4), Some(3));
    }
    #[test]
    fn categorical_exposes_exact_category_probabilities() {
        let dist = DistCategorical::new(&[0, 2, 3]).unwrap();
        assert_eq!(dist.probability_of(0), Some(Probability::ZERO));
        assert_eq!(dist.probability_of(1), Probability::new(2, 5));
        assert_eq!(dist.probability_of(2), Probability::new(3, 5));
        assert_eq!(dist.probability_of(3), None);
    }
}
