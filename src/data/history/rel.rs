// devela/src/data/history/rel.rs
//
#![doc = crate::_DOC_DATA_HISTORY_REL!()] // private
#![doc = crate::_doc!(modules: crate::data::history; rel)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//

use crate::ConstInit;
#[cfg(doc)]
use crate::Relation;

#[doc = crate::_tags!(data)]
/// A historical predicate stating that the subject was derived from the object.
#[doc = crate::_doc_meta! {
    location("data/history", struct DerivedFrom),
}]
/// Used as the predicate of a [`Relation`], the subject is the resulting datum
/// and the object is its antecedent. `via` may identify or describe the rule,
/// process, operation, or event through which the derivation occurred.
///
/// With `V = ()`, only the derivation relation itself is recorded.
///
/// # Example
/// ```
/// use devela::{DerivedFrom, Relation};
///
/// let derivation = Relation {
///     subject: "output.dvbf",
///     predicate: DerivedFrom::with("BDF-to-DVBF"),
///     object: "input.bdf",
/// };
/// assert_eq!(derivation.predicate.via, "BDF-to-DVBF");
/// ```
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DerivedFrom<V = ()> {
    /// The rule, process, operation, or event through which the derivation occurred.
    pub via: V,
}
impl DerivedFrom {
    /// Creates an unqualified derivation predicate.
    pub const fn new() -> Self {
        Self { via: () }
    }
}
impl<V> DerivedFrom<V> {
    /// Creates a derivation predicate qualified by `via`.
    pub const fn with(via: V) -> Self {
        Self { via }
    }
}
impl<V: ConstInit> ConstInit for DerivedFrom<V> {
    const INIT: Self = DerivedFrom::with(V::INIT);
}
impl<V: Default> Default for DerivedFrom<V> {
    fn default() -> Self {
        Self::with(V::default())
    }
}

#[doc = crate::_tags!(data)]
/// A historical predicate stating that the subject is a revision of the object.
#[doc = crate::_doc_meta! {
    location("data/history", struct RevisionOf),
}]
/// A revision preserves historical continuity with its antecedent while
/// allowing its contents or representation to change. `via` may identify
/// or describe the change, operation, or event that produced the revision.
///
/// With `V = ()`, only the revision relation itself is recorded.
///
/// # Example
/// ```
/// use devela::{Relation, RevisionOf};
///
/// let revision = Relation {
///     subject: 3,
///     predicate: RevisionOf::new(),
///     object: 2,
/// };
/// assert_eq!(revision.object, 2);
/// ```
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RevisionOf<V = ()> {
    /// The change, operation, event, or cause associated with the transition.
    pub via: V,
}
impl RevisionOf {
    /// Creates an unqualified revision predicate.
    pub const fn new() -> Self {
        Self { via: () }
    }
}
impl<V> RevisionOf<V> {
    /// Creates a revision predicate qualified by `via`.
    pub const fn with(via: V) -> Self {
        Self { via }
    }
}
impl<V: ConstInit> ConstInit for RevisionOf<V> {
    const INIT: Self = RevisionOf::with(V::INIT);
}
impl<V: Default> Default for RevisionOf<V> {
    fn default() -> Self {
        Self::with(V::default())
    }
}
