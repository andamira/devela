// devela/src/lang/sem/rel/relation.rs
//
//! Defines [`Relation`].
//

use crate::ConstInit;

#[doc = crate::_tags!(lang)]
/// A semantic triple of subject, predicate, and object.
#[doc = crate::_doc_meta! {
    location("lang/sem", struct Relation),
}]
/// `Relation` contains only its three semantic components.
/// Identity, storage, interpretation, evidence, qualification, and traversal
/// are external to the relation itself.
///
/// The component types are independent, and may coincide when all three
/// belong to the same semantic domain.
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Relation<S, P = S, O = S> {
    /// The subject of the relation.
    pub subject: S,
    /// The predicate relating the subject and object.
    pub predicate: P,
    /// The object of the relation.
    pub object: O,
}

impl<S, P, O> Relation<S, P, O> {
    /// Creates a semantic relation.
    pub const fn new(subject: S, predicate: P, object: O) -> Self {
        Self { subject, predicate, object }
    }
    /// Borrows the subject, predicate, and object.
    #[must_use]
    pub const fn as_parts(&self) -> (&S, &P, &O) {
        (&self.subject, &self.predicate, &self.object)
    }

    /// Decomposes the relation into its subject, predicate, and object.
    #[must_use]
    pub fn into_parts(self) -> (S, P, O) {
        (self.subject, self.predicate, self.object)
    }
}

impl<S: ConstInit, P: ConstInit, O: ConstInit> ConstInit for Relation<S, P, O> {
    const INIT: Self = Relation::new(S::INIT, P::INIT, O::INIT);
}

#[cfg(test)]
mod _test {
    use super::*;

    #[test]
    fn parts() {
        let relation = Relation::new("lamp", "powered_by", "battery");
        assert_eq!(relation.as_parts(), (&"lamp", &"powered_by", &"battery"),);
        assert_eq!(relation.into_parts(), ("lamp", "powered_by", "battery"),);
    }
    #[test]
    fn component_types() {
        let relation: Relation<u8, char, bool> = Relation::new(1, '=', true);
        assert_eq!(relation.into_parts(), (1, '=', true));
    }
    #[test]
    fn homogeneous_type() {
        let relation: Relation<u8> = Relation::new(1, 2, 3);
        assert_eq!(relation.into_parts(), (1, 2, 3));
    }
}
