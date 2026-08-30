// devela/src/num/prob/markov/kernel.rs
//
//! Defines [`MarkovKernel`].
//

use crate::{DistCategorical, Probability, is, unwrap, whilst};

#[doc = crate::_tags!(num)]
/// A finite-state Markov transition kernel.
#[doc = crate::_doc_meta!{
    location("num/prob/markov", struct MarkovKernel),
    #[cfg(target_pointer_width = "32")]
    test_size_of(MarkovKernel = 8|64; niche Option),
    #[cfg(target_pointer_width = "64")]
    test_size_of(MarkovKernel = 16|128; niche Option),
}]
/// Each state has one categorical distribution over the possible next
/// states. All rows therefore contain the same number of categories as
/// there are states.
///
/// The kernel borrows its transition distributions and performs no
/// allocation.
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MarkovKernel<'a> {
    transitions: &'a [DistCategorical<'a>],
}

impl<'a> MarkovKernel<'a> {
    /// Constructs a transition kernel from one distribution per state.
    ///
    /// Returns `None` if there are no states or if any distribution has
    /// a different number of categories from the number of states.
    #[must_use]
    pub const fn new(transitions: &'a [DistCategorical<'a>]) -> Option<Self> {
        let state_count = transitions.len();
        is! { state_count == 0, return None }
        whilst! { state in 0..state_count; {
            is! { transitions[state].category_count() != state_count, return None }
        }}
        Some(Self { transitions })
    }
    /// Returns the number of states.
    #[must_use]
    pub const fn state_count(&self) -> usize {
        self.transitions.len()
    }
    /// Returns all transition distributions in state order.
    #[must_use]
    pub const fn transitions(&self) -> &'a [DistCategorical<'a>] {
        self.transitions
    }
    /// Returns the next-state distribution for `state`.
    #[must_use]
    pub const fn transitions_from(&self, state: usize) -> Option<DistCategorical<'a>> {
        is! { state < self.transitions.len(), Some(self.transitions[state]), None }
    }
    /// Returns the exact transition probability from one state to another.
    #[must_use]
    pub const fn probability(&self, from: usize, to: usize) -> Option<Probability> {
        let transitions = unwrap![some? self.transitions_from(from)];
        transitions.probability_of(to)
    }
}

#[cfg(test)]
mod _test {
    use crate::{DistCategorical, MarkovKernel, Probability};

    #[test]
    fn rejects_empty_and_non_square_kernels() {
        assert_eq!(MarkovKernel::new(&[]), None);
        let a = DistCategorical::new(&[1, 1]).unwrap();
        let b = DistCategorical::new(&[1, 1, 1]).unwrap();
        assert_eq!(MarkovKernel::new(&[a, b]), None);
    }
    #[test]
    fn exposes_transition_structure() {
        let a = DistCategorical::new(&[1, 3]).unwrap();
        let b = DistCategorical::new(&[2, 2]).unwrap();
        let transitions = [a, b];
        let kernel = MarkovKernel::new(&transitions).unwrap();
        assert_eq!(kernel.state_count(), 2);
        assert_eq!(kernel.transitions(), &transitions);
        assert_eq!(kernel.transitions_from(0), Some(a));
        assert_eq!(kernel.transitions_from(1), Some(b));
        assert_eq!(kernel.transitions_from(2), None);
    }
    #[test]
    fn exposes_exact_transition_probabilities() {
        let transitions =
            [DistCategorical::new(&[1, 3]).unwrap(), DistCategorical::new(&[2, 2]).unwrap()];
        let kernel = MarkovKernel::new(&transitions).unwrap();
        assert_eq!(kernel.probability(0, 0), Probability::new(1, 4));
        assert_eq!(kernel.probability(0, 1), Probability::new(3, 4));
        assert_eq!(kernel.probability(1, 0), Probability::new(1, 2));
        assert_eq!(kernel.probability(1, 1), Probability::new(1, 2));
        assert_eq!(kernel.probability(2, 0), None);
        assert_eq!(kernel.probability(0, 2), None);
    }
}
