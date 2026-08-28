// devela/src/num/prob/dist/bernoulli.rs
//
//! Defines [`DistBernoulli`].
//

use crate::{Pcg32, Probability, Rand, is};

#[doc = crate::_tags!(num)]
/// A Bernoulli distribution over two boolean outcomes.
#[doc = crate::_doc_meta!{
    location("num/prob/dist", struct DistBernoulli),
    test_size_of(DistBernoulli = 16|128; niche Option),
}]
/// The parameter $p$ gives the probability of `true`, corresponding to
/// the conventional Bernoulli outcome $X=1$:
/// $$
/// P(X=1)=p, \qquad P(X=0)=1-p.
/// $$
///
/// Equivalently, for $x \in \{0,1\}$,
/// $$
/// P(X=x)=p^x(1-p)^{1-x}.
/// $$
///
/// The probability is represented exactly by [`Probability`].
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DistBernoulli {
    /// The probability of sampling `true`.
    pub probability: Probability,
}

impl DistBernoulli {
    /// Constructs a Bernoulli distribution with the given probability of `true`.
    #[must_use]
    pub const fn new(probability: Probability) -> Self {
        Self { probability }
    }
    /// Returns the probability of the given outcome.
    #[must_use]
    pub const fn probability_of(self, outcome: bool) -> Probability {
        is! { outcome, self.probability, self.probability.complement() }
    }
    /// Samples a boolean outcome using an infallible random source.
    #[must_use]
    pub fn sample<R: Rand + ?Sized>(&self, rng: &mut R) -> bool {
        rng.rand_below(self.probability.den()) < self.probability.num()
    }
    /// Samples a boolean outcome using the canonical const-capable [`Pcg32`].
    #[must_use]
    pub const fn sample_pcg32(&self, rng: &mut Pcg32) -> bool {
        rng.next_bounded_u64(self.probability.den()) < self.probability.num()
    }
}

#[cfg(test)]
mod _test {
    use crate::{DistBernoulli, Pcg32, Probability, RandFake, unwrap};

    const BERNOULLI_HALF: DistBernoulli =
        DistBernoulli::new(unwrap![some_expect Probability::new(1, 2), "valid probability"]);
    const BERNOULLI_CONST_SAMPLE: bool = {
        let mut rng = Pcg32::new(1, 2);
        BERNOULLI_HALF.sample_pcg32(&mut rng)
    };

    #[test]
    fn bernoulli_constructs_exactly() {
        let probability = Probability::new(2, 4).unwrap();
        let dist = DistBernoulli::new(probability);
        assert_eq!(dist.probability, Probability::new(1, 2).unwrap());
    }
    #[test]
    fn bernoulli_exposes_outcome_probabilities() {
        let dist = DistBernoulli::new(Probability::new(1, 4).unwrap());
        assert_eq!(dist.probability_of(true), Probability::new(1, 4).unwrap(),);
        assert_eq!(dist.probability_of(false), Probability::new(3, 4).unwrap(),);
    }
    #[test]
    fn bernoulli_samples_exact_ticket_partition() {
        type Rng<const N: usize> = RandFake<N, true>;
        let dist = DistBernoulli::new(Probability::new(1, 4).unwrap());
        let mut yes = Rng::new([0]);
        let mut no = Rng::new([1]);
        assert!(dist.sample(&mut yes));
        assert!(!dist.sample(&mut no));
    }
    #[test]
    fn bernoulli_extremes_are_exact() {
        type Rng<const N: usize> = RandFake<N, true>;
        let mut rng0 = Rng::new([123]);
        let mut rng1 = Rng::new([456]);
        assert!(!DistBernoulli::new(Probability::ZERO).sample(&mut rng0));
        assert!(DistBernoulli::new(Probability::ONE).sample(&mut rng1));
    }
    #[test]
    fn bernoulli_const_and_generic_pcg32_sampling_agree() {
        let mut rng = Pcg32::new(1, 2);
        assert_eq!(BERNOULLI_CONST_SAMPLE, BERNOULLI_HALF.sample(&mut rng),);
    }
}
