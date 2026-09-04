// devela/src/data/topol/ord/seq/prev_next.rs
//
//! Defines [`SeqNext`], [`SeqPrevNext`].
//

use crate::ConstInit;

#[doc = crate::_tags!(data topol)]
/// A local forward succession relation in an ordered sequence topology.
#[doc = crate::_doc_meta! {
    location("data/topol/ord", struct SeqNext),
}]
/// `next` identifies the immediate successor, if any.
///
/// This type describes only local succession.
/// Target resolution, sequence membership, and sequence anchoring are external.
///
/// In particular, the absence of a successor does not distinguish
/// a final sequence member from a value not participating in a sequence.
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SeqNext<I> {
    /// The immediate successor, if any.
    pub next: Option<I>,
}

impl<I> SeqNext<I> {
    /// Creates a succession relation with no successor.
    pub const fn new() -> Self {
        Self { next: None }
    }

    /// Creates a succession relation from an optional successor.
    pub const fn from_next(next: Option<I>) -> Self {
        Self { next }
    }
    /// Returns the optional successor.
    pub fn into_next(self) -> Option<I> {
        self.next
    }

    /// Borrows the optional successor.
    pub const fn as_next(&self) -> &Option<I> {
        &self.next
    }
    /// Borrows the successor while preserving the relation structure.
    pub const fn as_ref(&self) -> SeqNext<&I> {
        SeqNext::from_next(self.next.as_ref())
    }
    /// Exclusively borrows the successor while preserving the relation structure.
    pub const fn as_mut(&mut self) -> SeqNext<&mut I> {
        SeqNext::from_next(self.next.as_mut())
    }

    /// Returns whether a successor is present.
    pub const fn has_next(&self) -> bool {
        self.next.is_some()
    }
}
impl<I: Copy> SeqNext<I> {
    /// Returns a copy of the optional successor.
    pub const fn copy_next(&self) -> Option<I> {
        self.next
    }
}

impl<I> Default for SeqNext<I> {
    fn default() -> Self {
        Self::new()
    }
}
impl<I> ConstInit for SeqNext<I> {
    const INIT: Self = Self::new();
}

#[doc = crate::_tags!(data topol)]
/// Local predecessor and successor relations in an ordered sequence topology.
#[doc = crate::_doc_meta! {
    location("data/topol/ord", struct SeqPrevNext),
}]
/// `prev` and `next` identify the immediate predecessor and successor,
/// respectively, when present.
///
/// This type describes only local succession.
/// It does not own or resolve targets, establish sequence membership,
/// maintain reciprocal links, or determine the sequence's boundaries.
///
/// In particular, the absence of both links does not distinguish
/// a singleton sequence member from a value not participating in a sequence.
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SeqPrevNext<I> {
    /// The immediate predecessor, if any.
    pub prev: Option<I>,

    /// The immediate successor, if any.
    pub next: Option<I>,
}

impl<I> SeqPrevNext<I> {
    /// Creates a relation with no predecessor or successor.
    pub const fn new() -> Self {
        Self { prev: None, next: None }
    }

    /// Creates a relation from optional predecessor and successor links.
    pub const fn from_parts(prev: Option<I>, next: Option<I>) -> Self {
        Self { prev, next }
    }
    /// Decomposes this relation into predecessor and successor links.
    pub fn into_parts(self) -> (Option<I>, Option<I>) {
        (self.prev, self.next)
    }

    /// Borrows the predecessor and successor links.
    pub const fn as_parts(&self) -> (&Option<I>, &Option<I>) {
        (&self.prev, &self.next)
    }
    /// Borrows both links while preserving the relation structure.
    pub const fn as_ref(&self) -> SeqPrevNext<&I> {
        SeqPrevNext::from_parts(self.prev.as_ref(), self.next.as_ref())
    }
    /// Exclusively borrows both links while preserving the relation structure.
    pub const fn as_mut(&mut self) -> SeqPrevNext<&mut I> {
        SeqPrevNext::from_parts(self.prev.as_mut(), self.next.as_mut())
    }

    /// Returns whether a predecessor is present.
    pub const fn has_prev(&self) -> bool {
        self.prev.is_some()
    }
    /// Returns whether a successor is present.
    pub const fn has_next(&self) -> bool {
        self.next.is_some()
    }
}
impl<I: Copy> SeqPrevNext<I> {
    /// Returns copies of the predecessor and successor links.
    pub const fn copy_parts(&self) -> (Option<I>, Option<I>) {
        (self.prev, self.next)
    }
}

impl<I> Default for SeqPrevNext<I> {
    fn default() -> Self {
        Self::new()
    }
}
impl<I> ConstInit for SeqPrevNext<I> {
    const INIT: Self = Self::new();
}
