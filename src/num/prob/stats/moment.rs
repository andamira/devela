// devela/src/num/prob/stats/moment.rs
//
//! Defines [`StatsMoment`].
//

use crate::{_impl_init, is, unwrap};

#[doc = crate::_tags!(num)]
/// An online accumulator for the mean and variance.
#[doc = crate::_doc_meta!{
    location("num/prob/stats", struct StatsMoment),
    test_size_of(StatsMoment = 24|192; niche !Option),
}]
/// Values can be added one at a time without retaining the observations.
/// The accumulator keeps the observation count, running mean,
/// and the accumulated squared deviation $M_2$.
///
/// Independent accumulators can also be combined with [`merge`](#method.merge),
/// allowing the same statistics to be collected in separate batches.
///
/// Floating-point values follow normal `f64` arithmetic.
/// Non-finite inputs are accepted and may propagate to the results.
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StatsMoment {
    count: u64,
    mean: f64,
    m2: f64,
}

_impl_init! { Self::new() => StatsMoment }

impl Default for StatsMoment {
    fn default() -> Self {
        Self::new()
    }
}

impl StatsMoment {
    /// Creates an empty accumulator.
    #[must_use]
    pub const fn new() -> Self {
        Self { count: 0, mean: 0.0, m2: 0.0 }
    }
    /// Returns the number of incorporated observations.
    #[must_use]
    pub const fn count(&self) -> u64 {
        self.count
    }
    /// Returns whether no observations have been incorporated.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Returns the arithmetic mean, or `None` when empty.
    #[must_use]
    pub const fn mean(&self) -> Option<f64> {
        if self.count == 0 { None } else { Some(self.mean) }
    }
    /// Returns the accumulated squared deviation $M_2$.
    ///
    /// This is the unnormalized second central moment:
    /// $$
    /// M_2 = \sum_{i=1}^{n}(x_i-\mu)^2.
    /// $$
    ///
    /// It is zero for an empty accumulator and after one finite observation.
    #[must_use]
    pub const fn m2(&self) -> f64 {
        self.m2
    }
    /// Incorporates one observation.
    ///
    /// Uses Welford's online update, requiring no stored observations.
    ///
    /// # Formula
    /// For a new value $x$, with current count $n$ and mean $\mu$:
    /// $$
    /// n' = n + 1,\quad
    /// \delta = x-\mu,\quad
    /// \mu' = \mu+\frac{\delta}{n'},\quad
    /// M_2' = M_2+\delta(x-\mu').
    /// $$
    ///
    /// Returns `false` without modifying the accumulator if the observation
    /// count has already reached `u64::MAX`.
    #[must_use]
    pub const fn push(&mut self, value: f64) -> bool {
        let count = unwrap![some_or self.count.checked_add(1), return false];
        let delta = value - self.mean;
        let mean = self.mean + delta / count as f64;
        let delta2 = value - mean;
        self.count = count;
        self.mean = mean;
        self.m2 += delta * delta2;
        true
    }
    /// Returns the population variance, or `None` when empty.
    ///
    /// $$
    /// \sigma^2 = \frac{M_2}{n}.
    /// $$
    #[must_use]
    pub const fn variance_population(&self) -> Option<f64> {
        is! { self.count == 0, None, Some(self.m2 / self.count as f64) }
    }
    /// Returns the unbiased sample variance, or `None` for fewer than two observations.
    ///
    /// $$
    /// s^2 = \frac{M_2}{n-1}.
    /// $$
    #[must_use]
    pub const fn variance_sample(&self) -> Option<f64> {
        is! { self.count < 2, None, Some(self.m2 / (self.count - 1) as f64) }
    }
    /// Merges the observations accumulated in `other` into this accumulator.
    ///
    /// This combines the retained statistics directly, without replaying
    /// individual observations. An empty accumulator acts as an identity.
    ///
    /// # Formula
    /// For accumulators $a$ and $b$, let $\delta = \mu_b-\mu_a$ and $n=n_a+n_b$. Then:
    /// $$
    /// \mu = \mu_a + \delta\frac{n_b}{n},
    /// \qquad
    /// M_2 = M_{2,a} + M_{2,b} + \delta^2\frac{n_a n_b}{n}.
    /// $$
    ///
    /// Returns `false` without modifying the accumulator if the combined
    /// observation count would overflow `u64`.
    #[must_use]
    pub const fn merge(&mut self, other: Self) -> bool {
        is! { other.count == 0, return true }
        is! { self.count == 0, { *self = other; return true; } }
        let new_count = unwrap![some_or self.count.checked_add(other.count), return false];
        let count_a = self.count as f64;
        let count_b = other.count as f64;
        let count = new_count as f64;
        let delta = other.mean - self.mean;
        let mean = self.mean + delta * count_b / count;
        let m2 = self.m2 + other.m2 + delta * delta * count_a * count_b / count;
        self.count = new_count;
        self.mean = mean;
        self.m2 = m2;
        true
    }
}

#[cfg(test)]
mod _test {
    use crate::{StatsMoment, assert_approx_eq_all};

    const CONST_MOMENT: StatsMoment = {
        let mut stats = StatsMoment::new();
        let _ = stats.push(2.0);
        let _ = stats.push(4.0);
        let _ = stats.push(4.0);
        let _ = stats.push(4.0);
        let _ = stats.push(5.0);
        let _ = stats.push(5.0);
        let _ = stats.push(7.0);
        let _ = stats.push(9.0);
        stats
    };

    #[test]
    fn moment_starts_empty() {
        let stats = StatsMoment::new();
        assert_eq!(stats.count(), 0);
        assert!(stats.is_empty());
        assert_eq!(stats.mean(), None);
        assert_eq!(stats.m2(), 0.0);
        assert_eq!(stats.variance_population(), None);
        assert_eq!(stats.variance_sample(), None);
    }
    #[test]
    fn moment_accumulates_mean_and_second_moment() {
        assert_eq!(CONST_MOMENT.count(), 8);
        assert!(!CONST_MOMENT.is_empty());
        assert_eq!(CONST_MOMENT.mean(), Some(5.0));
        assert_eq!(CONST_MOMENT.m2(), 32.0);
    }
    #[test]
    fn moment_derives_population_and_sample_variance() {
        assert_eq!(CONST_MOMENT.variance_population(), Some(4.0));
        assert_approx_eq_all![
            tolerance: 1e-12_f64,
            CONST_MOMENT.variance_sample().unwrap(),
            32.0 / 7.0
        ];
    }
    #[test]
    fn moment_single_observation_has_zero_population_variance() {
        let mut stats = StatsMoment::new();
        assert!(stats.push(42.0));
        assert_eq!(stats.mean(), Some(42.0));
        assert_eq!(stats.m2(), 0.0);
        assert_eq!(stats.variance_population(), Some(0.0));
        assert_eq!(stats.variance_sample(), None);
    }
    #[test]
    fn moment_merge_combines_independent_batches() {
        let mut a = StatsMoment::new();
        for value in [2.0, 4.0, 4.0, 4.0] {
            assert!(a.push(value));
        }
        let mut b = StatsMoment::new();
        for value in [5.0, 5.0, 7.0, 9.0] {
            assert!(b.push(value));
        }
        assert!(a.merge(b));
        assert_eq!(a.count(), CONST_MOMENT.count());
        assert_eq!(a.mean(), CONST_MOMENT.mean());
        assert_eq!(a.m2(), CONST_MOMENT.m2());
        assert_eq!(a.variance_population(), CONST_MOMENT.variance_population());
        assert_approx_eq_all![
            tolerance: 1e-12_f64,
            a.variance_sample().unwrap(),
            CONST_MOMENT.variance_sample().unwrap()
        ];
    }
    #[test]
    fn moment_merge_treats_empty_as_identity() {
        let mut empty = StatsMoment::new();
        assert!(empty.merge(CONST_MOMENT));
        assert_eq!(empty, CONST_MOMENT);
        let mut stats = CONST_MOMENT;
        assert!(stats.merge(StatsMoment::new()));
        assert_eq!(stats, CONST_MOMENT);
    }
}
