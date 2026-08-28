// devela/src/num/prob/dist/categorical.rs
//
//! Defines [`DistCategorical`].
//

use crate::{DistError, NonZeroU64, Pcg32, Rand, Sign, is, unwrap, whilst};

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
///
/// `weights[index] / total_weight`.
///
/// Zero individual weights are allowed.
/// The total weight must be nonzero and must fit in `u64`.
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
    /// Returns [`DistError::PositiveRequired`]
    /// if all weights are zero or the slice is empty.
    ///
    /// Returns [`DistError::Overflow`]
    /// if their sum exceeds `u64::MAX`.
    pub const fn new(weights: &'a [u64]) -> Result<Self, DistError> {
        let mut total = 0u64;
        whilst! { i in 0..weights.len(); {
            total = match total.checked_add(weights[i]) {
                Some(total) => total,
                None => return Err(DistError::Overflow(Some(Sign::Positive))),
            };
        }}
        match NonZeroU64::new(total) {
            Some(total) => Ok(Self { weights, total }),
            None => Err(DistError::PositiveRequired),
        }
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
    /// Samples a category index using an infallible random source.
    ///
    /// This performs a linear scan over the weights.
    #[must_use]
    pub fn sample<R: Rand + ?Sized>(&self, rng: &mut R) -> usize {
        let ticket = rng.rand_below(self.total.get());
        unwrap![some_guaranteed_or_ub self.index_at(ticket)]
    }
    /// Samples a category index using the canonical const-capable [`Pcg32`].
    ///
    /// This is the const-compatible counterpart of [`sample`][Self::sample].
    ///
    /// This performs a linear scan over the weights.
    #[must_use]
    pub const fn sample_pcg32(&self, rng: &mut Pcg32) -> usize {
        let ticket = rng.next_bounded_u64(self.total.get());
        unwrap![some_guaranteed_or_ub self.index_at(ticket)]
    }
}
