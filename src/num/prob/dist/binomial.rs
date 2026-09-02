// devela/src/num/prob/dist/binomial.rs
//
//! Defines [`DistBernoulli`], [`DistBinomial`].
//

use crate::{Pcg32, Probability, Rand, RandTry, is, unwrap, whilst};

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
    pub const fn new(probability: Probability) -> Self {
        Self { probability }
    }
    /// Returns the probability of the given outcome.
    pub const fn probability_of(self, outcome: bool) -> Probability {
        is! { outcome, self.probability, self.probability.complement() }
    }
    /// Samples a boolean outcome using an infallible random source.
    #[must_use]
    pub fn sample<R: Rand + ?Sized>(&self, rng: &mut R) -> bool {
        // SAFETY: `Rand` requires `RandTry<Error = Infallible>`,
        // so `sample_try` cannot return `Err`.
        unwrap![ok_guaranteed_or_ub self.sample_try(rng)]
    }
    /// Attempts to sample a boolean outcome using a fallible random source.
    pub fn sample_try<R: RandTry + ?Sized>(&self, rng: &mut R) -> Result<bool, R::Error> {
        Ok(rng.rand_try_below(self.probability.den())? < self.probability.num())
    }
    /// Samples a boolean outcome using the canonical const-capable [`Pcg32`].
    #[must_use]
    pub const fn sample_pcg32(&self, rng: &mut Pcg32) -> bool {
        rng.next_bounded_u64(self.probability.den()) < self.probability.num()
    }
}

#[doc = crate::_tags!(num)]
/// A binomial distribution counting successes across repeated Bernoulli trials.
#[doc = crate::_doc_meta!{
    location("num/prob/dist", struct DistBinomial),
    test_size_of(DistBinomial = 24|192; niche Option),
}]
/// Each of `trials` independent trials has the same probability of success.
/// The sampled value is the number of successes, from `0` through `trials`.
///
/// A Bernoulli distribution is the special case with one trial.
/// Zero trials are valid and always produce zero successes.
///
/// Sampling performs one random draw per trial and requires no allocation.
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DistBinomial {
    /// The number of independent trials.
    pub trials: u64,
    /// The probability of success on each trial.
    pub probability: Probability,
}

impl DistBinomial {
    /// Constructs a binomial distribution.
    pub const fn new(trials: u64, probability: Probability) -> Self {
        Self { trials, probability }
    }
    /// Samples the number of successes using an infallible random source.
    #[must_use]
    pub fn sample<R: Rand + ?Sized>(&self, rng: &mut R) -> u64 {
        // SAFETY: `Rand` requires `RandTry<Error = Infallible>`,
        // so `sample_try` cannot return `Err`.
        unwrap![ok_guaranteed_or_ub self.sample_try(rng)]
    }
    /// Attempts to sample the number of successes using a fallible random source.
    pub fn sample_try<R: RandTry + ?Sized>(&self, rng: &mut R) -> Result<u64, R::Error> {
        let mut successes = 0;
        whilst! { trial in 0..self.trials; {
            let ticket = rng.rand_try_below(self.probability.den())?;
            is! { ticket < self.probability.num(), successes += 1 }
        }}
        Ok(successes)
    }
    /// Samples the number of successes using the canonical const-capable [`Pcg32`].
    #[must_use]
    pub const fn sample_pcg32(&self, rng: &mut Pcg32) -> u64 {
        let mut successes = 0;
        whilst! { trial in 0..self.trials; {
            let ticket = rng.next_bounded_u64(self.probability.den());
            is! { ticket < self.probability.num(), successes += 1 }
        }}
        successes
    }
}

#[cfg(test)]
mod _test {
    use crate::{DistBernoulli, DistBinomial, Pcg32, Probability, RandFake, unwrap};

    const BERNOULLI_HALF: DistBernoulli =
        DistBernoulli::new(unwrap![some_expect Probability::new(1, 2), "valid probability"]);
    const BERNOULLI_CONST_SAMPLE: bool = {
        let mut rng = Pcg32::new(1, 2);
        BERNOULLI_HALF.sample_pcg32(&mut rng)
    };

    const BINOMIAL_HALF: DistBinomial =
        DistBinomial::new(4, unwrap![some_expect Probability::new(1, 2), "valid probability"]);
    const BINOMIAL_CONST_SAMPLE: u64 = {
        let mut rng = Pcg32::new(1, 2);
        BINOMIAL_HALF.sample_pcg32(&mut rng)
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
    #[test]
    fn binomial_constructs_exactly() {
        let dist = DistBinomial::new(7, Probability::new(2, 4).unwrap());
        assert_eq!(dist.trials, 7);
        assert_eq!(dist.probability, Probability::new(1, 2).unwrap());
    }
    #[test]
    fn binomial_zero_trials_needs_no_randomness() {
        type Rng<const N: usize> = RandFake<N, true>;
        let dist = DistBinomial::new(0, Probability::new(1, 2).unwrap());
        let mut rng = Rng::<0>::new([]);
        assert_eq!(dist.sample(&mut rng), 0);
    }
    #[test]
    fn binomial_counts_successes() {
        type Rng<const N: usize> = RandFake<N, true>;
        let dist = DistBinomial::new(4, Probability::new(1, 2).unwrap());
        let mut rng = Rng::new([0, 1, 2, 3]);
        assert_eq!(dist.sample_try(&mut rng).unwrap(), 2);
    }
    #[test]
    fn binomial_const_and_generic_pcg32_sampling_agree() {
        let mut rng = Pcg32::new(1, 2);
        assert_eq!(BINOMIAL_CONST_SAMPLE, BINOMIAL_HALF.sample(&mut rng));
        assert!(BINOMIAL_CONST_SAMPLE <= BINOMIAL_HALF.trials);
    }
}
